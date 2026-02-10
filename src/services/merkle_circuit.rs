use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, P128Pow5T3},
    Hash, Pow5Chip, Pow5Config,
};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Expression, Instance, Selector},
    poly::Rotation,
};

pub const DEPTH: usize = 2;

#[derive(Clone, Debug)]
pub struct MerkleCircuit {
    /// Private leaf value
    pub leaf: Value<Fp>,

    /// Merkle path siblings (one per level)
    /// For a tree of arbitrary depth, this array stores one sibling per level.
    /// The circuit hashes the current value with each sibling in sequence,
    /// moving up the tree until reaching the root.
    pub siblings: [Value<Fp>; DEPTH],

    /// Direction bits (0 = cur is left, 1 = cur is right)
    /// These bits determine the order of hashing at each level:
    /// - If dir = 0: hash(cur, sibling) - current value is on the left
    /// - If dir = 1: hash(sibling, cur) - current value is on the right
    ///
    /// The circuit enforces:
    /// 1. Each direction bit must be binary (0 or 1)
    /// 2. left = cur * (1 - dir) + sibling * dir
    /// 3. right = cur * dir + sibling * (1 - dir)
    pub directions: [Value<Fp>; DEPTH],
}

#[derive(Clone, Debug)]
pub struct MerkleConfig {
    advice: Column<Advice>,
    pub instance: Column<Instance>,
    poseidon: Pow5Config<Fp, 3, 2>,

    //columns for conditional swap logic
    //we need columns for: current, sibling, direction, left, right
    swap_current: Column<Advice>,
    swap_sibling: Column<Advice>,
    swap_direction: Column<Advice>,
    swap_left: Column<Advice>,
    swap_right: Column<Advice>,

    //selector to enable the swap constraints
    swap_selector: Selector,
}

impl Circuit<Fp> for MerkleCircuit {
    type Config = MerkleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            leaf: Value::unknown(),
            siblings: [Value::unknown(); DEPTH],
            directions: [Value::unknown(); DEPTH],
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();

        //allow equality constraints / copying between cells
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        //poseidon config - needs state columns for hashing
        let state = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        let partial_sbox = meta.advice_column();
        meta.enable_equality(partial_sbox);

        //enable equality on state columns so we can copy values between regions
        for col in &state {
            meta.enable_equality(*col);
        }

        //round constants need many fixed columns for P128Pow5T3
        //allocate enough columns to store all round constants
        let rc_a = [
            meta.fixed_column(),
            meta.fixed_column(),
            meta.fixed_column(),
        ];
        let rc_b = [
            meta.fixed_column(),
            meta.fixed_column(),
            meta.fixed_column(),
        ];

        //mark these columns as constant columns
        meta.enable_constant(rc_a[0]);
        meta.enable_constant(rc_a[1]);
        meta.enable_constant(rc_a[2]);
        meta.enable_constant(rc_b[0]);
        meta.enable_constant(rc_b[1]);
        meta.enable_constant(rc_b[2]);

        let poseidon =
            Pow5Chip::<Fp, 3, 2>::configure::<P128Pow5T3>(meta, state, partial_sbox, rc_a, rc_b);

        //columns for conditional swap based on direction bit
        let swap_current = meta.advice_column();
        let swap_sibling = meta.advice_column();
        let swap_direction = meta.advice_column();
        let swap_left = meta.advice_column();
        let swap_right = meta.advice_column();

        //enable equality on swap columns so we can copy values
        meta.enable_equality(swap_current);
        meta.enable_equality(swap_sibling);
        meta.enable_equality(swap_direction);
        meta.enable_equality(swap_left);
        meta.enable_equality(swap_right);

        let swap_selector = meta.selector();

        //create custom gate for conditional swap
        //when selector is enabled, enforce:
        //  1. dir * (1 - dir) = 0  (direction must be 0 or 1)
        //  2. left = cur * (1 - dir) + sibling * dir
        //  3. right = cur * dir + sibling * (1 - dir)
        meta.create_gate("conditional swap", |meta| {
            let s = meta.query_selector(swap_selector);
            let cur = meta.query_advice(swap_current, Rotation::cur());
            let sibling = meta.query_advice(swap_sibling, Rotation::cur());
            let dir = meta.query_advice(swap_direction, Rotation::cur());
            let left = meta.query_advice(swap_left, Rotation::cur());
            let right = meta.query_advice(swap_right, Rotation::cur());

            vec![
                //constraint 1: dir must be binary (0 or 1)
                //dir * (1 - dir) = 0
                s.clone() * dir.clone() * (Expression::Constant(Fp::one()) - dir.clone()),

                //constraint 2: left = cur * (1 - dir) + sibling * dir
                s.clone()
                    * (left
                        - (cur.clone() * (Expression::Constant(Fp::one()) - dir.clone())
                            + sibling.clone() * dir.clone())),

                //constraint 3: right = cur * dir + sibling * (1 - dir)
                s * (right
                    - (cur * dir.clone()
                        + sibling * (Expression::Constant(Fp::one()) - dir))),
            ]
        });

        MerkleConfig {
            advice,
            instance,
            poseidon,
            swap_current,
            swap_sibling,
            swap_direction,
            swap_left,
            swap_right,
            swap_selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> std::result::Result<(), plonk::Error> {
        //assign the leaf value as the starting point
        let mut cur_cell = layouter.assign_region(
            || "assign leaf",
            |mut region| region.assign_advice(|| "leaf", config.advice, 0, || self.leaf),
        )?;

        //iterate through each level of the tree, from leaf to root
        for i in 0..DEPTH {
            //perform conditional swap based on direction bit
            //this region assigns all values and enables the swap constraint
            let (left_cell, right_cell) = layouter.assign_region(
                || format!("conditional swap level {}", i),
                |mut region| {
                    //enable the swap selector
                    config.swap_selector.enable(&mut region, 0)?;

                    //copy the current value into the swap region
                    let _cur_copy = region.assign_advice(
                        || format!("cur {}", i),
                        config.swap_current,
                        0,
                        || cur_cell.value().copied(),
                    )?;

                    //assign the sibling value
                    let _sibling = region.assign_advice(
                        || format!("sibling {}", i),
                        config.swap_sibling,
                        0,
                        || self.siblings[i],
                    )?;

                    //assign the direction bit
                    let _dir = region.assign_advice(
                        || format!("dir {}", i),
                        config.swap_direction,
                        0,
                        || self.directions[i],
                    )?;

                    //compute and assign left = cur * (1 - dir) + sibling * dir
                    let left_val = cur_cell
                        .value()
                        .zip(self.siblings[i])
                        .zip(self.directions[i])
                        .map(|((c, s), d)| c * (Fp::one() - d) + s * d);
                    let left = region.assign_advice(
                        || format!("left {}", i),
                        config.swap_left,
                        0,
                        || left_val,
                    )?;

                    //compute and assign right = cur * dir + sibling * (1 - dir)
                    let right_val = cur_cell
                        .value()
                        .zip(self.siblings[i])
                        .zip(self.directions[i])
                        .map(|((c, s), d)| c * d + s * (Fp::one() - d));
                    let right = region.assign_advice(
                        || format!("right {}", i),
                        config.swap_right,
                        0,
                        || right_val,
                    )?;

                    //the custom gate will automatically check:
                    // - dir is 0 or 1
                    // - left is computed correctly
                    // - right is computed correctly

                    Ok((left, right))
                },
            )?;

            //initialize the Poseidon hasher for this level
            let hasher = Hash::<_, _, P128Pow5T3, ConstantLength<2>, 3, 2>::init(
                Pow5Chip::<Fp, 3, 2>::construct(config.poseidon.clone()),
                layouter.namespace(|| format!("init hasher {}", i)),
            )?;

            //hash left and right to compute the parent node
            //the direction bit ensures we hash in the correct order
            cur_cell = hasher.hash(
                layouter.namespace(|| format!("hash level {}", i)),
                [left_cell, right_cell],
            )?;
        }

        //constrain the final hash (root) to equal the public input
        layouter.constrain_instance(cur_cell.cell(), config.instance, 0)?;

        Ok(())
    }
}

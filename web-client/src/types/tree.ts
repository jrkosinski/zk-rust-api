/**
 * Response from adding a value to the tree or registering a commitment
 */
export interface TreeResponse {
    data: string;
}

/**
 * Response from visualizing the tree
 */
export interface TreeVisualizationResponse {
    image_url: string;
}

/**
 * Request to add a raw value to the tree (legacy / debug use)
 */
export interface AddToTreeRequest {
    value: number;
}

/**
 * Request to register a Poseidon commitment in the tree.
 * The client computes commitment = Poseidon(secret) locally and sends only the commitment.
 */
export interface RegisterRequest {
    commitment: string; // 64-char hex Fp field element
}

/**
 * Request to generate a ZK proof for a secret
 */
export interface ZKProofRequest {
    secret: number;
}

/**
 * Response from the ZK proof endpoint
 */
export interface ZKProofResponse {
    proof: boolean;
}

import { useState } from 'react';
import { useMutation, useQuery } from '@tanstack/react-query';
import { treeApi, zkApi } from '../lib/api';
import { queryClient } from '../lib/queryClient';

// ─── Register Panel ──────────────────────────────────────────────────────────

/**
 * Panel for registering a new secret's commitment in the Merkle tree.
 * NOTE: In a real client the Poseidon hash would be computed locally in WASM.
 * For now, the user enters a secret and the server receives it to compute the
 * commitment server-side via the /zk endpoint instead. This panel demonstrates
 * the registration flow by accepting a raw commitment hex string directly.
 */
function RegisterPanel() {
    const [commitment, setCommitment] = useState('');
    const [error, setError] = useState<string | null>(null);

    const mutation = useMutation({
        mutationFn: () => treeApi.register({ commitment }),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['tree-visualization'] });
            setCommitment('');
            setError(null);
        },
        onError: (err: Error) => setError(err.message),
    });

    const handleSubmit = (e: React.SyntheticEvent) => {
        e.preventDefault();
        if (!commitment.match(/^[0-9a-fA-F]{64}$/)) {
            setError('commitment must be exactly 64 hex characters (32 bytes)');
            return;
        }
        mutation.mutate();
    };

    return (
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-1">Register Commitment</h2>
            <p className="text-sm text-gray-500 mb-4">
                Paste a 64-char hex Poseidon commitment (= Poseidon(secret)).
                The server stores the commitment — your secret never leaves this browser.
            </p>

            <form onSubmit={handleSubmit} className="flex gap-3 mb-3">
                <input
                    type="text"
                    value={commitment}
                    onChange={(e) => setCommitment(e.target.value)}
                    placeholder="64-char hex commitment"
                    className="flex-1 px-4 py-2 border border-gray-300 rounded-md font-mono text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                    disabled={mutation.isPending}
                />
                <button
                    type="submit"
                    disabled={mutation.isPending}
                    className="px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
                >
                    {mutation.isPending ? 'Registering…' : 'Register'}
                </button>
            </form>

            {error && <StatusBanner type="error" message={error} />}
            {mutation.isSuccess && (
                <StatusBanner type="success" message={`Registered! New root: ${mutation.data.data}`} />
            )}
        </div>
    );
}

// ─── Prove Panel ─────────────────────────────────────────────────────────────

/**
 * Panel for generating a ZK proof that the user knows a registered secret.
 * Seed secrets pre-loaded in the tree: 42, 99, 7, 13, 55, 77, 100, 200.
 */
function ProvePanel() {
    const [secret, setSecret] = useState('');
    const [error, setError] = useState<string | null>(null);

    const mutation = useMutation({
        mutationFn: () => zkApi.prove({ secret: parseInt(secret, 10) }),
        onError: (err: Error) => setError(err.message),
        onSuccess: () => setError(null),
    });

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const val = parseInt(secret, 10);
        if (isNaN(val)) {
            setError('please enter a valid integer secret');
            return;
        }
        mutation.mutate();
    };

    return (
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-1">Prove Membership</h2>
            <p className="text-sm text-gray-500 mb-4">
                Enter your secret. The server proves <code>Poseidon(secret)</code> is in the tree
                without revealing which commitment matches.
                Try one of the seed secrets: <code>42, 99, 7, 13, 55, 77, 100, 200</code>.
            </p>

            <form onSubmit={handleSubmit} className="flex gap-3 mb-3">
                <input
                    type="number"
                    value={secret}
                    onChange={(e) => setSecret(e.target.value)}
                    placeholder="Enter your secret"
                    className="flex-1 px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500"
                    disabled={mutation.isPending}
                />
                <button
                    type="submit"
                    disabled={mutation.isPending}
                    className="px-6 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
                >
                    {mutation.isPending ? 'Proving…' : 'Prove'}
                </button>
            </form>

            {error && <StatusBanner type="error" message={error} />}
            {mutation.isSuccess && (
                <StatusBanner
                    type={mutation.data.proof ? 'success' : 'error'}
                    message={mutation.data.proof
                        ? 'Proof valid — your secret is in the allowlist.'
                        : 'Proof failed — secret not found in the allowlist.'}
                />
            )}
        </div>
    );
}

// ─── Tree Visualization Panel ─────────────────────────────────────────────────

function TreePanel() {
    const { data: visualization, isLoading } = useQuery({
        queryKey: ['tree-visualization'],
        queryFn: () => treeApi.visualize(),
        refetchInterval: false,
    });

    const handleRefresh = () =>
        queryClient.invalidateQueries({ queryKey: ['tree-visualization'] });

    return (
        <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold text-gray-900">Tree Structure</h2>
                <button
                    onClick={handleRefresh}
                    disabled={isLoading}
                    className="px-4 py-1.5 bg-blue-600 text-white text-sm rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
                >
                    {isLoading ? 'Refreshing…' : 'Refresh'}
                </button>
            </div>

            {isLoading && (
                <div className="flex justify-center items-center py-12 text-gray-500">
                    Loading visualization…
                </div>
            )}

            {visualization && !isLoading && (
                <div className="overflow-auto">
                    <img
                        src={visualization.image_url}
                        alt="Merkle Tree Visualization"
                        className="max-w-full h-auto"
                        key={visualization.image_url}
                    />
                </div>
            )}
        </div>
    );
}

// ─── Shared helpers ───────────────────────────────────────────────────────────

function StatusBanner({ type, message }: { type: 'success' | 'error'; message: string }) {
    const colours = type === 'success'
        ? 'bg-green-100 border-green-400 text-green-700'
        : 'bg-red-100 border-red-400 text-red-700';
    return (
        <div className={`p-3 border rounded-md text-sm ${colours}`}>
            {message}
        </div>
    );
}

// ─── Root component ───────────────────────────────────────────────────────────

export function TreeVisualization() {
    return (
        <div className="min-h-screen bg-gray-50 p-8">
            <div className="max-w-4xl mx-auto">
                <div className="mb-8">
                    <h1 className="text-3xl font-bold text-gray-900 mb-1">
                        Anonymous Allowlist — ZK Proof Demo
                    </h1>
                    <p className="text-gray-500 text-sm">
                        Register a commitment, then prove membership without revealing your secret.
                    </p>
                </div>

                <RegisterPanel />
                <ProvePanel />
                <TreePanel />
            </div>
        </div>
    );
}

/**
 * API client for zk-rust-api
 * Base URL is proxied through Vite dev server to avoid CORS issues
 */

import type {
    TreeResponse,
    TreeVisualizationResponse,
    AddToTreeRequest,
    RegisterRequest,
    ZKProofRequest,
    ZKProofResponse,
} from '../types/tree';

const API_BASE = '/api';

async function fetchJson<T>(url: string, options?: RequestInit): Promise<T> {
    const response = await fetch(url, {
        ...options,
        headers: {
            'Content-Type': 'application/json',
            ...options?.headers,
        },
    });

    if (!response.ok) {
        throw new Error(`API error: ${response.statusText}`);
    }

    return response.json();
}

// Merkle Tree API
export const treeApi = {
    /**
     * Add a raw value to the Merkle tree (legacy / debug use)
     */
    addValue: (request: AddToTreeRequest) =>
        fetchJson<TreeResponse>(`${API_BASE}/tree`, {
            method: 'POST',
            body: JSON.stringify(request),
        }),

    /**
     * Register a Poseidon commitment in the tree.
     * The client should compute commitment = Poseidon(secret) locally.
     */
    register: (request: RegisterRequest) =>
        fetchJson<TreeResponse>(`${API_BASE}/register`, {
            method: 'POST',
            body: JSON.stringify(request),
        }),

    /**
     * Generate a visualization of the current tree
     */
    visualize: () =>
        fetchJson<TreeVisualizationResponse>(`${API_BASE}/tree/visualize`),
};

// ZK Proof API
export const zkApi = {
    /**
     * Prove knowledge of a secret whose Poseidon commitment is in the tree.
     * Returns { proof: true } if the ZK circuit verifies successfully.
     */
    prove: (request: ZKProofRequest) =>
        fetchJson<ZKProofResponse>(`${API_BASE}/zk`, {
            method: 'POST',
            body: JSON.stringify(request),
        }),
};

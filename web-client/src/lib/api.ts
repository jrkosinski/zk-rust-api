/**
 * API client for zk-rust-api
 * Base URL is proxied through Vite dev server to avoid CORS issues
 */

import type {
    TreeResponse,
    TreeVisualizationResponse,
    AddToTreeRequest,
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
     * Add a value to the Merkle tree
     */
    addValue: (request: AddToTreeRequest) =>
        fetchJson<TreeResponse>(`${API_BASE}/tree`, {
            method: 'POST',
            body: JSON.stringify(request),
        }),

    /**
     * Generate a visualization of the current tree
     */
    visualize: () =>
        fetchJson<TreeVisualizationResponse>(`${API_BASE}/tree/visualize`),
};

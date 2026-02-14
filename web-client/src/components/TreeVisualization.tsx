import { useState } from 'react';
import { useMutation, useQuery } from '@tanstack/react-query';
import { treeApi } from '../lib/api';
import { queryClient } from '../lib/queryClient';

export function TreeVisualization() {
    const [inputValue, setInputValue] = useState('');
    const [error, setError] = useState<string | null>(null);

    // Query to fetch tree visualization
    const { data: visualization, isLoading } = useQuery({
        queryKey: ['tree-visualization'],
        queryFn: () => treeApi.visualize(),
        refetchInterval: false,
    });

    // Mutation to add value to tree
    const addValueMutation = useMutation({
        mutationFn: (value: number) => treeApi.addValue({ value }),
        onSuccess: () => {
            // Refetch visualization after adding value
            queryClient.invalidateQueries({ queryKey: ['tree-visualization'] });
            setInputValue('');
            setError(null);
        },
        onError: (err: Error) => {
            setError(err.message);
        },
    });

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const value = parseInt(inputValue, 10);

        if (isNaN(value)) {
            setError('Please enter a valid number');
            return;
        }

        addValueMutation.mutate(value);
    };

    const handleRefresh = () => {
        queryClient.invalidateQueries({ queryKey: ['tree-visualization'] });
    };

    return (
        <div className="min-h-screen bg-gray-50 p-8">
            <div className="max-w-6xl mx-auto">
                <div className="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h1 className="text-3xl font-bold text-gray-900 mb-2">
                        Merkle Tree Visualization
                    </h1>
                    <p className="text-gray-600 mb-6">
                        Add values to the tree and visualize the structure with Poseidon hashing
                    </p>

                    {/* Add Value Form */}
                    <form onSubmit={handleSubmit} className="flex gap-3 mb-4">
                        <input
                            type="number"
                            value={inputValue}
                            onChange={(e) => setInputValue(e.target.value)}
                            placeholder="Enter a number to add to the tree"
                            className="flex-1 px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            disabled={addValueMutation.isPending}
                        />
                        <button
                            type="submit"
                            disabled={addValueMutation.isPending}
                            className="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
                        >
                            {addValueMutation.isPending ? 'Adding...' : 'Add Value'}
                        </button>
                        <button
                            type="button"
                            onClick={handleRefresh}
                            disabled={isLoading}
                            className="px-6 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
                        >
                            {isLoading ? 'Refreshing...' : 'Refresh'}
                        </button>
                    </form>

                    {error && (
                        <div className="p-3 bg-red-100 border border-red-400 text-red-700 rounded-md mb-4">
                            {error}
                        </div>
                    )}

                    {addValueMutation.isSuccess && (
                        <div className="p-3 bg-green-100 border border-green-400 text-green-700 rounded-md mb-4">
                            Value added successfully! New root hash:{' '}
                            <code className="font-mono text-sm break-all">
                                {addValueMutation.data.data}
                            </code>
                        </div>
                    )}
                </div>

                {/* Tree Visualization */}
                <div className="bg-white rounded-lg shadow-md p-6">
                    <h2 className="text-xl font-semibold text-gray-900 mb-4">
                        Tree Structure
                    </h2>

                    {isLoading && (
                        <div className="flex justify-center items-center py-12">
                            <div className="text-gray-600">Loading visualization...</div>
                        </div>
                    )}

                    {visualization && !isLoading && (
                        <div className="overflow-auto">
                            <img
                                src={visualization.image_url}
                                alt="Merkle Tree Visualization"
                                className="max-w-full h-auto"
                                key={visualization.image_url} // Force reload on URL change
                            />
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}

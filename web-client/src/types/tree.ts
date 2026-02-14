/**
 * Response from adding a value to the tree
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
 * Request to add a value to the tree
 */
export interface AddToTreeRequest {
    value: number;
}

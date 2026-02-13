use rust_api::prelude::*;

/// Response type for the tree endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreeResponse {
    pub data: String,
}

pub struct TreeService {
    //state here
}

impl Injectable for TreeService {}

impl TreeService {
    pub fn new() -> Self {
        Self {
            //initialize dependencies here
        }
    }

    pub fn add_to_tree(&self) -> TreeResponse {
        TreeResponse {
            data: "ok".to_string(),
        }
    }
}

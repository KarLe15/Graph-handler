use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UploadGraphParams {
    pub graphName: String,
    pub graphFile: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteGraphParams {
    pub graph_name: String,
}

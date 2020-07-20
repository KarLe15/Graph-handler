use serde::Deserialize;

#[derive(Deserialize)]
pub struct UploadGraphParams {
    pub graphName: String,
    pub graphFile: String,
}
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PutThumbnail {
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub file: Vec<u8>,
}

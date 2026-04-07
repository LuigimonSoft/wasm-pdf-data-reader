use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PdfPageViewport {
    pub page: u32,
    pub width: f64,
    pub height: f64,
}

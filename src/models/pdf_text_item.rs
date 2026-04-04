use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PdfTextItem {
    pub page: u32,
    pub text: String,
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub transform: Vec<f64>,
}

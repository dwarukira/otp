use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub secret: String,
}
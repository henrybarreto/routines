use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Condition {
    Less,
    Great,
    Equal,
}

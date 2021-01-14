use serde::{Deserialize, Serialize};

pub mod routines;
pub mod routine_logs;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum JSNumberType {
    Float(f64),
    Str(Option<String>),
}

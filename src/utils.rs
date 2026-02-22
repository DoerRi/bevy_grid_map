use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Display)]
pub enum FileFormat {
    Json,
    #[default]
    Csv,
}

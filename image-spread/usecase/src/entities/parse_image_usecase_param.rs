use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ParseImageUseCaseParam {
    pub grid_width: u32,
    pub path: String,
}

use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct BNum(String);
impl BNum {
    pub fn new(bnum: &str) -> Self {
        Self(bnum.to_string())
    }
}
impl Deref for BNum {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

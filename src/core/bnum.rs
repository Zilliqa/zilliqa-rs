use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
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

impl FromStr for BNum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

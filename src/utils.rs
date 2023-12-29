use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum TodoStatus {
    Done,
    NotDone,
    InProgress,
}

impl Display for TodoStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub uid: String,
    pub text: String,
    pub status: String,
    pub topic: Vec<String>,
}

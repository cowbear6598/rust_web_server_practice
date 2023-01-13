pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: u16,
    pub message: String,
    pub data: T
}
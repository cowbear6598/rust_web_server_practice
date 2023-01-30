use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wheel {
    pub wheel_id: u16,
    pub prizes: Vec<Prize>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prize {
    pub name: String,
    pub icon_url: String,
    pub count: u16
}


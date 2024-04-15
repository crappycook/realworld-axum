use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddPlayerReq {
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Debug)]
pub struct AddPlayerResp {
    pub success: bool,
}

#[derive(Serialize, Debug)]
pub struct PlayerResp {
    pub id: u64,
    pub name: String,
    pub club: String,
}

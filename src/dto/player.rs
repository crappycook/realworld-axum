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

#[derive(Debug, Deserialize)]
pub struct UpdatePlayerReq {
    pub id: u64,
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Debug)]
pub struct UpdatePlayerResp {
    pub success: bool,
}

#[derive(Serialize, Debug)]
pub struct PlayerResp {
    pub id: u64,
    pub name: String,
    pub club: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryPlayerReq {
    pub name: Option<String>,
    pub club: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct PlayerItem {
    pub id: u64,
    pub name: String,
    pub club: String,
    pub created_at: i64,
}

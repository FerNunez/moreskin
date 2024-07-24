use serde::Deserialize;


#[derive(Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkinDataBase {
    pub champion_id: u64,
    pub is_base: bool,
    pub id: u64,
    pub name: String,
    pub ownership: Ownership,
    pub splash_path: String,
    pub tile_path: String,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ownership {
    pub owned: bool,
}

#[derive(Clone, PartialEq)]
pub struct Inventory {
    pub owned: Vec<SkinDataBase>,
    pub loot: Vec<SkinDataBase>,
    pub unowned: Vec<SkinDataBase>,
}

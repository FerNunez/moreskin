use serde::Deserialize;

#[derive(Debug)]
enum SortBy {
    Name,
    Value,
    Category,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum DisplayCategory {
    SKIN,
    CHAMPION,
    CHEST,
    ETERNALS,
    UNKNOWN,
}

impl<'de> Deserialize<'de> for DisplayCategory {
    fn deserialize<D>(deserializer: D) -> Result<DisplayCategory, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "SKIN" => Ok(DisplayCategory::SKIN),
            "CHAMPION" => Ok(DisplayCategory::CHAMPION),
            "CHEST" => Ok(DisplayCategory::CHEST),
            "ETERNALS" => Ok(DisplayCategory::ETERNALS),
            _ => Ok(DisplayCategory::UNKNOWN),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rarity {
    DEFAULT,
    EPIC,
    LEGENDARY,
    UNKNOWN,
}

impl Rarity {
    pub fn to_string(&self) -> String {
        match self {
            Rarity::DEFAULT => "DEFAULT".to_string(),
            Rarity::EPIC => "EPIC".to_string(),
            Rarity::LEGENDARY => "LEGENDARY".to_string(),
            Rarity::UNKNOWN => "UNKNOWN".to_string(),
        }
    }
    
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Rarity, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "DEFAULT" => Ok(Rarity::DEFAULT),
            "EPIC" => Ok(Rarity::EPIC),
            "LEGENDARY" => Ok(Rarity::LEGENDARY),
            _ => Ok(Rarity::UNKNOWN),
        }
    }
}

#[derive(Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LootSkin {
    //pub disenchantRecipeName: String,
    pub disenchant_value: u64,
    pub display_categories: Option<String>, // type of skin
    pub item_desc: String,                  // name of skin
    //pub lootId: String, //CHAMPION_SKIN_RENTAL_113004,
    //pub lootName: String, //CHAMPION_SKIN_RENTAL_113004,
    //pub parentItemStatus: String,//OWNED,
    pub parent_store_item_id: i64,       //113, // id of champ
    pub rarity: Option<Rarity>, //EPIC,
    //pub redeemableStatus: REDEEMABLE_RENTAL,
    pub store_item_id: i64, // 113004, // skin id
    //pub tags: Jungle,Tank,freljord,rarity_epic,
    //pub tilePath: /lol-game-data/assets/ASSETS/Characters/Sejuani/Skins/Skin04/Images/sejuani_splash_tile_4.jpg,
    //pub type: SKIN_RENTAL,
    //pub upgradeEssenceName: CURRENCY_cosmetic,
    pub upgrade_essence_value: u64, // like discount value needed
    pub upgrade_loot_name: String,  //CHAMPION_SKIN_113004,
    pub value: i64,                 //1350
}

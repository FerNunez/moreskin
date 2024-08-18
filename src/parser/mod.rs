pub mod data;
pub mod loot;

use data::{Inventory, SkinDataBase};
use loot::LootSkin;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum OwnStatus {
    Owned(u64),           // date of purchase
    Loot(bool, u64, u64), // redeemable, essence, dischant
    NotOwned,
}

#[derive(Clone, PartialEq)]
pub struct Skin {
    pub base_champ_id: u64,
    pub name: String,
    pub tile_path: String,
    pub owned: OwnStatus,
    pub value: Option<u64>,
    pub rarity: Option<loot::Rarity>,
}

#[derive(Clone, PartialEq)]
pub struct LolParsed {
    pub name_to_id: HashMap<String, u64>,
    //pub id_to_inventory: HashMap<u64, Inventory>,
    pub id_to_skins: HashMap<u64, HashMap<u64, Skin>>,
}

impl LolParsed {
    pub fn parse_data(data_json_string: &str, loot_json_string: &str) -> Self {
        //parse data
        let skin_db = serde_json::from_str::<Vec<SkinDataBase>>(&data_json_string)
            .expect("Log: error deserialing data JSON");
        let skin_loot = serde_json::from_str::<Vec<LootSkin>>(&loot_json_string)
            .expect("Log: error deserializing loot JSON");

        let mut name_to_champion_id = HashMap::new();
        let mut champ_id_to_skinmap = HashMap::new();

        for skin in &skin_db {
            if skin.is_base {
                name_to_champion_id.insert(skin.name.clone(), skin.champion_id);
                champ_id_to_skinmap.insert(skin.champion_id, HashMap::new());
            }
        }

        for skin in &skin_db {
            if let Some(skins_for_id) = champ_id_to_skinmap.get_mut(&skin.champion_id) {
                skins_for_id.insert(
                    skin.id,
                    Skin {
                        base_champ_id: skin.champion_id,
                        name: skin.name.clone(),
                        tile_path: skin.tile_path.clone(),
                        owned: if skin.ownership.owned {
                            OwnStatus::Owned(0)
                        } else {
                            OwnStatus::NotOwned
                        },
                        value: None,
                        rarity: None,
                    },
                );
            }
        }

        for skin in &skin_loot {
            if let Some(skins_map) =
                champ_id_to_skinmap.get_mut(&(skin.parent_store_item_id as u64))
            {
                if let Some(skin_in_map) = skins_map.get_mut(&(skin.store_item_id as u64)) {
                    skin_in_map.rarity = skin.rarity;
                    skin_in_map.value = Some(skin.disenchant_value);
                    skin_in_map.owned =
                        OwnStatus::Loot(true, skin.upgrade_essence_value, skin.disenchant_value);
                }
            }
        }

        LolParsed {
            name_to_id: name_to_champion_id,
            id_to_skins: champ_id_to_skinmap,
        }
    }

    // pub fn print(&self, champ_name: &str) {
    //     match self.name_to_id.get(champ_name) {
    //         Some(searched_id) => {
    //             if let Some(skin_inventory) = self.id_to_inventory.get(searched_id) {
    //                 println!("* owneds:");
    //                 for owned in &skin_inventory.owned {
    //                     println!("{}, link: {}", owned.name, owned.tile_path);
    //                 }

    //                 println!("* loot:");
    //                 for loot in &skin_inventory.loot {
    //                     println!("{}, link: {}", loot.name, loot.tile_path);
    //                 }

    //                 println!("* not_owned:");
    //                 for not_owned in &skin_inventory.unowned {
    //                     println!("{}, link: {}", not_owned.name, not_owned.tile_path);
    //                 }
    //             }
    //         }
    //         None => println!("Champ not found"),
    //     }
    // }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub champ_inventory: LolParsed,
}

pub mod data;
pub mod loot;

use data::{Inventory, SkinDataBase};
use loot::LootSkin;
use std::collections::HashMap;
use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub struct LolParsed {
    pub name_to_id: HashMap<String, u64>,
    pub id_to_inventory: HashMap<u64, Inventory>,
    pub skin_loot: Vec<LootSkin>,
}

impl LolParsed {


    pub fn parse_data(data_json_string: &str, loot_json_string: &str) -> Self {
        let skin_db = serde_json::from_str::<Vec<SkinDataBase>>(&data_json_string)
            .expect("Log: error deserialing data JSON");
        let skin_loot = serde_json::from_str::<Vec<LootSkin>>(&loot_json_string)
            .expect("Log: error deserializing loot JSON");

        let mut name_to_champion_id = HashMap::new();
        let mut champ_id_to_inventory = HashMap::new();

        for skin in &skin_db {
            if skin.is_base {
                name_to_champion_id.insert(skin.name.clone(), skin.champion_id);
                champ_id_to_inventory.insert(
                    skin.champion_id,
                    Inventory {
                        owned: Vec::new(),
                        loot: Vec::new(),
                        unowned: Vec::new(),
                    },
                );
            }
        }

        for skin in skin_db {
            if let Some(inventory_skin) = champ_id_to_inventory.get_mut(&skin.champion_id) {
                if skin.ownership.owned {
                    inventory_skin.owned.push(skin.clone());
                } else {
                    inventory_skin.unowned.push(skin.clone());
                }
            }
        }

        LolParsed {
            name_to_id: name_to_champion_id,
            id_to_inventory: champ_id_to_inventory,
            skin_loot
        }
    }

    pub fn print(&self, champ_name: &str) {
        match self.name_to_id.get(champ_name) {
            Some(searched_id) => {
                if let Some(skin_inventory) = self.id_to_inventory.get(searched_id) {
                    println!("* owneds:");
                    for owned in &skin_inventory.owned {
                        println!("{}, link: {}", owned.name, owned.tile_path);
                    }

                    println!("* loot:");
                    for loot in &skin_inventory.loot {
                        println!("{}, link: {}", loot.name, loot.tile_path);
                    }

                    println!("* not_owned:");
                    for not_owned in &skin_inventory.unowned {
                        println!("{}, link: {}", not_owned.name, not_owned.tile_path);
                    }
                }
            }
            None => println!("Champ not found"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub champ_inventory: LolParsed,
}

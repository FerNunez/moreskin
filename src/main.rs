mod parser;

use crate::data::SkinDataBase;
use std::{collections::HashMap, str::FromStr};
use gloo::console::log;
use parser::*;
use stylist::Style;
use yew::prelude::*;

const CHAMPION_MAP: &str = include_str!("../res/names_map.json");
const CHAMPION_DB: &str = include_str!("../res/test.json");
const LOOT_DB: &str = include_str!("../res/loot.json");
const STYLE_FILE: &str = include_str!("main.css");

use serde_json::Value;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let val = use_state(|| serde_json::Value::Null);
    let first_load = use_state(|| true);
    let name_map_str = use_state(|| String::new());
    let champ_inventory = use_state(|| LolParsed {
        name_to_id: HashMap::new(),
        id_to_inventory: HashMap::new(),
        skin_loot: Vec::new(),
    });

    if *first_load {
        let champ_inventory = champ_inventory.clone();
        log!("Parsing content");

        let lol_parse = LolParsed::parse_data(&CHAMPION_DB, &LOOT_DB);
        champ_inventory.set(lol_parse);
        first_load.set(false);
        name_map_str.set(CHAMPION_MAP.to_string());

        let name_map = match serde_json::Value::from_str(CHAMPION_MAP) {
            Ok(value) => value,
            Err(_) => Value::Null,
        };
        val.set(name_map);
    }

    //html!{"HI"}

    //<p>
    //}
    html! {
        <div style="display: flex; flex-direction: column; align-items: flex-start; ">
            {champ_inventory_html(&*champ_inventory, &*val)}
        </div>
    }
}

fn champ_inventory_html(lol_parsed: &LolParsed, name_map: &serde_json::Value) -> Html {
    let champ_inventory = lol_parsed
        .name_to_id
        .iter()
        .filter(|(champ_name, _)| *champ_name == "Annie" || *champ_name == "Fizz")
        .map(|(champ_name, champ_id)| {
            let champ_name_cleaned = name_map[&champ_name
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()]
                .to_string();

            let mut champ_sorted = lol_parsed
                .id_to_inventory
                .get(champ_id)
                .unwrap()
                .owned
                .clone();
            champ_sorted.sort_by(|a, b| a.name.cmp(&b.name));

            let mut champ_sorted_notowned = lol_parsed
                .id_to_inventory
                .get(champ_id)
                .unwrap()
                .unowned
                .clone();
            champ_sorted_notowned.sort_by(|a, b| a.name.cmp(&b.name));
            html! {
                    <>
                        <div style="display:flex;">
                        {champ_list(&champ_sorted, &champ_name_cleaned)}
                        //<span style="display: inline-block; border-left: 10px solid gold; margin: 0 0px; height: 200;"></span>
                        {champ_list(&champ_sorted_notowned, &champ_name_cleaned)}
                        </div>
                    </>
            }
        })
        .collect::<Html>();

    html! {

            <>
            {champ_inventory}
            </>
    }
}

fn champ_list(owned_list: &Vec<SkinDataBase>, champ_name: &str) -> Html {
    let stylecss = Style::new(STYLE_FILE).unwrap();

    let owned_list = owned_list
        .iter()
        .map(|skin| {
            html! {

            <div class={stylecss.clone()}>
                  <div class="nft">
                    <div class="main">

                           <img class="tokenImage" src={get_skin_path(&skin, &champ_name)} alt="NFT" />
                           <img class="imageLoot" src={"img/Feature_Loot.png"} alt="NFT" />
                        <div class="skin-name">
                           <h2 >{&skin.name}</h2>
                        </div> //skin-name
                       <p class="description">{"Owned on: dd-mm-yyyy"}</p>

                       <div class={"tokenInfo"}>
                         <div class={"price"}>
                            <ins class={"ins"}>{"RP"}</ins>
                            <p>{"1350 "}</p>
                         </div> //price
                         <div class="duration">
                            <p>{"★90/100"}</p>
                         </div> //duration
                       </div>
                    </div> //main

                  </div> //nft

            </div>
            }
        })
        .collect::<Html>();

    owned_list
}

fn get_skin_path(skin: &SkinDataBase, champ_name: &str) -> String {
    let id: String = skin
        .tile_path
        .split("/")
        .last()
        .unwrap_or("annie_splash_tile_0.Asu_LeeSin.jpg")
        .split(".")
        .next()
        .unwrap_or("annie_splash_tile_0")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    let champ_name_whitout_space = champ_name
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphabetic())
        .collect::<String>();

    let fizz_string = format!("img/centered/{}_{}.jpg", champ_name_whitout_space, id);

    //log!("String fizz path: {}", &fizz_string);
    fizz_string
}

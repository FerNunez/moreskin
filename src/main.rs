mod parser;

use gloo::console::log;
use parser::*;
use std::{collections::HashMap, str::FromStr};
use stylist::Style;
use yew::prelude::*;

const CHAMPION_MAP: &str = include_str!("../res/names_map.json");
const CHAMPION_DB: &str = include_str!("../res/test.json");
const LOOT_DB: &str = include_str!("../res/loot.json");
const STYLE_FILE: &str = include_str!("main.css");

use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum SortBy {
    Name,
    Value,
    Category,
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let champ_sort_by = use_state(|| String::from("Name"));

    let on_change = {
        let sort_by = champ_sort_by.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target_dyn_into::<web_sys::HtmlSelectElement>()
                .unwrap();
            sort_by.set(target.value());
        })
    };
    let val = use_state(|| serde_json::Value::Null);
    let first_load = use_state(|| true);
    let name_map_str = use_state(|| String::new());
    let champ_inventory = use_state(|| LolParsed {
        name_to_id: HashMap::new(),
        id_to_skins: HashMap::new(),
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
    <>
        <div>
            <h1>{ "Sort Items" }</h1>
            <select onchange={on_change}>
                <option value="Value">{ "Value" }</option>
                <option value="Name">{ "Name" }</option>
            </select>
        </div>
        <div style="display: flex; flex-direction: column; align-items: flex-start; ">
            {champ_inventory_html(&*champ_inventory, &*val, &*champ_sort_by)}
        </div>
    </>
    }
}

fn champ_inventory_html(
    lol_parsed: &LolParsed,
    name_map: &serde_json::Value,
    sort_by: &str,
) -> Html {
    let mut name_vector = lol_parsed.name_to_id.keys().collect::<Vec<&String>>();
    name_vector.sort();

    let champs_shelf = name_vector
        .iter()
        .map(|&champ_name| {
            let champ_id = lol_parsed.name_to_id.get(champ_name).unwrap();
            let mut skins_vector = lol_parsed
                .id_to_skins
                .get(champ_id)
                .unwrap()
                .values()
                .collect::<Vec<&Skin>>();
            // Convert hashmap to vector

            let champ_name_cleaned = name_map[&champ_name
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()]
                .to_string();

            match sort_by {
                "Value" => {
                    skins_vector.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
                    skins_vector.reverse();

                    log!("Sorted by value");
                }
                "Name" => {
                    skins_vector.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
                    log!("Sorted by name");
                }
                _ => {
                    skins_vector.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
                }
            }

            html! {
                <>
                    <div style="display:flex;">
                    {champ_list(&skins_vector, &champ_name_cleaned)}
                    </div>
                </>
            }
        })
        .collect::<Html>();

    html! {
            <>
            {champs_shelf}
            </>
    }
}

fn champ_list(owned_list: &Vec<&Skin>, champ_name: &str) -> Html {
    let stylecss = Style::new(STYLE_FILE).unwrap();

    let owned_list = owned_list
        .iter()
        .map(|skin| {
            html! {

            <div class={stylecss.clone()}>
                  <div class="nft">
                    <div class="main">

                           <img class="tokenImage" src={get_skin_path(&skin, &champ_name)} alt="NFT" />

                        if let OwnStatus::Loot(..) = &skin.owned {
                           <img class="imageLoot" src={"img/Feature_Loot.png"} alt="NFT" />
                        }
                        <div class="skin-name">
                           <h2 >{&skin.name}</h2>
                        </div> //skin-name
                       <p class="description">{"Owned on: dd-mm-yyyy"}</p>

                       <div class={"tokenInfo"}>
                         <div class={"price"}>
                            <ins class={"ins"}>{"Ess."}</ins>
                            if let Some(value) = &skin.value {
                                <p>{value}</p>
                            }
                            else {
                                <p>{"0"}</p>
                            }
                         </div> //price
                         <div class="duration">
                        if let  Some(rarity) = &skin.rarity {
                        <p>{rarity.to_string()}</p>}
                        else {
                        <p>{""}</p>}

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

fn get_skin_path(skin: &Skin, champ_name: &str) -> String {
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

use std::{collections::HashMap, io::Error, str::FromStr};
mod parser;
use gloo::console::log;
use gloo_net::http::Request;
use parser::*;
use serde::de::value;
use wasm_bindgen_futures::js_sys::JSON;
use yew::prelude::*;

use serde_json::{Value, json};
fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let champ_inventory = use_state(|| LolParsed {
        name_to_id: HashMap::new(),
        id_to_inventory: HashMap::new(),
    });

    {
        let champ_inventory = champ_inventory.clone();
        use_effect_with((), move |_| {
            let champ_inventory = champ_inventory.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: String = Request::get("http://127.0.0.1:8080/res/test.json")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                log!("Hi");

                let lol_parse = LolParsed::parse_content(fetched_videos);

                //let searched_champ = "Lee Sin";
                //lol_parse.print(searched_champ);
                champ_inventory.set(lol_parse);
            });
            || ()
        });
    }

    let name_map_str = use_state(|| String::new());
    {
        let name_map_str = name_map_str.clone();
        use_effect_with((), move |_| {
            let name_map_str = name_map_str.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let content_str: String = Request::get("http://127.0.0.1:8080/res/names_map.json")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();


                name_map_str.set(content_str);
            });
            || ()
        });
    }

    let content : String = name_map_str.to_string();
    log!(&content);
    
    //let name_map: Value = serde_json::from_str("{\"Aatrox\":\"Aatrox\"}").expect("Fail: could not parse correclty");
    //
    //let name_map = serde_json::Value::from_str(&content).expect("Can't decode content");
    
   // match serde_json::Value::from_str(&content){
   //     Ok(value) => {
   //         log!("got a value");
   //         let asd: String = value["wukong"].to_string();
   //         log!(asd);

   //     },
   //     Err(er) => log!("freaking error: {er:?}"),

   let name_map= match serde_json::Value::from_str(&content){
        Ok(value) => value, 
        Err(_) =>  Value::Null,
    };

    html! {
        <div style="display: flex; flex-direction: column; align-items: flex-start; ">
            {champ_inventory_html(&*champ_inventory, &name_map)}
        </div>
    }
}

fn champ_inventory_html(lol_parsed: &LolParsed, name_map: &serde_json::Value ) -> Html {
    lol_parsed.name_to_id.iter().map(|(champ_name, champ_id)| 
        {
            let champ_name_cleaned = name_map[&champ_name.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>()].to_string();
        html!{
            <>
                <p> {champ_name}</p>
                <div style="display:flex;">
                <p> {"owned" }</p>
                {champ_list(&lol_parsed.id_to_inventory.get(champ_id).unwrap().owned, &champ_name_cleaned)}
                <p> {"unowned" }</p>
                {champ_list(&lol_parsed.id_to_inventory.get(champ_id).unwrap().unowned, &champ_name_cleaned)}
                </div>
            </>
    }}).collect::<Html>()
}

fn champ_list(owned_list: &Vec<Skin>, champ_name: &str) -> Html {
    let owned_list = owned_list.iter().map(|skin| html!{ 
        <div style="display: block; padding: 1px; margin:1px; width: 200px; height: 300px; line-height: 0;">
            <img src={ get_skin_path(&skin, &champ_name)} style="width: 100%; height: 100%; object-fit: cover; display: block; margin: 0px; padding: 0px;" />
        </div>
    }).collect::<Html>();
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

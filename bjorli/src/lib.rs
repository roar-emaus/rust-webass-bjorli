use wasm_bindgen::prelude::*;
use std::collections::HashMap;
pub mod game_scores;
pub mod game_data_structure;
pub mod weather;

use game_scores::{add_2021_09, add_2022_02, add_2022_10, add_2024_03};
use game_data_structure::GameData;


#[wasm_bindgen]
pub fn generate_html_table_for_date(date: &str) -> Result<JsValue, JsValue> {
    let mut game_data = GameData {
        data: HashMap::new(),
    };
    
    add_2021_09(&mut game_data);
    add_2022_02(&mut game_data);
    add_2022_10(&mut game_data);
    add_2024_03(&mut game_data);

    if let Some(table_data) = game_data.get_table_for_date(date) {
        let mut table_html = String::from("<table border='1'>");

        for row in table_data {
            table_html.push_str("<tr>");
            for cell in row {
                table_html.push_str(&format!("<td>{}</td>", cell));
            }
            table_html.push_str("</tr>");
        }
        table_html.push_str("</table>");
        
        return Ok(JsValue::from_str(&table_html));
    }

    Err(JsValue::from_str("Ingen data for denne datoen!"))
}

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
pub mod game_scores;
pub mod game_data_structure;

use game_scores::add_2021_09;
use game_data_structure::GameData;


#[wasm_bindgen]
pub fn generate_html_table_for_date() -> Result<JsValue, JsValue> {
    let mut game_data = GameData {
        data: HashMap::new(),
    };
    
    add_2021_09(&mut game_data);

    if let Some(table_data) = game_data.get_table_for_date("2021-09") {
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

    Err(JsValue::from_str("No data available for the specified date"))
}

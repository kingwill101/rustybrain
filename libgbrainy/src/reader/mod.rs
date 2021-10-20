use serde::Deserialize;
use serde_xml_rs::Deserializer;

use crate::models::game::GameCollection;

pub fn parse_game_data(data: &str) -> Option<GameCollection> {
    let mut ser = Deserializer::new_from_reader(data.as_bytes())
        .non_contiguous_seq_elements(true);

    match GameCollection::deserialize(&mut ser) {
        Ok(res) => { Some(res) }
        Err(e) => {
            log::error!("parse_game_data(): {}", e);
            None
        }
    }
}
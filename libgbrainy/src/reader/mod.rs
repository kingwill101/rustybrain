use serde::Deserialize;
use serde_xml_rs::Deserializer;

use crate::models::game::GameCollection;

pub fn parse_game_data(data: Box<&str>) -> GameCollection {
    let mut ser=  Deserializer::new_from_reader(data.as_bytes())
        .non_contiguous_seq_elements(true);

    GameCollection::deserialize(&mut ser).unwrap()

}
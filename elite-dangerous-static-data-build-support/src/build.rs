use std::io::{Read, Write};
use serde::de::DeserializeOwned;

fn create_enum<T>(output_writer: &mut dyn Write, csv_src: impl Read, type_name: &str)
where T: DeserializeOwned + std::fmt::Debug
{

    let mut reader = csv::Reader::from_reader(csv_src);

    output_writer.write_all(format!("pub enum {type_name} {{\n").as_bytes()).unwrap();

    for result in reader.deserialize() {

        let fs : T = result.unwrap();

        output_writer.write_all(format!("{}()", fs).to_string().as_bytes()).unwrap();
    }

}

#[cfg(test)]
mod tests {
    use convert_case::{Case, Casing};
    use crate::build::create_enum;
    use crate::types::faction::FactionState;

    #[test]
    fn test_generate_faction_state_enum() {

        let mut csv_string = "id,name\nNone,None\nBoom,Boom\nCivilUnrest,Civil Unrest".as_bytes();

        let type_name = "factionState";
        let type_name_pascal = type_name.to_case(Case::Pascal);
        assert_eq!(type_name_pascal, "FactionState");
        let type_name_snake = type_name.to_case(Case::Snake);
        assert_eq!(type_name_snake, "faction_state");

        let mut buffer = Vec::<u8>::with_capacity(1024);
        create_enum::<FactionState>(&mut buffer, csv_string, type_name_pascal.as_str());

        let result = std::str::from_utf8(&buffer).unwrap();
        println!("{:?}", result);
    }
}
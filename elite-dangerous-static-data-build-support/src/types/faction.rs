use serde::{Deserialize, Serialize};
use crate::CodeGeneratorName;

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionState {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="name")]
    pub name: String,
}


#[cfg(test)]
mod tests {
    use crate::types::faction::FactionState;

    #[test]
    fn test_parse_faction_state() {

        let csv_string = "id,name\nNone,None\nBoom,Boom\nCivilUnrest,Civil Unrest";

        let mut reader = csv::Reader::from_reader(csv_string.as_bytes());
        for result in reader.deserialize::<FactionState>() {
            let record: FactionState = result.unwrap();
            println!("{:?}", record);
        }
    }
}
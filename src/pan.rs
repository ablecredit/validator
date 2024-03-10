use crate::ValidationError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanMeta {
    pub code: String,
    pub holder_type: HolderType,
    pub name_char: char,
    pub registration_number: String,
    pub checksum: char,
}

#[derive(Debug, Copy, Clone, EnumString, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum HolderType {
    #[strum(serialize = "A", to_string = "A")]
    AOP,
    #[strum(serialize = "B")]
    BOI,
    #[strum(serialize = "C")]
    Company,
    #[strum(serialize = "F")]
    Firm,
    #[strum(serialize = "G")]
    Government,
    #[strum(serialize = "H")]
    HUF,
    #[strum(serialize = "L")]
    LocalAuthority,
    #[strum(serialize = "J")]
    ArtificialJuridicalPerson,
    #[strum(serialize = "P")]
    Person,
    #[strum(serialize = "T")]
    Trust,
}
impl PanMeta {
    pub fn from_pan(pan: &str) -> Result<PanMeta, ValidationError> {
        let pm = pan_parser::pan(pan)
            .map_err(|e| ValidationError::Pan(format!("Error parsing PAN: {}", e)))?;

        Ok(pm)
    }

    pub fn pan(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.code, self.holder_type, self.name_char, self.registration_number, self.checksum
        )
    }
}

peg::parser! {
    grammar pan_parser() for str {
        pub rule pan() -> PanMeta
            = code:pan_code() holder_type:pan_holder_type() name_char:$(['A'..='Z']) registration_number:registration_number() checksum:$(['A'..='Z']) {
                PanMeta {
                    code: code.to_string(),
                    holder_type,
                    name_char: name_char.chars().next().unwrap(),
                    registration_number,
                    checksum: checksum.chars().next().unwrap(),
                }
            }
        rule pan_code() -> String
            = c:$(['A'..='Z']['A'..='Z']['A'..='Z']) { c.to_string() }
        rule pan_holder_type() -> HolderType
            = t:$(['A'] / ['B'] / ['C'] / ['F'] / ['G'] / ['H'] / ['L'] / ['J'] / ['P'] / ['T']) { HolderType::from_str(t).unwrap() }
        rule registration_number() -> String
            = n:$(['0'..='9']['0'..='9']['0'..='9']['0'..='9']) { n.parse().unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pan() {
        let pan = "AAAPZ1234C";
        let pm = PanMeta::from_pan(pan).unwrap();
        assert_eq!(pm.code, "AAA");
        assert_eq!(pm.holder_type, HolderType::Person);
        assert_eq!(pm.name_char, 'Z');
        assert_eq!(pm.registration_number, "1234");
        assert_eq!(pm.checksum, 'C');
        assert_eq!(pm.pan(), pan);

        println!("{:#?}", pm);
        println!("PAN: {}", pm.pan());
    }

    #[test]
    fn invalid_holder_type() {
        let pan = "AAAXZ1234X";
        let pm = PanMeta::from_pan(pan);
        assert!(pm.is_err());
        println!("{:?}", pm);
    }
    
    #[test]
    fn invalid_registration_number() {
        let pan = "AAAPZ12G4X";
        let pm = PanMeta::from_pan(pan);
        assert!(pm.is_err());
        println!("{:?}", pm);
    }
    
    #[test]
    fn invalid_name_char() {
        let pan = "AAAP41234X";
        let pm = PanMeta::from_pan(pan);
        assert!(pm.is_err());
        println!("{:?}", pm);
    }
}

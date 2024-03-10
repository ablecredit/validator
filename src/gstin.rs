use super::ValidationError;
use crate::pan::PanMeta;
use crate::utils::State;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GstinMeta {
    pub state: State,
    pub pan: PanMeta,
    pub entity_digit: char,
    pub check_code: char,
}

impl GstinMeta {
    pub fn from_gstin(gstin: &str) -> Result<GstinMeta, ValidationError> {
        gstin_parser::gstin(gstin).map_err(|e| {
            crate::ValidationError::Gstin(format!("Parser Error while parsing GSTIN: {}", e))
        })?
    }

    pub fn gstin(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.state,
            self.pan.pan(),
            self.entity_digit,
            'Z',
            self.check_code
        )
    }
}

peg::parser! {
    grammar gstin_parser() for str {
        pub rule gstin() -> Result<GstinMeta, ValidationError>
            = state:state() pan:pan() entity_digit:entity_digit() default:$(['Z']) check_code:$(['A'..='Z']/['0'..='9']) {
                if state.is_err() {
                    return Err(state.unwrap_err());
                }
                if pan.is_err() {
                    return Err(pan.unwrap_err());
                }

                let state = state.unwrap();
                let pan = pan.unwrap();

                Ok(GstinMeta {
                    state,
                    pan,
                    entity_digit,
                    check_code: check_code.chars().next().unwrap(),
                })
            }
        rule state() -> Result<State, ValidationError>
            = s:$(['0'..='9']['0'..='9']) { State::from_str(s)
                .or(Err(ValidationError::Gstin(format!("invalid state {}", s)))) }
        rule pan() -> Result<PanMeta, ValidationError>
            = p:$(['A'..='Z']+ ['0'..='9']+ ['A'..='Z']) { PanMeta::from_pan(p) }
        rule entity_digit() -> char
            = d:$(['1'..='9'] / ['A'..='Z']) { d.parse().unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gstin() {
        let gstin = "29AAAPZ1234C1Z5";
        let gm = GstinMeta::from_gstin(gstin).unwrap();
        assert_eq!(gm.state, State::Karnataka);
        assert_eq!(gm.pan.code, "AAA");
        assert_eq!(gm.pan.holder_type, crate::pan::HolderType::Person);
        assert_eq!(gm.pan.name_char, 'Z');
        assert_eq!(gm.pan.registration_number, "1234");
        assert_eq!(gm.pan.checksum, 'C');
        assert_eq!(gm.entity_digit, '1');
        assert_eq!(gm.check_code, '5');

        println!("{:#?}", gm);
        println!("GSTIN: {}", gm.gstin());
    }
    
    #[test]
    fn invalid_state() {
        let gstin = "00AAAPZ1234C1Z5";
        let gm = GstinMeta::from_gstin(gstin);
        assert!(gm.is_err());
        println!("{:?}", gm);
    }
    
    #[test]
    fn invalid_pan() {
        let gstin = "29XXXXX0000X1Z5";
        let gm = GstinMeta::from_gstin(gstin);
        assert!(gm.is_err());
        println!("{:?}", gm);
    }
}

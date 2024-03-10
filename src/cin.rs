use crate::utils::State;
use crate::ValidationError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinMeta {
    pub listed: bool,
    pub industry_code: String,
    pub state: State,
    pub incorporation_year: String,
    pub classification: Classification,
    pub registration_number: String,
}

#[derive(Debug, Copy, Clone, Display, EnumString, Serialize, Deserialize, PartialEq, Eq)]
pub enum Classification {
    FLC,
    FTC,
    GAP,
    GAT,
    GOI,
    NPL,
    OPC,
    PLC,
    PTC,
    SGC,
    ULL,
    ULT,
}
impl CinMeta {
    pub fn from_cin(cin: &str) -> Result<CinMeta, ValidationError> {
        cin_parser::cin(cin)
            .map_err(|e| ValidationError::Cin(format!("Error parsing CIN: {}", e)))?
    }

    pub fn cin(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            if self.listed { "L" } else { "U" },
            self.industry_code,
            self.state.code(),
            self.incorporation_year,
            self.classification,
            self.registration_number
        )
    }
}

peg::parser! {
    grammar cin_parser() for str {
        rule listing_status() -> bool
            = l:$(quiet!{['L'] / ['U']} / expected!("listing status U or L")) { l.starts_with('L') }
        rule industry_code() -> String
            = n:$(['0'..='9']['0'..='9']['0'..='9']['0'..='9']['0'..='9']) { n.parse().unwrap() }
        rule state() -> Result<State, ValidationError>
            = s:$(['A'..='Z']['A'..='Z']?) { State::from_str(s)
                .or(Err(ValidationError::Cin(format!("invalid state {}", s)))) }
        rule incorporation_year() -> String
            = n:$(['0'..='9']['0'..='9']['0'..='9']['0'..='9']) { n.parse().unwrap() }
        rule classification() -> Result<Classification, ValidationError>
            = c:$(['A'..='Z']['A'..='Z']['A'..='Z']?) { Classification::from_str(c)
                .or(Err(ValidationError::Cin(format!("invalid classification {}", c)))) }
        rule registration_number() -> String
            = n:$(['0'..='9']['0'..='9']['0'..='9']['0'..='9']['0'..='9']['0'..='9']) { n.parse().unwrap() }

        pub(crate) rule cin() -> Result<CinMeta, ValidationError>
            = listing_status:listing_status()
              industry_code:industry_code()
              st:state()
              incorporation_year:incorporation_year()
              class:classification()
              registration_number:registration_number()
            {
                if st.is_err() {
                    return Err(ValidationError::Cin(st.unwrap_err().to_string()));
                }
                if class.is_err() {
                    return Err(ValidationError::Cin(class.unwrap_err().to_string()));
                }

                let st = st.unwrap();
                let class = class.unwrap();

                Ok(CinMeta {
                    listed: listing_status,
                    industry_code,
                    state: st,
                    incorporation_year,
                    classification: class,
                    registration_number,
                })
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cin() {
        let cin = "L93030DL2010PLC198141";
        let c = CinMeta::from_cin(cin).unwrap();
        assert!(c.listed);
        assert_eq!(c.industry_code, "93030");
        assert_eq!(c.state, State::Delhi);
        assert_eq!(c.incorporation_year, "2010");
        assert_eq!(c.classification, Classification::PLC);
        assert_eq!(c.registration_number, "198141");
        assert_eq!(c.cin(), cin);
        println!("{:#?}", c);
        println!("CIN {}", c.cin());
    }

    #[test]
    fn invalid_listing_status() {
        let cin = "X93030DL2010PLC198141";
        let c = CinMeta::from_cin(cin);
        assert!(c.is_err());
        println!("{:#?}", c);
    }

    #[test]
    fn invalid_industry_code() {
        let cin = "L934R04DL2010PLC198141";
        let c = CinMeta::from_cin(cin);
        assert!(c.is_err());
        println!("{:#?}", c);
    }

    #[test]
    fn invalid_state() {
        let cin = "L93030XX2010PLC198141";
        let c = CinMeta::from_cin(cin);
        assert!(c.is_err());
        println!("{:#?}", c);
    }

    #[test]
    fn invalid_classification() {
        let cin = "L93030DL2010XXX198141";
        let c = CinMeta::from_cin(cin);
        assert!(c.is_err());
        println!("{:#?}", c);
    }

    #[test]
    fn invalid_cin() {
        let cin = "L93030DL2010PLC1981414";
        let c = CinMeta::from_cin(cin);
        assert!(c.is_err());
        println!("{:#?}", c);
    }
}

/*
The format of the Udyam registration number can be explained further.
1. First 5 Letters – UDYAM
2. Next 2 Digits – STATE CODE
3. Next 2 Digits – CITY CODE
4. Next 7 Digits – Are special Udyam Code assigned to a company, similar to an Aadhar number.

Sample Udyam Registration Number: UDYAM-UP-00-0123456
 */

use super::ValidationError;
use crate::utils::State;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdyamMeta {
    pub state: State,
    pub city: String,
    pub udyam_code: String, // save as string to avoid leading zeros
}

impl UdyamMeta {
    pub fn from_udyam(udyam: &str) -> Result<UdyamMeta, ValidationError> {
        udyam_parser::udyam(udyam)
            .map_err(|e| crate::ValidationError::Udyam(format!("Error parsing Udyam: {}", e)))?
    }

    pub fn udyam(&self) -> String {
        format!(
            "UDYAM-{}-{}-{}",
            self.state.code(),
            self.city,
            self.udyam_code
        )
    }
}

peg::parser! {
    grammar udyam_parser() for str {
        pub rule udyam() -> Result<UdyamMeta, ValidationError>
            = "UDYAM-" state:state() "-" city:city() "-" udyam_code:udyam_code() {
                if state.is_err() {
                    return Err(state.unwrap_err());
                }

                let state = state.unwrap();

                Ok(UdyamMeta {
                    state,
                    city,
                    udyam_code,
                })
            }
        rule state() -> Result<State, ValidationError>
            = s:$(['A'..='Z']['A'..='Z']) { State::from_str(s).or(Err(ValidationError::Udyam(format!("invalid state {}", s)))) }
        rule city() -> String
            = c:$(['0'..='9']['0'..='9']) { c.parse().unwrap() }
        rule udyam_code() -> String
            = u:$(['0'..='9']['0'..='9']['0'..='9']['0'..='9']['0'..='9']['0'..='9']['0'..='9'])
            { u.parse().unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udyam() {
        let udyam = "UDYAM-UP-48-0123456";
        let um = UdyamMeta::from_udyam(udyam).unwrap();
        assert_eq!(um.state, State::UttarPradesh);
        assert_eq!(um.city, "48");
        assert_eq!(um.udyam_code, "0123456");
        assert_eq!(um.udyam(), udyam);

        println!("{:#?}", um);
        println!("Udyam: {}", um.udyam());
    }

    #[test]
    fn invalid_state() {
        let udyam = "UDYAM-XX-48-0123456";
        let um = UdyamMeta::from_udyam(udyam);
        assert!(um.is_err());
        println!("{:?}", um);
    }

    #[test]
    fn invalid_city() {
        let udyam = "UDYAM-UP-XX-0123456";
        let um = UdyamMeta::from_udyam(udyam);
        assert!(um.is_err());
        println!("{:?}", um);
    }

    #[test]
    fn invalid_udyam_code() {
        let udyam = "UDYAM-UP-48-012345X";
        let um = UdyamMeta::from_udyam(udyam);
        assert!(um.is_err());
        println!("{:?}", um);
    }
}

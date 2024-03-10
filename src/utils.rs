use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
/*
STATE NAME	STATE CODE
JAMMU AND KASHMIR	01
HIMACHAL PRADESH	02
PUNJAB	03
CHANDIGARH	04
UTTARAKHAND	05
HARYANA	06
DELHI	07
RAJASTHAN	08
UTTAR PRADESH	09
BIHAR	10
SIKKIM	11
ARUNACHAL PRADESH	12
NAGALAND	13
MANIPUR	14
MIZORAM	15
TRIPURA	16
MEGHALAYA	17
ASSAM	18
WEST BENGAL	19
JHARKHAND	20
ODISHA	21
CHATTISGARH	22
MADHYA PRADESH	23
GUJARAT	24
DADRA AND NAGAR HAVELI AND DAMAN AND DIU (NEWLY MERGED UT)	26*
MAHARASHTRA	27
ANDHRA PRADESH(BEFORE DIVISION)	28
KARNATAKA	29
GOA	30
LAKSHADWEEP	31
KERALA	32
TAMIL NADU	33
PUDUCHERRY	34
ANDAMAN AND NICOBAR ISLANDS	35
TELANGANA	36
ANDHRA PRADESH (NEWLY ADDED)	37
LADAKH (NEWLY ADDED)	38
OTHER TERRITORY	97
CENTRE JURISDICTION	99
 */

#[derive(Debug, Clone, Copy, EnumString, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum State {
    #[strum(serialize = "AP", serialize = "28")]
    AndhraPradesh,
    #[strum(serialize = "AR", serialize = "12")]
    ArunachalPradesh,
    #[strum(serialize = "AS", serialize = "18")]
    Assam,
    #[strum(serialize = "BR", serialize = "10")]
    Bihar,
    #[strum(serialize = "CG", serialize = "22")]
    Chhattisgarh,
    #[strum(serialize = "GA", serialize = "30")]
    Goa,
    #[strum(serialize = "GJ", serialize = "24")]
    Gujarat,
    #[strum(serialize = "HR", serialize = "06")]
    Haryana,
    #[strum(serialize = "HP", serialize = "02")]
    HimachalPradesh,
    #[strum(serialize = "JK", serialize = "01")]
    JammuAndKashmir,
    #[strum(serialize = "JH", serialize = "20")]
    Jharkhand,
    #[strum(serialize = "KA", serialize = "29")]
    Karnataka,
    #[strum(serialize = "KL", serialize = "32")]
    Kerala,
    #[strum(serialize = "MP", serialize = "23")]
    MadhyaPradesh,
    #[strum(serialize = "MH", serialize = "27")]
    Maharashtra,
    #[strum(serialize = "MN", serialize = "14")]
    Manipur,
    #[strum(serialize = "ML", serialize = "17")]
    Meghalaya,
    #[strum(serialize = "MZ", serialize = "15")]
    Mizoram,
    #[strum(serialize = "NL", serialize = "13")]
    Nagaland,
    #[strum(serialize = "OR", serialize = "21")]
    Odisha,
    #[strum(serialize = "PB", serialize = "03")]
    Punjab,
    #[strum(serialize = "RJ", serialize = "08")]
    Rajasthan,
    #[strum(serialize = "SK", serialize = "11")]
    Sikkim,
    #[strum(serialize = "TN", serialize = "33")]
    TamilNadu,
    #[strum(serialize = "TR", serialize = "16")]
    Tripura,
    #[strum(serialize = "UK", serialize = "05")]
    Uttarakhand,
    #[strum(serialize = "UP", serialize = "09")]
    UttarPradesh,
    #[strum(serialize = "WB", serialize = "19")]
    WestBengal,
    #[strum(serialize = "AN", serialize = "35")]
    AndamanAndNicobarIslands,
    #[strum(serialize = "CH", serialize = "04")]
    Chandigarh,
    #[strum(serialize = "DH", serialize = "26")]
    DadraAndNagarHaveli,
    #[strum(serialize = "DD", serialize = "25")]
    // gst code for daman and diu is now merged with dadra and nagar haveli 26 after jan 2020. previously it was 25
    DamanAndDiu,
    #[strum(serialize = "DL", serialize = "07")]
    Delhi,
    #[strum(serialize = "LD", serialize = "31")]
    Lakshadweep,
    #[strum(serialize = "PY", serialize = "34")]
    Puducherry,
    #[strum(serialize = "TS", serialize = "36")]
    Telangana,
    #[strum(serialize = "LA", serialize = "38")]
    Ladakh,
    #[strum(serialize = "OT", serialize = "97")]
    OtherTerritory,
    #[strum(serialize = "CJ", serialize = "99")]
    CentralJurisdiction,
}

impl State {
    pub fn code(&self) -> &str {
        match self {
            State::AndhraPradesh => "AP",
            State::ArunachalPradesh => "AR",
            State::Assam => "AS",
            State::Bihar => "BR",
            State::Chhattisgarh => "CG",
            State::Goa => "GA",
            State::Gujarat => "GJ",
            State::Haryana => "HR",
            State::HimachalPradesh => "HP",
            State::JammuAndKashmir => "JK",
            State::Jharkhand => "JH",
            State::Karnataka => "KA",
            State::Kerala => "KL",
            State::MadhyaPradesh => "MP",
            State::Maharashtra => "MH",
            State::Manipur => "MN",
            State::Meghalaya => "ML",
            State::Mizoram => "MZ",
            State::Nagaland => "NL",
            State::Odisha => "OR",
            State::Punjab => "PB",
            State::Rajasthan => "RJ",
            State::Sikkim => "SK",
            State::TamilNadu => "TN",
            State::Tripura => "TR",
            State::Uttarakhand => "UK",
            State::UttarPradesh => "UP",
            State::WestBengal => "WB",
            State::AndamanAndNicobarIslands => "AN",
            State::Chandigarh => "CH",
            State::DadraAndNagarHaveli => "DH",
            State::DamanAndDiu => "DD",
            State::Delhi => "DL",
            State::Lakshadweep => "LD",
            State::Puducherry => "PY",
            State::Telangana => "TS",
            State::Ladakh => "LA",
            State::OtherTerritory => "OT",
            State::CentralJurisdiction => "CJ",
        }
    }
}

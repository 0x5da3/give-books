use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OGP {
    pub og_title: String,
    pub og_desc: String,
    pub og_url: String,
    pub og_image: String,
}

impl OGP {
    pub fn new() -> Self {
        OGP {
            og_title: String::new(),
            og_desc: String::new(),
            og_url: String::new(),
            og_image: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasswordPolicy {
    pub has_numeric: bool,
    pub has_lowercase: bool,
    pub has_uppercase: bool,
    pub has_enough_length: bool,
}

impl PasswordPolicy {
    pub fn new() -> Self {
        PasswordPolicy {
            has_numeric: false,
            has_lowercase: false,
            has_uppercase: false,
            has_enough_length: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VOgp {
    pub records: Vec<OGP>,
}

impl VOgp {
    pub fn new() -> Self {
        VOgp {
            records: vec![OGP::new()],
        }
    }
}

#[allow(unused)]
struct Bid {
    num_of_bids: usize,
}

#[allow(unused)]
struct BidItem {
    ogp: OGP,
    Bid: Bid,
}

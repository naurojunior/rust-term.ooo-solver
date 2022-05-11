use serde_json::{Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rule{
    pub letter: char,
    pub rule_type: String
}

#[derive(Serialize, Deserialize)]
pub struct Ruleset {
    pub rules: Vec<Rule>
}


pub fn parse(json_with_rules: &str) -> Result<Ruleset>{

    let ruleset: Ruleset = serde_json::from_str(json_with_rules)?;
    Ok(ruleset)
}

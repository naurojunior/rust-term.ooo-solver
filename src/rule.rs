use serde_json::{Result, Value};

pub fn parse(json_with_rules: &str) -> Result<()>{
    let v: Value = serde_json::from_str(json_with_rules)?;

    println!("letter: {}, rule: {}", v["ruleset"][0]["letter"], v["ruleset"][0]["rule"]);
    println!("letter: {}, rule: {}", v["ruleset"][1]["letter"], v["ruleset"][1]["rule"]);
    println!("letter: {}, rule: {}", v["ruleset"][2]["letter"], v["ruleset"][2]["rule"]);
    println!("letter: {}, rule: {}", v["ruleset"][3]["letter"], v["ruleset"][3]["rule"]);
    println!("letter: {}, rule: {}", v["ruleset"][4]["letter"], v["ruleset"][4]["rule"]);

    Ok(())
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Read};
use std::process;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    for (key, value) in parse(&buffer)? {
        println!("export {:}={:?}", key, value);
    }

    Ok(())
}

fn parse(buffer: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let response: Secret = serde_json::from_str(&buffer).unwrap_or_else(|_| {
        println!("Invalid input:\n{}", &buffer);
        process::exit(1)
    });
    serde_json::from_str(&response.secret_string)
}

#[derive(Serialize, Deserialize)]
struct Secret {
    #[serde(alias = "ARN")]
    arn: String,

    #[serde(alias = "Name")]
    name: String,

    #[serde(alias = "SecretString")]
    secret_string: String,
}

use error_chain::error_chain;
//use std::fs::File;
//use std::io::Read;
//use std::collections::HashMap;
use serde_json:: {Value};

error_chain! {
     foreign_links {
         HttpRequest(reqwest::Error);
         IoError(::std::io::Error);
     }
 }
 #[tokio::main]

async fn main() -> Result<()> {
// Load the first file into a string.
let text = std::fs::read_to_string("example.json").unwrap();
println!("The text of the file is {}", text);

// Parse the string into a dynamically-typed JSON structure.
let json_file= serde_json::from_str::<Value>(&text).unwrap();


let client = reqwest::Client::new();
let res = client.post("https://paste.rs")
    .json(&json_file)
    .send()
    .await?;

     let response_text = res.text().await?;
   println!("Your paste is located at: {}",response_text );   
    Ok(())            
}
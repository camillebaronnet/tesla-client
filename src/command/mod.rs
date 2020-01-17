use clap::ArgMatches;
use std::io::{stdin,stdout,Write};
use serde_json::{Value};
use std::fs::File;
use std::io::prelude::*;
use std::fs::{DirBuilder};

extern crate dirs;

/**
 * Login command.
 */
#[tokio::main]
pub async fn login(args: &ArgMatches, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {

    // Ask login/password
    let username = ask(args.value_of("username").unwrap_or(""), "Username : ", false);
    let password = ask(args.value_of("password").unwrap_or(""), "Password : ", true);
    
    // Prepare HTTP query
    let params = [
        ("grant_type", "password"), 
        ("client_id", "81527cff06843c8634fdc09e8ac0abefb46ac849f38fe1e431c2ef2106796384"),
        ("client_secret", "c7257eb71a564034f9419ee651c7d0e5f7aa6bfbd18bafb5c5c033b093bb2fa3"),
        ("email", &username),
        ("password", &password),
    ];
    
    let client = reqwest::Client::new();
    let request = client.post("https://owner-api.teslamotors.com/oauth/token")
            .header("User-Agent", "curl/7.64.1")
            .header("Accept", "*/*")
            .form(&params)
    ;

    // Send query...
    let reponse = request.send().await?;

    // Display verbose informations...
    if matches.is_present("debug") {
        println!("{:#?}", &reponse);
    }
    
    // Parse JSON response...
    let text = &reponse.text().await?;
    let v: Value = serde_json::from_str(&text).expect("Failed to parse result from API.");

    // Display verbose informations...
    if matches.is_present("debug") {
        println!("");
        println!("RAW Return:");
        println!("{:?}", v);
    }

    // Define some values...
    let token = v["access_token"].as_str().expect("Login error, failed to get token.");
    let home_directory = dirs::config_dir().expect("Failed to get home directory.");
    let auth_dir = format!("{}/tesla-client", home_directory.display());
    let auth_path = format!("{}/tesla-client/auth.json", home_directory.display());

    // Try to store OAuth response...
    DirBuilder::new().recursive(true).create(&auth_dir)?;
    let mut file = File::create(auth_path)?;
    file.write_all(text.as_bytes())?;

    // Display success message...
    println!("");
    println!("Login succeeded!");
    println!("Token : {}", token);
    println!("");

    Ok(())     
}

/**
 * List products command.
 */
#[tokio::main]
pub async fn products(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {

    // Get config path
    let home_directory = dirs::config_dir().expect("Failed to get home directory.");
    let auth_path = format!("{}/tesla-client/auth.json", home_directory.display());

    // Read file
    let mut file = File::open(auth_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse JSON
    let o: Value = serde_json::from_str(&contents)?;
    let token = o["access_token"].as_str().expect("Failed to get token.");
    
    // Prepare HTTP Query
    let client = reqwest::Client::new();
    let request = client.get("https://owner-api.teslamotors.com/api/1/products")
            .header("User-Agent", "curl/7.64.1")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "*/*")
    ;

    // Send query...
    let reponse = request.send().await?;

    // Display verbose informations...
    if matches.is_present("debug") {
        println!("{:#?}", &reponse);
    }
    
    // Parse JSON response...
    let text = &reponse.text().await?;
    let v: Value = serde_json::from_str(&text).expect("Failed to parse result from API.");
    let result = &v["response"].as_array();

    print!("ID\t\t\tVIN\t\t\tState\t\tName\n");
    for item in result.unwrap() {
        println!("{}\t{}\t{}\t\t{}\n", 
            item["id"], 
            item["vin"].as_str().unwrap_or("-"),
            item["state"].as_str().unwrap_or("-"),
            item["display_name"].as_str().unwrap_or("Untitled")
        );
    }

    println!("");
    Ok(())
}

/**
 * Not found command.
 */
pub fn not_found() -> Result<(), Box<dyn std::error::Error>> {
    print!("Command not found.");
    println!("");
    Ok(())
}

/**
 * Prompt helper to ask some values.
 */
fn ask(default: &str, ask: &str, password: bool) -> String {
    if !default.is_empty() {
        return String::from(default);
    }

    let mut input = String::new();

    if password {
        rpassword::prompt_password_stdout("Password: ").unwrap()
    } else {
        print!("{}", ask);
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string.");
        input[0..input.len() - 1].to_string()
    }
}

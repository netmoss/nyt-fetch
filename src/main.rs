use serde::Deserialize;
use chrono::prelude::*;
use std::{error::Error, io, process::exit, collections::HashMap};

fn wordle() -> Result<String, Box<dyn Error>> {
    let date: String = Local::now().to_string();

    #[derive(Debug, Deserialize)]
    struct Wordle {
        solution: String,
    }

    let response = reqwest::blocking::get(format!("https://www.nytimes.com/svc/wordle/v2/{}.json", &date[..10]))?;
    let word: Wordle = response.json()?;
    Ok(word.solution.to_uppercase())
}

fn connections() -> Result<(), Box<dyn Error>> {
    let date: String = Local::now().to_string();

    #[derive(Debug, Deserialize)]
    struct Connections {
        groups: HashMap<String, Group>,
    }
    
    #[derive(Debug, Deserialize)]
    struct Group {
        members: Vec<String>,
    }

    let response = reqwest::blocking::get(format!("https://www.nytimes.com/svc/connections/v1/{}.json", &date[..10]))?;
    let data: Connections = response.json()?;

    let difficulties = ["Straightforward", "Intermediate", "Hard", "Tricky"];
    for (i, (group_name, group)) in data.groups.iter().enumerate() {
        println!("Difficulty: {}", difficulties[i]);
        println!("{}:", group_name);
        println!("{}\n", group.members.join(", "));
    }

    Ok(())
}

fn strands() -> Result<String, Box<dyn Error>> {
    let date: String = Local::now().to_string();

    #[derive(Debug, Deserialize)]
    struct Strands {
        spangram: String,
        themeWords: Vec<String>,
    }

    let response = reqwest::blocking::get(format!("https://www.nytimes.com/games-assets/strands/{}.json", &date[..10]))?;
    let data: Strands = response.json()?;

    Ok(format!("Spangram {}\nSolutions: {}", data.spangram, data.themeWords.join(", ")))

}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("Enter the game you want the answers to (Wordle, Connections, Strands, Mini), or type 'quit' to close the program.");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        
        match selection.trim().to_lowercase().as_str() {
            "wordle" => {
                println!("Today's solution is: {}", wordle()?);
            }
            "connections" => {
               connections()?;
            }
            "strands" => {
                println!("{}", strands()?);
            }
            "quit" => {
                exit(0)
            }
            &_ => {
            println!("Not a valid input")
            }
        }
    }
}


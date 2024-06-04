use serde::Deserialize;
use chrono::prelude::*;
use std::{error::Error, io, process::exit, collections::HashMap};

fn wordle() -> Result<String, Box<dyn Error>> {
    let date: String = Local::now().to_string();

    #[derive(Deserialize)]
    struct Wordle {
        solution: String,
    }

    let response = reqwest::blocking::get(format!("https://www.nytimes.com/svc/wordle/v2/{}.json", &date[..10]))?;
    let word: Wordle = response.json()?;
    Ok(word.solution.to_uppercase())
}

fn connections() -> Result<String, Box<dyn Error>> {
    let date: String = Local::now().to_string();

    #[derive(Deserialize)]
    struct Connections {
        groups: HashMap<String, Group>,
    }
    
    #[derive(Deserialize)]
    struct Group {
        members: Vec<String>,
    }

    let response = reqwest::blocking::get(format!("https://www.nytimes.com/svc/connections/v1/{}.json", &date[..10]))?;
    let data: Connections = response.json()?;

    let difficulties = ["Straightforward", "Intermediate", "Hard", "Tricky"];
    let mut result_string = String::new();
    for (i, (group_name, group)) in data.groups.iter().enumerate() {
        result_string.push_str(&format!("Difficulty: {}\n", difficulties[i]));
        result_string.push_str(&format!("{}:\n", group_name));
        result_string.push_str(&format!("{}\n\n", group.members.join(", ")));
    }

    Ok(result_string)
}

fn strands() -> Result<String, Box<dyn Error>> {
    let date: String = Local::now().to_string();

    #[derive(Deserialize)]
    struct Strands {
        spangram: String,
        themeWords: Vec<String>,
    }

    let response = reqwest::blocking::get(format!("https://www.nytimes.com/games-assets/strands/{}.json", &date[..10]))?;
    let data: Strands = response.json()?;

    Ok(format!("Spangram {}\nSolutions: {}", data.spangram, data.themeWords.join(", ")))

}

fn mini() -> Result<String, Box<dyn Error>> {

    #[derive(Deserialize)]
    struct Puzzle {
        body: Vec<Body>,
    }
 
    #[derive(Deserialize)]
    struct Body {
        cells: Vec<Cell>,
    }
    
    #[derive(Deserialize)]
    struct Cell {
        #[serde(default)] // make clues field optional
        clues: Option<Vec<u8>>,
        answer: Option<String>,
    }
   
    let response = reqwest::blocking::get("https://www.nytimes.com/svc/crosswords/v6/puzzle/mini.json")?;
    let data: Puzzle = response.json()?;

    let mut answers_list: Vec<String> = Vec::new();
    let mut row = 0;

    for cell in data.body[0].cells.iter() {
        if let Some(answer) = &cell.answer {
            if let Some(clues) = &cell.clues {
                if !clues.is_empty() && clues[0] > row {
                    row += 1;
                    answers_list.push(format!("\n{}", answer));
                } else {
                    answers_list.push(answer.clone());
                }
            }
        }
    }

    Ok(format!("Solutions across:\n{}", answers_list.concat()))
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("Enter the game you want the answers to (Wordle, Connections, Strands, Mini), or enter 'quit' to close the program.");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        
        match selection.trim().to_lowercase().as_str() {
            "wordle" => {
                println!("Today's solution is: {}", wordle()?);
            }
            "connections" => {
               println!("{}", connections()?);
            }
            "strands" => {
                println!("{}", strands()?);
            }
            "mini" => {
                println!("{}", mini()?);
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


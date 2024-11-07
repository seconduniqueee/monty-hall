use inquire::Select;
use std::io;

pub fn get_number_input(start: u64, end: u64, message: Option<&str>) -> u64 {
    if let Some(msg) = message {
        println!("{}", msg);
    }

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read the line\n");

        if let Ok(number) = input.trim().parse() {
            if number >= start && number <= end {
                return number;
            }
        }

        println!("Please, enter a number between {start} and {end}:");
    }
}


pub fn get_option_input<'a>(options: &'a Vec<GameOption<'a>>) -> Option<&'a GameOption<'a>> {
    let selection = Select::new(
        "Choose an option:",
        options.iter().map(|item| item.message).collect::<Vec<_>>(),
    )
        .prompt();

    match selection {
        Ok(message) => {
            if let Some(selected_option) = options.iter().find(|item| item.message == message) {
                return Some(selected_option);
            }

            return None;
        }
        Err(_) => {
            return None;
        }
    }
}

#[derive(Debug)]
pub enum GameOptionEnum {
    Play,
    Test,
    Keep,
    Switch,
}

#[derive(Debug)]
pub struct GameOption<'a> {
    pub option: GameOptionEnum,
    pub message: &'a str,
}

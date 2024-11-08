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

pub fn get_option_input<'opt, 'msg>(options: &'opt Vec<GameOption<'msg>>) -> Option<&'opt GameOption<'msg>> {
    let selection = Select::new(
        "Choose an option:",
        options.iter().map(|item| item.message).collect::<Vec<_>>(),
    )
        .prompt();

    return match selection {
        Ok(message) => {
            if let Some(selected_option) = options.iter().find(|item| item.message == message) {
                Some(selected_option);
            }

            None
        }
        Err(_) => {
            None
        }
    };
}

#[derive(Debug)]
pub enum GameOptionEnum {
    Play,
    Test,
    Keep,
    Switch,
}

#[derive(Debug)]
pub struct GameOption<'msg> {
    pub option: GameOptionEnum,
    pub message: &'msg str,
}

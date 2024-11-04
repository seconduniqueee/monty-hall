use inquire::Select;
use std::io;
use rand::Rng;

fn main() {
    let options = vec![
        GameOption { option: GameOptionEnum::Play, message: "Play a regular game" },
        GameOption { option: GameOptionEnum::Test, message: "Run multiple games to see the results" },
    ];

    let selected_option = match select_option(&options) {
        Some(value) => &value.option,
        None => panic!("Unable to process the input!")
    };

    match selected_option {
        GameOptionEnum::Play => play_regular_game(),
        GameOptionEnum::Test => test_problem(),
        _ => panic!("Whoops, it shouldn't have happened..."),
    }
}

fn select_option<'a>(options: &'a Vec<GameOption<'a>>) -> Option<&'a GameOption<'a>> {
    let selection = Select::new(
        "Choose an option:",
        options.iter().map(|item| item.message).collect::<Vec<_>>()
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

fn play_regular_game() {
    print_game_instructions();
    
    let prize = get_random_int(1, 3);
    let user_selection = get_user_selection();
    let revealed_door = get_revealed_door(1, 3, prize, user_selection);
    let last_door = get_last_door(1, 3, user_selection, revealed_door);

    print_followup_instructions(revealed_door, last_door);

    let is_switched = prompt_to_switch(user_selection, last_door);
    let user_selection = if is_switched { last_door } else { user_selection };

    if prize == user_selection {
        println!("\nYou won a brand new car. Congratulations!");
    } else {
        println!("\nEhh you lost, that's a bummer... But hey, now you have a pet goat!");
    }
}

fn test_problem() {
    println!("Unfortunatelly this mode is not available yet and still in development...");
}

fn print_game_instructions() {
    println!("");
    println!("Welcome to the Monty Hall game!");
    println!("Here you have 3 doors to choose from. One of the doors contains a brand new car while the other two have goats behind them.");
    println!("_____________   _____________   _____________");
    println!("|           |   |           |   |           |");
    println!("|  door#1   |   |  door#2   |   |  door#3   |");
    println!("|           |   |           |   |           |");
    println!("|           |   |           |   |           |");
    println!("|         O |   |         O |   |         O |");
    println!("|           |   |           |   |           |");
    println!("|           |   |           |   |           |");
    println!("|           |   |           |   |           |");
    println!("^^^^^^^^^^^^^   ^^^^^^^^^^^^^   ^^^^^^^^^^^^^");
    println!("");
    println!("            Please, select a door            ");
    println!("");
}

fn print_followup_instructions(door_with_goat: i32, last_door: i32) {
    println!("\nWow, that's a solid choice.");
    println!("Listen, I've got a hunch that door#{door_with_goat} is empty. Let's check it out.\n");
    
    println!("*Host opens a door number {door_with_goat}...*\n");

    println!("Yes, that's indeed a door with a goat.");
    println!("Now, listen. I'm giving you a chance to switch to door#{last_door}\n");
    println!("Would you like to switch or keep your initial choice?\n")
}

fn get_revealed_door(range_start: i32, range_end: i32, user_selection: i32, prize_position: i32) -> i32 {
    let mut available = vec![];
  
    for number in range_start..=range_end {
        if number != prize_position && number != user_selection {
            available.push(number)
        }
    }

    if available.len() == 0 {
        panic!("Whoops, it shouldn't have happened...");
    }

    let random_index = get_random_int(0, (available.len() - 1) as i32);

    return available[random_index as usize];
}

fn get_last_door(range_start: i32, range_end: i32, user_selection: i32, revealed_door: i32) -> i32 {
    for number in range_start..=range_end {
        if number != revealed_door && number != user_selection {
            return number;
        }
    }
    
    panic!("Whoops, it shouldn't have happened...");
}

fn get_random_int(start: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(start..=end);
    
    return random_number;
}

fn get_user_selection() -> i32 {        
    loop {
        let mut input = String::new();    

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line\n");

        let result: i32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please, enter a door number (1, 2 or 3):");
                continue;
            }
        };

        if result < 1 || result > 3 {
            println!("Please, enter a door number (1, 2 or 3):");
            continue;
        }

        return result;
    }
}

fn prompt_to_switch(user_selection: i32, last_door: i32) -> bool {
    let keep_message = format!("Keep the door #{}", user_selection);
    let switch_message = format!("Switch to the door #{}", last_door);
    let options = vec![
        GameOption { option: GameOptionEnum::Keep, message: keep_message.as_str() },
        GameOption { option: GameOptionEnum::Switch, message: switch_message.as_str() },
    ];
    
    let option = match select_option(&options) {
        Some(value) => &value.option,
        None => panic!("Unable to process the input!")
    };

    match option {
        GameOptionEnum::Keep => return false,
        GameOptionEnum::Switch => return true,
        _ => panic!("Whoops, it shouldn't have happened..."),
    }
}

#[derive(Debug)]
enum GameOptionEnum {
    Play,
    Test,
    Keep,
    Switch
}

#[derive(Debug)]
struct GameOption<'a> {
    option: GameOptionEnum,
    message: &'a str,
}

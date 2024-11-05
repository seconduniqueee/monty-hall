use inquire::Select;
use std::io;
use rand::Rng;

fn main() {
    let options = vec![
        GameOption { option: GameOptionEnum::Play, message: "Play a regular game" },
        GameOption { option: GameOptionEnum::Test, message: "Simulate multiple games" },
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

    println!("\n\nPress Enter to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
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

    let range_start = 1;
    let range_max = 3;
    
    let prize = get_random_int(range_start, range_max);
    let user_selection = get_user_selection(range_start, range_max);
    let revealed_door = get_revealed_door(range_start, range_max, prize, user_selection);
    let last_door = get_last_door(range_start, range_max, user_selection, revealed_door);

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
    print_test_instructions();

    let number_of_doors = get_user_selection(1, 1_000);

    println!("Enter a number of games to play (from 1 to 1 000 000):");

    let number_of_games = get_user_selection(1, 1_000_000);
    let options = vec![
        GameOption { option: GameOptionEnum::Keep, message: "Always keep initial door" },
        GameOption { option: GameOptionEnum::Switch, message: "Always switch to another door" },
    ];

    let selected_option = match select_option(&options) {
        Some(value) => &value.option,
        None => panic!("Unable to process the input!")
    };

    let to_switch = match selected_option {
        GameOptionEnum::Switch => true,
        GameOptionEnum::Keep => false,
        _ => panic!("Whoops, it shouldn't have happened..."),
    };

    let mut games_won = 0;

    println!("\nWaiting for games to be completed and processed...\n");

    for _ in 1..=number_of_games {
        let game_result = play_test_game(number_of_doors, to_switch);
        if game_result { games_won += 1; }
    }

    print_test_results(
        games_won as f64,
        number_of_games as f64,
        number_of_doors as f64,
        to_switch
    );
}

fn play_test_game(number_of_doors: u64, to_switch: bool) -> bool {
    let door_with_prize = get_random_int(1, number_of_doors);
    let user_random_selection = get_random_int(1, number_of_doors);
    let mut unrevealed_door = 0;
    let mut candidates = vec![];

    // We could just compare two numbers and get the result, but it's not how the game goes
    for door in 1..=number_of_doors {
        if door == user_random_selection { continue; }

        if door == door_with_prize {
            unrevealed_door = door;
            continue
        }

        let _ = &candidates.push(door);
    }

    // select a door what will not be revealed
    if unrevealed_door == 0 {
        let random_pick = get_random_int(1, candidates.len() as u64) - 1;
        unrevealed_door = candidates[random_pick as usize];
    }

    if to_switch && unrevealed_door == door_with_prize {
        return true;
    }

    if !to_switch && user_random_selection == door_with_prize {
        return true;
    }

    return false;
}

fn print_test_results(games_won: f64, number_of_games: f64, number_of_doors: f64, to_switch: bool) {
    let winning_percent: f64 = games_won / number_of_games * 100.0;
    let ideal_percent: f64 = (number_of_doors - 1.0) / number_of_doors * 100.0;

    println!("Here are the results:\n");
    println!("Played games: {number_of_games}.");
    println!("Number of doors to pick from: {number_of_doors}.");
    println!(
        "Games won: {games_won} ({:.2}%) when {}.\n",
        winning_percent,
        if to_switch { "switching" } else { "keeping the first pick" },
    );

    println!("The ideal percent for {number_of_doors} doors and {number_of_games} games is {ideal_percent:.2}%.\n");
}

fn print_game_instructions() {
    println!();
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
    println!();
    println!("            Please, select a door            ");
    println!();
}

fn print_test_instructions() {
    println!("In this mode we will run a certain amount of games for one particular choice - switch or keep.");
    println!("The purpose of this is to determine what is the best choice");
    println!("and basically prove the statement that switching is always beneficial");
    println!();
    println!("Select the amount of doors (from 1 to 1 000):");
}

fn print_followup_instructions(door_with_goat: u64, last_door: u64) {
    println!("\nWow, that's a solid choice.");
    println!("Listen, I've got a hunch that door#{door_with_goat} is empty. Let's check it out.\n");
    
    println!("*Host opens a door number {door_with_goat}...*\n");

    println!("Yes, that's indeed a door with a goat.");
    println!("Now, listen. I'm giving you a chance to switch to door#{last_door}\n");
    println!("Would you like to switch or keep your initial choice?\n")
}

fn get_revealed_door(range_start: u64, range_end: u64, user_selection: u64, prize_position: u64) -> u64 {
    let mut available = vec![];
  
    for number in range_start..=range_end {
        if number != prize_position && number != user_selection {
            available.push(number)
        }
    }

    if available.len() == 0 {
        panic!("Whoops, it shouldn't have happened...");
    }

    let random_index = get_random_int(0, (available.len() - 1) as u64);

    available[random_index as usize]
}

fn get_last_door(range_start: u64, range_end: u64, user_selection: u64, revealed_door: u64) -> u64 {
    for number in range_start..=range_end {
        if number != revealed_door && number != user_selection {
            return number;
        }
    }
    
    panic!("Whoops, it shouldn't have happened...");
}

fn get_random_int(start: u64, end: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(start..=end);
    
    return random_number;
}

fn get_user_selection(start: u64, end: u64) -> u64 {
    loop {
        let mut input = String::new();    

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line\n");

        let result: u64 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please, enter a number between {start} and {end}:");
                continue;
            }
        };

        if result < start || result > end {
            println!("Please, enter a number between {start} and {end}:");
            continue;
        }

        return result;
    }
}

fn prompt_to_switch(user_selection: u64, last_door: u64) -> bool {
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

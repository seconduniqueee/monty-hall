pub fn print_game_instructions() {
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

pub fn print_game_followup_instructions(door_with_goat: u64, last_door: u64) {
    println!("\nWow, that's a solid choice.");
    println!("Listen, I've got a hunch that door#{door_with_goat} is empty. Let's check it out.\n");

    println!("*Host opens a door number {door_with_goat}...*\n");

    println!("Yes, that's indeed a door with a goat.");
    println!("Now, listen. I'm giving you a chance to switch to door#{last_door}\n");
    println!("Would you like to switch or keep your initial choice?\n")
}

pub fn print_test_instructions() {
    println!("In this mode we will run a certain amount of games for one particular choice - switch or keep.");
    println!("The purpose of this is to determine what is the best choice");
    println!("and basically prove the statement that switching is always beneficial");
    println!();
}

pub fn print_game_results(prize_placement: u64, selected_door: u64) {
    if prize_placement == selected_door {
        println!("\nYou won a brand new car. Congratulations!");
    } else {
        println!("\nEhh you lost, that's a bummer... But hey, now you have a pet goat!");
    }
}

pub fn print_test_results(
    games_won: f64,
    number_of_games: f64,
    number_of_doors: f64,
    to_switch: bool,
) {
    let winning_percent: f64 = games_won / number_of_games * 100.0;
    let ideal_percent: f64 = (number_of_doors - 1.0) / number_of_doors * 100.0;

    println!("Here are the results:\n");
    println!("--- Games played: {number_of_games}.");
    println!("--- Number of doors to pick from: {number_of_doors}.");
    println!(
        "--- Games won: {games_won} ({:.2}%) when {}.\n",
        winning_percent,
        if to_switch { "switching" } else { "keeping the first pick" },
    );

    println!("--- The ideal percent for {number_of_doors} doors and {number_of_games} games is {ideal_percent:.2}%.\n");
}


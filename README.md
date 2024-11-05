# Monty Hall Game

Welcome to the Monty Hall Game! This is a console application written in Rust that allows users to play the classic Monty Hall problem. In this game, players choose between three doors, behind one of which is a brand new car, while the others hide goats.

## Goals

- [x] **Implement regular Monty Hall game.**
- [x] **Implement a test mode to see the outcome with a given number of doors and games.**

## Description

The Monty Hall problem is a probability puzzle based on a game show scenario. Here's a brief overview of how the game works:

1. A contestant is presented with three doors. Behind one door is a car (the prize), and behind the other two doors are goats.
2. The contestant selects one door.
3. The host, who knows what's behind each door, opens another door that contains a goat.
4. The contestant is then given the option to either stick with their original choice or switch to the remaining unopened door.
5. The question is: Is it to the contestant's advantage to switch?

For a deeper understanding of the Monty Hall problem, check out these resources:
- [Monty Hall Problem on Wikipedia](https://en.wikipedia.org/wiki/Monty_Hall_problem)
- [Understanding the Monty Hall Problem](https://betterexplained.com/articles/understanding-the-monty-hall-problem/)

## Features

- **Regular Mode:** Players can engage in a classic Monty Hall game with 3 doors.
- **Test Mode:** Users can simulate a specified number of games and analyze the results to see how switching or sticking with the initial choice affects outcomes.

## Getting Started

To get started with the Monty Hall game, clone the repository and run the Rust console application:

```bash
git clone https://github.com/seconduniqueee/monty-hall.git
cd monty-hall
cargo run

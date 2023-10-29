#![allow(nonstandard_style)]
use rand::Rng;
use std::{io, process};

fn main() {
    println!("\n\n\nWelcome to a Tic-Tac-Toe game, written entirely in the Rust programming language.");
    println!("You will see this board printed every turn:");
    let example_board: [&str; 9] = ["1","2","3","4","5","6","7","8","9"];
    print_board(example_board);
    println!("When it's your turn, enter the correlated number to place your token.\n");

    let mut squares_left: usize = 9;
    let mut board_tokens: [&str; 9] = ["-"; 9];
    let turn = rand::thread_rng().gen_range(0..2);
    let mut userToken = "X";
    let mut userInput = String::new();
    let victory: bool = false; 

    if turn == 0 {
        println!("You will be playing FIRST.");
    }
    
    else {
        println!("You will be playing SECOND.");
        userToken = "O";
        board_tokens = opponent_turn(board_tokens, squares_left, userToken);
        print_board(board_tokens);
    }
    
    println!("Your symbol will be: {}\n", userToken);
    

    while victory == false {
        userInput.clear();
        io::stdin().read_line(&mut userInput).expect("failed to readline");
        let inputIndex: usize = userInput.trim().parse().expect("Input is not an integer");

        if board_tokens[inputIndex-1] == "-" {
            board_tokens[inputIndex - 1] = userToken;
            print_board(board_tokens);
            win(board_tokens, userToken);
            squares_left = one_less_square(squares_left, board_tokens, userToken);

            board_tokens = opponent_turn(board_tokens, squares_left, userToken);
            print_board(board_tokens);
            win(board_tokens, userToken);
            squares_left = one_less_square(squares_left, board_tokens,userToken);
        }

        else {
            println!("Please choose a valid square.")
        }
    }    
}  

fn print_board(arr: [&str; 9]) {
    let divider = "----------";
    println!("\n {} | {} | {} \n{}\n {} | {} | {} \n{}\n {} | {} | {} \n", arr[0], arr[1], arr[2], divider, arr[3], arr[4], arr[5], divider, arr[6], arr[7], arr[8])
}

fn opponent_turn<'a>(mut arr: [&'a str; 9], squares_left: usize, userToken: &'a str) -> [&'a str; 9]{
    let computerIndex: usize = rand::thread_rng().gen_range(0..9);
    if arr[computerIndex] == "-" {
        if userToken == "X" {
            arr[computerIndex] = "O";
        }

        else {
            arr[computerIndex] = "X";
        }
    }

    else {
        arr = opponent_turn(arr, squares_left, userToken)
    }
    return arr;
}

fn win(arr: [&str; 9], userToken: &str) {
    let mut victory: bool = false;
    let mut winningToken = "";
    if arr[0] == arr[1] && arr[1] == arr[2] && arr[0] != "-" {
        victory = true;
        winningToken = arr[0];
    }
    else if arr[0] == arr[3] && arr[3] == arr[6] && arr[0] != "-" {
        victory = true;
        winningToken = arr[0];
    }
    else if arr[0] == arr[4] && arr[4] == arr[8] && arr[0] != "-" {
        victory = true;
        winningToken = arr[0];
    }
    else if arr[1] == arr[4] && arr[4] == arr[7] && arr[1] != "-" {
        victory = true;
        winningToken = arr[1];
    }
    else if arr[2] == arr[5] && arr[5] == arr[8] && arr[2] != "-" {
        victory = true;
        winningToken = arr[2];
    }
    else if arr[3] == arr[4] && arr[4] == arr[5] && arr[3] != "-" {
        victory = true;
        winningToken = arr[3];
    }
    else if arr[6] == arr[4] && arr[4] == arr[2] && arr[6] != "-" {
        victory = true;
        winningToken = arr[6];
    }
    else if arr[6] == arr[7] && arr[7] == arr[8] && arr[6] != "-" {
        victory = true;
        winningToken = arr[6];
    }
    if victory == true {
        if winningToken == userToken {
            println!("\nCongrats, you won!\n\n")
        }
        else {
            println!("\nBetter luck next time!\n\n")
        }
        process::exit(1);
    }
    else {}
}

fn one_less_square(mut squares_left: usize, arr: [&str; 9], userToken: &str) -> usize{
    squares_left -= 1;
    if squares_left == 0 {
        win(arr, userToken);
        println!("It's a tie!");
        process::exit(1);
    }
    return squares_left
}
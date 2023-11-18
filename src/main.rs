extern crate ncurses;
extern crate serde;
extern crate serde_json;

mod decode_pcd;
mod art;

use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

use ncurses::*;
use serde_json::Value;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time;

use decode_pcd::decode_pcd;
use tandem_http_client::{compute, MpcData, MpcProgram};
use tokio;

fn print_in_center(string: &str) {
    let row = LINES() / 2; // Calculate the row to center the text vertically
    let col = (COLS() - string.len() as i32) / 2; // Calculate the column to center the text horizontally

    mvprintw(row, col, string); // Print the text at the calculated position
}

#[tokio::main]
async fn main() {
    // init text2art
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);

    // start ncurses
    initscr();
    raw();

    // set print color
    start_color(); // Start color functionality
    init_pair(1, COLOR_GREEN, COLOR_BLACK); // Initialize a color pair
    attron(COLOR_PAIR(1)); // Use the color pair

    printw(art::frog_fight); // Print the ASCII art
    refresh(); // Refresh the screen to display the ASCII art

    printw("\n -------------------------\n");
    printw("  Welcome to BattleFrogz!");
    printw("\n -------------------------\n");
    printw("\n Here you can battle your Zupass frogs against other frogs using 2PC!\n");

    printw("\n Copy your favourite frog's PCD from zupass and paste it here:\n\n");

    refresh();

    // Get JSON input
    let mut json_str = String::new();

    loop {
        let ch = getch();
        if ch == '\n' as i32 {
            break;
        }
        json_str.push(ch as u8 as char);
    }

    deleteln(); // Delete the line

    let (intelligence, beauty, speed, jump) = decode_pcd(&json_str).unwrap_or_else(|| {
        printw("Invalid input\n");
        refresh();
        endwin();
        panic!("Invalid input");
    });

    let input = format!(
        "Frog {{ jmp: {}u8, spd: {}u8, int: {}u8, bty: {}u8 }}",
        intelligence, beauty, speed, jump
    );

    refresh();

    clear();
    printw(art::frog_opponent);
    printw("\n -------------------------\n");
    printw(" Choose your opponent!  ");
    printw("\n -------------------------\n");

    printw("\n You will not be able to know your opponents attributes, but you can be guaranteed that they are valid!\n\n");

    printw("\n Choices:\n");
    printw("\n 1. Frog 1");
    printw("\n 2. Frog 2");
    printw("\n 3. Frog 3\n\n");
    refresh();

    let mut frog_to_fight = "frog_1";
    let choice = getch();
    getch();
    match choice {
        49 => frog_to_fight = "frog_1",
        50 => frog_to_fight = "frog_2",
        51 => frog_to_fight = "frog_3",
        _ => {
            printw("Invalid input\n");
            refresh();
            endwin();
            panic!("Invalid input");
        }
    }

    clear();
    // Confirmation
    printw(art::frog_fight);
    printw("\n -------------------------\n");
    printw("  Are you ready to fight?  ");
    printw("\n -------------------------\n");
    printw(format!("\n You will be fighting {}!\n\n", frog_to_fight).as_str());
    printw("\n Press any key to continue\n\n");
    refresh();

    getch();


    let url = "http://127.0.0.1:8000";

    let battle_prg = include_str!("../program.garble.rs");

    let function = "main";

    let program = MpcProgram::new(battle_prg.to_string(), function.to_string())
        .expect("Could not parse source code");
    let my_input =
        MpcData::from_string(&program, input.to_string()).unwrap_or_else(|e| panic!("{e}"));
    let result = compute(
        url.to_string(),
        frog_to_fight.to_string(),
        program,
        my_input,
    )
    .await
    .unwrap_or_else(|e| panic!("{e}"))
    .to_literal_string();

    clear();
    refresh();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Hide the cursor


    // set text based on result
    match result.as_str() {
        "Result::ClientWin" => {
            printw(art::frog_winner);
        }
        "Result::ClientLoss" => {
            printw(art::frog_loser);
        }
        "Result::Tie" => {
            printw(art::frog_fight_tie);
        }
        _ => {
             printw("Something went wrong");
        }
    }

    getch();
    refresh();
    endwin();
}

// cargo run -- c 2
// cargo run -- p

//      CONTROLS:
// | -------------- | -------------------------------------- |
// | arrows         | navigate the field                     |
// | enter or space | make a move                            |
// | u (U)          | undo (available only against computer) |
// | esc            | leave the game                         |

use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
};
use clap::Parser;

mod ui;
mod game;
mod config;
mod ai;

use crate::ui::render_field;
use crate::game::check_win;
use crate::config::FIELD_SIZE;
use crate::ai::computer_move;


#[derive(Parser)]
struct Args {
    // Game mode
    // c - computer
    // p - 2 players 
    mode: char,

    // Difficulty level
    // 0 - Random
    // 4 - First palyable
    // 9 - The most difficult 
    #[arg(default_value_t = 9)]
    difficulty_level: usize,
}


fn main() {
    let args = Args::parse();

    let mode = args.mode;
    let difficulty_level = args.difficulty_level;

    // create 2d array 15x15 and fill it with ' '
    let mut field = [[' '; FIELD_SIZE]; FIELD_SIZE];
    let mut turn = 'X';

    let mut cursor_x = 7;
    let mut cursor_y = 7;

    let mut computer_move_x = usize::MAX;
    let mut computer_move_y = usize::MAX;

    let mut win = ' ';

    let mut move_history = vec![];

    // track if field has changed to remove unnessesary rerenders
    let mut cursor_moved = true;

    // Main loop
    loop {
        if cursor_moved == true {
            render_field(
                &field, 
                cursor_x, cursor_y, 
                Some(turn), None, 
                computer_move_x, computer_move_y,
                mode, difficulty_level
            );
            cursor_moved = false
        }

        // If human turn -> read input
        if (mode == 'c' && turn == 'X') || mode == 'p' {
            // Read input
            if let Event::Key(key_event) = read().unwrap() {

                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match key_event.code {
                    KeyCode::Up => if cursor_y > 0 { cursor_y -= 1; cursor_moved = true },
                    KeyCode::Down => if cursor_y < FIELD_SIZE - 1 { cursor_y += 1; cursor_moved = true },
                    KeyCode::Left => if cursor_x > 0 { cursor_x -= 1; cursor_moved = true },
                    KeyCode::Right => if cursor_x < FIELD_SIZE - 1 { cursor_x += 1; cursor_moved = true },
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        
                        if field[cursor_y][cursor_x] == ' ' {
                        
                            field[cursor_y][cursor_x] = turn;
                            move_history.push((cursor_x, cursor_y));
                            (win, _, _, _) = check_win(&field, turn);
                            turn = if turn == 'X' { 'O' } else { 'X' };
                        
                        
                            if win == 'X' || win == 'O' {
                                render_field(
                                    &field, 
                                    cursor_x, cursor_y, 
                                    None, Some(win), 
                                    computer_move_x, computer_move_y, 
                                    mode, difficulty_level
                                );
                                break
                            }
                            else {
                                render_field(
                                    &field, 
                                    cursor_x, cursor_y, 
                                    Some(turn), None, 
                                    computer_move_x, computer_move_y,
                                    mode, difficulty_level
                                );
                            }
                        }
                    },
                    KeyCode::Char('u') | KeyCode::Char('U') => {

                        // Undo is available only for playing against computer
                        if mode == 'c' {

                            if move_history.len() >= 2 {
                                for _ in 0..2 {
                                    let (x, y) = move_history[move_history.len() - 1];
                                    field[y][x] = ' ';
                                    move_history.pop();
                                }
                            }

                            if move_history.len() >= 3 {
                                (computer_move_x, computer_move_y) = move_history[move_history.len() - 1];
                            }
                            else {
                                (computer_move_x, computer_move_y) = (usize::MAX, usize::MAX)
                            }
                            
                            render_field(
                                &field, 
                                cursor_x, cursor_y, 
                                Some(turn), None, 
                                computer_move_x, computer_move_y,
                                mode, difficulty_level
                            );
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        // Computer move
        else {

            (computer_move_x, computer_move_y) = computer_move(&mut field, difficulty_level);
            
            field[computer_move_y][computer_move_x] = 'O';
            move_history.push((computer_move_x, computer_move_y));
            (win, _, _, _) = check_win(&field, turn);
            turn = 'X';
            
            if win == 'X' || win == 'O' {

                render_field(
                    &field, 
                    cursor_x, cursor_y, 
                    None, Some(win), 
                    computer_move_x, computer_move_y,
                    mode, difficulty_level
                );
                break
            }

            render_field(
                &field, 
                cursor_x, cursor_y, 
                Some(turn), None, 
                computer_move_x, computer_move_y,
                mode, difficulty_level
            );
        }
    }
}
use crossterm::{
execute,
terminal::{Clear, ClearType},
style::{SetForegroundColor, Color, ResetColor, Print, SetBackgroundColor, SetAttribute, Attribute},
};
use std::io::{stdout};

use crate::game::check_win;
use crate::config::FIELD_SIZE;


pub fn render_field(
    field: &[[char; FIELD_SIZE]; FIELD_SIZE], 
    cursor_x: usize, 
    cursor_y: usize, 
    turn: Option<char>, 
    win: Option<char>, 
    computer_move_x: usize, 
    computer_move_y: usize,
    mode: char,
    difficulty_level: usize,
) {
    // turn and win are optional parameters 
    let turn = turn.unwrap_or(' ');
    let win = win.unwrap_or(' ');

    clear_screen();

    // declare variables to store coordinates in case of winning
    let mut win_coords = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];

    // if no win yet
    if turn == 'X' || turn == 'O' {
        
        print!("\t\tNow is the turn of ");
        
        if turn == 'X' {
            highlight(&format!(" {} ", turn), 'r');
        }
        else if turn == 'O' {
            highlight(&format!(" {} ", turn), 'b');
        }
        
        print!("\n\n");
    }
    // if win
    else {
        
        print!("\t\tThe winner is ");
        highlight(&format!(" {} ", win), 'g');
        print!("\n\n");

        // find where win cells are located to highlight them
        let (_, y, x, d) = check_win(&field, win);

        // horizontal (right)
        if d == 'h' {
            win_coords = [(y, x), (y, x+1), (y, x+2), (y, x+3), (y, x+4)]
        }
        // vertical (down)
        else if d == 'v' {
            win_coords = [(y, x), (y+1, x), (y+2, x), (y+3, x), (y+4, x)]
        }
        // diagonal (down-right)
        else if d == 'd' {
            win_coords = [(y, x), (y+1, x+1), (y+2, x+2), (y+3, x+3), (y+4, x+4)]
        }
        // diagonal (up-right)
        else if d == 'u' {
            win_coords = [(y, x), (y-1, x+1), (y-2, x+2), (y-3, x+3), (y-4, x+4)]
        }
    }

    // for every row
    for i in 0..FIELD_SIZE {
    
        // print ──┼───┼───┼── between the lines
        print!("──┼");
        for k in 0..FIELD_SIZE {
            print!("───┼");
        }
        print!("──\n");

        // for every cell
        print!("  |");
        for j in 0..FIELD_SIZE {
            // if this area is under cursor -> highlight
            if i == cursor_y && j == cursor_x {
                if turn == 'X' || win == 'X' || mode == 'c' {
                    highlight(&format!(" {} ", field[i][j]), 'r')
                }
                else if turn == 'O' || win == 'O' {
                    highlight(&format!(" {} ", field[i][j]), 'b')
                }
            }
            // if this area is last computer move -> blue
            else if mode == 'c' && i == computer_move_y && j == computer_move_x {
                highlight(&format!(" {} ", field[i][j]), 'b')
            }
            // if win cell
            else if win != ' ' && win_coords.contains(&(i, j)) {
                highlight(&format!(" {} ", field[i][j]), 'g')
            } 
            // if none above -> no highlight
            else {
                print!(" {} ", field[i][j]);            
            }

            print!("|");
        }
        print!("  \n");
    }

    // print ──┼───┼───┼── in the end
    print!("──┼");
    for k in 0..FIELD_SIZE {
        print!("───┼");
    }
    print!("──\n");


    // print info about the mode you play with
    let message = if mode == 'c' {format!("\tYou play against computer with difficulty level {}/9", difficulty_level)} else {{format!("\t\tYou play against another person")}};
    println!("\n{}", message);
}


fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}


fn highlight(text: &str, color: char) {

    let color = match color {
        'r' => Color::Red,
        'g' => Color::Green,
        'b' => Color::Blue,
        _ => Color::Black,
    };

    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(text),
        SetAttribute(Attribute::Reset),
        ResetColor
    ).unwrap();

}
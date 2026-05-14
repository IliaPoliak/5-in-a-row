use rand::seq::SliceRandom;

use crate::config::FIELD_SIZE;


pub fn computer_move(
    field: &mut [[char; FIELD_SIZE]; FIELD_SIZE],
    difficulty_level: usize,
) -> (usize, usize) {

    // find moves close to already existing (to save calculations)
    let mut moves = vec![];
    for y in 0..FIELD_SIZE {
        for x in 0..FIELD_SIZE {
            if field[y][x] == ' ' && has_neighbor(field, x, y) {    
                moves.push((x, y));
            }
        }
    }

    let mut best_score = i32::MIN;
    let mut best_moves = vec![];

    for m in &moves {
        let x = m.0;
        let y = m.1;

        // Try move
        field[y][x] = 'O';

        // Evaluate move
        let score = evaluate(field, difficulty_level);

        // Undo move
        field[y][x] = ' ';

        if score > best_score {
            best_score = score;
            best_moves.clear();
            best_moves.push((x, y));
        } else if score == best_score {
            best_moves.push((x, y));
        }
    }

    // select random move from the best moves
    let mut rng = rand::thread_rng();
    if let Some(&best_move) = best_moves.choose(&mut rng) {
        return best_move;    
    }
    else {
        return (usize::MAX, usize::MAX)
    }


}


fn evaluate(field: &[[char; FIELD_SIZE]; FIELD_SIZE], difficulty_level: usize) -> i32 {
    let mut score = 0;

    for y in 0..FIELD_SIZE {
        for x in 0..FIELD_SIZE {
            score += evaluate_cell(field, x, y, difficulty_level);
        }
    }

    score
}


fn evaluate_cell(field: &[[char; FIELD_SIZE]; FIELD_SIZE], x: usize, y: usize, difficulty_level: usize) -> i32 {

    let player = field[y][x];

    let diff = 9 - difficulty_level;
    let mut score_multiplier = 1.0;
    for i in 0..diff {
        score_multiplier *= 0.1;
    }
    
    
    if diff == 0 { 1.0 } else { 0.1 * diff as f64 };

    let mut score = 0;

    // directions: (dx, dy)
    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

    // check 5 cells window in each direction
    for (dx, dy) in directions.iter() {

        // if out of scope go to the next
        let mut nx = x as isize + (dx * 4);
        let mut ny = y as isize + (dy * 4);
        if nx < 0 || nx >= FIELD_SIZE as isize ||
           ny < 0 || ny >= FIELD_SIZE as isize 
        {
            continue
        }

        let mut o_count = 0;
        let mut x_count = 0;
        let mut empty_count = 0;

        let mut nx = x as isize;
        let mut ny = y as isize;
        for i in 0..5 {
            if field[ny as usize][nx as usize] == 'O' {
                o_count += 1;
            }
            else if field[ny as usize][nx as usize] == 'X' {
                x_count += 1
            }
            else {
                empty_count += 1
            }
            nx += dx;
            ny += dy;
        }
        
//      | Computer cells | Score        |
//      | -------------- | ------------ |
//      | 5              | +100_000_000 |
//      | 4              | +1_000_000   |
//      | 3              | +10_000      |
//      | 2              | +100         |
//      | 1              | +1           |
        if o_count == 5 { 
            score += (100_000_000.0 * score_multiplier) as i32;
         }
        if o_count == 4 && empty_count == 1 { score += (1_000_000.0 * score_multiplier) as i32 }
        if o_count == 3 && empty_count == 2 { score += (10_000.0 * score_multiplier) as i32 }
        if o_count == 2 && empty_count == 3 { score += (100.0 * score_multiplier) as i32 }
        if o_count == 1 && empty_count == 4 { score += (1.0 * score_multiplier) as i32 }

//      | Oponent's cells | Score       |
//      | --------------- | ----------- |
//      | 4               | -10_000_000 |
//      | 3               | -100_000    |
//      | 2               | -1_000      |
//      | 1               | -10         |
        if x_count == 4 && empty_count == 1 { score -= (10_000_000.0 * score_multiplier) as i32 }
        if x_count == 3 && empty_count == 2 { score -= (100_000.0 * score_multiplier) as i32 }
        if x_count == 2 && empty_count == 3 { score -= (1_000.0 * score_multiplier) as i32 }
        if x_count == 1 && empty_count == 4 { score -= (10.0 * score_multiplier) as i32 }
    }

    score
}


fn has_neighbor(field: &[[char; FIELD_SIZE]; FIELD_SIZE], x: usize, y: usize) -> bool {
    
    let x = x as isize;
    let y = y as isize;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }

            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && ny >= 0 &&
               nx < FIELD_SIZE as isize &&
               ny < FIELD_SIZE as isize {

                if field[ny as usize][nx as usize] != ' ' {
                    return true;
                }
            }
        }
    }
    false
}
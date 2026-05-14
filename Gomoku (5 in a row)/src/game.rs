use crate::config::FIELD_SIZE;

pub fn check_win(field: &[[char; FIELD_SIZE]; FIELD_SIZE], player: char) -> (char, usize, usize, char) {
    for row in 0..FIELD_SIZE {
        for col in 0..FIELD_SIZE {

            // Check if the cell belongs to the last player
            if field[row][col] != player {
                continue;
            }

            // Check horizontal (right)
            // check bounderies
            if col + 4 < FIELD_SIZE {
                let mut count = 0;
                // check the 5 fields right
                for i in 0..5 {
                    if field[row][col + i] == player {
                        count += 1;
                    }
                }
                // check how many fields
                if count == 5 {
                    return (player, row, col, 'h');
                }
            }

            // Check vertical (down)
            // check bounderies
            if row + 4 < FIELD_SIZE {
                let mut count = 0;
                // check the 5 fields down
                for i in 0..5 {
                    if field[row + i][col] == player {
                        count += 1;
                    }
                }
                // check how many fields
                if count == 5 {
                    return (player, row, col, 'v');
                }
            }

            // Check diagonal (down-right)
            // check bounderies
            if row + 4 < FIELD_SIZE && col + 4 < FIELD_SIZE {
                let mut count = 0;
                // check the 5 fields down-right
                for i in 0..5 {
                    if field[row + i][col + i] == player {
                        count += 1;
                    }
                }
                // check how many fields
                if count == 5 {
                    return (player, row, col, 'd');
                }
            }

            // Check diagonal (up-right)
            // check bounderies
            if row >= 4 && col + 4 < FIELD_SIZE {
                let mut count = 0;
                // check the 5 fields down-right
                for i in 0..5 {
                    if field[row - i][col + i] == player {
                        count += 1;
                    }
                }
                // check how many fields
                if count == 5 {
                    return (player, row, col, 'u');
                }
            }
        }
    }

    // No winner
    (' ', 0, 0, ' ')
}


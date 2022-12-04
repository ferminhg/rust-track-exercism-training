const MINE_MARK:char = '*'; 

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mine_matrix = matrix_builder(minefield);
    for row in 0..mine_matrix.len() {
        for col in 0..mine_matrix[0].len() {
            if mine_matrix[row][col] == MINE_MARK { continue; }
            let bombs = mark_adj( &mine_matrix, row, col);
            if bombs > 0 {
                mine_matrix[row][col] = char::from_digit(bombs as u32, 10).unwrap();
            }
        }
    }
    matrix_to_vstring(mine_matrix)
}

fn matrix_to_vstring(mine_matrix: Vec<Vec<char>>) -> Vec<String> {
    mine_matrix.iter().map(|v| {
        (*v).iter().map(|c| *c).collect::<String>()
    }).collect::<Vec<String>>()
}

fn mark_adj<'a>(map: &'a Vec<Vec<char>>, col: usize, row: usize) -> usize {
    let x: isize = col as isize;
    let y: isize = row as isize;
    vec![(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1),
         (x+1, y-1), (x+1, y), (x+1, y+1), ]
        .into_iter()
        .filter(|pos| 
                                { 
                                is_valid_pos(pos.0, pos.1, map.len(), map[0].len()) && map[pos.0 as usize][pos.1 as usize] == MINE_MARK }).count()
}

fn is_valid_pos(x: isize, y: isize, max_x: usize, max_y: usize) -> bool {
    x >= 0 && y >= 0 && x < max_x as isize && y < max_y as isize
}

fn matrix_builder(minefield: &[&str]) -> Vec<Vec<char>> {
    let mut mine_matrix:Vec<Vec<char>> = vec![];
    for s in minefield {
        mine_matrix.push(s.chars().collect());
    }
    mine_matrix
}

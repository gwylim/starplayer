use std::collections::HashMap;

// Static precomputed data about the board
pub struct BoardInfo {
    // The length of a side
    pub size: usize,
    // The maximum value of coordinates + 1
    pub coords_range: usize,
    // The total number of points on the board
    pub count: usize,
    // Adjacency lists
    pub adjacencies: Vec<Vec<usize>>,
    // Coordinates
    pub coords: Vec<(usize, usize)>,
    // Map from index to coordinates
    pub reverse_coords: HashMap<(usize, usize), usize>,
}

/*
 * size = 3 board
 *
 *  0 1 2 3 4
 * 0 _ _ O O O
 *  1 _ O O O O
 *   2 O O O O O
 *    3 O O O O _
 *     4 O O O _ _
 */
fn in_bounds(size: usize, x: usize, y: usize) -> bool {
    let coords_range = size + size - 1;
    if x >= coords_range || y >= coords_range {
        return false;
    }
    let s = x + y;
    s >= size - 1 && s < 3 * size - 2
}

impl BoardInfo {
    pub fn new(size: usize) -> BoardInfo {
        let coords_range = size + size - 1;
        let count = coords_range * coords_range - size * (size - 1);
        let mut coords = Vec::with_capacity(count);
        let mut adj = Vec::with_capacity(count);
        let mut reverse_coords: HashMap<(usize, usize), usize> = HashMap::new();
        // TODO: remove duplication of this loop with in_bounds checks
        for x in 0..coords_range {
            for y in 0..coords_range {
                if !in_bounds(size, x, y) {
                    continue;
                }
                let point_number = coords.len();
                coords.push((x, y));
                reverse_coords.insert((x, y), point_number);
            }
        }
        for x in 0..coords_range {
            for y in 0..coords_range {
                if !in_bounds(size, x, y) {
                    continue;
                }
                let mut point_adj = Vec::new();
                for dx in -1..2 {
                    for dy in -1..2 {
                        if dx == 0 && dy == 0 || dx * dy == 1 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx < 0 || ny < 0 || !in_bounds(size, nx as usize, ny as usize) {
                            continue;
                        }
                        point_adj.push(*reverse_coords.get(&(nx as usize, ny as usize)).unwrap());
                    }
                }
                adj.push(point_adj);
            }
        }
        BoardInfo {
            size: size,
            coords_range: coords_range as usize,
            count: count,
            adjacencies: adj,
            coords: coords,
            reverse_coords: reverse_coords,
        }
    }

    pub fn on_boundary(&self, point: usize) -> bool {
        match self.coords.get(point) {
            None => {
                panic!("Not found")
            },
            Some(&(x, y)) => {
                x == 0 || y == 0 ||
                x == self.coords_range - 1 || y == self.coords_range - 1 ||
                x + y == self.size - 1 || x + y == 3 * self.size - 3
            }
        }
    }
}

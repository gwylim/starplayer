use std::collections::HashMap;

use boardvec::BoardVec;

/// Static precomputed data about the board
pub struct BoardInfo {
    /// The length of a side
    pub size: usize,
    /// The maximum value of coordinates + 1
    pub coords_range: usize,
    /// The total number of points on the board
    pub count: usize,
    /// Adjacency lists
    pub adjacencies: Vec<Vec<usize>>,
    // TODO: convert all coordinates to use signed values always
    /// Coordinates of each point
    pub coords: Vec<(usize, usize)>,
    /// Map from index to coordinates
    pub reverse_coords: HashMap<(usize, usize), usize>,
    /// Patterns to be used in playouts. Patterns are local; for each possible position of the
    /// opponent's last move, we have a list of patterns that could be applicable.
    pub patterns: Vec<Vec<Pattern>>,
}

/// A pattern to be used in playouts, e.g. protect bridges
#[derive(Copy, Clone)]
pub struct Pattern {
    /// Cells which must be occupied by us for the pattern to be applicable
    pub ours: BoardVec,
    /// Cells which must not be occupied for the pattern to be applicable
    pub empty: BoardVec,
    /// Point to play
    pub to_play: usize,
}

impl Pattern {
    /// Check whether pattern is applicable given the points occupied by each player
    pub fn check(&self, last_player: &BoardVec, current_player: &BoardVec) -> Option<usize> {
        if !last_player.intersects(&self.empty) && !current_player.intersects(&self.empty) && current_player.contains(&self.ours) {
            Some(self.to_play)
        } else {
            None
        }
    }
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

// FIXME: don't take coords_range
fn on_boundary(size: usize, coords_range: usize, coords: &Vec<(usize, usize)>, point: usize) -> bool {
    match coords.get(point) {
        None => {
            panic!("Not found")
        },
        Some(&(x, y)) => {
            x == 0 || y == 0 ||
            x == coords_range - 1 || y == coords_range - 1 ||
            x + y == size - 1 || x + y == 3 * size - 3
        }
    }
}

// Offsets of adjacent points *in order* around a point
const ADJACENT: [(isize, isize); 6] = [(-1, 0), (-1, 1), (0, 1), (1, 0), (1, -1), (0, -1)];

fn get_adjacent(size: usize, x: usize, y: usize, i: usize) -> Option<(usize, usize)> {
    let (dx, dy) = ADJACENT[i];
    let nx = x as isize + dx;
    let ny = y as isize + dy;
    if nx < 0 || ny < 0 {
        return None;
    }
    if !in_bounds(size, nx as usize, ny as usize) {
        return None;
    }
    Some((nx as usize, ny as usize))
}

impl BoardInfo {
    pub fn new(size: usize) -> BoardInfo {
        let coords_range = size + size - 1;
        let count = coords_range * coords_range - size * (size - 1);
        let mut coords = Vec::with_capacity(count);
        let mut adj = Vec::with_capacity(count);
        let mut reverse_coords: HashMap<(usize, usize), usize> = HashMap::new();
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
        for i in 0..count {
            let (x, y) = coords[i];
            let mut point_adj = Vec::new();
            for j in 0..ADJACENT.len() {
                if let Some((nx, ny)) = get_adjacent(size, x, y, j) {
                    point_adj.push(*reverse_coords.get(&(nx, ny)).unwrap());
                }
            }
            adj.push(point_adj);
        }
        let mut patterns = Vec::new();
        // Bridge patterns
        for i in 0..count {
            let (x, y) = coords[i];
            let mut adjacent1 = get_adjacent(size, x, y, ADJACENT.len() - 2);
            let mut adjacent2 = get_adjacent(size, x, y, ADJACENT.len() - 1);
            let mut patterns_for_point = Vec::new();
            for j in 0..ADJACENT.len() {
                let adjacent3 = get_adjacent(size, x, y, j);
                match (adjacent1, adjacent2, adjacent3) {
                    (Some(coords1), Some(coords2), Some(coords3)) => {
                        let mut ours = BoardVec::new();
                        ours.set(*reverse_coords.get(&coords1).unwrap());
                        ours.set(*reverse_coords.get(&coords3).unwrap());
                        let mut empty = BoardVec::new();
                        let to_play = *reverse_coords.get(&coords2).unwrap();
                        empty.set(to_play);
                        patterns_for_point.push(Pattern {
                            ours: ours,
                            empty: empty,
                            to_play: to_play,
                        });
                    },
                    (Some(coords1), Some(coords2), None) => {
                        let point1 = *reverse_coords.get(&coords1).unwrap();
                        let point2 = *reverse_coords.get(&coords2).unwrap();
                        // FIXME: we're converting to index and then converting back in on_boundary
                        let mut empty = BoardVec::new();
                        let to_play;
                        if on_boundary(size, coords_range, &coords, point1) {
                            to_play = point1;
                            empty.set(point1);
                        } else {
                            to_play = point2;
                            empty.set(point2);
                        }
                        patterns_for_point.push(Pattern {
                            ours: BoardVec::new(),
                            empty: empty,
                            to_play: to_play,
                        });
                    },
                    _ => {
                    },
                }
                adjacent1 = adjacent2;
                adjacent2 = adjacent3;
            }
            patterns.push(patterns_for_point);
        }
        BoardInfo {
            size: size,
            coords_range: coords_range as usize,
            count: count,
            adjacencies: adj,
            coords: coords,
            reverse_coords: reverse_coords,
            patterns: patterns,
        }
    }

    /// Whether a given point is on the edge of the board
    pub fn on_boundary(&self, point: usize) -> bool {
        on_boundary(self.size, self.coords_range, &self.coords, point)
    }
}

#[derive(Debug)]
struct Floor {
    matrix: Vec<Vec<u64>>
}

impl Floor {
    /// Reads the lines as a height x width matrix
    fn from_input_lines(lines: &Vec<String>) -> Self {
        let matrix = lines
            .iter()
            .map(|line| line.chars()
                // this is a trick to transform the character (ASCII POS)
                // to its actual value ('0' -> 0)
                .map(|c| (c as u64 - '0' as u64) as u64)
                .collect())
            .collect::<Vec<Vec<u64>>>();

        return Floor {
            matrix
        }
    }

    /// Gets a single element (n, m)
    pub fn get(&self, x: usize, y: usize) -> u64 {
        self.matrix.get(y).unwrap().get(x).unwrap().clone()
    }

    /// width of the matrix (m)
    pub fn width(&self) -> usize {
        self.matrix.get(0).unwrap().len()
    }

    /// height of the matrix
    pub fn height(&self) -> usize {
        self.matrix.len()
    }

    /// starts at [pos] and moves to all neighbour fields if (recursive call)
    /// - they are not already visited in [current_track]
    /// - are not a `9`
    pub fn follow_track<'a>(&self, pos: (usize, usize), current_track: &'a mut Vec<(usize, usize)>) {

        // current pos split
        let (x, y) = pos;

        // Check if not visited and not 9
        if current_track.contains(&pos) || self.get(pos.0, pos.1) == 9 {
            return;
        }

        // remember we have seen this place
        current_track.push((x,y));

        // and go to the next directions (up down left right)
        if y > 0 {
            let neighbour_pos = (x, y - 1);
            self.follow_track(neighbour_pos, current_track);
        }

        if y < self.height() - 1 {
            let neighbour_pos = (x, y + 1);
            self.follow_track(neighbour_pos, current_track);
        }

        if x > 0 {
            let neighbour_pos = (x - 1, y);
            self.follow_track(neighbour_pos, current_track);
        }

        if x < self.width() - 1 {
            let neighbour_pos = (x + 1, y);
            self.follow_track(neighbour_pos, current_track);
        }
    }

    /// Collects all [get_lowest_points] and follows the paths using [follow_track]
    pub fn get_basins(&self) -> Vec<Vec<(usize, usize)>> {
        let mut basins: Vec<Vec<(usize, usize)>> = vec!();

        for low_point in self.get_lowest_points() {
            let mut curr : Vec<(usize, usize)> = Vec::new();
            self.follow_track(low_point, &mut curr);
            basins.push(curr);
        }

        basins
    }

    /// returns all lowest points as in their is no direct neighbour which is smaller or equal
    /// the the point seen
    pub fn get_lowest_points(&self) -> Vec<(usize, usize)> {
        let mut basins = Vec::<(usize, usize)>::new();

        for x in 0..self.width() {
            for y in 0..self.height() {
                // walk upwards
                let curr = self.get(x,y);

                let mut found_smaller = false;

                // upwards check
                if y > 0 {
                    found_smaller = found_smaller || self.get(x, y-1) <= curr;
                }

                // downards check
                if y < self.height() -1 {
                    found_smaller = found_smaller || self.get(x, y+1) <= curr;
                }

                // leftwards check
                if x > 0 {
                    found_smaller = found_smaller || self.get(x - 1, y) <= curr;
                }

                // rightwards check
                if x < self.width() -1 {
                    found_smaller = found_smaller || self.get(x + 1, y) <= curr;
                }

                if !found_smaller {
                    basins.push((x, y));
                }
            }

        }
        basins
    }

    /// Just collects the numbers of [get_lowest_points]
    pub fn get_lowest_numbers(&self) -> Vec<u64>{
        self.get_lowest_points().iter().map(|p| self.get(p.0, p.1)).collect()
    }
}


pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 9 task 2");
    let floor = Floor::from_input_lines(data);
    println!("{:?} \n lowest {:?}", floor, floor.get_lowest_numbers());
    return floor.get_lowest_numbers().iter().map(|f| f + 1).sum();
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 9 task 2");
    let floor = Floor::from_input_lines(data);
    // find the basin areas
    let basins = floor.get_basins();

    // ... turn them into their size
    let mut basin_sizes = basins
        .iter()
        .map(|b| b.len())
        .collect::<Vec<usize>>();

    println!("Basins: {:?}\nBasin Sizes: {:?}", basins, basin_sizes);

    // sort them (ascending)
    basin_sizes.sort();

    // get the last (biggest) entries of the sorted array
    let greatest_three = (0 as usize..3)
        .map(|index|
            basin_sizes.get(basin_sizes.len() - 1  - index).unwrap()
        )
        .collect::<Vec<&usize>>();

    // and return the product (multiply them)
    return greatest_three.iter().fold(1, |n, p| n * **p as u64);
}

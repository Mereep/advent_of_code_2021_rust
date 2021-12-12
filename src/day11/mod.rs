const NEIGHBOURS: [(i8, i8); 8] = [
    (0, -1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

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

    /// Gets a single element (n, m) if it exists
    pub fn get(&self, x: usize, y: usize) -> Option<&u64> {
        self.matrix.get(y).unwrap().get(x)
    }

    /// Gets a single element (n, m) if it exists
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u64> {
        self.matrix.get_mut(y).unwrap().get_mut(x)
    }

    /// width of the matrix (m)
    pub fn width(&self) -> usize {
        self.matrix.get(0).unwrap().len()
    }

    /// height of the matrix
    pub fn height(&self) -> usize {
        self.matrix.len()
    }

    /// increases each entry by one
    fn increase_all_by_one(&mut self) {
        for row in self.matrix.iter_mut() {
            for entry in row.iter_mut() {
                *entry += 1;
            }
        }
    }

    /// sets back all flashed entries to 0
    fn reset_flashed(&mut self) {
        for row in self.matrix.iter_mut() {
            for entry in row.iter_mut() {
                if *entry > 9 {
                    *entry = 0;
                }
            }
        }
    }

    /// Finds all Octopuses with energy level > 9 which did not already
    /// flash according to [flashed] and increases their neighbours value by 1
    fn flash_all(&mut self, flashed: &mut Vec<(usize, usize)>) {

        // iterate over the field
        for x in 0..self.width() {
            for y in 0..self.height() {
                let index = (x, y);

                // check if the energy is > 9 and check if the position didn't already flash
                if  !flashed.contains(&index) && self.get(x, y).unwrap() > &9 {
                    // remember this flash
                    flashed.push((x as usize, y as usize));
                    // iterate over all neighbours
                    for neighbour in NEIGHBOURS {
                        let x1 = neighbour.0 + x as i8;
                        let y1 = neighbour.1 + y as i8;

                        // Check if that neighbour exists (boundaries)
                        if !(x1 >= 0 && y1 >= 0 && x1 < self.width() as i8 && y1 < self.height() as i8) {
                            continue;
                        }

                        // increase the neighbours' energy by 1
                        if let Some(val) = self.get_mut(x1 as usize, y1 as usize) {
                            *val += 1;
                        }
                    }
                }
            }
        }
    }

    /// We play until we find a round where every Octopus flashed
    /// (Task II)
    /// the algorithm is almost identical to task 1
    pub fn play_until_all_flash(&mut self) -> u64{
        let mut round: u64 = 0;
        let mut flashed_round: Vec<(usize, usize)> = vec!();
        let mut flash_count = 0;

        while flash_count < self.width() * self.height() {
            flash_count = 0;
            // println!("{}", flash_count);
            self.increase_all_by_one();
            // The following could be factored out (same code as in task 1)
            loop {
                self.flash_all(&mut flashed_round);
                let flash_new = flashed_round.len();
                if flash_new >  flash_count {
                    flash_count = flash_new;
                } else {
                    break;
                }
            }
            self.reset_flashed();
            flashed_round.clear();
            round += 1;
        }

        round
    }

    /// Does the a given amount of steps and returns all flashed positions for each round
    pub fn play_n_rounds(&mut self, rounds: u64) -> Vec<Vec<(usize, usize)>>{
        let mut flashed_total: Vec<Vec<(usize, usize)>> = vec!();

        // do each stap
        for _ in 0..rounds as usize {
            // remember, which positions actually flashed that round
            let mut flashed_round: Vec<(usize, usize)> = vec!();

            // now just add 1 energy for each position
            self.increase_all_by_one();

            // remember how many positions flashed until now
            let mut flash_count = flashed_round.len();

            // repeat until no change...
            loop {
                // Flash all positions that did not already and increase their neighbours by 1
                self.flash_all(&mut flashed_round);

                // Count how many flashes we have in total
                let flash_new = flashed_round.len();

                // and stop if we didnt increase
                if flash_new > flash_count {
                    flash_count = flash_new; //... if we increased we remember the new count
                } else {
                    break;
                }
            }
            // Remember which ones we flashed this round
            flashed_total.push(flashed_round.clone());

            // clear the current flashies
            flashed_round.clear();

            // reset the energy levels to 0
            self.reset_flashed();
        }

        flashed_total
    }
}
pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 11 task 1");
    let mut floor = Floor::from_input_lines(data);
    let flashed = floor.play_n_rounds(100);
    return flashed.iter().fold(0, |p, n| p + n.len() as u64);
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 11 task 2");
    let mut floor = Floor::from_input_lines(data);
    let round = floor.play_until_all_flash();
    return round;
}

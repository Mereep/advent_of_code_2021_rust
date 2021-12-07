use crate::day4::parse_line_of_numbers;

/// Stores how many days the fishes need to reproduce
#[derive(Debug)]
struct SeaOfFishes {
    fish_reproductions: [u64; 9],
}

impl SeaOfFishes {
    fn from_input(line: &String) -> Self {
        let line = parse_line_of_numbers::<u64>(line, ',');
        let mut fishes = [0 as u64; 9];

        // add each fishes' current reproduction days (init)
        for number in line {
            fishes[number as usize] += 1;
        }

        return SeaOfFishes {
            fish_reproductions: fishes
        }
    }

    /// Lets one day pass
    fn tick_a_day(&mut self) {

        // remember how many fishes will reproduce
        let reproducing_fishes = self.fish_reproductions[0];

        // Move each fish one slot (day) forwars
        for i in 0 as usize..=7 {
            self.fish_reproductions[i] = self.fish_reproductions[i+1];
        }

        // new fishes will need 8 days to reproduce
        self.fish_reproductions[8] = reproducing_fishes;

        // Also the same fishes which reproduced will reproduce again in 6 days
        self.fish_reproductions[6] += reproducing_fishes;
    }

    /// just a wrapper using [tick_a_day] multiple times
    fn tick_n_days(&mut self, days: usize) {
        for _ in 0..days {
            self.tick_a_day();
        }
    }

    /// how many fishes we have in the pool?
    fn count_the_pool(&self) -> u64 {
        self.fish_reproductions.iter().sum()
    }
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 6 task 2");
    let mut field = SeaOfFishes::from_input(data.get(0).unwrap());
    field.tick_n_days(80);
    return field.count_the_pool();
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 6 task 2");
    let mut field = SeaOfFishes::from_input(data.get(0).unwrap());
    field.tick_n_days(256);
    return field.count_the_pool();
}
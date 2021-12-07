use std::collections::HashMap;
use crate::day4::parse_line_of_numbers;

#[derive(Debug)]
struct Field {
    // map from position -> number crabs at the position
    positions: HashMap<u64, u64>
}

impl Field {
    fn from_line(line: &String) -> Self {
        let values =  parse_line_of_numbers::<u64>(line, ',');
        let mut vec : HashMap<u64, u64> = HashMap::new();

        // Just count how often each number appears
        for number in values {
            if !vec.contains_key(&number) {
                vec.insert(number, 0);
            }
            let to_change = vec.get_mut(&number).unwrap();
            *to_change += 1;
        }

        return Field {
            positions: vec.into()
        }
    }

    /// Calculates the cost if all crabs move to [position]
    fn cost_for(&self, position: u64, linear_cost: bool) -> u64 {
        let mut total_cost: u64 = 0;

        // Iterate over all crab positions (key is the position)
        for key in self.positions.keys() {
            // distance from the crab position to the target position
            let dist = (*key as i64 - position as i64).abs() as u64;

            if linear_cost { // task 1
                // one position movement costs exactly one fuel (linear cost)
                total_cost += self.positions.get(key).unwrap() * dist;
            } else { // task 2
                // the farther the crab moves the more expensive it gets
                // e.g., distance of 4 costs 1 + 2 + 3 + 4 per crab
                let sum_of_distances = (dist * (dist + 1)) / 2;
                //Trivial formula: (1..=dist).fold(0 as u64, |prev, val| val + prev);
                total_cost += self.positions.get(key).unwrap() * sum_of_distances;
            }
        }

        return total_cost;
    }

    fn calculate_minimum(&self, linear_cost: bool) -> u64 {
        let mut min_dist = u64::MAX;

        // Very simple solution: just try all the numbers between the two farthest-away crabs
        // Better solution would be to use binary search in range (finds in log(distance_range);
        // only for task 1, since task 2 is not linear)
        for position in *self.positions.keys().min().unwrap()..=*self.positions.keys().max().unwrap() {
            let new_val = self.cost_for(position, linear_cost);
            if new_val < min_dist{
                min_dist = new_val;
            }

        }

        return min_dist;
    }
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 7 task 1");
    let field = Field::from_line(data.get(0).unwrap());
    let least_fuel = field.calculate_minimum(true);
    println!("{:?}: least fuel {}", field, least_fuel);
    return least_fuel;
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 7 task 2");
    let field = Field::from_line(data.get(0).unwrap());
    let least_fuel = field.calculate_minimum(false);
    println!("{:?}: least fuel {}", field, least_fuel);
    return least_fuel;
}
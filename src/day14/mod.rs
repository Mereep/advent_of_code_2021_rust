use std::collections::HashMap;

/// Stores the (current) polymer and a set of substitution rules
#[derive(Debug)]
struct PolymerSlow {
    polymer: Vec<char>,
    instructions: HashMap<(char, char), char>
}

impl PolymerSlow {
    pub fn from_input(lines: &Vec<String>) -> Self {
        let mut polymer = String::new();
        let mut instructions : HashMap<(char, char), char> = HashMap::new();

        for (i, line) in lines.iter().enumerate() {

            // First line is the input polymer
            if i == 0 {
                polymer += line;
            }

            // beginning from the 3rd line we have the rules
            if i > 1  {
                // split the line on the arrow (notice the spaces)
                let instruction = line.split(" -> ").collect::<Vec<_>>();
                let from = instruction
                    .get(0)
                    .unwrap()
                    .chars()
                    .collect::<Vec<_>>();

                instructions.insert((*from.get(0).unwrap(), *from.get(1).unwrap()),
                                    instruction.get(1).unwrap().chars().last().unwrap());

            }
        }

        Self {
            polymer: polymer.chars().collect(),
            instructions
        }
    }

    pub fn substitute(&mut self) {
        let polymer_new_str = self.polymer.windows(2).map(|chunk| {
            let pair = (*chunk.get(0).unwrap(), *chunk.get(1).unwrap());
            let mut res = String::from(pair.0);
            if let Some(sub) = self.instructions.get(&pair) {
                res.push(*sub);
            }
            res
        }).collect::<Vec<String>>();

        let mut polymer_new = polymer_new_str
            .iter()
            .fold(String::new(), |last,new| {
               last + new
            });

        polymer_new.push(*self.polymer.last().unwrap());
        self.polymer = polymer_new.chars().collect();
    }

    /// Counts each single letter / polymer and returns their respective counts
    pub fn count_polymers(&self) -> HashMap<char, u64>{
       let mut res: HashMap<char, u64> = HashMap::new();
        for char in &self.polymer {
            if let Some(count) = res.get_mut(char) {
               *count += 1;
            } else {
                res.insert(*char, 0);
            }
        }
        res
    }
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 14 task 1");
    let mut polymer = PolymerSlow::from_input(data);
    for _ in 0..10 {
        polymer.substitute();
    }
    // println!("{:?}", polymer.count_polymers());
    let counts = polymer.count_polymers();
    return counts.values().max().unwrap() - counts.values().min().unwrap();
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 14 task 2");
    return 0;
}

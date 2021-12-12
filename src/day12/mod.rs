use std::collections::HashMap;
use std::ptr::addr_of;

/// Stores all connections as entries in a map as in 'a' -> C, D, end
#[derive(Debug)]
struct Cave {
    edges: HashMap<String, Vec<String>>
}

impl Cave {
    pub fn from_input(lines: &Vec<String>) -> Self {
        let mut edges : HashMap<String, Vec<String>> = HashMap::new();

        for line in lines {
            // split the line in left an right part
            let splits = line.split('-').collect::<Vec<_>>();
            let left = *splits.get(0).unwrap();
            let right = *splits.get(1).unwrap();

            // if we didn't see that node name, we create an empty vector of nodes it points to
            if !edges.contains_key(left) {
                edges.insert(String::from(left), vec!());
            }
            if !edges.contains_key(right) {
                edges.insert(String::from(right), vec!());
            }

            // add the connection from left -> right
            let left_node = edges.get_mut(left).unwrap();
            left_node.push(String::from(right));

            // and add a inverse connection (we can flow in both directions)
            let right_node = edges.get_mut(right).unwrap();
            right_node.push(String::from(left));
        }

        return Self {
            edges
        }
    }

    /// task 1 version
    /// This function keeps track on where it came from ([current_track]) and which paths where
    /// already found in [found_paths]
    /// [from] dictates from which node / cave we want to start searching
    fn follow_track<'a>(&'a self, from: &'a String,
                    found_paths: &mut Vec<Vec<&'a String>>,
                    current_track: &mut Vec<&'a String>) {

        // append the current from-node to the path
        current_track.push(from);
        // found an end-node?
        if from == "end" {
            if !found_paths.contains(current_track) {
                found_paths.push(current_track.clone())
            }
            return;
        }

        // iterate over all paths
        for neighbour in self.edges.get(from.into()).unwrap() {
            // extract the first char of the caves' name
            let first_char = neighbour.chars().take(1).last().unwrap();

            // Check if we can visit that cave (node)
            // that is if it is a upper case letter or we didn't already see it
            if (first_char as u8) < ('a' as u8) || !current_track.contains(&neighbour) {
                // recursively follow from the neighbour
                self.follow_track(neighbour, found_paths, current_track);
                // remove what we added here before we go back
                current_track.pop();
            }
        }
    }
    /// task 2 version
    /// this is like the task 1 version except it remembers additionally if on the current track
    /// there has allready been a cave with a small letter visited twice
    fn follow_track_task_2<'a>(&'a self, from: &'a String,
                    found_paths: &mut Vec<Vec<&'a String>>,
                    current_track: &mut Vec<&'a String>,
                    mut already_double_visit: &bool,
    ) {

        // found an end-node?
        current_track.push(from);
        if from == "end" {
            if !found_paths.contains(current_track) {
                found_paths.push(current_track.clone())
            }
            return;
        }

        for neighbour in self.edges.get(from.into()).unwrap() {
            let first_char = neighbour.chars().take(1).last().unwrap();

            // Check if we can visit that cave (node)
            // that is if it is a upper case letter or we didn't already see it
            // OR (task 2) if it is already double visited but we only visited one small guy
            // exactly twice
            let is_big_char = (first_char as u8) < ('a' as u8);
            let mut can_visit = neighbour != "start";
            let mut did_double_visit = false; // remember if we did a double visit this turn
            if !is_big_char && can_visit {
                if current_track.contains(&neighbour) {
                    if !already_double_visit { // allow exactly one double visit
                        already_double_visit = &true;
                        did_double_visit = true;
                    } else {
                        can_visit = false;
                    }
                }
            }

            if can_visit {
                self.follow_track_task_2(neighbour, found_paths, current_track, already_double_visit);
                // remove what we added here before we go back
                current_track.pop();

                // Toggle the double visit flag back if we remove the double visited element
                if did_double_visit {
                    already_double_visit = &false;
                }
            }
        }
    }

    /// This initiates the process
    /// by just initializing the data structures and calling
    /// the [follow_track]-functions
    /// :param allow_two_times_visit will initiate the task if true otherwise task 1
    pub fn gather_paths(&self, allow_two_times_visit: bool) -> Vec<Vec<&String>> {
        let mut paths : Vec<Vec<&String>> =  Vec::new();
        paths.reserve(200_000);
        let mut current_track = Vec::<&String>::new();

        if !allow_two_times_visit {
            self.follow_track(self.edges.keys().find(|k| *k == "start").unwrap(),
                              &mut paths,
                              &mut current_track);
        } else {
            self.follow_track_task_2(self.edges.keys().find(|k| *k == "start").unwrap(),
                              &mut paths,
                              &mut current_track,
                                     &false);
        }

        paths
    }
}

pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 12 task 1");
    let cave = Cave::from_input(data);
    let paths = cave.gather_paths(false);
    println!("{:?}, \n {:?}" , cave, paths);
    return paths.len() as u64;
}

pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 12 task 2");
    let cave = Cave::from_input(data);
    let paths = cave.gather_paths(true);
    println!("{:?}, \n {:?}" , cave, paths);
    return paths.len() as u64;
}

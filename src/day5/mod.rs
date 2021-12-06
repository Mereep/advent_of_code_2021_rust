use std::cmp::min;
use std::cmp::max;
use std::fmt::{Debug};

/// Stores a "Box" (basically a line)
/// as in (x1, y1, x3, y2)
#[derive(Debug)]
#[derive(Clone)]
struct LineSegment(u64, u64, u64, u64);


/// Code for a line segment
impl LineSegment {

    /// factory for building a line segment given a input line as String
    /// (i.e., `x1,y1 -> x2,y2`)
    pub fn from_input(line: &String) -> Self{
        // separate into left and right half
        let data: Vec<&str> = line.split(&" -> ").collect();

        // get the numbers on the left
        let x1y1 = crate::day4::parse_line_of_numbers::<u64>(
            &String::from(*data.get(0).unwrap()),
            ',');

        // ... and on the right half
        let x2y2 = crate::day4::parse_line_of_numbers::<u64>(
            &String::from(*data.get(1).unwrap()),
            ',');

        // and return the struct
        return LineSegment (
            *x1y1.get(0).unwrap(),
            *x1y1.get(1).unwrap(),
            *x2y2.get(0).unwrap(),
            *x2y2.get(1).unwrap(),
        );
    }
}


/// Describes our floor which has dangerous hot vents along lines
#[derive(Debug)]
struct OceanFloor {
    vents: Vec<LineSegment>,
}

/// Helper function that lets you print some vector as a matrix
/// (only works for single digit numbers atm)
fn print_as_matrix<T: Debug>(mat: &Vec<T>, width: usize) {
    for (i, entry) in mat.iter().enumerate() {
        if (i % width) == 0 {
            println!();
        }
        print!(" {:?} ", entry);
    }
    println!();
}

/// methods and functions of the [OceanFloor]
impl OceanFloor {

    /// creates a linearized matrix of the oceans' floor
    pub fn build_scene(&self) -> Vec<u64> {

        // load x-y exntend
        let (width, height) = self.get_extends();

        // stores our linearized floor as numbers
        let mut scene: Vec<u64> =
            (0..width*height)
            .map(|_| 0).collect::<Vec<u64>>();

        // iterate over all vents
        for vent in &self.vents {
            // get each vents' extend
            let min_x = min(vent.0, vent.2);
            let max_x = max(vent.0, vent.2);
            let min_y = min(vent.1, vent.3);
            let max_y = max(vent.1, vent.3);

            // and calculate their x and y-spreads
            let diff_x = max_x - min_x;
            let diff_y = max_y - min_y;

            // check which one is faster (i.e., has the greater difference)
            if diff_x > diff_y {
                // stores the increase in y-units per x unit
                // this reads as move dy units into y direction when moving 1 unit into x direction
                let mut dy: f64 = 0.0;
                if (max_x - min_x) > 0 {
                    // ... and is calculated as the relative speed of the smaller y-span
                    // to the x-span
                    dy = (vent.1 as f64 - vent.3 as f64) / (vent.0 as f64 - vent.2 as f64);
                }
                // get the points which have to be set
                let mut steps: Vec<u64> = (vent.0..=vent.2).collect();
                if vent.0 > vent.2 {
                    // ... and invert the walking direction depending in which direction we walk
                    steps = (vent.2..=vent.0).rev().collect();
                    dy *= -1.0;
                }
                // iterate over the steps
                for (step, x) in steps.iter().enumerate() {
                    //... calculate the y position
                    let y = (vent.1 as f64 + step as f64 * dy).round() as u64;
                    // .. and do some rust-y things to set the ocean floors value' value
                    let attr_to_set: &mut u64 = scene.get_mut((x + y * width) as usize).unwrap();
                    *attr_to_set += 1;
                }
            } else {
                // This is the code path for y-speed >= x speed
                // i.e., here the main direction is the y - direction
                // otherwise the code is almost identical
                let mut dx: f64 = 0.0;
                if (max_y - min_y) > 0 {
                    dx = (vent.0 as f64 - vent.2 as f64) / (vent.1 as f64 - vent.3 as f64);
                }
                let mut steps: Vec<u64> = (vent.1..=vent.3).collect();
                if vent.1 > vent.3 {
                    steps = (vent.3..=vent.1).rev().collect();
                    dx *= -1.0;
                }
                for (step, y) in steps.iter().enumerate() {
                    // we could even save the multiplication by increasing each time by
                    // dx and store the result in a separate var (mult is more expensive
                    // and we do basic drawing algorithms here)
                    let x = (vent.0 as f64 + step as f64 * dx).round() as u64;
                    let attr_to_set: &mut u64 = scene.get_mut((x + y * width) as usize).unwrap();
                    *attr_to_set += 1;
                }
            }

        }

        return scene;
    }

    /// returns the tasks' result
    /// this is: counting all patches of the scene where "lines" do overlap
    pub fn calculate_overlap(&self) -> u64 {
        return self.build_scene().iter().fold(0 as u64, |p,n| {
            if *n > 1 as u64 {
                return p + 1;
            }
            return p;
        })
    }

    /// factory for building the class from a list of lines optionally
    /// filters out all inputs which are diagonal lines
    pub fn from_input(lines: &Vec<String>, filter_diagonal: bool) -> Self {
        let mut lines: Vec<LineSegment> = lines
            .iter()
            .map(|line| LineSegment::from_input(line))
            .collect();

        if filter_diagonal {
            lines = lines
                .into_iter()
                .filter(|line| {
                    return line.0 == line.2 || line.1 == line.3;
            })
            .collect();
        }
        return Self{
            vents: lines
        }
    }

    /// Gets the highest x and y spread
    pub fn get_extends(&self) -> (u64, u64) {
        let inner = self.vents
            .iter()
            .fold((0, 0), |prev, vent| {
              return (
                    max(prev.0, max(vent.0, vent.2)),
                    max(prev.1, max(vent.1, vent.3))
                  );
            });

        return (inner.0 + 1, inner.1 + 1)
    }
}


pub fn task1(data: &Vec<String>) -> u64 {
    println!("Executing day 5 task 1");
    let ocean = OceanFloor::from_input(data, true);
    let overlap = ocean.calculate_overlap();
    println!("extends: {:?}, overlap: {}", ocean.get_extends(), overlap);
    return overlap;
}


pub fn task2(data: &Vec<String>) -> u64 {
    println!("Executing day 5 task 2");
    let ocean = OceanFloor::from_input(data, false);
    // print_matrix(&ocean.build_scene(), ocean.get_extends().0 as usize);
    let overlap = ocean.calculate_overlap();
    println!("extends: {:?}, overlap: {}", ocean.get_extends(), overlap);
    return overlap;
}

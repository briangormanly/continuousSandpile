#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

/*

 - Discrete sandpile targeting criticality
  Starts with everything from 2-sandpile-basic-random
  Additions / Changes:
   * Density for each pile location
     * Sandpile locations can have between 1 and 6 grains in the location
     *  
   * Moment
     * Gains move with a magnatude (speed) and direction 
       * Initial speed starts at 1 but kinetic energy can be transferred in collisions
       * speed increases as grain falls 
     * Direction of impacted grain movement is determined by direction of impacting grain
   * Energy from impacts radiate through surrounding grains

*/

extern crate rand;
use rand::Rng;
use std::vec::Vec;

// turn on and off debug output
const DEBUG: bool = false;

// total grains to drop
const TOTAL_GRAINS: usize = 20;

// total allowed demensions of the pile
const X_SIZE: usize = 5;
const Y_SIZE: usize = 5;
const Z_SIZE: usize = 4;

struct Grain {
    id: u32,
    energy: usize,
    direction: usize,
}

/**
 * Model for a grain of sand in the system
 * Will be initialized with initial energy and direction of 0
 * which should be set for grains in motion
 */
impl Grain {
    // Constructor to create a new Grain with a specific id
    pub fn new(id: u32) -> Grain {
        Grain { 
            id, 
            energy: 0,
            direction: 0, 
        }
    }
}

/**
 * Model for a location in the sandpile
 * Locations are static and do not move, they represent a point in the 3D space
 * They have a capacity for grains and a resilience to purturbations which is 
 * determined as a random value between 1 and 6
 */
struct Location {
    x: i32,
    y: i32,
    z: i32,
    capacity: usize,
    grains: Vec<Grain>,
    resilience: usize,
}

impl Location {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut rng = rand::thread_rng();
        Location {
            x,
            y,
            z,
            capacity: rng.gen_range(1..=6),  // Corrected range to include 6
            grains: Vec::new(),  // Initialize as empty vector
            resilience: rng.gen_range(1..=6),
        }
    }

    fn add_grain(&mut self, grain: Grain) {
        if self.grains.len() < self.capacity {
            self.grains.push(grain);
        } else {
            println!("Capacity reached, cannot add more grains");
        }
    }
}



struct Avalanche {
    grains: Vec<Grain>,
    energy: usize,
    direction: usize,
}

impl Avalanche {
    pub fn new() -> Self {
        Avalanche {
            grains: Vec::new(),
            energy: 0,
            direction: 0,
        }
    }

    // Changed from &self to &mut self to allow modification
    fn add_grain(&mut self, grain: Grain) {
        self.grains.push(grain);
        self.energy += 1;  // Correctly increment the energy
    }
}

// struct Location {
//     id: usize,
//     mass: rand::thread_rng().gen_range(1..5),
// }

fn main() {

    // create a 3D array to hold the sandpile
    let mut array = vec![vec![vec![0; Z_SIZE]; Y_SIZE]; X_SIZE];

    // createa Location
    let mut location = Location::new(0, 0, 0);

}
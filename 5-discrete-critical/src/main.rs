#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

/*

 - Discrete sandpile targeting criticality
  Starts with everything from 2-sandpile-basic-random
  Additions / Changes:
   * Density for each pile location
     * Sandpile locations have a capacity of 4 grains plus the output of the order of 
     *  magnitude power-law distribution
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

/**
 * Master Debug flag
 */
const DEBUG: bool = true;
const DEBUG_INIT: bool = true;
const DEBUG_FALLING_GRAIN: bool = true;

// total grains to drop
const TOTAL_GRAINS: usize = 100;

// Power-law distribution parameters
const ALPHA: f64 = 2.0;
const X_MIN: f64 = 1.0;


// total allowed demensions of the pile
const X_SIZE: usize = 10;
const Y_SIZE: usize = 10;
const Z_SIZE: usize = 3;

struct Grain {
    id: u32,
    // current energy of the grain, 
    // 0 if stationary, > 0 if in motion
    // energy is transferred to other grains on impact
    energy: usize,
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
            // current energy of the grain, 
            // 0 if stationary, > 0 if in motion
            // energy is transferred to other grains on impact
            energy: 0,
        }
    }
    pub fn impact(&mut self, energy: usize) {
        // TODO determine the probability of energy transfer and the magnitude
        //self.energy += energy;
        // TODO detrmine which of the surrounding locations will be impacted 
        // by this grains motion
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
    pub fn new(x: i32, y: i32, z: i32, rnd: &mut impl Rng ) -> Self {
        // get the order of magnitude of a random power-law distribution
        let additionalCap = normalizedPowerLawByOrdersOfMagnitude(rnd) as usize;
        let additionalRes = normalizedPowerLawByOrdersOfMagnitude(rnd) as usize;
        Location {
            x,
            y,
            z,
            capacity: 4 + additionalCap,  
            grains: Vec::new(),  // Initialize as empty vector
            resilience: 4 + additionalRes,  
        }
    }
    pub fn emptySpace(x: i32, y: i32, z: i32) -> Self {
        Location {
            x,
            y,
            z,
            capacity: 0,  
            grains: Vec::new(),  // Initialize as empty vector
            resilience: 0,  
        }
    }
    fn add_grain(&mut self, grain: Grain) {
        // Check if the location has capacity to add a grain
        if self.grains.len() < self.capacity {
            // the location is not full, add the grain
            self.grains.push(grain);
        } else {
            // if full, start an avalanche
            println!("Capacity reached, cannot add more grains");
        }
    }
    pub fn getNumberOfGrains(&self) -> usize {
        return self.grains.len();
    }
}


/**
 * Model for an avalanche in the sandpile
 * An avalanche is a collection of grains that have been preturbed and are moving
 */
struct Avalanche {
    id: u32,
    // Grains that are currently part of the avalanche
    grains: Vec<Grain>,
    // Current locations being impacted by the avalanche
    locations: Vec<Location>,
    
    // direction of the avalanche, determines which
    direction: usize,
}

impl Avalanche {
    pub fn new(id: u32) -> Self {
        Avalanche {
            id,
            grains: Vec::new(),
            locations: Vec::new(),
            direction: 0,
        }
    }

    // Changed from &self to &mut self to allow modification
    fn add_grain(&mut self, grain: Grain) {
        self.grains.push(grain);
    }
}

// struct Location {
//     id: usize,
//     mass: rand::thread_rng().gen_range(1..5),
// }

fn main() {
    // create a random number generator
    let mut rnd = rand::thread_rng();

    // create a 3D array to hold the sandpile
    //let mut array = vec![vec![vec![0; Z_SIZE]; Y_SIZE]; X_SIZE];

    // Create a 3D vector
    let mut array: Vec<Vec<Vec<Location>>> = Vec::with_capacity(X_SIZE);

    // cerate locations for all the points in the array within the slope of criticality
    for x in 0..X_SIZE {
        let mut layer_y: Vec<Vec<Location>> = Vec::with_capacity(Y_SIZE);
        for y in 0..Y_SIZE {
            let mut column_z: Vec<Location> = Vec::with_capacity(Z_SIZE);
            for z in 0..Z_SIZE {
                // create a location
                if x>=z && x<=X_SIZE-z-1 && y>=z && y<=Y_SIZE-z-1 {
                    // create a location
                    let location = Location::new(x as i32, y as i32, z as i32, &mut rnd);
                    column_z.push(location);
                    // add the location to the array
                    //if DEBUG && DEBUG_INIT { println!("Creating location x: {}, y: {}, z: {} has capacity {} and resilience {}", x, y, z, array[x][y][z].capacity, array[x][y][z].resilience); }


                }
                else {
                    // empty 
                    let location = Location::emptySpace(x as i32, y as i32, z as i32);
                    column_z.push(location);
                    //println!("!!!Location outside of critical slope x: {}, y: {}, z: {}", x, y, z);
                }
            }
            layer_y.push(column_z);
        }
        array.push(layer_y);
    }

    

    for i in 0..TOTAL_GRAINS {

        // create a grain 
        let grain = Grain::new(i as u32);

        // start with center of the array
        let mut x = X_SIZE / 2 - 1;
        let mut y = Y_SIZE / 2 - 1 ;

        // find the gains landing variance from center with more variance in the center
        // using an alpha of 1.5
        let mut xVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(1.4, &mut rnd);
        let mut yVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(1.4, &mut rnd);

        // check that the variance is within the bounds of the array
        if xVariance > x as f64 {
            xVariance = x as f64;
        }
        if yVariance > y as f64 {
            yVariance = y as f64;
        }

        // find the gains landing direction
        let xDirection = rnd.gen_range(0..2);
        let yDirection = rnd.gen_range(0..2);
   
        // compute the new location of the grain given the variance and direction
        if xDirection == 0 {
            x -= xVariance as usize;
        } else {
            x += xVariance as usize;
        }
        if yDirection == 0 {
            y -= yVariance as usize;
        } else {
            y += yVariance as usize;
        }
        
        // set the z level to the highest level 
        let mut z = Z_SIZE - 1;

        // see if the array location is empty and fall until it is not
        println!("Grain {} landed at x: {}, y: {}, z: {}", i, x, y, z);
        while !array[x][y][z].grains.is_empty() && array[x][y][z].grains.len() == 0 && z > 0 {
            
            z -= 1;
        }

        if DEBUG && DEBUG_FALLING_GRAIN { println!("Grain {} landed at x: {}, y: {}, z: {}", i, x, y, z); }

        // add the grain to the location
        array[x][y][z].add_grain(grain);

    }

    // draw the pile
    drawPile(&array);

    // createa Location
    //let mut location = Location::new(0, 0, 0);

}

fn drawPile(array: &Vec<Vec<Vec<Location>>>) {
    println!("Drawing the pile");
    for z in (0..Z_SIZE -1).rev() {

        for y in 0..Y_SIZE {

            print!("\n");

            for x in 0..X_SIZE {
                //print!("x:{}, y:{}, z:{} value:{}", x, y, z, array[x][y][z]);
                print!("{}", array[x][y][z].getNumberOfGrains());
            }
            
        }
        println!("\n");
    }
    println!(" ");
}


/**
 * Mathematics and probability functions
 */

/**
 * Arguments
 * 'alpha' - The exponent of the distribution. 
 * 'x_min' - The minimum value for the power-law distribution.
 * 'rng' - A random number generator.
 * 
 */
fn power_law(alpha: f64, rnd: &mut impl Rng) -> f64 {
    let uniform_rand = rnd.gen::<f64>();  // Generates a random number between 0 and 1
    X_MIN * (1.0 - uniform_rand).powf(-1.0 / (alpha - 1.0))
}

/**
 * 
 */
pub fn normalizedPowerLawByOrdersOfMagnitude(rnd: &mut impl Rng) -> f64{
    return normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA, rnd);

}

pub fn normalizedPowerLawByOrdersOfMagnitudeWithAlpha(alphaOverride: f64, rnd: &mut impl Rng) -> f64{
    // call the power_law function
    let value = power_law(alphaOverride, rnd);
    // return the order of magnitude of the value
    let orderOfMagnitude = value.log10().floor();
    return orderOfMagnitude;
    
}
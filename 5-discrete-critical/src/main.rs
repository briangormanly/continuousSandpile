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
const DEBUG: bool = false;
const DEBUG_INIT: bool = true;
const DEBUG_FALLING_GRAIN: bool = true;

// total grains to drop
const TOTAL_GRAINS: usize = 100;

// minimum value (multiplier) for the power-law distribution
// can be used to set a lower bound.
const X_MIN: f64 = 1.0;
// Power-law distribution parameters
const ALPHA_MAIN: f64 = 2.0;
const ALPHA_LANDING: f64 = 1.4;
const ALPHA_EXTRA_ENERGY: f64 = 2.0;
const ALPHA_AVALANCHE_SIZE: f64 = 1.2;

// total allowed demensions of the pile
const X_SIZE: usize = 10;
const Y_SIZE: usize = 10;
const Z_SIZE: usize = 3;

// Physics constants
const TERMINAL_FREE_FALL_SPEED: usize = 3;
const BASE_RESILIENCE: usize = 3;
const BASE_CAPACITY: usize = 4;

#[derive(Clone)]
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
    pub fn increaseEnergy(&mut self, energy: usize) {
        self.energy += energy;
    }
    pub fn incrementEnergy(&mut self) {
        self.energy += 1;
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
    id: usize,
    x: i32,
    y: i32,
    z: i32,
    capacity: usize,
    grainIds: Vec::<u32>,
    resilience: usize,
}

impl Location {
    pub fn new(x: i32, y: i32, z: i32, rnd: &mut impl Rng ) -> Self {
        let id = generateXyzId(x as usize, y as usize, z as usize);

        // get the order of magnitude of a random power-law distribution
        let additionalCap = normalizedPowerLawByOrdersOfMagnitude(rnd) as usize;
        let additionalRes = normalizedPowerLawByOrdersOfMagnitude(rnd) as usize;
        Location {
            id,
            x,
            y,
            z,
            capacity: BASE_CAPACITY + additionalCap,  
            grainIds: Vec::<u32>::new(),    // Initialize as empty vector
            resilience: BASE_RESILIENCE + additionalRes,  
        }
    }
    pub fn emptySpace(x: i32, y: i32, z: i32) -> Self {
        let id = generateXyzId(x as usize, y as usize, z as usize);

        Location {
            id,
            x,
            y,
            z,
            capacity: 0,  
            grainIds: Vec::<u32>::new(),    // Initialize as empty vector
            resilience: 0,  
        }
    }


    // call purtubation
    // both require:
    //  - the impacting grains energy at impact (parameter)
    //  - location resilience (self.resilience)
    //  - the number of grains in the location (self.gains.len())
    //  - the ability to add a grain to the avalanche [DO NOT HAVE]
    // both produce:
    //  - determining the total impact force
    //  - determines if resilience is broken and grains avalanche
    //  - determines the size of the avalanche if one occurs
    //  - ensures an avalanche is not leger the total number of grains at the location




    pub fn grainImpact(&mut self, grainId: u32, grainEnergy: usize, rnd: &mut impl Rng) -> Vec<u32> {

        // first check the impact of the incoming grain on the location
        let looseGrainIds = self.purtubation(grainId, grainEnergy, rnd);

        // check to see if the location was perturbed
        if looseGrainIds.len() == 0 {
            // the location was not perturbed, add the grain
            // Check if the location has capacity to add a grain
            if self.grainIds.len() < self.capacity {
                // the location is not full, add the grain
                self.grainIds.push(grainId);
            } else {
                // pile is full, but was not perturbed, let the grain fall.
                println!("Capacity reached, cannot add more grains");
            }
            return looseGrainIds;
        } else {
            // the location was perturbed, add the loose grains to the avalanche
            return looseGrainIds;
        }
    }

    fn purtubation(&mut self, incomingGrain: u32, incomingGrainEnergy: usize, rnd: &mut impl Rng) -> Vec<u32> {
        // get the order of magnitude of a random power-law distribution
        // as random additional energy representing a purtubation of the location
        // add this value to the grains current energy
        let additionalEnergy = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_EXTRA_ENERGY, rnd);
        let tempSpeed = incomingGrainEnergy + additionalEnergy as usize;

        // determine if this purturbation will cause an avalanche
        if self.resilience < tempSpeed {
            // start an avalanche
            println!("Avalanche started at location x: {}, y: {}, z: {}", self.x, self.y, self.z);
            // set the size of the avalanche
            let mut avalancheSize = 0;
            avalancheSize = 2 + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
            
            // ensure that the base avalanche size is not larger than the number of grains
            if self.grainIds.len() < avalancheSize {
                avalancheSize = self.grainIds.len();
            }

            // add the perturbed grain to the avalanche
            //avalanche.addGrain();

            println!("Avalanche size: {}", avalancheSize);
            let mut looseGrainIds: Vec<u32> = Vec::new();

            // return the grains that are part of the avalanche
            for i in 0..avalancheSize {
                looseGrainIds.push(self.grainIds.pop().unwrap());
            }
            return looseGrainIds;

        } else {
            Vec::new() // Return an empty vector
        }
    }

    //fn grainImpact(&mut self, grain: &'a Grain, avalanche: &'a mut Avalanche<'a>, rnd: &mut impl Rng) {


    //fn grainImpact(&mut self) {
        // // first check the impact of the incoming grain on the location
        // self.purtubation(grain, avalanche, rnd);

        // // Check if the location has capacity to add a grain
        // if self.grains.len() < self.capacity {
        //     // the location is not full, add the grain
        //     self.grains.push(grain);
        // } else {
        //     // if full, start an avalanche
        //     println!("Capacity reached, cannot add more grains");
        // }
        
    //}

    // fn purtubation(&mut self, grain: &'a Grain, avalanche: &'a mut Avalanche<'a>, rnd: &mut impl Rng) {
    //     // get the order of magnitude of a random power-law distribution
    //     // as random additional energy representing a purtubation of the location
    //     // add this value to the grains current energy
    //     let additionalEnergy = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_EXTRA_ENERGY, rnd);
    //     let tempSpeed = grain.energy + additionalEnergy as usize;

    //     // determine if this purturbation will cause an avalanche
    //     if self.resilience < tempSpeed {
    //         // start an avalanche
    //         println!("Avalanche started at location x: {}, y: {}, z: {}", self.x, self.y, self.z);
    //         // set the size of the avalanche
    //         let mut baseAvalancheSize = 2 + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
            
    //         // ensure that the base avalanche size is not larger than the number of grains
    //         if self.grains.len() < baseAvalancheSize {
    //             baseAvalancheSize = self.grains.len();
    //         }

    //         // add the perturbed grain to the avalanche
    //         avalanche.addGrain(&self.grains.pop().unwrap());

    //         println!("Avalanche size: {}", avalanche.grains.len());

    //     }
    // }
    pub fn getNumberOfGrains(&self) -> usize {
        return self.grainIds.len();
    }
}


/**
 * Model for an avalanche in the sandpile
 * An avalanche is a collection of grains that have been preturbed and are moving
 */
struct Avalanche {
    id: u32,
    // Grains that are currently part of the avalanche
    grainIds: Vec::<u32>,
    // Current locations being impacted by the avalanche
    locationIds: Vec::<u32>,
    
    // direction of the avalanche, determines which
    direction: usize,
}

impl Avalanche {
    pub fn new(id: u32) -> Self {
        Avalanche {
            id,
            grainIds: Vec::<u32>::new(),
            locationIds: Vec::<u32>::new(),
            direction: 0,
        }
    }

    pub fn addGrain(&mut self, grainId: u32) {
        self.grainIds.push(grainId);
    }

    // Changed from &self to &mut self to allow modification
    // pub fn addGrain(&mut self, grain: &'a Grain) {
    //     self.grains.push(grain);
    // }
    // pub fn getFullAvalancheEnergy(&self) -> usize {
    //     let mut totalEnergy = 0;
    //     for grain in &self.grains {
    //         totalEnergy += grain.energy;
    //     }
    //     return totalEnergy;
    // }
}

// struct Location {
//     id: usize,
//     mass: rand::thread_rng().gen_range(1..5),
// }

fn main() {
    // create a random number generator
    let mut rnd = rand::thread_rng();

    // Create a 3D vector of locations
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

    // initialize a vec of all grains
    let mut grains: Vec<Grain> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the grains in the array
    for i in 0..TOTAL_GRAINS {
        // create a grain 
        let mut grain = Grain::new(i as u32);
        // set the grain in motion
        grain.incrementEnergy();
        grains.push(grain);
    }

    // initialize a vec of all avalanches
    let mut avalanches: Vec<Avalanche> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the avalanches in the array each grain causes an avalanche 
    // of some size, might be as small as joining the first location it lands on
    // or as big
    for i in 0..TOTAL_GRAINS {
        // create a grain 
        let mut avalanche = Avalanche::new(i as u32);
        avalanches.push(avalanche);
    }
    

    for i in 0..TOTAL_GRAINS {


        // Start the avalanche for the this grains motion
        avalanches[i].addGrain(grains[i].id);
        

        // start with center of the array
        let mut x = X_SIZE / 2 - 1;
        let mut y = Y_SIZE / 2 - 1 ;

        // find the gains landing variance from center with more variance in the center
        // using an alpha of 1.5
        let mut xVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, &mut rnd);
        let mut yVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, &mut rnd);

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

        // see if the array location is not at capacity and fall until it is not
        if DEBUG && DEBUG_FALLING_GRAIN { println!("Grain {} started at x: {}, y: {}, z: {}", i, x, y, z) };
        while array[x][y][z].grainIds.len() < array[x][y][z].capacity && z > 0 {
            z -= 1;
            // increase the energy of the grain up to terminal velocity
            if grains[i].energy < TERMINAL_FREE_FALL_SPEED {
                grains[i].incrementEnergy();
            }
        }

        if DEBUG && DEBUG_FALLING_GRAIN { println!("Grain {} landed at x: {}, y: {}, z: {}", i, x, y, z); }

        // add the grain to the location
        array[x][y][z].grainImpact(grains[i].id, grains[i].energy, &mut rnd);
        

        if DEBUG && DEBUG_FALLING_GRAIN { println!("array at location x: {}, y: {}, z: {} has grains {}", x, y, z, array[x][y][z].getNumberOfGrains()); }

    }

    // draw the pile
    drawPile(&array);

    // createa Location
    //let mut location = Location::new(0, 0, 0);

}

fn drawPile(array: &Vec<Vec<Vec<Location>>>) {
    println!("Drawing the pile");
    let mut grandTotal = 0;
    for z in (0..Z_SIZE -1).rev() {

        for y in 0..Y_SIZE {

            print!("\n");

            for x in 0..X_SIZE {
                //print!("x:{}, y:{}, z:{} value:{}", x, y, z, array[x][y][z]);
                print!("{}", array[x][y][z].getNumberOfGrains());
                grandTotal += array[x][y][z].getNumberOfGrains();
            }
            
        }
        println!("\n");
    }
    println!(" ");
    println!("Total grains in the pile: {}", grandTotal);
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
    return normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_MAIN, rnd);

}

pub fn normalizedPowerLawByOrdersOfMagnitudeWithAlpha(alphaOverride: f64, rnd: &mut impl Rng) -> f64{
    // call the power_law function
    let value = power_law(alphaOverride, rnd);
    // return the order of magnitude of the value
    let orderOfMagnitude = value.log10().floor();
    return orderOfMagnitude;
    
}

fn num_bits_needed(max_value: usize) -> usize {
    // Compute the number of bits required to store max_value
    // This calculates the floor of the logarithm base 2 of max_value and adds 1
    (max_value as f64).log2().ceil() as usize
}

/**
 * Generate a unique id based on the x, y, z coordinates
 */
fn generateXyzId(x: usize, y: usize, z: usize) -> usize {
    let max_x_index = X_SIZE - 1;
    let max_y_index = Y_SIZE - 1;
    let max_z_index = Z_SIZE - 1;

    // Determine the number of bits needed for each dimension
    let x_bits = num_bits_needed(max_x_index);
    let y_bits = num_bits_needed(max_y_index);
    let z_bits = num_bits_needed(max_z_index);

    // Encode x, y, z into a single usize using bit shifts
    // We shift x by the sum of the bits required for y and z
    // and shift y by the bits required for z
    (x << (y_bits + z_bits)) | (y << z_bits) | z
}
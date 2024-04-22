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

// external structs and functions
use models::avalanche::Avalanche;
use models::grain::Grain;
use models::location::Location;
use util::sandpileUtil::drawPile;
use util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitudeWithAlpha;

// external modules
extern crate rand;
use rand::Rng;
use std::vec::Vec;

// internal modules
pub mod models;
pub mod util;

// Constants
use util::constants::ALPHA_LANDING;
use util::constants::DEBUG;
use util::constants::DEBUG_AVALANCHE;
use util::constants::DEBUG_INIT;
use util::constants::TOTAL_GRAINS;
use util::constants::X_SIZE;
use util::constants::Y_SIZE;
use util::constants::Z_SIZE;
use util::constants::TERMINAL_FREE_FALL_SPEED;









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

                    if DEBUG && DEBUG_INIT { println!("Creating location x: {}, y: {}, z: {} has Id {}, capacity {} and resilience {}", x, y, z, location.id, location.capacity, location.resilience); }
                    // add the location to the array
                    column_z.push(location);
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

    if DEBUG && DEBUG_INIT {
        println!("---------------- Array created with x: {}, y: {}, z: {} ----------------", array.len(), array[0].len(), array[0][0].len());
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

    if DEBUG && DEBUG_INIT {
        println!("---------------- Grains created with count: {} ----------------", grains.len());
    }

    // initialize a vec of all avalanches
    let mut avalanches: Vec<Avalanche> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the avalanches in the array each grain causes an avalanche 
    // of some size, might be as small as joining the first location it lands on
    // or as big
    for i in 0..TOTAL_GRAINS {
        // create a grain 
        let avalanche = Avalanche::new(i as u32);
        avalanches.push(avalanche);
    }

    if DEBUG && DEBUG_INIT {
        println!("---------------- Avalanches created with count: {} ----------------", avalanches.len());
    }
    

    for i in 0..TOTAL_GRAINS {

        // Start the avalanche for the this grains motion
        avalanches[i].addGrain(grains[i].id);

        if DEBUG && DEBUG_AVALANCHE { println!("Grain {} is falling", i); }

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
        if DEBUG && DEBUG_AVALANCHE { println!("Grain {} started at x: {}, y: {}, z: {}", i, x, y, z) };

        // fall until the grain lands on a location that is not at capacity
        // fall through any locations that are empty (resilience == 0)
        // if not at z=0, check the location below to see if it has capacity
        while array[x][y][z].resilience == 0 || ( z > 0 && array[x][y][z-1].grainIds.len() < array[x][y][z-1].capacity ) {
            z -= 1;
            // increase the energy of the grain up to terminal velocity
            if grains[i].energy < TERMINAL_FREE_FALL_SPEED {
                grains[i].incrementEnergy();
            }
        }

        // if z > 0, check that the locations in the neighborhood at z-1 are not at capacity


        if DEBUG && DEBUG_AVALANCHE { println!("Grain {} landed at x: {}, y: {}, z: {}", i, x, y, z); }

        // add the grain to the location
        array[x][y][z].grainImpact(grains[i].id, grains[i].energy, &mut rnd);
        

        if DEBUG && DEBUG_AVALANCHE { println!("array at location x: {}, y: {}, z: {} has grains {}", x, y, z, array[x][y][z].getNumberOfGrains()); }

    }

    // draw the pile
    drawPile(&array);

    // createa Location
    //let mut location = Location::new(0, 0, 0);

}


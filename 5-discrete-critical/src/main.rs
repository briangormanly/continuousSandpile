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

// external modules
extern crate rand;
use rand::Rng;
use std::vec::Vec;

// internal modules
pub mod models;
pub mod util;

// external structs and functions
use models::avalanche::Avalanche;
use models::grain::Grain;
use models::location::Location;

use util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitudeWithAlpha;


// Constants
use util::constants::ALPHA_LANDING;
use util::constants::DEBUG;
use util::constants::DEBUG_AVALANCHE;
use util::constants::DEBUG_INIT;
use util::constants::DEBUG_DISPLAY_PILE;
use util::constants::DEBUG_LOCAL_NEIGHBORS;
use util::constants::X_SIZE;
use util::constants::Y_SIZE;
use util::constants::Z_SIZE;
use util::constants::TERMINAL_FREE_FALL_SPEED;
use util::constants::BASE_RESILIENCE;
use util::constants::BASE_CAPACITY;
use util::constants::TOTAL_GRAINS;



fn main() {
    // create a random number generator
    let mut rnd = rand::thread_rng();

    // initialize the locations as a static mutex hashmap
    models::location::Location::initializeLocations(&mut rnd);


    // initialize a vec of all grains
    //let mut grains: Vec<Grain> = Vec::with_capacity(TOTAL_GRAINS);
    models::grain::Grain::initializeGrains();


    // initialize all the grains in the array
    //initializeGrains(&mut grains, &mut rnd);

    // initialize a vec of all avalanches
    let mut avalanches: Vec<Avalanche> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the avalanches in the array each grain causes an avalanche
    // of some size, might be as small as joining the first location it lands on
    // or as big
    initializeAvalanches(&mut avalanches);

    if DEBUG && DEBUG_INIT {
        println!("---------------- Avalanches created with count: {} ----------------", avalanches.len());
    }

    //
    // Start the simulation
    // for each grain:
    // - add the gain to an avalanche
    // - determine the initial location
    // - fall until the grain lands on a location that is not at capacity
    //
    for i in 0..TOTAL_GRAINS {

        // Add the new falling grain to the avalanche, this is grain 0
        avalanches[i].addGrain(i as u32);

        if DEBUG && DEBUG_AVALANCHE { println!("------------ AVALANCHE {} START ------------", i); }

        // print out all of the states of the grains in the avalanche
        for grainId in &avalanches[i].grainIds {
            let grain = models::grain::Grain::getGrainById(*grainId).unwrap();
        }

        // Run through the avalanche until all grains have come to rest
        // first get the initial number of grains in the avalanche
        let mut totalGrains = avalanches[i].grainIds.len();

        // while the number of grains in the avalanche is greater than 0, this avalanche is still active
        while totalGrains > 0 {
            // determine the number of grains in the avalanche at this point in time
            totalGrains = avalanches[i].grainIds.len();

            // for each grain currently in the avalanche, update the grain at this time period
            let mut previous_len = totalGrains;
            for mut j in 0..totalGrains {
                // get the grains id
                println!("about to look for avalanche index {} with grain index {}, the total grains in the avalanche is {} and the previous index was {}", i, j, avalanches[i].grainIds.len(), previous_len);
                // if the number of grains in the avalanche has changed, decrease the index
                if avalanches[i].grainIds.len() < previous_len && j > 0 {
                    
                    j = avalanches[i].grainIds.len() -1;
                    println!("removing grain(s) from avalanche, new j value is {}", j);
                }
                let grainId = avalanches[i].grainIds[j];
                // get the amount of grains in the avalanche before the update
                let previous_len = avalanches[i].grainIds.len();

                // perform the update on the grain
                avalanches[i].update( grainId );

                println!("total grains in avalanche based on array length is {}, previous length is {}, j value is {}",avalanches[i].grainIds.len(), previous_len, j);

                

                //println!("total grains in avalanche based on array length is {}, previous length is {}, j value is {}",avalanches[i].grainIds.len(), previous_len, j);
                // // check to see if the total grains in the avalanche has changed in the update
                // totalGrains = avalanches[i].grainIds.len();
                // println!("total grains in avalanche based on array length is {}, j value is {}",totalGrains, j);
                // if j >= totalGrains && j > 0 {
                //     j = totalGrains - 1;
                // }

            }




            // // for each grain in the avalanche update the grain
            // for j in 0..avalanches[i].grainIds.len() {
            //     let grainId = avalanches[i].grainIds[j];
            //     avalanches[i].update( grainId );
            // }

            // // update the total grains in the avalanche
            // totalGrains = avalanches[i].grainIds.len();
        }



        // // for each grain in the avaanche update the grain
        // // update the avalanche while there are grains in the avalanche
        // while avalanches[i].grainIds.len() > 0{
        //     for j in 0..avalanches[i].grainIds.len() {
        //         let grainId = avalanches[i].grainIds[j];
        //         avalanches[i].update( grainId );
        //     }
        // }

        // while avalanches[i].grainIds.len() > 0 {
        //     let mut j = 0;
        //     while j < avalanches[i].grainIds.len() {
        //         let grainId = avalanches[i].grainIds[j];
        //         let previous_len = avalanches[i].grainIds.len();
        //         avalanches[i].update(grainId);
        //         if avalanches[i].grainIds.len() < previous_len {
        //             // an element was removed, decrease the index
        //             j -= 1;
        //         }
        //         j += 1;
        //     }
        // }
        
        

        // print out all of the states of the grains in the avalanche
        for grainId in &avalanches[i].grainIds {
            let grain = models::grain::Grain::getGrainById(*grainId).unwrap();
            println!("Grain {} is in state {:?} at the end of the run", grain.id, grain.state);
            // print each grains location
            println!("Grain {} is at location x {}, y {}, z {}", grain.id, grain.x, grain.y, grain.z);
        }


        if DEBUG && DEBUG_AVALANCHE { println!("------------ AVALANCHE {} END ------------\n", i) }


        // // determine initial x, y, z location
        // let (mut x, mut y, mut z) = initialGrainPosition(i, &mut array, &mut grains, &mut rnd);

        // // see if the array location is not at capacity and fall until it is not
        // if DEBUG && DEBUG_AVALANCHE { println!("Grain {} started at x: {}, y: {}, z: {}", i, x, y, z) };

        // // determine if the initial grain position is at capacity
        // if array[x][y][z].grainIds.len() >= array[x][y][z].capacity {
        //     if DEBUG && DEBUG_AVALANCHE { println!("----Capacity Start---- Grain {} landed at x: {}, y: {}, z: {} is at capacity", i, x, y, z); }

        //     // fird the lower neighborhood for this location
        //     //let lowerNeighborhood: Vec<&Location> = Vec::new();
        //     let lowerNeighborhood = Location::getLowerNeighborhood(&mut array, x, y, z);

        //     // for the grain to want to fall at least one of the lower neighborhood locations must have capacity.
        //     // if none of the lower neighborhood locations have capacity, the grain sit 1 z level higher
        //     let mut canFall = false;
        //     for location in &lowerNeighborhood {
        //         if location.capacity - location.grainIds.len() > 0 {
        //             canFall = true;
        //         }
        //     }

        //     if !canFall {
        //         z += 1;
        //         println!("----Capacity Cannot Fall ---- Grain {} moved up to z {}", i, z);

        //     }
        //     else {
        //         // pick a location at random from the lower neighborhood and fall to it.
        //         let mut locationIndex = rnd.gen_range(0..lowerNeighborhood.len());
        //         x = lowerNeighborhood[locationIndex].x as usize;
        //         y = lowerNeighborhood[locationIndex].y as usize;
        //         z = lowerNeighborhood[locationIndex].z as usize;
        //         grains[i].incrementEnergy();

        //         println!("----Capacity Can Fall ---- Grain {} moved to x: {}, y: {}, z: {}", i, x, y, z);
        //     }

        //     // print out all the locations in the lower neighborhood
        //     // if DEBUG && DEBUG_LOCAL_NEIGHBORS {
        //     //     println!("Lower neighborhood for x: {}, y: {}, z: {}", x, y, z);
        //     //     for location in lowerNeighborhood {
        //     //         println!("Location x: {}, y: {}, z: {} can fit {} more grains", location.x, location.y, location.z, location.capacity - location.grainIds.len(),);
        //     //     }
        //     // }


        //     if DEBUG && DEBUG_AVALANCHE { println!("----Capacity End---- Grain {} landed at x: {}, y: {}, z: {}", i, x, y, z); }

        // }




        // // add the grain to the location
        // let looseGrains = array[x][y][z].grainImpact(grains[i].id, grains[i].energy, &mut rnd);




    }

    //draw the pile
    if DEBUG && DEBUG_DISPLAY_PILE {
        
        models::location::Location::displayAllLocationFinalPositions();
        models::grain::Grain::displayAllGrainsLocations();
        models::location::Location::displayPile();
    }


    // createa Location
    //let mut location = Location::new(0, 0, 0);

}


fn initializeAvalanches(avalanches: &mut Vec<Avalanche>) {
    for i in 0..TOTAL_GRAINS {
        // create a grain
        let avalanche = Avalanche::new(i as u32);
        avalanches.push(avalanche);
    }
}






// fn initialGrainPosition( i: usize, array: &mut Vec<Vec<Vec<Location>>>, grains: &mut Vec<Grain>, rnd: &mut impl Rng ) -> (usize, usize, usize) {
//     // start with center of the array
//     let mut x = X_SIZE / 2;
//     let mut y = Y_SIZE / 2;

//     // find the gains landing variance from center with more variance in the center
//     // using an alpha of 1.5
//     let mut xVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, rnd);
//     let mut yVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, rnd);

//     // check that the variance is within the bounds of the array
//     if xVariance > x as f64 {
//         xVariance = x as f64;
//     }
//     if yVariance > y as f64 {
//         yVariance = y as f64;
//     }

//     // find the gains landing direction
//     let xDirection = rnd.gen_range(0..2);
//     let yDirection = rnd.gen_range(0..2);

//     // compute the new location of the grain given the variance and direction
//     if xDirection == 0 {
//         x -= xVariance as usize;
//     } else {
//         x += xVariance as usize;
//     }
//     if yDirection == 0 {
//         y -= yVariance as usize;
//     } else {
//         y += yVariance as usize;
//     }

//     let mut z = Z_SIZE - 1;

//     // fall until the grain lands on a location that is not at capacity
//     // fall through any locations that are empty (resilience == 0)
//     // if not at z=0, check the location below to see if it has capacity
//     while array[x][y][z].resilience == 0 || ( z > 0 && array[x][y][z-1].grainIds.len() < array[x][y][z-1].capacity ) {
//         z -= 1;
//         // increase the energy of the grain up to terminal velocity
//         if grains[i].energy < TERMINAL_FREE_FALL_SPEED {
//             //grains[i].incrementEnergy();
//         }
//     }

//     // return the location
//     (x, y, z)
// }
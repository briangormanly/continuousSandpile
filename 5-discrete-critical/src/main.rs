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
use std::collections::HashMap;
use std::vec::Vec;
use rand::Rng;

// internal modules
pub mod models;
pub mod util;

// external structs and functions
use models::avalanche::Avalanche;
use models::grain::Grain;
use models::location::Location;
use models::avalanche;


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


    // for each grain, create an avalanche
    for i in 0..TOTAL_GRAINS {

        // Add the new falling grain to the avalanche, this is grain 0
        avalanches[i].addGrain(i as u32);

        if DEBUG && DEBUG_AVALANCHE { println!("\n\n----------------------------------------------------------------------------------------------") };
        if DEBUG && DEBUG_AVALANCHE { println!("Avalanche {} START", i) };

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
            let previous_len = totalGrains;
            for mut j in 0..totalGrains {
                // get the grains id
                //println!("about to look for avalanche index {} with grain index {}, the total grains in the avalanche is {} and the previous index was {}", i, j, avalanches[i].grainIds.len(), previous_len);

                // if the number of grains in the avalanche has changed, decrease the index
                if avalanches[i].grainIds.len() < previous_len && j > 0 {
                    j = avalanches[i].grainIds.len() -1;
                }
                let grainId = avalanches[i].grainIds[j];
                // get the amount of grains in the avalanche before the update
                let previous_len = avalanches[i].grainIds.len();

                // perform the update on the grain
                avalanches[i].update( grainId );

            }
        }

        if DEBUG && DEBUG_AVALANCHE { println!("Avalanche {} END: total movement: {}, total grains involved: {}", i, avalanches[i].totalMovement, avalanches[i].totalGrainsInvolved) };
        if DEBUG && DEBUG_AVALANCHE { println!("/n/n----------------------------------------------------------------------------------------------") };
    }

    //draw the pile
    if DEBUG && DEBUG_DISPLAY_PILE {
        
        // models::location::Location::displayAllLocationFinalPositions();
        // models::grain::Grain::displayAllGrainsLocations();
        models::location::Location::displayPile();

        // print the total movement of the avalanche
        displayAvalancheTotalMovementStats(&avalanches);
        println!("----------------------------------------------------------------------------------------------");
        displayAvalarcheTotalGrainsStats(&avalanches);
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

pub fn displayAvalarcheTotalGrainsStats(avalanches: &Vec<Avalanche>) {
    // build a hashmap that will store a vector of ids of avalanches for each discrete total grain value within the avalanches vector.
    let mut avalancheTotalGrainsMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // for each avalanche in the vector, add the avalanche id to the vector of ids for the total grain value
    for avalanche in avalanches {
        let totalGrains = avalanche.totalGrainsInvolved;
        if avalancheTotalGrainsMap.contains_key(&totalGrains) {
            avalancheTotalGrainsMap.get_mut(&totalGrains).unwrap().push(avalanche.id);
        } else {
            avalancheTotalGrainsMap.insert(totalGrains, vec![avalanche.id]);
        }
    }

    // print out the total grain value the ids of the avalanches that have that total grain value in ascending order of grain value
    let mut sortedKeys: Vec<usize> = avalancheTotalGrainsMap.keys().cloned().collect();

    sortedKeys.sort();
    println!("Avalanche Grain Count: | Number Avalanches:");
    for totalGrains in sortedKeys {
        println!("{}, {:?}", totalGrains, avalancheTotalGrainsMap.get(&totalGrains).unwrap().len());
    }

    // print out the total grain value and the ids of the avalanches that have that total grain value
    // for (totalGrains, ids) in avalancheTotalGrainsMap {
    //     println!("Total Grains: {}", totalGrains);
    //     println!("Avalanche Ids: {:?}", ids);
    // }

}

pub fn displayAvalancheTotalMovementStats(avalanches: &Vec<Avalanche>) {
    // build a hashmap that will store a vector of ids of avalanches for each discrete total movement value within the avalanches vector.
    let mut avalancheTotalMovementMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // for each avalanche in the vector, add the avalanche id to the vector of ids for the total movement value
    for avalanche in avalanches {
        let totalMovement = avalanche.totalMovement;
        if avalancheTotalMovementMap.contains_key(&totalMovement) {
            avalancheTotalMovementMap.get_mut(&totalMovement).unwrap().push(avalanche.id);
        } else {
            avalancheTotalMovementMap.insert(totalMovement, vec![avalanche.id]);
        }
    }

    // print out the total movment value the ids of the avalanches that have that total movement value in ascending order of movement value
    let mut sortedKeys: Vec<usize> = avalancheTotalMovementMap.keys().cloned().collect();

    sortedKeys.sort();
    println!("Avalanche Movement: | Number Avalanches:");
    for totalMovement in sortedKeys {
        println!("{}, {:?}", totalMovement, avalancheTotalMovementMap.get(&totalMovement).unwrap().len());
    }

    // print out the total movement value and the ids of the avalanches that have that total movement value
    // for (totalMovement, ids) in avalancheTotalMovementMap {
    //     println!("Total Movement: {}", totalMovement);
    //     println!("Avalanche Ids: {:?}", ids);
    // }

}


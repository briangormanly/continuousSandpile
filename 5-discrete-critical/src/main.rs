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
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::vec::Vec;
use rand::Rng;
use chrono::Local;


// internal modules
pub mod models;
pub mod util;

// external structs and functions
use models::avalanche::Avalanche;
use models::grain::Grain;
use models::location::Location;
use models::avalanche;

extern crate rayon;
use rayon::prelude::*;


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
    
    // Each run's data is stored in a folder named with the current timestamp-number of grains-size of pile
    
    // Generate the current timestamp as a folder name
    let timestamp: String = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let start_time: chrono::prelude::DateTime<Local> = Local::now();    

    //let timestamp = format!("{}-{}-{}", timestamp, TOTAL_GRAINS, X_SIZE * Y_SIZE * Z_SIZE);
    let folder_path = format!("./data/{}", timestamp + "-gs-" + &TOTAL_GRAINS.to_string() + "-ps-" + &X_SIZE.to_string() + "-" + &Y_SIZE.to_string() + "-" + &Z_SIZE.to_string());

    // Create the directory using the path
    let _ = fs::create_dir_all(&folder_path);


    // create a random number generator
    let mut rnd = rand::thread_rng();

    // initialize the locations as a static mutex hashmap
    models::location::Location::initializeLocations(&mut rnd);

    // initialize a vec of all grains
    //let mut grains: Vec<Grain> = Vec::with_capacity(TOTAL_GRAINS);
    models::grain::Grain::initializeGrains();

    // initialize a vec of all avalanches
    let mut avalanches: Vec<Avalanche> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the avalanches in the array each grain causes an avalanche
    // of some size, might be as small as joining the first location it lands on
    // or as big
    // Initialize the avalanches
    initializeAvalanches(&mut avalanches);

    // Using Rayon to parallelize this loop
    avalanches.par_iter_mut().enumerate().for_each(|(i, avalanche)| {
        avalanche.addGrain(i as u32);

        // Debugging start of the avalanche
        if DEBUG && DEBUG_AVALANCHE { 
            println!("Avalanche {} START", i);
        }

        // Placeholder for logic to process each grain in the avalanche
        let mut totalGrains = avalanche.grainIds.len();
        while totalGrains > 0 {
            // Simulate some update logic
            totalGrains = avalanche.grainIds.len();
            let previous_len = totalGrains;

            let grain_ids = avalanche.grainIds.clone(); // Create a clone of grainIds

            for grain_id in grain_ids {
                avalanche.update(grain_id);
            }

            // If number of grains changes, possibly due to some grains settling
            if avalanche.grainIds.len() < previous_len {
                // Adjust processing logic if needed
            }
        }

        // Debugging end of the avalanche
        if DEBUG && DEBUG_AVALANCHE {
            println!("Avalanche {} END: total movement: {}, total grains involved: {}", i, avalanche.totalMovement, avalanche.totalGrainsInvolved);
        }
    });

    //draw the pile
    if DEBUG && DEBUG_DISPLAY_PILE {
        
        let _ = models::location::Location::displayAllLocationFinalPositions(folder_path.clone());
        //models::grain::Grain::displayAllGrainsLocations();
        let _ = models::location::Location::displayPile(folder_path.clone());

        // print the total movement of the avalanche
        let _ = displayAvalancheTotalMovementStats(&avalanches, folder_path.clone());
        println!("----------------------------------------------------------------------------------------------");
        let _ = displayAvalarcheTotalGrainsStats(&avalanches, folder_path.clone());
        println!("----------------------------------------------------------------------------------------------");
        let _ = displayAvalancheTotalMagnatude(&avalanches, folder_path.clone());
    }

    // output the total running time of the program using the start_time
    let end_time: chrono::prelude::DateTime<Local> = Local::now();
    let duration: chrono::TimeDelta = end_time.signed_duration_since(start_time);
    println!("Total time: {:?}", duration);

}


fn initializeAvalanches(avalanches: &mut Vec<Avalanche>) {
    for i in 0..TOTAL_GRAINS {
        // create a grain
        let avalanche = Avalanche::new(i as u32);
        avalanches.push(avalanche);
    }
}

pub fn displayAvalarcheTotalGrainsStats(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {
    
    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/grain-stats.csv")?;
    let mut writer = BufWriter::new(file);


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
    writeln!( writer, "Avalanche Grain Count,  Number Avalanches")?;
    for totalGrains in sortedKeys {
        writeln!( writer, "{}, {:?}", totalGrains, avalancheTotalGrainsMap.get(&totalGrains).unwrap().len())?;
    }

    // print out the total grain value and the ids of the avalanches that have that total grain value
    // for (totalGrains, ids) in avalancheTotalGrainsMap {
    //     println!( "Total Grains: {}", totalGrains);
    //     println!( "Avalanche Ids: {:?}", ids);
    // }

    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())

}

pub fn displayAvalancheTotalMovementStats(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {

    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/avalanche-movement-stats.csv")?;
    let mut writer = BufWriter::new(file);
    
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
    writeln!( writer, "Avalanche Movement, Number Avalanches")?;
    for totalMovement in sortedKeys {
        writeln!( writer, "{}, {:?}", totalMovement, avalancheTotalMovementMap.get(&totalMovement).unwrap().len())?;
    }

    // print out the total movement value and the ids of the avalanches that have that total movement value
    // for (totalMovement, ids) in avalancheTotalMovementMap {
    //     println!( "Total Movement: {}", totalMovement);
    //     println!( "Avalanche Ids: {:?}", ids);
    // }

    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())

}

/**
 *  Experimental function to display the total magnatude of the avalanche
 * given as the total grains involved times the total movement of the avalanche
 */
pub fn displayAvalancheTotalMagnatude(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {

    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/avalanche-total-magnitude.csv")?;
    let mut writer = BufWriter::new(file);

    // build a hashmap that will store a vector of ids of avalanches for each discrete total movement value within the avalanches vector.
    let mut avalancheTotalMagnatudeMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // for each avalanche in the vector, add the avalanche id to the vector of ids for the total movement value
    for avalanche in avalanches {
        let totalMagnatude = avalanche.totalGrainsInvolved * avalanche.totalMovement;
        if avalancheTotalMagnatudeMap.contains_key(&totalMagnatude) {
            avalancheTotalMagnatudeMap.get_mut(&totalMagnatude).unwrap().push(avalanche.id);
        } else {
            avalancheTotalMagnatudeMap.insert(totalMagnatude, vec![avalanche.id]);
        }
    }

    // print out the total movment value the ids of the avalanches that have that total movement value in ascending order of movement value
    let mut sortedKeys: Vec<usize> = avalancheTotalMagnatudeMap.keys().cloned().collect();

    sortedKeys.sort();
    writeln!( writer, "Avalanche Magnatude, Number Avalanches")?;
    for totalMagnatude in sortedKeys {
        writeln!( writer, "{}, {:?}", totalMagnatude, avalancheTotalMagnatudeMap.get(&totalMagnatude).unwrap().len())?;
    }
    
    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())

}


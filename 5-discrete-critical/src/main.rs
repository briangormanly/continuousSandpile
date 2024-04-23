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
use util::constants::DEBUG_DISPLAY_PILE;
use util::constants::DEBUG_LOCAL_NEIGHBORS;
use util::constants::TOTAL_GRAINS;
use util::constants::X_SIZE;
use util::constants::Y_SIZE;
use util::constants::Z_SIZE;
use util::constants::TERMINAL_FREE_FALL_SPEED;
use util::constants::BASE_RESILIENCE;
use util::constants::BASE_CAPACITY;



fn main() {
    // create a random number generator
    let mut rnd = rand::thread_rng();

    // Create a 3D vector of locations
    let mut array: Vec<Vec<Vec<Location>>> = Vec::with_capacity(X_SIZE);

    // cerate locations for all the points in the array within the slope of criticality
    initializeLocations(&mut array, &mut rnd);

    if DEBUG && DEBUG_INIT {
        println!("---------------- Array created with x: {}, y: {}, z: {} ----------------", array.len(), array[0].len(), array[0][0].len());
    }

    // initialize a vec of all grains
    let mut grains: Vec<Grain> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the grains in the array
    initializeGrains(&mut grains);

   
    if DEBUG && DEBUG_INIT {
        println!("---------------- Grains created with count: {} ----------------", grains.len());
    }

    // initialize a vec of all avalanches
    let mut avalanches: Vec<Avalanche> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the avalanches in the array each grain causes an avalanche 
    // of some size, might be as small as joining the first location it lands on
    // or as big
    initializeAvalanches(&mut avalanches);
    
    // 
    // Start the simulation
    // for each grain:
    // - add the gain to an avalanche
    // - determine the initial location
    // - fall until the grain lands on a location that is not at capacity
    // 
    for i in 0..TOTAL_GRAINS {

        // Start the avalanche for the this grains motion
        avalanches[i].addGrain(grains[i].id);

        if DEBUG && DEBUG_AVALANCHE { println!("Grain {} is falling", i); }

        // determine initial x, y, z location
        let (x, y, z) = initialGrainPosition(i, &mut array, &mut grains, &mut rnd);

        // see if the array location is not at capacity and fall until it is not
        if DEBUG && DEBUG_AVALANCHE { println!("Grain {} started at x: {}, y: {}, z: {}", i, x, y, z) };

        

        // fird the lower neighborhood for this location
        //let lowerNeighborhood: Vec<&Location> = Vec::new();
        let lowerNeighborhood = getLowerNeighborhood(&mut array, x, y, z);



        if DEBUG && DEBUG_AVALANCHE { println!("Grain {} landed at x: {}, y: {}, z: {}", i, x, y, z); }

        // add the grain to the location
        array[x][y][z].grainImpact(grains[i].id, grains[i].energy, &mut rnd);
        

        if DEBUG && DEBUG_AVALANCHE { println!("array at location x: {}, y: {}, z: {} has grains {}", x, y, z, array[x][y][z].getNumberOfGrains()); }

    }

    // draw the pile
    if DEBUG && DEBUG_DISPLAY_PILE {
        drawPile(&array);
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

    if DEBUG && DEBUG_INIT {
        println!("---------------- Avalanches created with count: {} ----------------", avalanches.len());
    }
}

fn initializeLocations(array: &mut Vec<Vec<Vec<Location>>>, rnd: &mut impl Rng) {
    let mut count = 0;
    for x in 0..X_SIZE {
        let mut layer_y: Vec<Vec<Location>> = Vec::with_capacity(Y_SIZE);
        for y in 0..Y_SIZE {
            let mut column_z: Vec<Location> = Vec::with_capacity(Z_SIZE);
            for z in 0..Z_SIZE {
                
                // create a location
                if x>=z && x<=X_SIZE-z-1 && y>=z && y<=Y_SIZE-z-1 {
                    // create a location
                    let location = Location::new(count as usize, x as i32, y as i32, z as i32, rnd);
                    if DEBUG && DEBUG_INIT { println!("Creating location x: {}, y: {}, z: {} has Id {}, capacity {} and resilience {}", x, y, z, location.id, location.capacity, location.resilience); }
                    
                    // add the location to the array
                    column_z.push(location);
                }
                else {
                    // empty 
                    let location = Location::emptySpace(count as usize, x as i32, y as i32, z as i32);
                    if DEBUG && DEBUG_INIT { println!("Creating location x: {}, y: {}, z: {} has Id {}, capacity {} and resilience {}", x, y, z, location.id, location.capacity, location.resilience); }

                    column_z.push(location);
                    //println!("!!!Location outside of critical slope x: {}, y: {}, z: {}", x, y, z);
                }
                count += 1;
            }
            layer_y.push(column_z);
            
        }
        array.push(layer_y);
    }
}


fn initializeGrains(grains: &mut Vec<Grain>) {
    

    // initialize all the grains in the array
    for i in 0..TOTAL_GRAINS {
        // create a grain 
        let mut grain = Grain::new(i as u32);
        // set the grain in motion
        grain.incrementEnergy();
        grains.push(grain);
    }
}

fn initialGrainPosition( i: usize, array: &mut Vec<Vec<Vec<Location>>>, grains: &mut Vec<Grain>, rnd: &mut impl Rng ) -> (usize, usize, usize) {
    // start with center of the array
    let mut x = X_SIZE / 2;
    let mut y = Y_SIZE / 2;

    // find the gains landing variance from center with more variance in the center
    // using an alpha of 1.5
    let mut xVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, rnd);
    let mut yVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, rnd);

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

    let mut z = Z_SIZE - 1;
    
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
    
    // return the location
    (x, y, z)
}

fn getLowerNeighborhood(array: &mut Vec<Vec<Vec<Location>>>, x: usize, y: usize, z: usize) -> Vec<&Location> {
    let mut lowerNeighborhood: Vec<&Location> = Vec::with_capacity(9);

    if z > 0 {
        // add the locations in the neighborhood at z-1
        // to the lowerNeighborhood


        let minX = if x == 0 { 0 } else { x-1 };
        let maxX = if x+1 < X_SIZE { x+1 } else { X_SIZE };
        let minY = if y == 0 { 0 } else { y-1 };
        let maxY = if y+1 < Y_SIZE { y+1 } else { Y_SIZE };
        if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("Neighborhood to check - minX: {}, maxX: {}, minY: {}, maxY: {} for z:: {}", minX, maxX, minY, maxY, z-1); }

        // keep track of how many locations are not at capacity in the lower neighborhood
        let mut belowNumberOpen = 0;

        // iterate for each level below the current level
        for i in minX..maxX + 1 {
            for j in minY..maxY + 1 {
                
                // check to see is the spot is in bounds
                if i >= X_SIZE || j >= Y_SIZE {
                    if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("checkSlope: out of bounds spot possible: x: {}, y: {}", i, j); }
                    // add an out of bounds spot to the belowSlice array
                    //belowSlice[belowNumberOpen] = (i, j);
                    //belowNumberOpen += 1;

                    //continue;
                }
                else if array[i as usize][j as usize][(z-1) as usize].grainIds.len() < array[i as usize][j as usize][(z-1) as usize].capacity {
                    if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("checkSlope: Found open spot at x: {}, y: {}, z: {}", i, j, z-1); }
                    println!("len: lowerNeighborhood: {} array length: {}, {}, {}", lowerNeighborhood.len(), array.len(), array[0].len(), array[0][0].len());
                    lowerNeighborhood.push(&array[i as usize][j as usize][(z-1) as usize]);
                    belowNumberOpen += 1;
                }
            }
        }

        if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("checkSlope: Below number open: {}", belowNumberOpen); }
        //if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("checkSlope: Below slice: {:?}", lowerNeighborhood); }
    }

    
    
    return lowerNeighborhood;
}
// external modules
extern crate rand;
use rand::Rng;
use std::vec::Vec;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

// internal modules
use crate::util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitude;
use crate::util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitudeWithAlpha;

// Constants
use crate::util::constants::DEBUG;
use crate::util::constants::DEBUG_AVALANCHE;
use crate::util::constants::DEBUG_LOCATION;
use crate::util::constants::DEBUG_LOCAL_NEIGHBORS;
use crate::util::constants::DEBUG_GRAIN_IMPACT;
use crate::util::constants::DEBUG_DISPLAY_PILE;
use crate::util::constants::DEBUG_INIT;
use crate::util::constants::BASE_CAPACITY;
use crate::util::constants::BASE_RESILIENCE;
use crate::util::constants::ALPHA_EXTRA_ENERGY;
use crate::util::constants::ALPHA_AVALANCHE_SIZE;
use crate::util::constants::X_SIZE;
use crate::util::constants::Y_SIZE;
use crate::util::constants::Z_SIZE;


/**
 * Static HashMap to store all the locations in the sandpile
 */
lazy_static! { // Require the lazy_static crate to handle static Mutex
    // create a static mutex HashMap to store all the locations, use the location coordinates as the key for constant time access
    static ref LOCATIONS: Mutex<HashMap<(i32, i32, i32), Location>> = Mutex::new(HashMap::new());
}


/**
 * Model for a location in the sandpile
 * Locations are static and do not move, they represent a point in the 3D space
 * They have a capacity for grains and a resilience to purturbations which is 
 * determined as a random value between 1 and 6
 */
#[derive(Clone)]
pub struct Location {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub capacity: usize,
    pub grainIds: Vec::<u32>,
    pub resilience: usize,
}

impl Location {
    pub fn new(id: u32, x: i32, y: i32, z: i32, rnd: &mut impl Rng ) -> Self {

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
    pub fn emptySpace(id: u32, x: i32, y: i32, z: i32) -> Self {

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

    /**
     * retrieve a location by its id from the static HashMap
     */

    // Modify addLocation to use coordinates as the key
    fn addLocation(location: Location) {
        let mut locations = LOCATIONS.lock().unwrap();
        locations.insert((location.x, location.y, location.z), location);
    }

    // Add getLocationByLocation to retrieve a location by coordinates
    pub fn getLocationByLocation(x: i32, y: i32, z: i32) -> Option<Location> {
        let locations = LOCATIONS.lock().unwrap();
        locations.get(&(x, y, z)).cloned()
    }


    // pub fn getLocationById(id: u32) -> Option<Location> {
    //     let locations = LOCATIONS.lock().unwrap();
    //     return locations.get(&id).cloned()
    // }

    // fn addLocation(location: Location) {
    //     let mut locations = LOCATIONS.lock().unwrap();
    //     locations.insert(location.id, location);
    // }

    pub fn initializeLocations(rnd: &mut impl Rng) {
        let mut count = 0;
        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                for z in 0..Z_SIZE {

                    let location = if x>=z && x<=X_SIZE-z-1 && y>=z && y<=Y_SIZE-z-1 {
                        Location::new(count as u32, x as i32, y as i32, z as i32, rnd)
                    } else {
                        Location::emptySpace(count as u32, x as i32, y as i32, z as i32)
                    };

                    Location::addLocation(location); // Add location to the HashMap
                    count += 1;
                    
                    // // create a location
                    // if x>=z && x<=X_SIZE-z-1 && y>=z && y<=Y_SIZE-z-1 {
                    //     // create a location
                    //     let location = Location::new(count as u32, x as i32, y as i32, z as i32, rnd);
                    //     if DEBUG && DEBUG_INIT { println!("Creating location x: {}, y: {}, z: {} has Id {}, capacity {} and resilience {}", x, y, z, location.id, location.capacity, location.resilience); }
                        
                    //     // add the location to the array
                    //     column_z.push(location);
                    // }
                    // else {
                    //     // empty 
                    //     let location = Location::emptySpace(count as u32, x as i32, y as i32, z as i32);
                    //     if DEBUG && DEBUG_INIT { println!("Creating location x: {}, y: {}, z: {} has Id {}, capacity {} and resilience {}", x, y, z, location.id, location.capacity, location.resilience); }
    
                    //     column_z.push(location);
                    //     //println!("!!!Location outside of critical slope x: {}, y: {}, z: {}", x, y, z);
                    // }
                    // count += 1;
                }
                //layer_y.push(column_z);
                
            }
            //locations.push(layer_y);
        }

        if DEBUG && DEBUG_INIT {
            let locations = LOCATIONS.lock().unwrap();
            let length = locations.len();
            println!("---------------- Array of locations created with length: {} ----------------", length);
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

        println!("-------- GRAIN IMPACT START ---------\nGrain impact at location x: {}, y: {}, z: {}", self.x, self.y, self.z);
        // first check the impact of the incoming grain on the location
        let looseGrainIds = self.purtubation(grainId, grainEnergy, rnd);
        println!(" purtubation produced looseGrainIds: {:?}", looseGrainIds);

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
        if DEBUG && DEBUG_AVALANCHE { 
            println!("resilience {} < total energy: {} for location {}, {}, {}", self.resilience, tempSpeed, self.x, self.y, self.z); 
        }

        if self.resilience < tempSpeed && self.z > 0 {
            // start an avalanche
            if DEBUG && DEBUG_AVALANCHE { println!("+++++ Avalanche started at location x: {}, y: {}, z: {} which contains {} grains", self.x, self.y, self.z, self.grainIds.len()) };
            // set the size of the avalanche
            let mut avalancheSize = 2 + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
            
            // ensure that the base avalanche size is not larger than the number of grains
            if self.grainIds.len() < avalancheSize {
                avalancheSize = self.grainIds.len();
            }

            // add the perturbed grain to the avalanche
            //avalanche.addGrain();

            if DEBUG && DEBUG_AVALANCHE { println!("+++++ Avalanche size: {}", avalancheSize) };
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

    pub fn getLowerNeighborhood(array: &mut Vec<Vec<Vec<Location>>>, x: usize, y: usize, z: usize) -> Vec<&Location> {
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
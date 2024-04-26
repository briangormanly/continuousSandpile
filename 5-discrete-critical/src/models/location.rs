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

// internal models
use crate::models::grain::Grain;
use crate::models::avalanche::Avalanche;
use crate::models::grain::GrainState;


//Static HashMap to store all the locations in the sandpile
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
    pub fn getLocationByXyz(x: i32, y: i32, z: i32) -> Option<Location> {
        let locations = LOCATIONS.lock().unwrap();
        locations.get(&(x, y, z)).cloned()
    }

    pub fn saveLocation(&mut self) {
        let mut locations = LOCATIONS.lock().unwrap();
        let location_key = (self.x, self.y, self.z);
        locations.insert(location_key, self.clone());

    }

    /**
     * Initialize all of the locations in the sandpile
     */
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
                    
                }                
            }
        }

        if DEBUG && DEBUG_INIT {
            let locations = LOCATIONS.lock().unwrap();
            let length = locations.len();
            println!("---------------- Array of locations created with length: {} ----------------", length);
        }
    }

    /**
     * Attempt to add a grain to the location
     * 
     */
    pub fn incomingGrain(&mut self, grainId: u32) -> usize {

        // Check if the location has capacity to add a grain
        if self.grainIds.len() < self.capacity {
            // the location is not full, add the grain
            self.grainIds.push(grainId);

            // get the grain by its id
            let mut grain = Grain::getGrainById(grainId as u32).unwrap();

            // set the grain state to sationary
            grain.state = GrainState::Stationary;

            // remove the grains energy
            let energy = grain.energy;
            grain.energy = 0;

            // note that the grain stoped at this location
            //println!("Grain {} stopped at location x: {}, y: {}, z: {} Grian x: {}, y: {}, z: {}", grain.id, self.x, self.y, self.z, grain.x, grain.y, grain.z);

            // save the grain
            grain.saveGrain();


            return energy;
            

        } else {
            // if full the grain will roll down the pile
            // get the grain by its id
            let mut grain = Grain::getGrainById(grainId as u32).unwrap();

            // set the grain state back to rolling
            grain.state = GrainState::Rolling;

            let energy: usize = grain.energy;
            // reduce the grains energy from the impact
            if grain.energy > 1 {
                grain.energy = 1;
            }
            // save the grain state
            grain.saveGrain();

            return energy;
        }
        
    }


    pub fn purtubation(&mut self, incomingGrainEnergy: usize, rnd: &mut impl Rng) -> Vec<u32> {
        // get the order of magnitude of a random power-law distribution
        // as random additional energy representing a purtubation of the location
        // add this value to the grains current energy
        let additionalEnergy = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_EXTRA_ENERGY, rnd);
        let totalEnergy = incomingGrainEnergy + additionalEnergy as usize;

        // determine if this purturbation will cause an avalanche
        if DEBUG && DEBUG_AVALANCHE { 
            println!("resilience {} < total energy: {} ({} + {}) for location {}, {}, {}", self.resilience, totalEnergy, incomingGrainEnergy, additionalEnergy, self.x, self.y, self.z); 
        }

        if self.resilience < totalEnergy && self.z > 0 {
            // start an avalanche
            if DEBUG && DEBUG_AVALANCHE { println!("**************************!! Avalanche started at location x: {}, y: {}, z: {} location contains {} grains (before pertubation)", self.x, self.y, self.z, self.grainIds.len()) };
            // set the size of the avalanche
            let mut avalancheSize = 2 + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
            
            // ensure that the base avalanche size is not larger than the number of grains
            if self.grainIds.len() < avalancheSize {
                avalancheSize = self.grainIds.len();
            }

            if DEBUG && DEBUG_AVALANCHE { println!("+++++ Avalanche size: {}", avalancheSize) };
            let mut looseGrainIds: Vec<u32> = Vec::new();

            // return the grains that are part of the avalanche
            for i in 0..avalancheSize {
                looseGrainIds.push(self.grainIds.pop().unwrap());
            }

            // remove the grain from the location ids
            self.grainIds.retain(|&x| x != looseGrainIds[0]);

            // create a vector to hold any additional grains that fall from above
            let mut additionalGrains: Vec<u32> = Vec::new();

            // change the grains state to rolling
            for grainId in &looseGrainIds {
                let mut grain = Grain::getGrainById(*grainId).unwrap();
                grain.state = GrainState::Rolling;
                grain.energy += 1;
                additionalGrains.push(grain.id);
                

                grain.saveGrain();
            }

            //add the additional grains to the looseGrainIds
            looseGrainIds.append(&mut additionalGrains);

            // save the location
            self.saveLocation();

            // check locations above to ensure they fall into the avalanche
            for (x, y, z) in Location::getCeilingLocations(self.x, self.y, self.z) {
                
                let mut location = Location::getLocationByXyz(x, y, z).unwrap();
                //println!("~~~~~~~~~~~~~ Location x: {}, y: {}, z: {} ~~~~~~~~ had location above with {} gains", x, y, z, location.grainIds.len());
                for grainId in &location.grainIds {
                    let mut grain = Grain::getGrainById(*grainId).unwrap();
                    grain.state = GrainState::Rolling;
                    grain.energy += 1;
                    grain.saveGrain();
                    //println!("~~~~~~~~~~~~~ Grain id: {} x: {}, y: {}, z: {} joined from above ~~~~~~~~", grain.id, grain.x, grain.y, grain.z);
                }
            }

            if DEBUG && DEBUG_AVALANCHE { println!("**************************!! Avalanche at location x: {}, y: {}, z: {} location contains {} grains (after pertubation)", self.x, self.y, self.z, self.grainIds.len()) };
            return looseGrainIds;

        } else {
            if DEBUG && DEBUG_AVALANCHE { println!("Location x: {}, y: {}, z: {} was not perturbed", self.x, self.y, self.z) };
            Vec::new() // Return an empty vector
        }
    }

    /**
     * Get the lower neighborhood of a location by its x, y, z coordinates
     */
    pub fn getLowerNeighborhood( x: i32, y: i32, z: i32 ) -> Vec<(i32, i32, i32)> {
        let mut lowerNeighborhood: Vec<(i32, i32, i32)> = Vec::with_capacity(9);

        let minX = if x == 0 { 0 } else { x-1 } as i32;
        let maxX = if x+1 < X_SIZE { x+1 } else { X_SIZE } as i32;
        let minY = if y == 0 { 0 } else { y-1 } as i32;
        let maxY = if y+1 < Y_SIZE { y+1 } else { Y_SIZE } as i32;
        if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("Neighborhood to check - minX: {}, maxX: {}, minY: {}, maxY: {} for z:: {}", minX, maxX, minY, maxY, z-1); }

        // keep track of how many locations are not at capacity in the lower neighborhood
        //let belowNumberOpen = 0;

        for i in minX..=maxX {
            for j in minY..=maxY {
                if z > 0 {
                    // If not at the ground level, normal neighborhood logic
                    lowerNeighborhood.push((i, j, z - 1));
                } else {
                    // Handling edge cases where grain might "fall off"
                    if i == x && j == y {
                        // Do not add the current location itself when z is 0
                        lowerNeighborhood.push((i, j, z - 1));
                    }
                    if i == 0 || i == X_SIZE - 1 || j == 0 || j == Y_SIZE - 1 {
                        // Marked locations indicating falling off the pile
                        lowerNeighborhood.push((-1, -1, -1)); // Use an invalid location (-1, -1, -1) to indicate falling off
                    } else {
                        // Add surrounding locations at the same level
                        lowerNeighborhood.push((i, j, z));
                    }
                }
            }
        }
        
        return lowerNeighborhood;
    }

    fn getCeilingLocations(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
        let mut ceilingLocations: Vec<(i32, i32, i32)> = Vec::with_capacity(Z_SIZE as usize);

        // any grains located in locations above the current location should join the avalanche by falling down
        if z < Z_SIZE - 1 {
            for i in z..Z_SIZE {
                ceilingLocations.push((x, y, i));
            }
        }
        return ceilingLocations;
    }

    /**
     * Display the contents of the sandpile
     */
    pub fn displayPile() {
        // show the contents of all the locations in the sandpile
        let mut grandTotal = 0;
        for z in (0..Z_SIZE).rev() {
            for y in 0..Y_SIZE {
                print!("\n");
                for x in 0..X_SIZE {
                    // get the location at this x, y, z
                    let location = Location::getLocationByXyz(x, y, z).unwrap();

                    //print!("x:{}, y:{}, z:{} value:{}", x, y, z, );
                    print!("{}", location.getNumberOfGrains());
                    grandTotal += location.getNumberOfGrains();
                }
            }
            println!("\n");
        }
        println!(" ");
        println!("Total grains in the pile: {}", grandTotal);

    }

    pub fn displayAllLocationFinalPositions() {
        // show the contents of all the locations in the sandpile
        for z in (0..Z_SIZE).rev() {
            for y in 0..Y_SIZE {
                for x in 0..X_SIZE {
                    // get the location at this x, y, z
                    let location = Location::getLocationByXyz(x, y, z).unwrap();

                    // print the location information
                    println!("\nx:{}, y:{}, z:{} grains: {:?}", x, y, z, location.grainIds);
                    // get all of the grains at this location and print their information
                    for grainId in &location.grainIds {
                        let grain = Grain::getGrainById(*grainId).unwrap();
                        println!(" Grain id: {}, x: {}, y: {}, z: {}, energy: {}", grain.id, grain.x, grain.y, grain.z, grain.energy);
                    }
                }
            }
        }
    }

    pub fn getNumberOfGrains(&self) -> usize {
        return self.grainIds.len();
    }
}
// external modules
extern crate rand;
use rand::Rng;

// constants
use crate::util::constants::{ALPHA_LANDING, X_SIZE, Y_SIZE, Z_SIZE};
use crate::util::constants::{DEBUG, DEBUG_LOCATION};

// internal utilities
use crate::util::sandpileUtil::{normalizedPowerLawByOrdersOfMagnitudeWithAlpha};

#[derive(PartialEq)]
pub enum GrainState {
    Unknown,
    Falling,
    Resting,
    Avalanche,
}

pub struct Grain {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    // current energy of the grain, 
    // 0 if stationary, > 0 if in motion
    // energy is transferred to other grains on impact
    pub energy: usize,
    // state of the grain
    pub state: GrainState,
}

/**
 * Model for a grain of sand in the system
 * Will be initialized with initial energy and direction of 0
 * which should be set for grains in motion
 */
impl Grain {
    // Constructor to create a new Grain with a specific id
    pub fn new(id: u32) -> Grain {
        let (x, y, z) = Grain::determineInitialPosition(id);
        Grain { 
            id, 
            // current energy of the grain, 
            // 0 if stationary, > 0 if in motion
            // energy is transferred to other grains on impact
            energy: 0,
            x,
            y,
            z,    
            state: GrainState::Unknown,        
        }
    }
    
    pub fn fall(&mut self) {
        // fall until the grain lands on a location that is not at capacity
        // fall through any locations that are empty (resilience == 0)
        // if not at z=0, check the location below to see if it has capacity



        // while array[x][y][z].resilience == 0 || ( z > 0 && array[x][y][z-1].grainIds.len() < array[x][y][z-1].capacity ) {
        //     z -= 1;
        //     // increase the energy of the grain up to terminal velocity
        //     if grains[i].energy < TERMINAL_FREE_FALL_SPEED {
        //         //grains[i].incrementEnergy();
        //     }
        // }



        
    }


    /**
     * Determine the initial position of the grain
     * 
     * @param id - the id of the grain
     * @return (x, y, z) - the initial position of the grain
     */
    fn determineInitialPosition(id: u32) -> (i32, i32, i32) {

        let mut rnd = rand::thread_rng();

        // start with center of the array
        let mut x = (X_SIZE / 2) as i32;
        let mut y = (Y_SIZE / 2) as i32;

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
            x -= xVariance as i32;
        } else {
            x += xVariance as i32;
        }
        if yDirection == 0 {
            y -= yVariance as i32;
        } else {
            y += yVariance as i32;
        }

        let z = (Z_SIZE - 1) as i32;

        if DEBUG && DEBUG_LOCATION {
            println!("Grain {} initialized at: {}, y: {}, z: {}", id, x, y, z);
        }

        (x, y, z)
    }
    fn increaseEnergy(&mut self, energy: usize) {
        self.energy += energy;
    }
    fn incrementEnergy(&mut self) {
        self.energy += 1;
    }
    
}

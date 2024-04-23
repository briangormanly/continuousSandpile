// external modules
extern crate rand;
use rand::Rng;
use std::vec::Vec;

// internal modules
use crate::util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitude;
use crate::util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitudeWithAlpha;

// Constants
use crate::util::constants::DEBUG;
use crate::util::constants::DEBUG_AVALANCHE;
use crate::util::constants::DEBUG_LOCATION;
use crate::util::constants::DEBUG_LOCAL_NEIGHBORS;
use crate::util::constants::BASE_CAPACITY;
use crate::util::constants::BASE_RESILIENCE;
use crate::util::constants::ALPHA_EXTRA_ENERGY;
use crate::util::constants::ALPHA_AVALANCHE_SIZE;
use crate::util::constants::X_SIZE;
use crate::util::constants::Y_SIZE;
use crate::util::constants::Z_SIZE;



/**
 * Model for a location in the sandpile
 * Locations are static and do not move, they represent a point in the 3D space
 * They have a capacity for grains and a resilience to purturbations which is 
 * determined as a random value between 1 and 6
 */
pub struct Location {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub capacity: usize,
    pub grainIds: Vec::<u32>,
    pub resilience: usize,
}

impl Location {
    pub fn new(id: usize, x: i32, y: i32, z: i32, rnd: &mut impl Rng ) -> Self {

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
    pub fn emptySpace(id: usize, x: i32, y: i32, z: i32) -> Self {

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
        if DEBUG && DEBUG_AVALANCHE { 
            println!("resilience {} < total energy: {} for location {}, {}, {}", self.resilience, tempSpeed, self.x, self.y, self.z); 
        }

        if self.resilience < tempSpeed {
            // start an avalanche
            if DEBUG && DEBUG_AVALANCHE { println!("Avalanche started at location x: {}, y: {}, z: {} which contains {} grains", self.x, self.y, self.z, self.grainIds.len()) };
            // set the size of the avalanche
            let mut avalancheSize = 2 + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
            
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
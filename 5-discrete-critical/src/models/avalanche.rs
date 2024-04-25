extern crate rand;
use rand::Rng;

use crate::models::grain::Grain;
use crate::models::grain::GrainState;


/**
 * Model for an avalanche in the sandpile
 * An avalanche is a collection of grains that have been preturbed and are moving
 */
pub struct Avalanche {
    pub id: u32,
    // Grains that are currently part of the avalanche
    pub grainIds: Vec::<u32>,
    // List of all locations that have been affected by any of the gains in the avalanche
    pub locationIds: Vec::<u32>,
    
    // direction of the avalanche, determines which
    pub direction: usize,
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

    // update the movement of all the grains currently in the avalanche
    pub fn update( &mut self, grainId: u32 ) {

        // keep track of grains that need to be removed from the avalanche
        let mut toRemove = Vec::new();

        // get the grain from the grain list
        let mut grain = crate::models::grain::Grain::getGrainById(grainId).unwrap();


        println!("Start Update for Grain {} in {:?} state at location x: {}, y: {}, z: {} has energy {}", grain.id, grain.state, grain.x, grain.y, grain.z, grain.energy);
        match grain.state {
            GrainState::Unknown => {
                //println!("Grain {} is responding to {:?} state", grain.id, grain.state);
                grain.state = GrainState::Falling;
                //grain.fall();
                grain.saveGrain();
            },
            GrainState::Falling => {
                //println!("Grain {} is responding to {:?} state", grain.id, grain.state);
                // let the grain fall until it imparts a location
                grain.fall();
                grain.saveGrain();
            },
            GrainState::Impact => {
                // get the location with the same x, y, z as the gain
                let mut location = crate::models::location::Location::getLocationByXyz(grain.x, grain.y, grain.z).unwrap();
                println!("------- Location {} is starting with {} grains", location.id, location.grainIds.len()); 
                let transferredEnergy: usize = location.incomingGrain(grain.id);
                location.saveLocation();

                // if the location has more then 1 grain, check to see if the location has been perturbed by the impact
                // call the location purtubation method
                let mut rnd = rand::thread_rng();
                let perturbedGrains: Vec<u32> = location.purtubation(transferredEnergy, &mut rnd);

                // if there are grains that have been perturbed, add them to the avalanche
                for perGrainId in perturbedGrains {
                    // retrieve the grain from the grain list
                    let mut grain = crate::models::grain::Grain::getGrainById(perGrainId).unwrap();
                    grain.state = GrainState::Rolling;
                    grain.energy += 1;
                    grain.saveGrain();

                    self.addGrain(perGrainId);
                }           


                println!("------- Location {} is ending with {} grains", location.id, location.grainIds.len()); 
                
            },
            GrainState::Rolling => {
                grain.roll();
                grain.saveGrain();
            },
            GrainState::Stationary => {
                // remove the grain from the avalanche
                toRemove.push(grain.id);
            },
        }

        println!("End Update for Grain {} in {:?} state at location x: {}, y: {}, z: {} has energy {}", grain.id, grain.state, grain.x, grain.y, grain.z, grain.energy);
        // Remove the grains that were marked for removal
        self.grainIds.retain(|id| !toRemove.contains(id));
            
        

        
    }

    fn grainFall(&mut self, grain: &mut Grain) {


        // // fall until the grain lands on a location that is not at capacity
        // // fall through any locations that are empty (resilience == 0)
        // // if not at z=0, check the location below to see if it has capacity
        // while array[x][y][z].resilience == 0 || ( z > 0 && array[x][y][z-1].grainIds.len() < array[x][y][z-1].capacity ) {
        //     z -= 1;
        //     // increase the energy of the grain up to terminal velocity
        //     if grains[i].energy < TERMINAL_FREE_FALL_SPEED {
        //         //grains[i].incrementEnergy();
        //     }
        // }

        // while grain.energy == 0 {
        //     grain.z -= 1;
        //     // increase the energy of the grain up to terminal velocity
        //     // if grains[i].energy < TERMINAL_FREE_FALL_SPEED {
        //     //     //grains[i].incrementEnergy();
        //     // }
        // }
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
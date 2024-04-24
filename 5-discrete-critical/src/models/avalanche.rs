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
    // Current locations being impacted by the avalanche
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
        for grainId in &self.grainIds {
            // get the grain from the grain list
            let mut grain = crate::models::grain::Grain::getGrainById(*grainId).unwrap();

            // TODO: get the grain from Grain.GRAINS

            println!("Updating grain {}", grainId);
            if grain.state == GrainState::Unknown {
                grain.state = GrainState::Falling;
                grain.fall();
                grain.saveGrain();
            }
            else if grain.state == GrainState::Falling {
                // do nothing
            }
            else if grain.state == GrainState::Resting {
                // do nothing
            }
            else if grain.state == GrainState::Avalanche {
                // do nothing
            }
            
        }
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
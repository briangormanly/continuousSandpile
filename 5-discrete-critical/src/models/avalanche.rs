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
    pub fn update(&mut self, grain: &mut Grain) {
        for grainId in &self.grainIds {
            println!("Updating grain {}", grainId);
            if grain.state == GrainState::Unknown {
                grain.state = GrainState::Falling;
                grain.fall();
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
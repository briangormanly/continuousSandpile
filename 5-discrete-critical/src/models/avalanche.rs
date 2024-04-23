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
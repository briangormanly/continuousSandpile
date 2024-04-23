

pub struct Grain {
    pub id: u32,
    // current energy of the grain, 
    // 0 if stationary, > 0 if in motion
    // energy is transferred to other grains on impact
    pub energy: usize,
}

/**
 * Model for a grain of sand in the system
 * Will be initialized with initial energy and direction of 0
 * which should be set for grains in motion
 */
impl Grain {
    // Constructor to create a new Grain with a specific id
    pub fn new(id: u32) -> Grain {
        Grain { 
            id, 
            // current energy of the grain, 
            // 0 if stationary, > 0 if in motion
            // energy is transferred to other grains on impact
            energy: 0,
        }
    }
    pub fn increaseEnergy(&mut self, energy: usize) {
        self.energy += energy;
    }
    pub fn incrementEnergy(&mut self) {
        self.energy += 1;
    }
    pub fn impact(&mut self, energy: usize) {
        // TODO determine the probability of energy transfer and the magnitude
        //self.energy += energy;
        // TODO detrmine which of the surrounding locations will be impacted 
        // by this grains motion
    }
}

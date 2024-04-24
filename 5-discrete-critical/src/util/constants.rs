/**
 * Master Debug flag
 */
pub const DEBUG: bool = true;
pub const DEBUG_INIT: bool = true;
pub const DEBUG_AVALANCHE: bool = true;
pub const DEBUG_LOCATION: bool = true;
pub const DEBUG_LOCAL_NEIGHBORS: bool = false;
pub const DEBUG_GRAIN_IMPACT: bool = false;

pub const DEBUG_DISPLAY_PILE: bool = false;

// minimum value (multiplier) for the power-law distribution
// can be used to set a lower bound.
pub const X_MIN: f64 = 1.0;

// Power-law distribution parameters
pub const ALPHA_MAIN: f64 = 2.0;
pub const ALPHA_LANDING: f64 = 1.4;
pub const ALPHA_EXTRA_ENERGY: f64 = 2.0;
pub const ALPHA_AVALANCHE_SIZE: f64 = 1.2;

// total allowed demensions of the pile
pub const X_SIZE: usize = 5;
pub const Y_SIZE: usize = 5;
pub const Z_SIZE: usize = 2;

// Physics constants
pub const TERMINAL_FREE_FALL_SPEED: usize = 3;
pub const BASE_RESILIENCE: usize = 3;
pub const BASE_CAPACITY: usize = 4;
pub const TOTAL_GRAINS: usize = 10;

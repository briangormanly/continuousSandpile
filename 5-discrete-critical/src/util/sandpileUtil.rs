// external modules
extern crate rand;
use rand::Rng;
use std::vec::Vec;

// internal model
use crate::models::location::Location;
use crate::models::grain::Grain;
use crate::models::avalanche::Avalanche;

// constants
use crate::util::constants::{ALPHA_MAIN, ALPHA_AVALANCHE_SIZE, X_SIZE, Y_SIZE, Z_SIZE, X_MIN};



/**
 * 
 */
pub fn drawPile(array: &Vec<Vec<Vec<Location>>>) {
    println!("Drawing the pile");
    let mut grandTotal = 0;
    for z in (0..Z_SIZE).rev() {

        for y in 0..Y_SIZE {

            print!("\n");

            for x in 0..X_SIZE {
                //print!("x:{}, y:{}, z:{} value:{}", x, y, z, array[x][y][z]);
                print!("{}", array[x][y][z].getNumberOfGrains());
                grandTotal += array[x][y][z].getNumberOfGrains();
            }
            
        }
        println!("\n");
    }
    println!(" ");
    println!("Total grains in the pile: {}", grandTotal);
}


/**
 * Mathematics and probability functions
 */

/**
 * Arguments
 * 'alpha' - The exponent of the distribution. 
 * 'x_min' - The minimum value for the power-law distribution.
 * 'rng' - A random number generator.
 * 
 */
fn power_law(alpha: f64, rnd: &mut impl Rng) -> f64 {
    let uniform_rand = rnd.gen::<f64>();  // Generates a random number between 0 and 1
    X_MIN * (1.0 - uniform_rand).powf(-1.0 / (alpha - 1.0))
}

/**
 * 
 */
pub fn normalizedPowerLawByOrdersOfMagnitude(rnd: &mut impl Rng) -> f64{
    return normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_MAIN, rnd);

}

pub fn normalizedPowerLawByOrdersOfMagnitudeWithAlpha(alphaOverride: f64, rnd: &mut impl Rng) -> f64{
    // call the power_law function
    let value = power_law(alphaOverride, rnd);
    // return the order of magnitude of the value
    let orderOfMagnitude = value.log10().floor();
    return orderOfMagnitude;
    
}

fn num_bits_needed(max_value: usize) -> usize {
    // Compute the number of bits required to store max_value
    // This calculates the floor of the logarithm base 2 of max_value and adds 1
    (max_value as f64).log2().ceil() as usize
}

/**
 * Generate a unique id based on the x, y, z coordinates
 */
pub fn generateXyzId(x: usize, y: usize, z: usize) -> usize {

    // Determine the number of bits needed for each dimension
    let x_bits = num_bits_needed(X_SIZE);
    let y_bits = num_bits_needed(Y_SIZE);
    let z_bits = num_bits_needed(Z_SIZE);

    println!("x_bits: {}, y_bits: {}, z_bits: {}", x_bits, y_bits, z_bits);

    // Encode x, y, z into a single usize using bit shifts
    // We shift x by the sum of the bits required for y and z
    // and shift y by the bits required for z
    println!("(x << (y_bits + z_bits)) | (y << z_bits) | z {}", (x << (y_bits + z_bits)) | (y << z_bits) | z);
    (x << (y_bits + z_bits)) | (y << z_bits) | z
}
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rand::Rng;

const TOTAL_GRAINS: usize = 600;
// const X_SIZE: usize = 120;
// const Y_SIZE: usize = 120;
// const Z_SIZE: usize = 60;

// X_SIZE and Y_SIZE must be a minimum of 8 because of the way the random number for distance from center is generated
const X_SIZE: usize = 15;
const Y_SIZE: usize = 15;
const Z_SIZE: usize = 8;
const DEBUG: bool = false;
const FOLLOW_GRAIN: bool = false;
const SHOW_PILE: bool = true;

fn main() {
    println!("Hello, sandpile!");

    let mut fallen_grains = 0;

    //let mut array = [[[0usize; X_SIZE]; Z_SIZE]; Y_SIZE];
    //let mut array: [[[usize; Z_SIZE]; Y_SIZE]; X_SIZE] = [[[0_usize; Z_SIZE]; Y_SIZE]; X_SIZE];
    let mut array = vec![vec![vec![0; Z_SIZE]; Y_SIZE]; X_SIZE];

    // array that contain the total number of avalanches of each number of grains
    let mut avalancheSizes = [0; Z_SIZE * 2];
    let mut largestAvalanche = 0;
    
    for i in 0..TOTAL_GRAINS {
        // add a grain
        let mut current_z = 0;

        // add the grain to the center neighborhood of the pile using a weighted random number for distance from center
        // and a random number for the direction from the center
        // weights for distance from center [7, 5, 2, 1]
        let mut rng = rand::thread_rng();
        let distance_raw = rng.gen_range(0..15);
        let distance;
        if distance_raw < 9 {
            distance = 0;
        }
        else if distance_raw < 12 {
            distance = 1;
        }
        else if distance_raw < 14 {
            distance = 2;
        }
        else {
            distance = 3;
        }
       
        // generate a random number for the direction from the center on x and y
        let x_dir: usize = rng.gen_range(0..3);
        let y_dir: usize = rng.gen_range(0..3);

        if DEBUG { println!("distance_raw {} - distance: {}", distance_raw, distance); }
        if DEBUG { println!("x_dir: {}, y_dir: {}", x_dir, y_dir); }

        let mut x = X_SIZE / 2;
        let mut y = Y_SIZE / 2;
        if x_dir == 0 && x > 1 {
            x -= distance;
        }
        else if x_dir == 2 && x < X_SIZE - 1 {
            x += distance;
        }
        if y_dir == 0 && y > 1{
            y -= distance;
        }
        else if y_dir == 2 && y < Y_SIZE - 1{
            y += distance;
        }

        // determine z by finding the first open spot on this x,y
        while array[x][y][current_z] > 0 {
            if current_z + 1 == Z_SIZE {
                fallen_grains += 1;
                
            }
            else {
                current_z += 1;
            }
        }

        if DEBUG || FOLLOW_GRAIN { println!("\n\n placing grain at x: {}, y: {}, z:{}", x, y, current_z); }

        // add the grain to the pile
        array[x][y][current_z] += 1;

        //  check slope of the newly added grain
        let aSize: usize = checkSlope(&mut array, x, y, current_z, 0, &mut fallen_grains);

        // print the total number of grains in the avalanche from the last update
        if DEBUG || FOLLOW_GRAIN { println!("iteration {} had avalanche size: {}", i, aSize); }

        // record the avalanche in the approapriate index of the avalanche sizes array
        avalancheSizes[aSize] += 1;

        // see if this was the largest avalanche
        if aSize > largestAvalanche {
            largestAvalanche = aSize;
        }

        
    }

    // draw the pile
    if SHOW_PILE {
        drawPile(&array);
    }
    //drawLevel(&array, 0);

    // validate the pile 
    validatePile(&array, &mut fallen_grains);

    // move the recorded avalanche sizes to the new array
    for i in 0..largestAvalanche + 2 {
        println!("Avalanche size: {} had {} occurrences", i, avalancheSizes[i]);
    }

}

//fn checkSlope( array: &mut [ [ [usize; Z_SIZE]; Y_SIZE]; X_SIZE], x: usize, y: usize, z: usize, mut aSize: usize, fallen_grains: &mut usize) -> usize {
fn checkSlope( array: &mut Vec<Vec<Vec<usize>>>, x: usize, y: usize, z: usize, mut aSize: usize, fallen_grains: &mut usize) -> usize {
    
    if z == 0 {
        // return, noting to do, we are at the bottom of the pile
        //println!("checkSlope: Nothing to do - We are at the bottom of the pile");
        return aSize;
    }
    else {
        if DEBUG { println!("checkSlope for new grain at: x: {}, y: {}, z: {}", x, y, z); }
        
        // create an 2D array that contains tuples of open spots for each level below the current level (z)
        //let mut openSpots [ [(0, 0); 9]; z ];
        let mut belowSlice = [(0, 0); 8];
        let mut belowNumberOpen = 0;

        let minX = if x == 0 { 0 } else { x-1 };
        let maxX = if x+1 < X_SIZE { x+1 } else { X_SIZE };
        let minY = if y == 0 { 0 } else { y-1 };
        let maxY = if y+1 < Y_SIZE { y+1 } else { Y_SIZE };
        if DEBUG { println!("Neighborhood to check - minX: {}, maxX: {}, minY: {}, maxY: {} for z:: {}", minX, maxX, minY, maxY, z-1); }

        
        
        // iterate for each level below the current level
        for i in minX..maxX + 1 {
            for j in minY..maxY + 1 {
                // check to see is the spot is in bounds
                if i >= X_SIZE || j >= Y_SIZE  {
                    if DEBUG { println!("checkSlope: out of bounds spot possible: x: {}, y: {}", i, j); }
                    // add an out of bounds spot to the belowSlice array
                    belowSlice[belowNumberOpen] = (i, j);
                    belowNumberOpen += 1;

                    //continue;
                }
                else if array[i][j][z-1] == 0 {
                    if DEBUG { println!("checkSlope: Found open spot at x: {}, y: {}, z: {}", i, j, z-1); }
                    belowSlice[belowNumberOpen] = (i, j);
                    belowNumberOpen += 1;
                }
            }
        }

        if DEBUG { println!("checkSlope: Below number open: {}", belowNumberOpen); }
        if DEBUG { println!("checkSlope: Below slice: {:?}", belowSlice); }

        if belowNumberOpen > 0 {
            // move the grain to the first open spot in the below level
            //array[belowSlice[0].0][z-1][belowSlice[0].1] += 1;
            //determine which spot to move to based on a random number in the range of open spots
            let mut rng = rand::thread_rng();
            let spot = rng.gen_range(0..belowNumberOpen);
            let mut stopFlag: bool = false;

            // check to see if the spot is out of bounds
            if belowSlice[spot].0 >= X_SIZE || belowSlice[spot].1 >= Y_SIZE {
                if DEBUG { println!("checkSlope: Spot chosen is out of bounds: x: {}, y: {}", belowSlice[spot].0, belowSlice[spot].1); }
                *fallen_grains += 1;
                stopFlag = true
            }
            else if belowSlice[spot].0 == 0 && z > 0 && (z-1) == 1 {
                // check for the special case where we are on the second to lowest layer, the spot 
                // is in the 0 index and the lowest layer has a grain there in that spot.
                if array[belowSlice[spot].0][belowSlice[spot].1][z-2] == 1 {
                    if DEBUG { println!("checkSlope: special condition triggered!!!!!: x: {}, y: {}", belowSlice[spot].0, belowSlice[spot].1); }
                    *fallen_grains += 1;
                    stopFlag = true;
                }
            }
            else if belowSlice[spot].1 == 0 && z > 0 && (z-1) == 1 {
                // check for the special case where we are on the second to lowest layer, the spot 
                // is in the 0 index and the lowest layer has a grain there in that spot.
                if array[belowSlice[spot].0][belowSlice[spot].1][z-2] == 1 {
                    if DEBUG { println!("checkSlope: special condition triggered!!!!!: x: {}, y: {}", belowSlice[spot].0, belowSlice[spot].1); }
                    *fallen_grains += 1;
                    stopFlag = true;
                }
            }
            else {
                array[belowSlice[spot].0][belowSlice[spot].1][z-1] += 1;

            }

            if DEBUG || FOLLOW_GRAIN { println!("moving grain at x: {}, y: {}, z: {}", x, y, z); }
            array[x][y][z] -= 1;
            if DEBUG || FOLLOW_GRAIN { println!("checkSlope: -> Grain moved to x: {}, y: {}, z: {}", belowSlice[spot].0, belowSlice[spot].1, z-1); }

            // add the movment to the avalanche total
            aSize += 1;

            // check to see if the moved grain needs is settled
            if !stopFlag {
                aSize = checkSlope(array, belowSlice[spot].0, belowSlice[spot].1, z-1, aSize, fallen_grains);
            }
        }

        return aSize;
    }
}

fn drawPile(array: &Vec<Vec<Vec<usize>>>) {
    for z in 0..Z_SIZE -1 {

        for y in 0..Y_SIZE {

            print!("\n");

            for x in 0..X_SIZE {
                //print!("x:{}, y:{}, z:{} value:{}", x, y, z, array[x][y][z]);
                print!("{}", array[x][y][z]);
            }
            
        }
        println!("\n\n");
    }
    println!(" ");
}

fn drawLevel(array: &Vec<Vec<Vec<usize>>>, level: usize) {

    for y in 0..Y_SIZE {

        print!("\n");

        for x in 0..X_SIZE {
            //print!("x:{}, y:{}, z:{} value:{}", x, y, z, array[x][y][z]);
            print!("{}", array[x][y][level]);
        }
        
    }
    println!("\n\n");
}

fn validatePile(array: &Vec<Vec<Vec<usize>>>, fallen_grains: &mut usize) {

    let mut pile_grains = 0;
    let mut empty_spots = 0;

    for j in 0..X_SIZE {
        for i in 0..Y_SIZE {
            for k in 0..Z_SIZE {
                if array[i][j][k] > 1 {
                    println!("validatePile: Grain at x: {}, z: {}, y: {} has value: {}", i, j, k, array[i][j][k]);
                }
                else if array[i][j][k] == 1 {
                    pile_grains += 1;
                }
                else {
                    empty_spots += 1;
                }
            }
        }
    }
    println!("validatePile: Pile validated");
    println!("validatePile: Total grains: {}", pile_grains);
    println!("validatePile: Fallen grains: {}", fallen_grains);
    println!("validatePile: Empty spots: {}", empty_spots);
}

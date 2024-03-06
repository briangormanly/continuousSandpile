#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rand::Rng;

const TOTAL_GRAINS: usize = 20;
// const X_SIZE: usize = 120;
// const Y_SIZE: usize = 120;
// const Z_SIZE: usize = 60;

// X_SIZE and Y_SIZE must be a minimum of 8 because of the way the random number for distance from center is generated
const X_SIZE: usize = 3;
const Y_SIZE: usize = 3;
const Z_SIZE: usize = 4;

fn main() {
    println!("Hello, sandpile!");

    let mut fallen_grains = 0;

    //let mut array = [[[0usize; X_SIZE]; Z_SIZE]; Y_SIZE];
    let mut array: [[[usize; Z_SIZE]; Y_SIZE]; X_SIZE] = [[[0_usize; Z_SIZE]; Y_SIZE]; X_SIZE];

    // array that contain the total number of avalanches of each number of grains
    let mut avalancheSizes = [0; TOTAL_GRAINS];
    let mut largestAvalanche = 0;
    
    for i in 0..TOTAL_GRAINS {
        // add a grain
        let mut current_z = 0;

        // add the grain to the center neighborhood of the pile using a weighted random number for distance from center
        // and a random number for the direction from the center
        // weights for distance from center [7, 5, 2, 1]
        let mut rng = rand::thread_rng();
        let distance_raw = rng.gen_range(0..12);
        let distance;
        if distance_raw < 9 {
            distance = 0;
        }
        else {
            distance = 1;
        }
        // else if distance_raw < 14 {
        //     distance = 2;
        // }
        // else {
        //     distance = 3;
        // }
       
        // generate a random number for the direction from the center on x and y
        let x_dir: usize = rng.gen_range(0..3);
        let y_dir: usize = rng.gen_range(0..3);

        println!("distance_raw {} - distance: {}", distance_raw, distance);
        println!("x_dir: {}, y_dir: {}", x_dir, y_dir);

        let mut x = X_SIZE / 2;
        let mut y = Y_SIZE / 2;
        if x_dir == 0 {
            x -= distance;
        }
        else if x_dir == 2 {
            x += distance;
        }
        if y_dir == 0 {
            y -= distance;
        }
        else if y_dir == 2 {
            y += distance;
        }

        // determine z by finding the first open spot on this x,y
        //println!("x: {}, y: {}, z {}", x, y, current_z);
        while array[x][y][current_z] > 0 {
            if current_z + 1 == Z_SIZE {
                println!("No open spots for grain at x: {}, y: {} : Z index increase required", x, y);
                fallen_grains += 1;
                
            }
            else {
                current_z += 1;
            }
            //println!("x: {}, y: {}, z {}", x, y, current_z);
        }

        println!(" placing grain at x: {}, y: {}, z:{}", x, y, current_z);

        // add the grain to the pile
        array[x][y][current_z] += 1;

        //  check slope of the newly added grain
        let aSize: usize = checkSlope(&mut array, x, y, current_z, 0, &mut fallen_grains);

        // print the total number of grains in the avalanche from the last update
        //println!("iteration {} had avalanche size: {}", i, aSize);

        // record the avalanche in the approapriate index of the avalanche sizes array
        avalancheSizes[aSize] += 1;

        // see if this was the largest avalanche
        if aSize > largestAvalanche {
            largestAvalanche = aSize;
        }

        
    }

    // move the recorded avalanche sizes to the new array
    for i in 0..largestAvalanche {
        println!("Avalanche size: {} had {} occurrences", i, avalancheSizes[i]);
    }

    // draw the pile
    drawPile(&array);

    // validate the pile 
    validatePile(&array, &mut fallen_grains);

    // print all the recorded avalanche sizes
    //println!("Avalanche sizes: {:?}", largestAvalancheSizes);
}

fn checkSlope( array: &mut [ [ [usize; Z_SIZE]; Y_SIZE]; X_SIZE], x: usize, y: usize, z: usize, mut aSize: usize, fallen_grains: &mut usize) -> usize {
    //println!("checkSlope for new grain at: x: {}, y: {}, z: {}", x, y, z);

    if z == 0 {
        // return, noting to do, we are at the bottom of the pile
        //println!("checkSlope: Nothing to do - We are at the bottom of the pile");
        return aSize;
    }
    else {

        // create an 2D array that contains tuples of open spots for each level below the current level (z)
        //let mut openSpots [ [(0, 0); 9]; z ];
        let mut belowSlice = [(0, 0); 8];
        let mut belowNumberOpen = 0;

        let minX = if x == 0 { 0 } else { x-1 };
        let maxX = if x+1 < X_SIZE { x+1 } else { X_SIZE };
        let minY = if y == 0 { 0 } else { y-1 };
        let maxY = if y+1 < Y_SIZE { y+1 } else { Y_SIZE };
        println!("Neighborhood to check - minX: {}, maxX: {}, minY: {}, maxY: {} for z:: {}", minX, maxX, minY, maxY, z);

        // check for the case when the minX or minY is 0 which means there are open spots out of bounds
        if (maxX - minX) < 2 || (maxY - minY) < 2 {
            println!("checkSlope: Out of bounds spot possible: maxX: {}, minX: {}, maxY {}, minY {}", maxX, minX, maxY, minY);
        }
        
        // iterate for each level below the current level
        for i in minX..maxX + 1 {
            for j in minY..maxY + 1 {
                // check to see is the spot is in bounds
                if i >= X_SIZE || j >= Y_SIZE {
                    //println!("checkSlope: out of bounds spot possible: x: {}, y: {}", i, j);
                    // add an out of bounds spot to the belowSlice array
                    belowSlice[belowNumberOpen] = (i, j);
                    belowNumberOpen += 1;

                    //continue;
                }

                else if array[i][j][z-1] == 0 {
                    //println!("checkSlope: Found open spot at x: {}, y: {}, z: {}", i, j, z-1);
                    belowSlice[belowNumberOpen] = (i, j);
                    belowNumberOpen += 1;
                }
            }
        }

        // println!("checkSlope: Below number open: {}", belowNumberOpen);
        // println!("checkSlope: Below slice: {:?}", belowSlice);

        if belowNumberOpen > 0 {
            // move the grain to the first open spot in the below level
            //array[belowSlice[0].0][z-1][belowSlice[0].1] += 1;
            //determine which spot to move to based on a random number in the range of open spots
            let mut rng = rand::thread_rng();
            let spot = rng.gen_range(0..belowNumberOpen);

            // check to see if the spot is out of bounds
            if belowSlice[spot].0 >= X_SIZE || belowSlice[spot].1 >= Y_SIZE {
                println!("checkSlope: Spot chosen is out of bounds: x: {}, y: {}", belowSlice[spot].0, belowSlice[spot].1);
                *fallen_grains += 1;
            }
            else {
                array[belowSlice[spot].0][belowSlice[spot].1][z-1] += 1;

            }

            //println!("moving grain at x: {}, y: {}, z: {}", x, y, z);
            array[x][y][z] -= 1;
            println!("checkSlope: Grain moved to x: {}, y: {}, z: {}", belowSlice[spot].0, belowSlice[spot].1, z-1);

            // add the movment to the avalanche total
            aSize += 1;

            // check to see if the moved grain needs is settled
            aSize = checkSlope(array, belowSlice[spot].0, belowSlice[spot].1, z-1, aSize, fallen_grains);
        
        }
        

        return aSize;
    }
}

fn drawPile(array: &[[[usize; Z_SIZE]; Y_SIZE]; X_SIZE]) {
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

fn validatePile(array: &[[[usize; Z_SIZE]; Y_SIZE]; X_SIZE], fallen_grains: &mut usize) {

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
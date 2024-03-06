#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rand::Rng;

const TOTAL_GRAINS: usize = 50;
// const X_SIZE: usize = 120;
// const Y_SIZE: usize = 120;
// const Z_SIZE: usize = 60;

// X_SIZE and Y_SIZE must be a minimum of 8 because of the way the random number for distance from center is generated
const X_SIZE: usize = 12;
const Y_SIZE: usize = 12;
const Z_SIZE: usize = 5;

fn main() {
    println!("Hello, sandpile!");

    //let mut array = [[[0usize; X_SIZE]; Z_SIZE]; Y_SIZE];
    let mut array: [[[usize; X_SIZE]; Z_SIZE]; Y_SIZE] = [[[0_usize; X_SIZE]; Z_SIZE]; Y_SIZE];


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
        let distance_raw = rng.gen_range(0..16);
        let distance;
        if distance_raw < 7 {
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

        //println!("distance: {}", distance);
        //println!("x_dir: {}, y_dir: {}", x_dir, y_dir);

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
        while array[x][current_z][y] > 0 {
            current_z+=1;
        }

        println!("x: {}, y: {}, z:{}", x, y, current_z);

        // add the grain to the pile
        array[x][current_z][y] += 1;

        //  check slope of the newly added grain
        let aSize: usize = checkSlope(&mut array, x, current_z, y, 0);

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

    // print all the recorded avalanche sizes
    //println!("Avalanche sizes: {:?}", largestAvalancheSizes);
}

fn checkSlope( array: &mut [ [ [usize; X_SIZE]; Z_SIZE]; Y_SIZE], x: usize, z: usize, y: usize, mut aSize: usize) -> usize {
    //println!("checkSlope for new grain at: x: {}, y: {}, z: {}", x, y, z);

    if z == 0 {
        // return, noting to do, we are at the bottom of the pile
        //println!("checkSlope: Nothing to do - We are at the bottom of the pile");
        return aSize;
    }
    else {

        // create an 2D array that contains tuples of open spots for each level below the current level (z)
        //let mut openSpots [ [(0, 0); 9]; z ];
        let mut belowSlice = [(0, 0); X_SIZE];
        let mut belowNumberOpen = 0;

        let minX = if x == 0 { 0 } else { x-1 };
        let maxX = if x+1 < X_SIZE { x+1 } else { X_SIZE };
        let minY = if y == 0 { 0 } else { y-1 };
        let maxY = if y+1 < Y_SIZE { y+1 } else { Y_SIZE };
        //println!("minX: {}, maxX: {}, minY: {}, maxY: {} for z:: {}", minX, maxX, minY, maxY, z);
        
        // iterate for each level below the current level
        for i in minX..maxX + 1 {
            for j in minY..maxY + 1 {
                // check to see is the spot is in bounds
                if i >= X_SIZE || j >= Y_SIZE {
                    println!("checkSlope: Spot is out of bounds: x: {}, y: {}", i, j);
                    continue;
                }
                //println!("checkSlope: Checking for grain at i: {}, z: {}, j: {} which has value: {}", i, z-1, j, array[i][z-1][j]);
                else if array[i][z-1][j] == 0 {
                    //println!("checkSlope: Found open spot at x: {}, z: {}, y: {}", i, z-1, j);
                    belowSlice[belowNumberOpen] = (i, j);
                    belowNumberOpen += 1;
                }
                else {

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

            array[belowSlice[spot].0][z-1][belowSlice[spot].1] += 1;

            //println!("moving grain at x: {}, z: {}, y: {}", x, z, y);
            array[x][z][y] -= 1;
            //println!("checkSlope: Grain moved to x: {}, z: {}, y: {}", belowSlice[spot].0, z-1, belowSlice[spot].1);

            // add the movment to the avalanche total
            aSize += 1;

            // check to see if the moved grain needs is settled
            aSize = checkSlope(array, belowSlice[spot].0, z-1, belowSlice[spot].1, aSize);
        
        }

        return aSize;
    }
}

fn drawPile(array: &[[[usize; X_SIZE]; Z_SIZE]; Y_SIZE]) {
    for i in 0..X_SIZE {
        for j in 0..Z_SIZE - 1 {

            print!("   ");

            for k in 0..Y_SIZE {
                print!("{}", array[i][j][k]);
            }
            
        }
        println!(" ");
    }
    println!(" ");
}

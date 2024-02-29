#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]


const TOTAL_GRAINS: usize = 4;
const X_SIZE: usize = 5;



const Y_SIZE: usize = 5;
const Z_SIZE: usize = 5;


fn main() {
    println!("Hello, sandpile!");

    let mut array = [[[0usize; 5]; 5]; 5];
    let mut x: usize = 2;
    let mut y: usize = 2;
    let mut z: usize = 0;
    
    for i in 0..TOTAL_GRAINS {
        // add a grain
        let mut i = 0;
        while array[2][i][2] > 0 {
            i+=1;
        }
        array[2][i][2] += 1;

        // check slope
        checkSlope(array, 2, i, 2);

        drawPile(array);
    }
}

fn checkSlope(mut array: [[[usize; 5]; 5]; 5], x: usize, z: usize, y: usize) {
    println!("checkSlope: x: {}, y: {}, z: {}", x, y, z);

    if z == 0 {
        // return, noting to do, we are at the bottom of the pile
        println!("checkSlope: Complete - We are at the bottom of the pile");
        return;
    }
    else {

        // check for the event that grain is surrounded (grain cannot fall)
        let mut currentLevelNumberFilled = 0;
        // keep the slice of the current level (grains cannot fall under the current level)
        // TODO

        for i in 0..5 {
            for j in 0..5 {
                if array[i][z][j] == 0 {
                    //println!("checking for grain at i: {}, z: {}, j: {}", i, z, j);
                    if array[i][z][j] > 0 {
                        currentLevelNumberFilled += 1;
                    }
                }
            }
        }

        println!("checkSlope: Current level number filled: {}", currentLevelNumberFilled);

        if currentLevelNumberFilled < 8 {
            // create an 2D array that contains tuples of open spots for each level below the current level (z)
            //let mut openSpots [ [(0, 0); 9]; z ];
            let mut belowSlice = [(0, 0); 9];
            let mut belowNumberOpen = 0;

            // iterate for each level below the current level
            for i in x-1..x+1 {
                for j in y-1..y+1 {
                    println!("checkSlope: Checking for grain at i: {}, z: {}, j: {} which has value: {}", i, z, j, array[i][z-1][j]);
                    if array[i][z-1][j] == 0 {
                        belowSlice[belowNumberOpen] = (i, j);
                        belowNumberOpen += 1;
                    }
                }
            }

            println!("checkSlope: Below number open: {}", belowNumberOpen);

            if belowNumberOpen > 0 {
                // move the grain to the first open spot in the below level
                array[belowSlice[0].0][z-1][belowSlice[0].1] += 1;
                array[x][z][y] -= 1;
            
            }
        }

        
    }
}

fn drawPile(array: [[[usize; 5]; 5]; 5]) {
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                print!("{}", array[i][j][k]);
            }
            print!("   ");
        }
        println!(" ");
    }
    println!(" ");


    // for dy in array {
    //     for dz in array {
    //         println!("{:?}", dz);
    //     }
    //     println!("");
    // }
}

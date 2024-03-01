#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

const TOTAL_GRAINS: usize = 47;
const X_SIZE: usize = 11;
const Y_SIZE: usize = 11;
const Z_SIZE: usize = 4;

fn main() {
    println!("Hello, sandpile!");

    let mut array = [[[0usize; X_SIZE]; Z_SIZE]; Y_SIZE];
    let x: usize = 2;
    let y: usize = 2;
    let z: usize = 0;

    // array that contain the total number of avalanches of each number of grains
    let mut avalancheSizes = [0; TOTAL_GRAINS];
    let mut largestAvalanche = 0;
    
    for i in 0..TOTAL_GRAINS {
        // add a grain
        let mut current_z = 0;
        while array[X_SIZE / 2][current_z][X_SIZE / 2] > 0 {
            current_z+=1;
        }
        array[X_SIZE / 2][current_z][X_SIZE / 2] += 1;

        // start off with no avalanche size
        let mut aSize = 0;

        // check slope of the newly added grain
        (array, aSize) = checkSlope(array, X_SIZE / 2, current_z, Y_SIZE / 2, 0);

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
    drawPile(array);

    // print all the recorded avalanche sizes
    //println!("Avalanche sizes: {:?}", largestAvalancheSizes);
}

fn checkSlope(mut array: [[[usize; X_SIZE]; Z_SIZE]; Y_SIZE], x: usize, z: usize, y: usize, mut aSize: usize) -> ([[[usize; X_SIZE]; Z_SIZE]; Y_SIZE], usize) {
    //println!("checkSlope for new grain at: x: {}, y: {}, z: {}", x, y, z);

    if z == 0 {
        // return, noting to do, we are at the bottom of the pile
        //println!("checkSlope: Nothing to do - We are at the bottom of the pile");
        return (array, aSize);
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
        //println!("minX: {}, maxX: {}, minY: {}, maxY: {}", minX, maxX, minY, maxY);
        
        // iterate for each level below the current level
        for i in minX..maxX + 1 {
            for j in minY..maxY + 1 {
                //println!("checkSlope: Checking for grain at i: {}, z: {}, j: {} which has value: {}", i, z-1, j, array[i][z-1][j]);
                if array[i][z-1][j] == 0 {
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
            array[belowSlice[0].0][z-1][belowSlice[0].1] += 1;
            array[x][z][y] -= 1;
            //println!("checkSlope: Grain moved to x: {}, z: {}, y: {}", belowSlice[0].0, z-1, belowSlice[0].1);

            // add the movment to the avalanche total
            aSize += 1;

            // check to see if the moved grain needs is settled
            (array, aSize) = checkSlope(array, belowSlice[0].0, z-1, belowSlice[0].1, aSize);
        
        }

        return (array, aSize);
    }
}

fn drawPile(array: [[[usize; X_SIZE]; Z_SIZE]; Y_SIZE]) {
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

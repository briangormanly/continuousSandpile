#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

const TOTAL_GRAINS: usize = 19;

fn main() {
    writeln!( writer, "Hello, sandpile!");

    let mut array = [[[0usize; 5]; 5]; 5];
    let x: usize = 2;
    let y: usize = 2;
    let z: usize = 0;
    
    for i in 0..TOTAL_GRAINS {
        // add a grain
        let mut i = 0;
        while array[2][i][2] > 0 {
            i+=1;
        }
        array[2][i][2] += 1;

        // check slope
        array = checkSlope(array, 2, i, 2);

        drawPile(array);
    }
}

fn checkSlope(mut array: [[[usize; 5]; 5]; 5], x: usize, z: usize, y: usize) -> [[[usize; 5]; 5]; 5] {
    writeln!( writer, "checkSlope for new grain at: x: {}, y: {}, z: {}", x, y, z);

    if z == 0 {
        // return, noting to do, we are at the bottom of the pile
        writeln!( writer, "checkSlope: Nothing to do - We are at the bottom of the pile");
        return array;
    }
    else {

        // check for the event that grain is surrounded (grain cannot fall)
        let mut currentLevelNumberFilled = 0;
        // keep the slice of the current level (grains cannot fall under the current level)
        // TODO

        for i in 0..5 {
            for j in 0..5 {
                if array[i][z][j] == 0 {
                    //writeln!( writer, "checking for grain at i: {}, z: {}, j: {}", i, z, j);
                    if array[i][z][j] > 0 {
                        currentLevelNumberFilled += 1;
                    }
                }
            }
        }

        writeln!( writer, "checkSlope: Current level number filled: {}", currentLevelNumberFilled);

        if currentLevelNumberFilled < 8 {
            // create an 2D array that contains tuples of open spots for each level below the current level (z)
            //let mut openSpots [ [(0, 0); 9]; z ];
            let mut belowSlice = [(0, 0); 9];
            let mut belowNumberOpen = 0;

            let minX = if x == 0 { 0 } else { x-1 };
            let maxX = if x+1 < 5 { x+1 } else { 5 };
            let minY = if y == 0 { 0 } else { y-1 };
            let maxY = if y+1 < 5 { y+1 } else { 5 };
            //writeln!( writer, "minX: {}, maxX: {}, minY: {}, maxY: {}", minX, maxX, minY, maxY);
            
            // iterate for each level below the current level
            for i in minX..maxX + 1 {
                for j in minY..maxY + 1 {
                    //writeln!( writer, "checkSlope: Checking for grain at i: {}, z: {}, j: {} which has value: {}", i, z-1, j, array[i][z-1][j]);
                    if array[i][z-1][j] == 0 {
                        belowSlice[belowNumberOpen] = (i, j);
                        belowNumberOpen += 1;
                    }
                    else {

                    }
                }
            }

            // writeln!( writer, "checkSlope: Below number open: {}", belowNumberOpen);
            // writeln!( writer, "checkSlope: Below slice: {:?}", belowSlice);

            if belowNumberOpen > 0 {
                // move the grain to the first open spot in the below level
                array[belowSlice[0].0][z-1][belowSlice[0].1] += 1;
                array[x][z][y] -= 1;
                writeln!( writer, "checkSlope: Grain moved to x: {}, z: {}, y: {}", belowSlice[0].0, z-1, belowSlice[0].1);
            
            }
        }

        return array;
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
        writeln!( writer, " ");
    }
    writeln!( writer, " ");


    // for dy in array {
    //     for dz in array {
    //         writeln!( writer, "{:?}", dz);
    //     }
    //     writeln!( writer, "");
    // }
}

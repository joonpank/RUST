use std::io;

fn main() {
    let mut buffer = String::new();
    
    // Read length of array
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    let mut array_size: i64 = buffer.trim().parse().expect("Input not an integer");

    buffer.clear();
    buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    // Read array in form "1 2 3 4 5 ... 9 10"
    let mut array = buffer.trim().split(" ");   
   
    // create vector and convert string values to integers.
    // create vector of integers.
    let mut vector = Vec::new();
    for item in array {
        vector.push(item.parse::<i64>().unwrap());
    }
    // For loop and calculation here
    let mut length = vector.iter().len();
    //println!("{}", length);
    //println!("{:?}", vector);

    let mut counter: i64 = 0;
    let mut diff: i64 = 0;

    for i in 0..(length - 1) {
        if diff != 0 {
            vector[i] += diff;
        }

        if vector[i] <= vector[i+1] {
            diff = 0;
            continue;

        } else if vector[i] > vector[i+1] {
            diff = vector[i] - vector[i+1];
            counter += diff;
        }
    }

    println!("{}", counter);
  
}
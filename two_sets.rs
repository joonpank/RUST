use std::io;

fn main() {
    let mut buffer = String::new();
    
    // Read length of array
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    let mut number: i64 = buffer.trim().parse().expect("Input not an integer");

    if number <= 2 {
        println!("NO");
        return;
    }

    let mut total_sum: i64 = (number * (number + 1)) / 2;

    if total_sum == 1 {
        println!("NO");
        return;
    
    } else if total_sum % 2 == 1 {
        println!("NO");
        return;
    } 

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
        
    let mut ans = total_sum / 2;

    for i in (1..(number+1)).rev() {
        if (i <= ans) {
            vec1.push(i);
            ans -= i;
        
        } else {
            vec2.push(i);
        }
    }

    

    println!("YES");
    println!("{:?}", vec1.iter().len());
    
    for v in &vec1{
        print!("{} ", v);
    }

    println!("\n{:?}", vec2.iter().len());

    for v in &vec2{
        print!("{} ", v);
    }

}    
use std::io;
// Consider an algorithm that takes as input a positive integer n. 
// If n is even, the algorithm divides it by two, and if n is odd, 
// the algorithm multiplies it by three and adds one. 
// The algorithm repeats this, until n is one. For example, the sequence for n=3 is as follows:
// 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 
fn main() {
 
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut n: i64 = input_text.trim().parse().expect("Input not an integer");

    let mut vec = Vec::new();

    let mut ind: i64 = 0;
    
    loop {
        vec.push(n);
        if n == 1 {
            
            break;
        }
        else if n % 2 == 1 {
            
            n = 3*n + 1;
        
        } else if n % 2 == 0{
            
            n = n / 2;
        }
        ind += 1;
    }
    for v in vec{
        print!("{} ", v)
    }
}

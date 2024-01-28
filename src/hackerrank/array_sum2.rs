use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("N can't be read.");

    let _arr_size: i32 = n.trim().parse().expect("a can't be converted into integer...");

    // let mut arr: [i32; 1000] = [0; 1000]; // note here we can't write arr_size in place of 1000
    // either you use vector or have it defined like above

    // vector implementation

    // array input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the line into individual numbers
    let v: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    let mut sum: i32 = 0;

    for element in &v {
        sum += element;
    }
    
    println!("{}", sum);
}
use std::io;
// how to import a module in a rust programming 
// this will help us when we want to read a user inpout from the terminal 


fn main() {
    // println!("Hello, world!");
    let mut name:String = String::new();
    let greeting="nice to meet you.";
    // how to take a input from the user 
    io::stdin().read_line(&mut name)
    // you need refrence to be working with when taking the input 
        .expect("didtn recieve a input");
            // println!("{} {}", name, greeting);
    println!("hello {}! {}",name.trim_end(), greeting);
    // the trim end will get rid of the extra line 
}

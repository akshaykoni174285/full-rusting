pub fn run(){
    // normal printing a statemtnt 
    println!("hello world from the hello.rs file");
    
    // formatting in rust 
    println!("{} is very {} {}","akshay","good","boy");

    // formatting in posistional parameters 
    println!("{0} is very {1} {2} and rahul is a bad {2}","akshay","good","boy");
    
    
    
    // formatting in named parameters
    
    println!("{name} is a good {activity}", name="akshay",activity="programmer");

    // formatting binary hex and octal values
    println!("binary:{:b}{:x}{:o}",10,10,10);

    //for debugging purpose 
    println!("{:?}",("hello",true,39.34));
    
}
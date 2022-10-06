pub fn run(){
    // strings 
    // * in rust there are two types of ways you can define strings 
    // * 1) str and another is 
    // * 2) String 
    /* now when using str it will expect a single character 
    and when you are using String you can use methods */

    let mut s = String::from("hello world ");
    println!("{}", s.len());

    s.push('T');
    println!("{}",s);
    s.push_str("his is awsome!");

    println!("{}",s);

    // adding a capacity
    let vars = String::with_capacity(10);
    assert!(vars.capacity()>=10);
    println!("is empty {}",vars.is_empty());


    // * checks if that string contains that substring or not 

    println!("is 'world' contains in the string {}",s.contains("hello"));

    println!("replce wod is {}",s.replace("world","there"));
    // println!("{}",s);

    // ! if you wanna loop thorugh the strings then you can do this 

    for character in s.split_whitespace() {
        println!("{}",character);
    }




}
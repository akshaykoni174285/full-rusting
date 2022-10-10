pub fn run(){

    greeting("hello","akshay");
    let mut result = return_addition(4,6);
    println!("{}",result);


    // closure 
    let mut result1 = |n1:i32,n2:i32| n1+n2;
    println!("{}",result1(4,5));

}

fn greeting(greeting: &str,name: &str){

    println!("{} {} nice to meet you",greeting,name);
}



fn return_addition(num1: i32,num2: i32)-> i32 {
    // return num1 + num2;
    num1+num2
    // this will directly return the value 
}
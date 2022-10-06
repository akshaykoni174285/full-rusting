pub fn run(){
    let mut numbers: [i32;5] = [1,2,3,4,5];
    numbers[0] = 20;
    // println!("{:?}", numbers);
    // * note you cant add another data type in this and also you can alter the 
    // * the size of the array 
    // * how to access a array in rust 
    print!("first value is :{}\n", numbers[0]);

    println!("the len is {}",numbers.len());

    // * to find the memory usage by the array 
    println!("this array occupies {} bytes",std::mem::size_of_val(&numbers));

    // let slice= &numbers;
    // let slice: &[i32] = &numbers;
    // println!("slice is {:?}",slice[0..2]);
    let slice: &[i32] = &numbers[3..5];

    println!("slice is {:?}",slice);



}
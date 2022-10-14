fn main() {
    // how do you define a number
    let number:u32 = 100001;
    const PI: f32 = 3.14159;
    const DIGIT: i32 = 174285;
    let age = "28";
    let mut age:u32 = age.trim().parse()
        .expect("the age variable doesnt have a valid value");
    age = age+10;
    println!("i am {} and i want this much money ${}", age, number);
    // now remember cahracter is single quotes and the str is in doube quotes
}

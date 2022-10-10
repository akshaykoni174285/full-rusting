pub fn run(){
    // for primitive 
    let array1 = [2,3,4,5];
    let array2=array1;

    // println!("values: {:?}",(array1,array2));
    
    // for non primitive 
    
    let vec1 = vec![0,1,2,3,4,5,6,7,8];
    let vec2 = &vec1;
    
    println!("values: {:?}",(&vec1,vec2));

}
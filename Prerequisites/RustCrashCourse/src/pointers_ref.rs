pub fn run(){
    //primitive array
    let array1 = [1,2,3];
    let array2 = array1;

    //non primitive values
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values {:?}", (&vec1, vec2));
}
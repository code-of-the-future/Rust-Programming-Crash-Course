// Pointers and References

fn main() {
    
    // Arrays - these are primitive!
    let array1 = [1, 2, 3, 4];
    let array2 = array1;
    let array3 = array2;
    println!("{:?}", (array1, array2, array3));

    // Vectors - primitive or non-primitive?
    // Vectors are NON-PRIMITIVE. We can't do this:
    // let vector1 = vec![1, 2, 3, 4];
    // let vector2 = vector1;
    // println!("{:?}", (vector1, vector2));

    // To make it work we make vector1 a reference
    // using the & to point to the resource 
    let vector1 = vec![1, 2, 3, 4];
    let vector2 = &vector1;
    let vector3 = &vector2;
    // we must pass a '&' when calling vector1 since it's borrowed
    println!("{:?}", (&vector1, vector2, vector3));



}

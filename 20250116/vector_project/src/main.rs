mod vector; // Declare the module `vector`

fn main() {
    // Create a new mutable instance of a `Vector` from the `vector` module, initialized with a capacity of 0
    let mut vec = vector::Vector::new(0);

    // Push a value (10) onto the vector
    vec.push(10);

    // Print the current size of the vector
    println!("Vector size: {}", vec.size());

    // Print the first element in the vector (at index 0)
    println!("First lement: {}", vec.at(0));
}

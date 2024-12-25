fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Safe way to access the second element using .get()
    if let Some(second_element) = vec.get(1) {
        println!("The second element is: {}", second_element);
    } else {
        println!("Vector doesn't have a second element.");
    }
} 
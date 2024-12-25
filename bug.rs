fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; // Potential panic if vec.len() < 2
    println!("The second element is: {}", second_element);
}
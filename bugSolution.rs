fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe and idiomatic way to modify the vector element
    println!("The first element is: {}", v[0]);
}

//Alternative:
fn main() {
    let mut v = vec![1, 2, 3];
    let first = v.get_mut(0);
    if let Some(first_element) = first {
        *first_element = 10;
    }
    println!("The first element is: {}", v[0]);
} 
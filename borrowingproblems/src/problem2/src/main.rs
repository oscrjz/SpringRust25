fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut new_string = s.clone();
    new_string.push_str("World!");
    new_string
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
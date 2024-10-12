fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from(" World!");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("Concatenated string is: {}", concatenated_string);

}

fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    result.push_str(a);
    result.push_str(b);
    result
}
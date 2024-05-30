// fn main() {
//     // Immutable variable
//     let name = "Alice";
//     println!("Name: {}", name);

//     // Mutable variable
//     let mut age = 25;
//     println!("Age: {}", age);
//     age = 26;
//     println!("New Age: {}", age);

//     // Shadowing
//     let age = "twenty-six";
//     println!("Shadowed Age: {}", age);

//     // Constant
//     const PI: f64 = 3.141592653589793;
//     println!("PI: {}", PI);

//     // Type inference
//     let is_active = true; // Type inferred as bool
//     println!("Is active: {}", is_active);

//     // Specifying the type explicitly
//     let score: u32 = 100;
//     println!("Score: {}", score);
// }

fn main() {
    let sentence = String::from("my name is Uzair Rehman");
    let first_word = get_first_word(sentence);
    println!("first word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

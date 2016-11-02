fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: &String) -> String {
    // Goal #1: What is needed here to make this compile?
    // Ans: You need to add `mut`
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}

fn print_out(name: String) {
    let devowelized_name = remove_vowels(&name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2: What happens when you uncomment the `println` below?
    // Can you change the code above so that the code below compiles
    // successfully?
    //
    // Ans: You can make `remove_vowels(name)` => `remove_vowels(name.clone())`

    println!("Removing vowels from {:?} yields {:?}",
             name, devowelized_name);

    // Extra credit: Can you do it without copying any data?
    // (Using only ownership transfer)
    // Ans: borrow `name` => remove_vowels(&name)
    //      You also need to change the function argument `name: String` to `name: &String`
}


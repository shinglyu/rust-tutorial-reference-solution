pub fn main() {
    let (mut str1, str2) = two_words();
    join_words(&mut str1, &str2);
    println!("concatenated string is {:?}", str1);

    let mut same = format!("Mozilla");
    join_words(&mut same, &same);
    println!("concatenated string is {:?}", str1);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

/// Concatenate `suffix` onto the end of `prefix`.
fn join_words(prefix: &mut String, suffix: &String) -> () {
    prefix.push(' '); // separate the words with a space
    for ch in suffix.chars() {
        prefix.push(ch);
    }
}

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`.
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://rust-tutorials.com/exercises/hint-mutable-borrow-1.html>.

// Question: Now that you've converted `strcat`, what happens if you
// call `strcat` using the same string for `prefix` and `suffix`?
// Why?

/*
Ans:
--------------------------
04-Mutable-borrows-answer.rs:7:28: 7:32 error: cannot borrow `same` as immutable because it is also borrowed as mutable [E0502]
04-Mutable-borrows-answer.rs:7     join_words(&mut same, &same);
                                                          ^~~~
04-Mutable-borrows-answer.rs:7:21: 7:25 note: mutable borrow occurs here
04-Mutable-borrows-answer.rs:7     join_words(&mut same, &same);
                                                   ^~~~
04-Mutable-borrows-answer.rs:7:32: 7:33 note: mutable borrow ends here
04-Mutable-borrows-answer.rs:7     join_words(&mut same, &same);
                                                              ^
04-Mutable-borrows-answer.rs:7:28: 7:32 help: run `rustc --explain E0502` to see a detailed explanation

*/


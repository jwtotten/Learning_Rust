fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("The first word is: {}", word);
    let word = first_word(&my_string[..]);
    println!("The first word is: {}", word);
    
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("The first word is: {}", word);
    
    let my_string_literal = "Hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    
    let word = first_word(&my_string_literal[0..6]);
    println!("The first word is: {}", word);
    
    let word = first_word(&my_string_literal[..]);
    println!("The first word is: {}", word);
    
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("The first word is: {}", word);
    
}

fn first_word(s: &str) -> &str // s is a reference to a string slice as an input as it can take both a string slice and a string literal
{
    let bytes = s.as_bytes(); // returning a reference to the bytes of a String

    for (i, &item) in bytes.iter().enumerate() 
    // iter() returns each element in a collection and 
    //enumerate() wraps the result of iter() and returns each element as part of a tuple instead.
    // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
    {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("The first word is: {}", word);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> &str {
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

fn main() {
    {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
    }

    {
        let s = String::from("hello");

        let slice = &s[0..2];
        let slice = &s[..2];
    }

    {
        let s = String::from("hello");

        let len = s.len();

        let slice = &s[3..len];
        let slice = &s[3..];
    }

    {
        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];
    }

    {
        let s = String::from("hello world");

        let word = first_word_slice(&s);

        // s.clear(); // error!

        println!("the first word is: {}", word);
    }

    {
        let my_string = String::from("hello world");

        // `first_word_slice` works on slices of `String`s, whether partial or whole
        let word = first_word_slice(&my_string[0..6]);
        let word = first_word_slice(&my_string[..]);
        // `first_word_slice` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word_slice(&my_string);

        let my_string_literal = "hello world";

        // `first_word_slice` works on slices of string literals, whether partial or whole
        let word = first_word_slice(&my_string_literal[0..6]);
        let word = first_word_slice(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word_slice(my_string_literal);
    }
    // Other Slices
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// A more experienced Rustacean would write the signature &str
// because it allows us to use the same function on both &String values and &str values.

// fn first_word(s: &String) -> &str {
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

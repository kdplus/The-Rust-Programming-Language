fn main() {
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let s = String::from("hello");

        change(&s);
    }

    // Mutable References
    {
        let mut s = String::from("hello");

        change_mut(&mut s);

        println!("{s}");
    }

    // Restriction of mutable references: if you have a mutable reference to a value, you can have no other references to that value
    // By doing this, Rust can prevent data races at compile time
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{}, {}", r1, r2);
    }

    // we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;
    }

    // We also cannot have a mutable reference while we have an immutable one to the same value.
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem, the reference to a mutable var is still immutable
        let r2 = &s; // no problem

        // let r3 = &mut s; // BIG PROBLEM

        println!("{}, {}, and ", r1, r2);
    }

    // A reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    // A mutable reference's scope can't overlap with any other refenrence's scope to the same value
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    // Dangling References
    {
        // let reference_to_nothing = dangle();

        let reference_to_nothing = no_dangle();
    }
}

// These ampersands represent references, and they allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, the value pointed to by the reference is not dropped when s stops being used

fn change(some_string: &String) {
    // some_string.push_str(", world");
    // Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello"); // s is a new String

    s
}

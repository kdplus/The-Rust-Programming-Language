fn main() {
    {
        let s = "hello";
        println!("{s}");
    }

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid
      // When a variable goes out of scope, Rust calls a special function for us. This function is called drop
      // Rust calls drop automatically at the closing curly bracket.

    {
        let x = 5;
        let y = x; // bind the value 5 to x; then make a copy of the value in x and bind it to y
                   // because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

        let s1 = String::from("hello");
        let s2 = s1;
        // A String is made up of three parts: a pointer to the memory that holds the contents of the string,
        // a length, and a capacity. This group of data is stored on the stack.
        // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
        // We do not copy the data on the heap that the pointer refers to.

        // println!("{}, world!", s1); error: value borrowed here after move
        // To avoid double free error, Rust considers s1 as no longer valid, Rust doesn’t need to free anything when s1 goes out of scope.
        // Looks like shallow copy, but because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
        // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
        // there’s no difference between deep and shallow copying here, and no reason we would want to prevent x from being valid after we create the variable y,
        // since types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make
        // so no need to use clone, and x is still valid
    }

    {
        // Ownership and Functions:  passing a variable to a function will move or copy, just as assignment does
        let s = String::from("Hello"); // s comes into scope

        takes_ownership(s);
        // s's value moves into the function...
        // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    {
        // Return Values and Scope
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    {
        // Rust does let us return multiple values using a tuple
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

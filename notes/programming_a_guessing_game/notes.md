# Code

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

# Notes

1. `use std::io;` allows us to take input from user via cmd line. It uses the "io" module, which comes from the "standard" library. 
2. Everything that you can use "out of the box" from the standard library is called the *prelude*.
3. Everything included in your rust program prior to importing it (using `use`) can be found [here](https://doc.rust-lang.org/std/prelude/index.html).

    ```rust
        std::marker::{Copy, Send, Sized, Sync, Unpin}, marker traits that indicate fundamental properties of types.
        std::ops::{Drop, Fn, FnMut, FnOnce}, various operations for both destructors and overloading ().
        std::mem::drop, a convenience function for explicitly dropping a value.
        std::boxed::Box, a way to allocate values on the heap.
        std::borrow::ToOwned, the conversion trait that defines to_owned, the generic method for creating an owned type from a borrowed type.
        std::clone::Clone, the ubiquitous trait that defines clone, the method for producing a copy of a value.
        std::cmp::{PartialEq, PartialOrd, Eq, Ord}, the comparison traits, which implement the comparison operators and are often seen in trait bounds.
        std::convert::{AsRef, AsMut, Into, From}, generic conversions, used by savvy API authors to create overloaded methods.
        std::default::Default, types that have default values.
        std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}, iterators of various kinds.
        std::option::Option::{self, Some, None}, a type which expresses the presence or absence of a value. This type is so commonly used, its variants are also exported.
        std::result::Result::{self, Ok, Err}, a type for functions that may succeed or fail. Like Option, its variants are exported as well.
        std::string::{String, ToString}, heap-allocated strings.
        std::vec::Vec, a growable, heap-allocated vector.
        std::convert::{TryFrom, TryInto},
        std::iter::FromIterator.
    ```
4. If what you want to use is not in the prelude (see the above list), then you need to bring it in with `use`, ie: `use std::io;`.
5. `println!()` is a *macro*. Macro's are denoted with the `!` in the name. 

```rust
let mut guess = String::new();
```
6. By default, variables are immutable. Meaning their values can't change once they are assigned to a variable. To make a variable *mutable*, use `mut` after the `let` keyword when creating a variable, as shown in the above example. 
7. `String::new()` is a function from the String type that returns a new instance of a string - which is a data type like char * in C.  
8. `::new()` is an associated function for the String type. In this case it *creates a new, empty string*.
9. `new()` functions exist for many types. 
10. `io::stdin()` allows us to handle user input. 
11. Instead of importing the library at the beggining of the program, we could've just used `std::io::stdin()`.
12. The `stdin()` function returns a instance of `std::io::Stdin`, which is a type that represents a handle to the standard input of the terminal. 
13. `&` indicates a *Reference*. In my previous experience with C, references are addresses in memory. ie: `&guess` means the address of the data stored in `guess`. That being said, I don't think that's the case here.
14. References are immutable by default. To make a reference mutable, you need to type it as such `&mut guess`. Notice the *&* before *mut* and not *guess*. 
15. `.read_line(&mut guess)` calls the read line method on the std input handle that we got from `io::stdin()`. The value is stored in the *guess* variable via *reference* (see the above bullets).
16. Best practices include splitting method calls into new lines as done for the *stdin()* function. See below:
    ```rust
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    ```
17. `.expect("Failed to read line");` handles potential errors in the *Result Type* of the stdin. 
18. `read_line()` inserts data into the passed in variable via *&*, but it also returns a *Result* value. *Result* is an *enumeration* or *enum*. *enum*'s is a type that can be in one of multiple states, or *variant*(s).
19. `Result` types has an `.expect()` method that you can call. If  `Result` is an `Err` that the program crashes and displays the passed in string, ie: `"Failed to read line"`. 
20. If the `Result` is an `Ok` value, `Result.expect()` returns the held data of the `Ok` value. In our code, the held value is the number of bytes in the user input. 
21. Printing formatted strings in Rust is basically the same as python f-strings, but you don't need the `f` in `f"blah {var1}"`, ie: `println!("blah blah blah {var1});`.
22. Not listed in the book yet, but there is a format macro used to create formatted strings outside of println!. The syntax is as follows: `format!("Hello, {name}!");` -> "Hello, Nick!".
23. To generate a random number, you have to import the rand crate. Add it to your dependencies in the `Cargo.toml` as such:
    ```toml
    [dependencies]
    rand = "0.8.4"
    ```
24. `Cargo.lock` handles versioning for Cargo dependencies. 
25. To use *rand* to generate a "secret number", you need to `use` it, by including the below line at the top of your code:
    ```rust 
    use rand::Rng;
    ```
26. In the below line of code, we create a random number generator with `rand::thread_rng()`, then we generate a number with it by calling the `.gen_range()` method with a range of `1..=100` (1-100, non-inclusive).
    ```rust
    let secret_number = rand::thread_rng().gen_range(1..=100);
    ```
27. `.cmp()` returns an enum of Ordering. When used with `match`, it is looking for a matching enum type in the following curly brackets. See below code:
    ```rust 
    match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    ```
28. Converting strings to numbers can be done with the following code:
    ```rust
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    ```
29. In the above code, the steps taken are as follows:
    ```
    guess (string) -> trim whitespace -> attempt to parse string to integer (unsigned 32 bit) -> if unable to parse, raise exception (panic!) -> guess is redefined as a 32 bit unsigned integer
    ```
30. The `.parse()` method infers the type to be parsed by the type of variable it is being saved to? Needs clarification.
31. Similar to a `while(true)` loop in C or Python, the syntax `loop {}` causes an infinite loop in Rust.
32. Loops can be broken with the `break` keyword just like in C and Python. 
33. To allow for errors in number input, and to prompt the user for another try, we can convert the `guess` variable into a match expression as follows:
    ```rust
    let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("This is not a number! Try again...");
                    continue;
                },
            };
    ```
34. In the above match expression, we are once again *trim*ming and *parse*ing the `guess` input. Parse returns either an Ok or Err enum, and we use `match` expression to tell the program which *arm* the code needs to flow through. 
35. If the returned enum from the *parse* method is `Ok`, we pass `num` (which is a tmp variable, think lambda in Python) to guess. 
36. If the returned enum from *parse* is Err, we capture `_` instead of `num`. `_` (underscore) is a catch-all value. 
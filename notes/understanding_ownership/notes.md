# Code

#### Ownership and Functions
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
#### Returning Values with Ownership
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

#### Returning Tuples
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

#### Using References
```rust


fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
#### Mutable References
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

#### Error With Two References
```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```
#### Implicit Borrowing
For example, mutable references can be moved by direct assignment:
```rust
[This code does not compile!] 
fn main() {
  let mut s = String::from("Hello world");
  let s2 = &mut s;
  let s3 = s2;
  println!("{}", s2); // Not valid because s2 is moved
}
```
But mutable references are not moved by function calls:
```rust
fn consume(_s: &mut String) {}
fn main() {
  let mut s = String::from("Hello world");
  let s2 = &mut s;
  consume(s2);
  println!("{}", s2); // Valid because s2 is NOT moved
}
```

#### Dangling References
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```


# Notes
 1. Ownership in Rust removes the need for garbage collection. It essentialy promises the compiler that the memory will be dealt with. 
 2. Other languages -> Garbage collection, which regularly looks for no longer used memory. 
 3. Rust commonly uses *stack* and *heap* methodology.
 4. *Stack* stores values in the order it gets them, ie: *last in, first out*. 
 5. Adding data to the stack is called *pushing into the stack*. Removing data is called *popping off the stack*. 
 6. All data stored on stack must have a known fixed size. 
 7. When adding data to the *heap*, it's less organized. You must request the space. 
 8. When adding to heap, memory alocater finds space and returns a pointer which is the address of the memory. 
 9. Pushing to stack is faster than the heap. 
 10. Ownership Rules:
     * Each value has an owner
     * There can only be one owner for a value at any given time.
     * When the owner leaves the scope, the value is dropped.
 11. Scope can be shown in the following scenario:
```rust
    {                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```
 12. String types are stored on the *heap* due to it's unknown size. 
 13. A *string* is considered a *complex data type*. 
 14. There are two *string* types:
     * String literals: Immutable, not suitable for every situation where text is needed. Not every string value can be known when code is written.
     * `String`: Manages data allocated on the heap. Can store an amount of text that is unknown at compile time. 
 15. You can create a `String` from a `string literal` using the `from` keyword, as shown in the below code:
```rust
let s = String::from("hello");
```
 16. You can mutate a `String` with the below example:
```rust
    let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```
 17. With the `String` type, we need to allocate memory to the heap at compile time, which means we:
     * Must request the memory from the memory allocator at runtime. 
     * Need a way of returning the memory to the allocator when we are done with it. 
 18. `String::from` requests the memory from the allocator at runtime. 
 19. Rust calls `drop` when memory leaves the scope. 
 20. A `String` type has three values:
     * ptr: The address in memory to the first index of a string
     * len: How much memory, in bytes, the contents of the string are using. 
     * capacity: How much memory, in bytes, the the string has recieved from the allocator. 
 21. Assigning another variable to the value of a `String` type essentially references the same place in memory that the first variable was, ie:
```rust
let s1 = String::from("hello");
let s2 = s1; // s2 is the same thing as s1. Manipulating s2 would be doing the same to s1
```
![pointer example](images/trpl04-02.svg)

 22. Once `s2` is declared and set to the value of `s1`, `s1` is no longer valid and has been removed from the scope. This is because `s2` is now the owner of that data and data can only have one owner. 
 23. Rust's automatic implementation for copying complex data types like `String` is to use *shallow copy* better known as *move*. This means that only ptr, len, and capacity are copied and not the data itself. 
 24. If you wanted to copy *ALL* the data, you would perform a *deep copy*. In rust, a common method for deep copying is *clone*, ie:
```rust


let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```
 25. In the above example, the heap data *DOES* get copied. This can be expensive and should not be used if it's not needed. 
 26. Any scalar-type, a simple type stored on the stack can be copied by assigning a variable to another variable, or using the `Copy` trait. Here are types that implement `Copy` trait:
    
    * All the integer types, such as u32.
    * The Boolean type, bool, with values true and false.
    * All the floating point types, such as f64.
    * The character type, char.
    * Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
 27. When a non-scalar variable is passed to a functin, it's ownership changes to the function. If no value is returned from the function, the variable is essentialy *dropped* when the function ends. See [this code](#ownership-and-functions).
 28. When scalar variables, like an int, are passed to a function, it passes a copy. The original variable is still in scope when the function closes. 
 29. If you return a value from a function, you are passing ownership through the returned value. This can be seen [here](#returning-values-with-ownership).
 30. You can return *tuples* from functions as a way of returning the passed in value (ownership purposes) as well as a returned value. This can be seen [here](#returning-tuples).
 31. To use a value without transferring ownership, we use *references*. In C this is the *address of* operator, '&'. 
 32. *References* are great! We don't change ownership when we use a reference of a value. In [this](#using-references) code, we are creating a *String* (s1), passing a *reference* of it (`&s1`) to a function which paramenters are defined as taking a *reference* (`s: &String`), and returning a length of the String value at s1. 

 ![reference-image](images/trpl04-05.svg)

 33. *Reference*s are just pointers to pointers basically. Used for passing values without changing ownership. 
 34. The opposite of *Referencing* with `&` is *Dereferencing* with `*`. This is discussed later, but I assume it's the same as C. 
 35. Creating a reference is more commonly known as *Borrowing*. 
 36. We can't modify something we are referencing, we can only reference it's value, unless we change it to a *mutable reference*. 
 37. Mutable references can be declared with the *mut* keyword following the '&', as shown [here](#mutable-references).
 38. When a reference is made mutable, the value being referenced can be changed without changing ownership. 
 39. A mutable reference can only be borrowed once in scope. You will get an exception if you attempt to do it more than once. See [this code](#error-with-two-references).
 40. Rust prevents multiple mutable references to prevent *race cases* or *data race*. This occurs when two or more pointers are trying to access data at once, one pointer is being used to write, and no mechanism being used to sync access to the data, ie: a queue. 
 41. Multiple mutable references can be made, as long as they are not in the same scope, ie:
 ```rust
     let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
 ```
 42. Can't have both an immutable and mutable reference, referencing the same data in the same scope. 
 43. Multiple immuatable values are allowed because this won't affect race cases since the data can't be mutated anyways. 
 44. Technically a mutable and immutable ref can be in the same function if the immutable values are not used after the mutable reference is introduced, ie: 
```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```
 45. The compiler's ability to tell that a reference is no longer being used at a point before the end of the scope (like the above code) is called a *Non-Lexical Lifetimes* or *NLL* for short. 
 46. Mutable references can be moved from variable to variable by direct assignment, but not by function calls. Check [this code](#implicit-borrowing) out to see it in action. 
 47. Mutable references are not moved when passed to a function call because Rust automatically *reborrows* the reference when passed. 
 48. Dangling references are automatically not allowed at compile time. This can be seen [here](#dangling-references). <- This code would fail because the String s is dropped, but the reference to it is not, but instead passed to `reference_to_nothing`. 
 49. Slices let you reference a part of a sequence of elements, better known as a collection.
 50. Slices don't have ownership.
 51. 
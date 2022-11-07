# Code

#### Ownership & Functions
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




# Notes
 1. Ownership in Rust removes the need for garbage collection. It essentialyl promises the compiler that the memory will be dealt with. 
 2. Other languages -> Garbage collection, which regularly looks for no longer used memory. 
 3. Rust commonly uses *stack* and *heap* in it's intended implementation.
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

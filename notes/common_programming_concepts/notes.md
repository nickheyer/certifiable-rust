# Code

#### Variables and Mutability

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

#### Shadowing

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```


#### Numeric Operators
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```
#### Compound Types - Tuples
```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

#### Compound Types - Arrays
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```


# Notes

1. `mut` makes variables mutable, but also shows intent to change value for future code readers.
2. `const` are immuatable inherently and can not be changed to mutate.
3. `const` variable are created using the `const` keyword instead of `let`. They _data type_ must always be annotated when creating the variable, ie: _u32, i32, etc._
4. Proper naming convention for `const` variables is all capital letters, ie:
   `const THIS_IS_A_CONST_INT: u32 = 69`.
5. `const` values are valid the entire time a program is run, while in the scope they are declared. Useful for hard-coded values.
6. `const` can be used in the _Global_ scope, while `let` can only be used inside function scope.
7. _Shadowing_ can be used to replace the value of a variable with a new value that references the previous one. This new value can be a different data type. ie:
   ```rust
   let shadow_val = 1;
   let shadow_val = shadow_val + 1;
   //shadow_val == 2
   ```
8. _Shadowing_ is not the same as making a variable `mut`, because the compiler is only seeing the latest declaration of the variable.
9. Shadowing can change the type of a variable by assigning it a new value, but it cannot change the type of a runtime value itself.
10. In cases where many types are possible, like when parsing a string to int, float, etc. - a type must be declared after the variable name, ie:
    ```rust
    let guess: u32 = "42".parse().expect("Not a number!");
    ```
11. Common Types are as follows:
    <table>
    <thead><tr><th>Length</th><th>Signed</th><th>Unsigned</th></tr></thead><tbody>
    <tr><td>8-bit</td><td><code class="hljs">i8</code></td><td><code class="hljs">u8</code></td></tr>
    <tr><td>16-bit</td><td><code class="hljs">i16</code></td><td><code class="hljs">u16</code></td></tr>
    <tr><td>32-bit</td><td><code class="hljs">i32</code></td><td><code class="hljs">u32</code></td></tr>
    <tr><td>64-bit</td><td><code class="hljs">i64</code></td><td><code class="hljs">u64</code></td></tr>
    <tr><td>128-bit</td><td><code class="hljs">i128</code></td><td><code class="hljs">u128</code></td></tr>
    <tr><td>arch</td><td><code class="hljs">isize</code></td><td><code class="hljs">usize</code></td></tr>
    
    <tr><td>32-bit float (single)</td><td><code class="hljs">f32</code></td></tr>
    <tr><td>64-bit float (double)</td><td><code class="hljs">f64</code></td></tr>
    <tr><td>Boolean Type</td><td><code class="hljs">bool</code></td></tr>
    <tr><td>Char Type</td><td><code class="hljs">char</code></td></tr>
    </tbody>
    </table>
12. Using `i32` is generally the best place to start when deciding on which type to use. When indexing, it's best to use `isize`/`usize`.
13. You can use an `_` (underscore) as a visual seperator in a number, like a `,`. ie: `1_000_000_000` is much easier to read than `1000000000`.
14. When releasing with `--release`, Rust doesn't check for integer overflow. ie: if a `u8` goes 1 over 255, it restarts at 0.
15. For *floating point* values, the default type is `f64` because it has a higher level of precision and roughly the same speed. 
16. Like in C, `char` types are expressed with single quotes like `'A'`, not double quotes. Double quotes denote a string literal, single quotes are for char literals. 
17. *Compound Types* means multiple values can be grouped into one type. 
18. *Tuples* are great for grouping different types together into one compound type, ie:
    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1)
    ```
19. You can destructure a tuple with the below syntax:
    ```rust
    let (x, y, z) = tup;
    ```
20. Tuple elements can also be accessed by usign a period as such:
    ```rust
    let (x, y, z) = tup;
    let first_elem = tup.0;
    ```
21. An empty tuple with no values is called a *unit*. 
22. Arrays are also fixed size, but can only contain one type. They are mutable. Useful when you want data on stack and not heap. They can be initialized in either of the two ways below:
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //[1, 2, 3, 4, 5]
    let b = [3; 5]; //[3, 3, 3, 3, 3]
    ```
23. If you don't know the size of the array, use a vector instead (I wish C had this!)
24. 
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

#### Functions
```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

#### Parameters
```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```
#### Multiple Parameters
```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

#### Expressions to Variables
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); //prints 4
}
```
#### Returning values from Functions
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}"); //The value of x is 5
}
```

#### Returning from Function w/ Params
```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
#### Control Flow w/ if Expressions 
```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

#### Multiple Conditions w/ if else
```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
#### Infinite loop
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

#### Returning Values from Loops
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
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
24. Function and variable naming convention uses *snake case*, ie: `some_variable = "test`.
25. In rust, you can define functions anywhere within your code, as long as it's in scope. Whereas in C, a function must be defined before main, or declared, or within a header file. 
26. In function *signatures*, you must declare the type of paramenter. ie: `fn another_function(x: i32){}`.
27. Multiple parameters are seperated by a comma, ie: `fn print_labeled_measurement(value: i32, unit_label: char)`.
28. Functions that return a value must declare the return type with `->` operator, ie: `fn five() -> i32`.
29. Comments should be placed above code that is being annotated. 
    ```rust
    fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
    }
    ```
30. Unlike other languages, Rust will not auto convert a non-zero integer to a boolean value. 
31. You do not need to put expressions in `()` when evaluating, ie:
    ```rust
    fn main() {
    let number = 3;

        if number != 0 {
            println!("number was something other than zero");
        }
    }
    ```
32. Using too many `if else` statements in your code can clutter it, use `match` instead for these.
33. You can use `if` in a let statement when defining a variable, ie: 
    ```rust
    fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    }
    ```
34. The potentially returned values from an `if` expression must the same type since the `let` type evaluates to the type of the last statement when compiled. 
35. An if-expression without an else-branch always returns the unit type.
36. Rust has three kinds of loops: loop, while, and for.
37. `break` stops a loop and moves on to the next code.
38. `continue` skips the remaining code in this loop iteration and starts the next iteration. 
39. Just like expressions, you can return values from loops by adding the returned value after the `break` keyword followed by a `;`. 
40. To break a specific loop, you can label a loop as such and break it as such:
    ```rust
    fn main() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }
    ```
    The label must be prepended by a `'` (single-quote). 
41. `loop` loops generally are intended to run for an unknown period of time, but `while` loops run until a condition is satisfied (which also may be an unknown period of time). ie: 
    ```rust
    fn main() {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }
    ```
42. When iterating over a collection, a `while` loop is possible, but the most efficient method is a `for` loop. ie: 
    ```rust
    fn main() {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }
    }
    ```
43. The syntax for a `for` loop is similar to python. `for *element* in *collection*`...
44. When running a loop a specific number of times, it's still best to use a `for` loop. Using a `Range` value as it's itterable/collection is ideal, as such:
    ```rust
    fn main() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    ```
45. `Range` values can be generated using the following syntax:
    ```rust
    let r = (1..16) //[1, 2, 3, 4, 5 ... 16]
    ```
46. `.rev()` will reverse a range. 
47. 
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

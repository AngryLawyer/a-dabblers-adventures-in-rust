# A Dabbler's Adventures in Rust
## Tony Aldridge
## Functional Brighton
## 21st January 2014

---

# What is Rust?
## (Baby don't hurt me, don't hurt me, no more)
PROVIDE PICTURE
* New programming language developed by Mozilla
* Multi-paradigm. Functional, imperative, object-oriented, whenever it makes sense.
* Low-level. Targets the same problem-space as C and C++
* Safe. Lovely, lovely types and pointer lifetimes guard against a lot of errors.

---

# Where can I get it?
* Prebuilt binaries are available at http://www.rust-lang.org/
* Source code is available from GitHub https://github.com/mozilla/rust
* You can also use Aptitude to install nightly builds https://launchpad.net/~hansjorg/+archive/rust

---

# The traditional Hello World 

!!!intro.rs

* It's very C-like. Curly braced. Imported packages at the top of the page.
* Semicolons aren't needed in a lot of cases. They imply 'new statement' rather than end of line, so if you want your function to return a value, you leave it off (or use the return keyword).

---

# The basics

Rust is staticly typed, but uses type inference to make sure we don't end up with Java-style mouthfuls like 'Integer myInteger = new Integer(5)'

!!!basics.rs:1

You can specify the type if you want, which comes in handy sometimes.

---

# No automatic type casts

!!!basics.rs:2

As you can see, if we want to multiply an int by a float, we have to make sure that it typechecks.

---

# Immutability

As Rust is designed with concurrency as a goal, everything's immutable by default.

!!!basics.rs:3

---

# Expressions
Everything's an expression in Rust. If you put a semicolon after it, it'll return void instead.

!!!expressions.rs

---

# Functions

Functions are one of the places you have to be explicit with types

!!!functions.rs:1

---

# Generics

We get around the inflexibility of typed functions using generics.

!!!functions.rs:2

---

# Basic data types

!!!datatypes.rs

---

# Pointers

Pointers are a key part of understanding Rust

!!!pointers.rs:1

---

# Pointers

!!!pointers.rs:2

---

# Algebraic data types

!!!functional.rs:1

---

# No Nulls! 

!!!functional.rs:2

---

# Functional fun 

So, let's do the classic of Functional Programming.
Define a list type.

!!!functional.rs:tut1

It's recursively defined as data plus an owned pointer to itself, or to a terminal

---

# Functional fun 

So, now we can define a list!

!!!functional.rs:tut2

This function borrows the original list, so the original remains useful. In the pattern match, we have to specify that we're borrowing rather than moving once again, otherwise no compile.

We then have to dereference the borrowed pointer with a * otherwise it's a double borrow

---

# Functional fun 

But why would we hardcode what this can do? We can take a first class function, and also make it generic across all types!

!!!functional.rs:tut3

Of course, Rust's standard library has already defined all of these useful things.

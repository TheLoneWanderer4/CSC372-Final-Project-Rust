# Language Study : Rust

**(A) History and Current Status :**

*Rust* was started as a personal project in 2006 by Graydon Hoare, then an employee at the Mozilla corporation. Mozilla began to sponsor the project in 2009 and accounted it in 2010. The first compiler was written before 2010 in OCaml. After the Mozilla announcement work shifted to a self-hosting compiler `rustc`, which successfully compiled itself in 2011.  

The language was developed as a replacement for the older systems languages `c` and `c++`, with the express intent of preventing the problems that arise in those languages from users having to specify their own memory. 

The first stable version *1.0.0* was released in 2015. The added support for *cargo* rust's built in dependency management system. Since then there have ben *47* releases. 

The current stable version is *1.47.0* released on the 8th of October 2020, and a compiler can be found [here](https://doc.rust-lang.org/book/ch01-01-installation.html). The compiler itself is installed using the `rustup` program. 

Major changes throughout the languages history include...

- Major changes to the *cargo* package manager in *1.5.0*
- The stabilization of the `libcore` package in *1.6.0*
- The addition of a `try` operator in *1.13.0*
- The stabilization of asynchronous operations in *1.39.0* 

Among many many other smaller changes! 

One of the most interesting parts about rust is it's entirely open source nature, with the entire source code of the language and all it's systems available for browsing [here](https://github.com/rust-lang). 



**(B) Paradigm :**

Rust is an extremely *multi-paradigm* language, with it's two main designs being 

- **Functional :** Rust isn't a "purely functional" language like Haskell, but it does adhere to many of the tenets of functional programming. Namely ... 
  - Rust is Immutable by default. In fact, all variables that one wants to be mutable must be specified as such
  - Rust supports lambda functions, and allows them to be treated as *first-class* objects with Higher-Order functions like `map` and `fold`. 
  - However, rust does still allow for side effects, iteration, and more, leading many to call it an imperative language with functional elements. 
- **Imperative / OO :** Rust's object oriented nature comes from it's `trait` system. Traits are *basically* interfaces, which contain method headers specifying input and return types, without implementation. These traits are then *applied* to structs (exactly like the c `struct`) in a similar manner to how a Java class would implement and interface.  While a little convoluted at first, this does provide for OO programming using the two systems `structs` and `traits` to create classes.

Rust is also a *concurrent*, *generic* and more language, though I will leave out descriptions of those Paradigms as they were not covered and I only vaguely understand what they mean. 



**(C) Typing System :**  

Rust is *strongly* typed, however the programmer isn't expected to declare the types of variables. Instead, rust gives the variable the type of the first value it is assigned. While possibly confusing, this makes a lot of sense in the greater context of rust, namely, in rust are immutable by default. Mutable variables still exist, specified with the `mut` keyword, but are discouraged. In addition, 

Rust is also *strongly* strongly typed. Once the compiler figures out the type of a variable you are only allowed to do operations on that variable that are available to it's traits, similar to Haskell's type classes. 

In addition, the programmer can specify new types through the use of `structs` and `traits`, the details of which are discussed in part B. 

In addition, rust supports generic function parameters, where a parameter will be required to support a set of traits, rather than be a specific type. This allows for generics to still be type checked by the compiler. 

Finally, functions ARE first class types in Rust. This means that they can be passed and stored / used like any other value.



**(D) Control Structures :** 

Rust is mostly quite standard in it's control structures. Things like `while loops` and `if` statements are the exact same as any other language. However, there are some interesting differences. 

- `loop` is a specific keyword denoting something similar to a `while(true)` loop, that runs until you `break` it. However, these structures explicitly support the ability for a value to be defined and returned. 

  ```rust
  let result = loop {
      counter += 1;
  
      if counter == 10 {
          break counter * 2;
      }
  };
  ```

- `for` loops exist in rust as well, they are similar to those in python, specifically being `for-each` loops that allow for the iteration over a collection of elements. 

In addition to these rust also supports recursion, which combined with it's immutable by default and generic parameters seems to be the preferred manner of doing things for true *Rustacean*. 

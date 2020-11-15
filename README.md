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



**(E) Semantics :** 
Scope in rust is weird, It’s static but  scope doesn’t just affect if you can access the data in a variable but the underlying memory as well. When a variable goes out of scope, the memory is automatically freed, you can transfer the ownership of that memory (in addition to the value). This is called a move, for this to work a variable must be saved on the heap (using the Box type) instead of on the stack by default. Since variables are immutable by default, the difference between them and a constant is that constants are declared with the const keyword, and its type must be declared, unlike variables which can infer them, constants must be set to a constant static expression, they cannot be set to the result of a function call, constants cannot be shadowed by a new value, unlike variables. Garbage collection in rust is also unique, while it does not have a garbage collector in the traditional sense, as stated before memory is automatically freed once it goes out of scope. Rust memory rules are enforced at compile time so everything is guaranteed to be leak safe.So you don’t have to worry about doing it yourself unless you use the unsafe keyword 

**(F) Desirable Language Characteristics :** 
Efficiency: Rust is similar to C and C++ in terms of efficiency because it compiles directly to machine code for a specific platform. Like C and C++, this also comes at the cost of code being platform specific, meaning code must be recompiled for different platforms, unlike a language like Java or Python

Regularity: 
*Finish this after writing part2* 
Seems like it mostly adheres to this, although that seems to be more out of making sure the compiler is happy with your code. 

Security/Reliability: Major focus of Rust, and a major upgrade over other system programming languages. Memory management is handled by rust unless the user uses the unsafe keyword for manual control, and the rules of memory management are enforced during compile time. All of this means that any compiled rust program is memory safe.
https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html

Extensibility: 
Rust comes with the cargo package manager, something I haven’t seen for a system programming language and is usually a feature reserved for high level languages like ruby, python, or javascript. Cargo can manage dependencies, download, install and update modules, and build and test projects. It also has a cool fancy website where you can brows different libraries submitted by the community at https://crates.io/

**(G) Support for Data Abstractions :** 
Data types in the standard library are fairly standard for any programming language, scalar types represent types with a single value: integers can be signed or unsigned and rust lets you pick the size from 8 all the way to 128 bits, 32 is the default. Floats can be 32 or 64 bits as well, Strings are a higher abstraction of the character type, which abstracts the unicode values, (you can even put emojis in your code if you wanted to???). Rust has tuples for pattern matching and arrays, which are not resizable, and unlike C actually enforces its size to keep memory safe. Classes aren’t really a thing. Like C there is a struct type that allows you to hold many different types of data under one label, and enums allow you to set custom types with defined states. However in rust we can create an impl block, to add implementation to a strut or enums, where we can add methods.  We also have a trait block, which behaves similar to an interface in java for implementation of struct methods. Again, memory management is a big abstraction handled by rust, and custom data types can have their own deallocation block for code to run before it gets deallocated by implementing the deref trait. 

**(H) Syntax :** 
Sorta a mix of C, C++. Java and Haskell. It’s confusing when types don’t have to be explicitly stated and when they do, creating new data types can be a bit convoluted compared to something like python, where the format for creating classes is very clear. You don’t include a “;” for expressions, which can be confusing as well. However the documentation for the syntax is the best I’ve ever seen for a language, making it easy to get over these hurdles. Code is fairly easy to read once you understand Rust rules on mutability and memory access. 









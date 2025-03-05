use crate::List::{Cons, Nil};

/*
A pointer is a general concept for a variable that contains an address in memory. 
This address refers to, or “points at,” some other data. 
The most common kind of pointer in Rust is a reference.
References are indicated by the & symbol and borrow the value they point to. 
They don’t have any special capabilities other than referring to data, and have no overhead.

Smart pointers, on the other hand, are data structures that act like a pointer but also 
have additional metadata and capabilities. 
The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ 
and exist in other languages as well. Rust has a variety of smart pointers defined in 
the standard library that provide functionality beyond that provided by references. 
To explore the general concept, we’ll look at a couple of different examples of smart pointers, 
including a reference counting smart pointer type. 
This pointer enables you to allow data to have multiple owners by keeping track 
of the number of owners and, when no owners remain, cleaning up the data.

Rust, with its concept of ownership and borrowing, has an additional difference between 
references and smart pointers: while references only borrow data, 
in many cases, smart pointers own the data they point to. (String and Vec<T> are smart pointers)

Smart pointers are usually implemented using structs. Unlike an ordinary struct, 
smart pointers implement the Deref and Drop traits. The Deref trait allows an instance 
of the smart pointer struct to behave like a reference so you can write your code to work
with either references or smart pointers. 
The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

-> most common smart pointers in the standard library:

    Box<T> for allocating values on the heap
    Rc<T>, a reference counting type that enables multiple ownership
    Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces 
    the borrowing rules at runtime instead of compile time
 */

//--------------BOX---------------
/*
The most straightforward smart pointer is a box, whose type is written Box<T>. 
Boxes allow you to store data on the heap rather than the stack. 
What remains on the stack is the pointer to the heap data.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
But they don’t have many extra capabilities either. You’ll use them most often in these situations:

-> When you have a type whose size can’t be known at compile time and you want to use a value of that type 
in a context that requires an exact size

-> When you have a large amount of data and you want to transfer ownership but ensure the data won’t be
copied when you do so

-> When you want to own a value and you care only that it’s a type that implements a particular trait
rather than being of a specific type
 */
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
/*
We define the variable b to have the value of a Box that points to the value 5, which is 
allocated on the heap. This program will print b = 5; in this case, we can access the data
in the box similar to how we would if this data were on the stack. Just like any owned value,
when a box goes out of scope, as b does at the end of main, it will be deallocated.
The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).
*/

//------ENABLING RECURSIVE TYPES -----
/*
A value of recursive type can have another value of the same type as part of itself. 
Recursive types pose an issue because at compile time Rust needs to know how much space 
a type takes up. However, the nesting of values of recursive types could theoretically 
continue infinitely, so Rust can’t know how much space the value needs. Because boxes 
have a known size, we can enable recursive types by inserting a box in the recursive type definition.

As an example of a recursive type, let’s explore the cons list. 
This is a data type commonly found in functional programming language
 */

 enum List {
    Cons(i32, List),
    Nil,
}
let list = Cons(1, Cons(2, Cons(3, Nil)));
//it won't compile because rust doesnt really know how much bigger teh list variable can get 
//this is why we use box:
/*
“indirection” means that instead of storing a value directly, we should change the data 
structure to store the value indirectly by storing a pointer to the value instead.

Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: 
a pointer’s size doesn’t change based on the amount of data it’s pointing to. 

This means we can put a Box<T> inside the Cons variant instead of another List value directly. 
The Box<T> will point to the next List value that will be on the heap rather than 
inside the Cons variant. Conceptually, we still have a list, created with lists holding other lists, 
but this implementation is now more like placing the items next to one another rather than inside one another.
 */
enum List {
    Cons(i32, Box<List>),
    Nil,
}
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
/*
The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> 
values to be treated like references. When a Box<T> value goes out of scope, 
the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation. 
 */
}

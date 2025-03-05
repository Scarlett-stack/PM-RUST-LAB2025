/*
    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    let s = "hello";
    //A scope is the range within a program for which an item is valid
    // ex daca pun { aici e s cvea} -> dupa asta nu mai exista
    //Rust calls drop automatically at the closing curly bracket.
    /*
    In C++, this pattern of deallocating resources at the end of an item’s lifetime
     is sometimes called Resource Acquisition Is Initialization (RAII).
     The drop function in Rust will be familiar to you if you’ve used RAII patterns.
     */
    let s1 = String::from("hello");
    let s2 = s1;
    // aici s1 si s2 pointeaza la aceasi zona de memorie
    //daca ambele fac drop in acelasi timp?  double free error 
    //To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope
    /*
    If you’ve heard the terms shallow copy and deep copy while working with other languages,
    the concept of copying the pointer, length, and capacity without copying the data 
    probably sounds like making a shallow copy. But because Rust also invalidates the 
    first variable, instead of being called a shallow copy, it’s known as a move. 
    In this example, we would say that s1 was moved into s2
     */

    //If we do want to deeply copy the heap data of the String, 
    //not just the stack data, we can use a common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone(); //deep copy

    println!("s1 = {s1}, s2 = {s2}");

    /*
    When you see a call to clone, you know that some arbitrary code is 
    being executed and that code may be expensive. 
    It’s a visual indicator that something different is going on.
     */

     let x = 5;
     let y = x;
 
     println!("x = {x}, y = {y}");
 /*But this code seems to contradict what we just learned: we don’t have a call to clone, 
 but x is still valid and wasn’t moved into y.
 The reason is that types such as integers that have a known size at compile time 
 are stored entirely on the stack, so copies of the actual values are quick to make.
  That means there’s no reason we would want to prevent x from being valid after 
  we create the variable y. In other words, there’s no difference 
  between deep and shallow copying here, so calling clone wouldn’t do anything different 
  from the usual shallow copying, and we can leave it out.
*/

//----------COPY TRAIT--------------
/*
    Rust has a special annotation called the Copy trait that we can place on types that 
    are stored on the stack, as integers are.
    If a type implements the Copy trait, variables that use it do not move, 
    but rather are trivially copied, making them still valid after assignment to another variable.
    Rust won’t let us annotate a type with Copy if the type, or any of its parts, 
    has implemented the Drop trait. If the type needs something special to happen 
    when the value goes out of scope and we add the Copy annotation to that type, 
    we’ll get a compile-time error.

    any group of simple scalar values can implement Copy, 
    and nothing that requires allocation or is some form of resource can implement Copy
    (u32, f64, char, bool, i32 ...)
 */

 //--------OWNERSHIP AND FUNCTIONS--------
//If we tried to use s after the call to takes_ownership, 
//Rust would throw a compile-time error. These static checks protect us from mistakes.

let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

    //--------RETURN VALUES AND SCOPE-------
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    /*
    The ownership of a variable follows the same pattern every time:
    assigning a value to another variable moves it. 
    When a variable that includes data on the heap goes out of scope, 
    the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

    While this works, taking ownership and then returning ownership with every 
    function is a bit tedious. What if we want to let a 
    function use a value but not take ownership?

    Rust has a feature for using a value without transferring ownership, called references.
     */

    //-----REFERENCE & BORROWING----
    /*
    A reference is like a pointer in that it’s an address 
    we can follow to access the data stored at that address; 
    that data is owned by some other variable. Unlike a pointer, 
    a reference is guaranteed to point to a valid value of a 
    particular type for the life of that reference.

    Note: The opposite of referencing by using & is dereferencing,
    which is accomplished with the dereference operator, *

    Just as variables are immutable by default, 
    so are references. We’re not allowed to modify something we have a reference to.
     */
    //---------MUTABLE REFERENCES--------
    //i can change them here!
    let mut s = String::from("hello");

    change(&mut s);
    //Mutable references have one big restriction: 
    //if you have a mutable reference to a value, you can have no other references to that value
    /*
The benefit of having this restriction is that Rust can prevent data races at compile time. 
A data race is similar to a race condition and happens when these three behaviors occur:

    -Two or more pointers access the same data at the same time.
    -At least one of the pointers is being used to write to the data.
    -There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when 
you’re trying to track them down at runtime; 
Rust prevents this problem by refusing to compile code with data races!

We also cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don’t expect the value to suddenly change out from under them! 
However, multiple immutable references are allowed because
no one who is just reading the data has the ability to affect anyone else’s reading of the data.
     */

//----------DANGLING REFERENCES------
//da aparent exista asa ceva si aici...
/*
dangling pointer—a pointer that references a location in memory that may have been given 
to someone else—by freeing some memory while preserving a pointer to that memory. 
In Rust, by contrast, the compiler guarantees that references will never be dangling references: 
if you have a reference to some data, 
the compiler will ensure that the data will not go out of scope before the reference to the data does.
 */
    let reference_to_nothing = dangle();
 }

 fn dangle() -> &String { //returns a ref to string
    let s = String::from("hello"); // i create string here
    
    &s //return a reference to s
}
//but s it goes out of scope here!! -> s is droped 
//DANGER! SOLUTION? RETURN S DIRECTLY 

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

// ALSO Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

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
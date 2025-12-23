// consider a trait as a collection of methods. If you are from an object-oriented background, traits can be thought of as abstract classes or inter- faces. If your programming experience is in functional languages, you can think of traits as type classes.


use std::convert::TryInto; // bring TryInto trait into local scope, which unlocks try_into method of b variable.

fn main(){
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into() // try_into returns an i32 value wrapped in a result, it can contain either success value or an error value, the unwrap method can handle the success value and returns the value of b as an i32 here.
                .unwrap();
    
    if a < b_ {
        println!("Ten is less than one hundred");
    }
}

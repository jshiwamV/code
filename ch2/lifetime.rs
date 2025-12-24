fn id<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let r;
    { // remove this inner braces and error is gone. scope of y is limited within these, hence we get lifetime error. Better than CPP because in CPP this code will compile and it will give Undefined behaviour during runtime.
        let y = 10;
        r = id(&y);
    }
    println!("{}", r); // ❌ Rust error, UB in C++
}

// Stack state BEFORE calling id

// STACK (top grows downward)

// ┌─────────────────────────┐
// │ id() frame              │  ← not created yet
// └─────────────────────────┘

// ┌─────────────────────────┐
// │ main() frame            │
// │                         │
// │ r : (uninitialized)     │
// │                         │
// │ ┌───────────────┐       │
// │ │ y = 10        │◄──────┐
// │ └───────────────┘       │
// └─────────────────────────┘

// Stack DURING function call id(&y)

// STACK

// ┌─────────────────────────┐
// │ id() frame              │
// │                         │
// │ x : &i32 ───────────────┼─────► y
// │                         │
// └─────────────────────────┘

// ┌─────────────────────────┐
// │ main() frame            │
// │                         │
// │ r : (uninitialized)     │
// │                         │
// │ ┌───────────────┐       │
// │ │ y = 10        │◄──────┘
// │ └───────────────┘       │
// └─────────────────────────┘

// Stack AFTER id() returns

// STACK

// ┌─────────────────────────┐
// │ id() frame              │  ← popped
// └─────────────────────────┘

// ┌─────────────────────────┐
// │ main() frame            │
// │                         │
// │ r : &i32 ───────────────┼─────► y
// │                         │
// │ ┌───────────────┐       │
// │ │ y = 10        │◄──────┘
// │ └───────────────┘       │
// └─────────────────────────┘

// Stack AFTER inner scope ends (🔥 bug point)

// STACK

// ┌─────────────────────────┐
// │ main() frame            │
// │                         │
// │ r : &i32 ───────────────┼─────► ❌ freed memory
// │                         │
// │   (y is gone)           │
// │                         │
// └─────────────────────────┘


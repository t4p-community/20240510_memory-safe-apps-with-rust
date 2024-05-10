mod counter;

use crate::counter::Counter;

// fn immutable_vector() {
//     let v = vec![1, 2, 3];
//     // the error complains that v cannot be borrowed
//     // as mutable, what does this mean?
//     v.push(4);
//     println!("{:?}", v);
// }

// fn mutable_vector() {
//     let mut v = vec![1, 2, 3];

//     // the mutable reference to v is passed to the push method
//     v.push(4);
//     println!("{:?}", v);
// }

// fn immutable_struct() {
//     let counter = Counter::new();
//     counter.increment();
//     counter.increment();
//     counter.increment();
//     counter.decrement();
//     println!("Counter: {}", counter.get_count());
// }

fn mutable_struct() {
    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    counter.decrement();
    println!("Counter: {}", counter.get_count());
}

fn main() {
    // immutable_vector();
    // mutable_vector();
    // immutable_struct();
    // mutable_struct();
}

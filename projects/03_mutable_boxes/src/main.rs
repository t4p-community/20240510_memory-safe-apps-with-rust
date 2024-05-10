// fn read_boxed_data() {
//     let boxed_data = Box::new(42);

//     // dereference the Box to read the data
//     println!("{}", *boxed_data);
//     // automatically dereference the Box to read the data
//     println!("{}", boxed_data);
// }

fn modify_boxed_data() {
    let mut boxed_data = Box::new(42);

    // this will not work, because the assignment is applied to the Box,
    // not the contents of the box
    // boxed_data = 84;

    // to modify the data, explicit dereference is required
    *boxed_data = 84;

    println!("{}", boxed_data);
}

fn main() {
    // read_boxed_data();
    modify_boxed_data();
}

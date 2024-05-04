// fn move_example() {
//     fn print_vec(v: Vec<i32>) {
//         println!("{:?}", v);
//     }

//     let v = vec![1, 2, 3];
//     print_vec(v);

//     // This will not work because v has been moved into the function
//     //print_vec(v);
// }

// fn borrow_example() {
//     // the & symbol is used to create a reference
//     fn print_vec(v: &Vec<i32>) {
//         println!("{:?}", v);
//     }

//     let v = vec![1, 2, 3];
//     print_vec(&v);

//     // This will work because v is borrowed
//     print_vec(&v);
// }

// fn mutable_borrow_example() {
//     // the &mut symbol is used to create a mutable reference
//     fn add_one(v: &mut Vec<i32>) {
//         for i in v {
//             *i += 1;
//         }
//     }

//     fn print_vec(v: &Vec<i32>) {
//         println!("{:?}", v);
//     }

//     let mut v = vec![1, 2, 3];
//     // v is borrowed mutably
//     add_one(&mut v);
//     print_vec(&v);
// }

fn main() {
    // move_example();
    // borrow_example();
    // mutable_borrow_example();
}

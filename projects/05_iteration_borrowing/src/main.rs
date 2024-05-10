mod cart_item;

use crate::cart_item::CartItem;

// fn iteration_mutable_struct() {
//     let mut cart = vec![
//         CartItem {
//             name: "apple".to_string(),
//             price: 1.0,
//             quantity: 2,
//         },
//         CartItem {
//             name: "banana".to_string(),
//             price: 0.5,
//             quantity: 3,
//         },
//     ];

//     for cart_item in cart.iter_mut() {
//         cart_item.quantity = 0;
//     }

//     for cart_item in cart.iter() {
//         println!(
//             "Name: {}, Price: {}, Qty: {}",
//             cart_item.name, cart_item.price, cart_item.quantity
//         );
//     }
// }

fn iteration_mutable_value() {
    let mut nums = vec![1, 2, 3, 4, 5];

    for num in nums.iter_mut() {
        // when you want to modify the value that the mutable
        // reference points to directly (not just a field of
        // that value), as is the case with your
        // `iteration_mutable` function, you need to use the
        // dereference operator `*` explicitly
        *num *= 2;
    }

    for num in nums.iter() {
        println!("{}", num);
    }
}

fn main() {
    iteration_mutable_struct();
    // iteration_mutable_value();
}

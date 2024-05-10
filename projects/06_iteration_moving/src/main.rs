mod product_inventory;

use crate::product_inventory::ProductInventory;

fn main() {
    let products = vec![
        ProductInventory::new("Apple".to_string(), 1.0, 10),
        ProductInventory::new("Banana".to_string(), 0.5, 20),
        ProductInventory::new("Cherry".to_string(), 2.0, 5),
    ];

    // let products_in_stock = products
    //     .iter()
    //     .map(|product| {
    //         // must clone to use a new product in the new vector
    //         // let mut product = product.clone();
    //         product.update_quantity(-5);
    //         product
    //     })
    //     .filter(|product| product.quantity > 0)
    //     .collect::<Vec<ProductInventory>>();

    let products_in_stock = products
        .into_iter()
        .map(|mut product| {
            // cloning is not necessary because we are moving the product
            product.update_quantity(-5);
            product
        })
        .filter(|product| product.quantity > 0)
        .collect::<Vec<ProductInventory>>();

    // println!("{:?}", products);
    println!("{:?}", products_in_stock);
}

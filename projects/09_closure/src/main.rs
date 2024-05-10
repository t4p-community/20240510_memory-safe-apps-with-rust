fn closure() {
    // let mut x = 2;

    // let add_two = |y| x + y;

    // let z = 3;
    // let result = add_two(z);
    // println!("result: {} + {} => {}", x, z, result);

    // x = 3;

    // // when this code is commented, the assignment to x above works because the closure is destroyed
    // // but if the command is uncommented, then the closure is still alive and the assignment to x
    // // above will fail because x is still borrowed by the closure.
    // // using move will fix this issue.
    // let result = add_two(z);
    // println!("result: {} + {} != {}", x, z, result);
}

fn move_closure() {
    // let mut x = 2;

    // let add_two = move |y| x + y;

    // let z = 3;
    // let result = add_two(z);
    // println!("result: {} + {} => {}", x, z, result);

    // x = 3;

    // // when this code is commented, the assignment to x above works because the closure is destroyed
    // // but if the command is uncommented, then the closure is still alive and the assignment to x
    // // above will fail because x is still borrowed by the closure.
    // // using move will fix this issue.
    // let result = add_two(z);
    // println!("result: {} + {} != {}", x, z, result);
}

#[derive(Debug, Clone)]
struct Person {
    first_name: String,
    last_name: String,
}

fn move_closure_with_struct() {
    let mut person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let full_name = move || format!("{} {}", person.first_name, person.last_name);

    let person2 = person.clone();
    let full_name2 = || format!("{} {}", person2.first_name, person2.last_name);

    person.first_name = "Tim".to_owned();

    // let full_name2 = || format!("{} {}", person.first_name, person.last_name);

    println!("Full name: {}", full_name());
    println!("Full name: {}", full_name2());
}

fn main() {
    // closure();
    // move_closure();
    move_closure_with_struct();
}

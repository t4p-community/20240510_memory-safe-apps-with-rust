// mod contact_move;
// use crate::contact_move::{Address, Person, PersonReporter};

mod contact_borrow;
use crate::contact_borrow::{Address, Person, PersonReporter};

fn main() {
    let address = Address {
        street: "1234 Elm St".to_string(),
        city: "Springfield".to_string(),
        state: "IL".to_string(),
        zip_code: "62701".to_string(),
    };

    let person = Person::new("Bob", "Smith", address);

    // let reporter1 = PersonReporter::new(person);
    let reporter1 = PersonReporter::new(&person);
    reporter1.report();

    // when moving person to reporter, person is no longer available
    // this code will not work when moving
    // let reporter = PersonReporter::new(&person);
    // reporter.report();

    // this code will work when moving because a new address and person are created
    let address = Address {
        street: "1234 Elm Rd".to_string(),
        city: "Springfield".to_string(),
        state: "IL".to_string(),
        zip_code: "62701".to_string(),
    };

    let person = Person::new("Jane", "Smith", address);

    // let reporter2 = PersonReporter::new(person);
    let reporter2 = PersonReporter::new(&person);
    reporter2.report();

    // can still report because of the lifetime of the person
    reporter1.report();
}

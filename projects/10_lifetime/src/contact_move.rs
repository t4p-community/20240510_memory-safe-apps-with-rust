pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub address: Address,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str, address: Address) -> Self {
        Self {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            address,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn mail_label(&self) -> String {
        format!(
            "{} {}\n{}\n{}, {} {}",
            self.first_name,
            self.last_name,
            self.address.street,
            self.address.city,
            self.address.state,
            self.address.zip_code
        )
    }
}

pub struct PersonReporter {
    pub person: Person,
}

impl PersonReporter {
    pub fn new(person: Person) -> Self {
        Self { person }
    }

    pub fn report(&self) {
        println!("Full name: {}", self.person.full_name());
        println!("Mail label:\n{}", self.person.mail_label());
    }
}

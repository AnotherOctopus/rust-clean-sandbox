pub struct User {
    name: String,
    password_hash: String,
}
pub struct Admin {
    name: String,
    password_hash: String,
    permissions: u8,
}
pub trait Named {
    fn sayName(&self);
    fn stringify(self) -> String;
}

impl User {
    pub fn new(name: String, password: String) -> User {
        User {
            name: name,
            password_hash: password,
        }
    }
}

impl Admin {
    pub fn new(name: String, password: String) -> Admin {
        Admin {
            name: name,
            password_hash: password,
            permissions: 127,
        }
    }
}

impl Named for User {
    fn sayName(&self) {
        println!("{}", self.name);
    }
    fn stringify(self) -> String {
        format!("{} {}", self.name, self.password_hash)
    }
}

impl Named for Admin {
    fn sayName(&self) {
        println!("Admin: {}", self.name);
    }
    fn stringify(self) -> String {
        format!("{} {} {}", self.name, self.password_hash, self.permissions)
    }
}

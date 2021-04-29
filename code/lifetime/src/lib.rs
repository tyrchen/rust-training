#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> User<'a> {
    // doesn't work since lifetime differs (cannot guarantee 'b > 'a)
    // pub fn new<'b>(name: &'b str, age: u8) -> Self {
    //     Self { name, age }
    // }

    pub fn new(name: &'a str, age: u8) -> Self {
        Self { name, age }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fails() {
        let name = String::from("Alice");
        let mut user = User::new(name.as_str(), 10);
        assert_eq!(user.name, "Alice");
        let user_ref = &mut user;

        // { // not live long enough
        //     let name = String::from("Bob");
        //     user_ref.name = name.as_str();
        // }

        println!("user: {:?}", user);
    }
}

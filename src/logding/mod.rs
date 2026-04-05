use std::{collections::HashMap, fmt::Display};

pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();
}

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    // fn get_description(&self) -> String {
    //     format!("{} is the pinnacle of luxury", self.name)
    // }

    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T: Display> Description for Hotel<T> {}

#[derive(Debug)]
pub struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) -> () {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

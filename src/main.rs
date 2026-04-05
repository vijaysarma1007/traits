use std::{collections::HashMap, fmt::Display};
use traits::logding::{Accommodation, AirBnb, Description, Hotel};
use traits::utils::{book_for_one_night, mix_and_match};

fn main() {
    let mut hotel = Hotel::new("The luxe");
    let mut airbnb = AirBnb::new("Peter");
    // mix_and_match(&mut hotel, &mut airbnb, "Vijay");
    // println!("{hotel:#?} {airbnb:#?}");

    let hotel1 = Hotel::<String>::new(String::from("The luxe"));
    println!("{}", hotel1.summarize()); //works

    let hotel2 = Hotel::<&str>::new("The Golden standard");
    println!("{}", hotel2.summarize()); //works

    let hotel3 = Hotel::<Vec<&str>>::new(vec!["The Sweet Escape", "hitman Hotel"]);
    // println!("{}", hotel3.summarize()) //do not work

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("vijay", 2);
    stays[1].book("Mohan", 3);

    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);
}

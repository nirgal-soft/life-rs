use std::char;
use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() 
{
    let cell = char::from_u32(100);
    //creates the cursive root, required for every application
    let mut siv = Cursive::default();

    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(TextView::new(char::from_u32(100).to_string()));

    //start the siv loop
    siv.run();
}

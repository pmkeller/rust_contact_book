use crate::{controller, App};

pub fn main_ui() {
    //TODO make main menu for contact book.
    // Add, Edit, Delete, Show, Search, Save Contact Book, Load Contact Book
    cyan_ln!("-------------Contact Address Book--------------");
    blue_ln!("Designed in Rust Language \t By Peter Keller");
    cyan_ln!("-----------------------------------------------");
    println!();
    white!("1) ");
    yellow_ln!("Add New Contact");
    white!("2) ");
    yellow_ln!("Edit Contact");
    white!("3) ");
    yellow_ln!("Delete Contact");
    white!("4) ");
    yellow_ln!("Search For Contact");
    white!("5) ");
    yellow_ln!("Show All Contacts");
    white!("6) ");
    yellow_ln!("Quit");
    cyan_ln!("-----------------------------------------------");
}

pub fn search_ui() {
    cyan_ln!("-------------Search Address Book--------------");
    println!();
    white!("1) ");
    yellow_ln!("Search by first name");
    white!("2) ");
    yellow_ln!("Search by surname");
    white!("3) ");
    yellow_ln!("Search by first name and surname");
    white!("4) ");
    yellow_ln!("Search by telephone number");
    white!("5) ");
    yellow_ln!("Search by email address");
    white!("6) ");
    yellow_ln!("Leave Search Menu");
    cyan_ln!("-----------------------------------------------");
}

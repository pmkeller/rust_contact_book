use std::collections::HashMap;
use std::io::{stdin, self, Write};
use std::error::Error;
use std::process;

use csv::Reader;
use serde::{Serialize, Deserialize};



//Global variables
static PATH: &str = "data/contactbook.csv";

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
struct Contact {
    first_name: String,
    surname: String,
    date_of_birth: String,
    address: String,
    tel: String,
    email: String,
    #[serde(skip)]
    //TODO put this vector of hashmap as variable outside the struct
    contact_book: Vec<HashMap<String,String>>,
}

#[allow(dead_code)]
impl Contact {
    fn new(first_name: String, surname: String, date_of_birth: String, address: String, tel: String, email: String) -> Self {
        Self {
            first_name: first_name,
            surname: surname,
            date_of_birth: date_of_birth,
            address: address,
            tel: tel,
            email: email,
            contact_book: vec![]
        }
    }

    //Function used to push struct data into struct vector of hashmaps contact_book
    //TODO rewrite function to use the new global variable of contact_book as its no longer be used in the struct Contact
    fn data_push(&mut self) {
        let mut contact_hash = HashMap::new(); //remove this

        //replace contact_hash with contact_book
        contact_hash.insert("Name".to_string(), self.first_name.trim_end().to_string());
        contact_hash.insert("Surname".to_string(), self.surname.trim_end().to_string());
        contact_hash.insert("DateofBirth".to_string(), self.date_of_birth.trim_end().to_string());
        contact_hash.insert("Address".to_string(), self.address.trim_end().to_string());
        contact_hash.insert("Tel".to_string(), self.tel.trim_end().to_string());
        contact_hash.insert("Email".to_string(), self.email.trim_end().to_string());

        self.contact_book.push(contact_hash);
    }

    //Function to print contact book to screen
    fn print_data(&self) {
        if self.contact_book.len() > 0 {
            for contact_hash in &self.contact_book {
                println!("--------------------------------------------------------------");
                println!("{} {}", contact_hash.get("Name").unwrap(), contact_hash.get("Surname").unwrap());
                println!("Tel: {} \tEmail: {}", contact_hash.get("Tel").unwrap(), contact_hash.get("Email").unwrap());
                println!("Address: {}", contact_hash.get("Address").unwrap());
                println!("Date of Birth: {}", contact_hash.get("DateofBirth").unwrap());
                println!("--------------------------------------------------------------");
            }
        } else {
            println!("Contact Book is empty");
        }
    }

    //TODO remove data based on key search from vector
    fn remove_data() {

    }

    //TODO edit data based on key search from vector
    fn edit_data() {

    }

    //TODO Export CSV Function
// Function to export contact vector to a csv file
    fn export_csv() {
    }

    //TODO Search Function
// iterate through
    fn search_contact() {
    }

}

// Function for getting user input and returns the input
fn input_capture(text: &str) -> String {
    let mut input = String::new();

    println!("{}: ", text);
    io::stdout().flush().unwrap();

    stdin().read_line(&mut input)
        .expect("Did not enter a correct string");

    return input;
}

// Function to import contact names from a csv file
fn import_csv(p_contact: &mut Contact) -> Result<(), Box<dyn Error>>{

    let mut rdr = Reader::from_path(PATH)?;
    for result in rdr.deserialize() {
        let t_contact:Contact = result?;

        //TODO Find another way to copy struct data from struct to another
        //cant use std::mem::swap as it will swap the contact_book vector leaving it with only one entry
        p_contact.first_name = t_contact.first_name;
        p_contact.surname = t_contact.surname;
        p_contact.date_of_birth = t_contact.date_of_birth;
        p_contact.address = t_contact.address;
        p_contact.tel = t_contact.tel;
        p_contact.email = t_contact.email;

        //function called to push new data into hashmap
        p_contact.data_push();
    }

    Ok(())
}

//Functions calls input_capture and stores into struct
fn  user_data_capture(p_contact: &mut Contact) {
    
    let name = input_capture("Enter First Name");
    let surname = input_capture("Enter Surname");
    let dob = input_capture("Enter date of birth");
    let address = input_capture("Enter your address");
    let tel = input_capture("Enter your telephone number");
    let email = input_capture("Enter email address");

    //assigning captured data to struct
    p_contact.first_name = name;
    p_contact.surname = surname;
    p_contact.date_of_birth = dob;
    p_contact.address = address;
    p_contact.tel = tel;
    p_contact.email = email;

    //function called to push new data into hashmap
    p_contact.data_push()
}

//TODO function for search Function
// Menu for searching by content(HashMaps Keys)
fn search_ui(){
}


//Function that has the main menu
fn main_ui(){
    //TODO make main menu for contact book.
    // Add, Edit, Delete, Show, Search, Save Contact Book, Load Contact Book
}



fn main() {

    let mut contact = Contact{..Default::default()};

    contact.print_data();

   //import_csv()
    if let Err(err) = import_csv(&mut contact) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    user_data_capture(&mut contact);
    contact.print_data();
}

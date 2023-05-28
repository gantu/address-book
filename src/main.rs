use std::{io::{self}};

use addressbook::{print_menu, print_menu_add, add, list};
 
fn main() {
    let mut address_list = Vec::new();
    loop {
        print_menu();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        print!("{}", buffer);
        match buffer.as_str().trim() {
             "1" => {
                print_menu_add();
                let contact = read_string_from_stdin();
                if !contact.is_empty() {
                    let new_addr_list = add(contact, &address_list);
                    new_addr_list.clone_into(&mut address_list);
                    
                    list(&address_list);
                }
            },
            "-1" => break,
            _ => println!("not yet implemented")
        }
    }
    
    
}
    
fn read_string_from_stdin() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer.as_str().trim().to_owned()
}
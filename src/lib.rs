

pub fn print_menu() {
    println!("-1 to exit!\n1.add\n2.list\n3.delete\n4.edit\n5.find\n");
}

pub fn print_menu_add() {
  println!("Enter contact in format of - Name Surname Phonenumber : ");
}

pub fn add<T: Clone>(contact: T, addr_book: &Vec<T>) -> Vec<T>{  
  let mut list_internal = addr_book.to_vec();
  list_internal.push(contact);
  list_internal
}

pub fn delete<'a>(number: &'a str, addr_book: Vec<&'a str>) -> Vec<&'a str> {
  addr_book.into_iter().filter(|c| !c.contains(number)).collect::<Vec<&str>>()
}

pub fn list<T: Clone + std::fmt::Display>(addr_book: &Vec<T>) {
  for contact in addr_book {
    print!("{}", contact);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn add_on_empty() {
    let contact = "John Smith 1234567890";
    let addr_book = vec![];
    let new_addr_book = add(contact, &addr_book);
    assert_eq!(1, new_addr_book.len());
  }
  #[test]
  fn add_on_non_empty() {
    let contact = "John Smith 1234567890";
    let addr_book = vec![contact];
    let new_addr_book = add(contact, &addr_book);
    assert_eq!(2, new_addr_book.len());
  }

  #[test]
  fn delete_matching_contact() {
    let contact = "John Smith 1234567890";
    let contact1 = "John Wick 1112223330";
    let addr_book = vec![contact, contact1];
    let new_addr_book = delete("1234567890", addr_book);
    assert_eq!(1, new_addr_book.len());
  }
}
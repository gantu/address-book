use::std::io::Result;

pub fn print_menu() {
    println!("-1 to exit!\n1.add\n2.list\n3.delete\n4.edit\n5.find\n");
}

pub fn add<'a>(contact: &'a str, addr_book: Vec<&'a str>) -> Vec<&'a str>{
  [&addr_book[..], &[&contact]].concat()
}

pub fn delete<'a>(number: &'a str, addr_book: Vec<&'a str>) -> Vec<&'a str> {
  addr_book.into_iter().filter(|c| !c.contains(number)).collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn add_on_empty() {
    let contact = "John Smith 1234567890";
    let addr_book = vec![];
    let new_addr_book = add(contact, addr_book);
    assert_eq!(1, new_addr_book.len());
  }
  #[test]
  fn add_on_non_empty() {
    let contact = "John Smith 1234567890";
    let addr_book = vec![contact];
    let new_addr_book = add(contact, addr_book);
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
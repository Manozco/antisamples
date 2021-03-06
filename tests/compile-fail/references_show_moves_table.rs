// Passing a non-copyable data structure by value moves it.

// error-pattern: use of moved value: `table`
// error-pattern: move occurs because `table` has type `std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>`, which does not implement the `Copy` trait

use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    show(table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}

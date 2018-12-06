use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
  for (artist, works) in table {
    println!("works by {}:", artist);
    for work in works {
      println!(" {}", work);
    }
  }
}

fn ref_show(table: &Table) {
  for (artist, works) in table {
    println!("works by {}:", artist);
    for work in works {
      println!(" {}", work);
    }
  }
}

fn main() {
  let mut table = Table::new();
  table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenabrae Responsoria".to_string()]);
  table.insert("Caravaggio".to_string(), vec!["The calling of
St.Matthew".to_string()]);
  table.insert("Cellini".to_string(), vec!["Perseus with the head of
Medusa".to_string()]);
  ref_show(&table);
  show(table);
}
//Note: Reference are never NULL

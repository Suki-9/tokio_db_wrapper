mod client;

pub fn run_query(query: &str, panic: bool) {
  println!("[DB Service] Run query\n  ->{query:}");
  match futures::executor::block_on((&client::DB_CLIENNT).batch_execute(query)) {
    Ok(_) => {},
    Err(e) => if panic { panic!("{e:}") } else { print!("{e:}") },
  }
}

#[macro_export]
macro_rules! create_table {
  ($t:expr, $( $k_t:expr ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {}", $k_t.0, $k_t.1));)*
    $crate::run_query(&format!("CREATE TABLE {} ({});", $t ,tmp.join(",")), true);
  };
}

#[macro_export]
macro_rules! drop_table {
  ($t:expr) => {
    $crate::run_query(&format!("DROP TABLE {} ;", $t), true);
  };
}
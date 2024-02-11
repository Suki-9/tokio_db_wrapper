mod client;
use futures::executor::block_on;
use client::DB_CLIENNT;

pub(crate) fn run_query(query: &str, panic: bool) {
  println!("[DB Service] Run query\n  ->{query:}");
  match block_on((&DB_CLIENNT).batch_execute(&query)) {
    Ok(_) => {},
    Err(e) => if panic { panic!("{e:}") } else { print!("{e:}") },
  }
}

#[macro_export]
macro_rules! create_table {
  ($t:expr, $( $k_t:expr ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {}", $k_t.0, $k_t.1));)*
    run_query(format!("CREATE TABLE {} ({});", $t ,tmp.join(",")), true);
  };
}

#[macro_export]
macro_rules! drop_table {
  ($t:expr) => {
    run_query(format!("DROP TABLE {} ;", $t), true);
  };
}
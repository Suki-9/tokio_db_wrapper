mod client;

pub fn run_query(query: &str, panic: bool) {
  println!("[DB Service] Run query\n  ->{query:}");
  match futures::executor::block_on((&client::DB_CLIENNT).batch_execute(query)) {
    Ok(_) => {},
    Err(e) => if panic { panic!("{e:}") } else { print!("{e:}") },
  }
}

pub fn run_query2(query: &str, panic: bool) -> Option<Vec<tokio_postgres::Row>> {
  println!("[DB Service] Run query\n  ->{query:}");
  match futures::executor::block_on((&client::DB_CLIENNT).query(query, &[])) {
    Ok(row) => Some(row),
    Err(e) =>
      if panic { panic!("{e:}") }
      else { print!("{e:}"); None },
  }
}

//TODO 文字列じゃなくてRustの形を受け取ってpostgre側の方に変換するようにすると読む時幸せになれるかも。
#[macro_export]
macro_rules! create_table {
  ($t:expr, $( ($k:expr, $tp:ty) ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {:?}", $k, 
      match std::any::type_name::<$tp>() {
        "i32" | "i16" | "i8" | "isize" => "INTEGER",
        "i64" => "BIGINT",
        "bool" => "BOOL",
        "u32" | "u16" | "u8" | "usize" => "FROAT8",
        "String" | "str" => "TEXT",
        _ => panic!("[DB Service] Type Error!!"),
      }
    ));)*
    println!("CREATE TABLE {} ({});", $t ,tmp.join(","));
    $crate::run_query(&format!("CREATE TABLE {} ({});", $t ,tmp.join(",")), true);
  };
}

#[macro_export]
macro_rules! drop_table {
  ($t:expr) => {
    $crate::run_query(&format!("DROP TABLE {};", $t), false);
  };
}

#[macro_export]
macro_rules! insert_record {
  ($t:expr, $( $k_t:expr ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {}", $k_t.0, $k_t.1));)*
    $crate::run_query(&format!("CREATE TABLE {} ({});", $t ,tmp.join(",")), false)
  };
}

#[macro_export]
macro_rules! select_record {
  ($t:expr, $( $k_t:expr ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {}", $k_t.0, $k_t.1));)*
    $crate::run_query(&format!("CREATE TABLE {} ({});", $t ,tmp.join(",")), false)
  };
}


#[macro_export]
macro_rules! update_record {
  ($t:expr, $( $k_t:expr ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {}", $k_t.0, $k_t.1));)*
    $crate::run_query(&format!("CREATE TABLE {} ({});", $t ,tmp.join(",")), false)
  };
}

#[macro_export]
macro_rules! delete_record {
  ($t:expr, $op:expr) => {
    $crate::run_query(&format!("DELETE FROM {} WHERE {};", $t ,$op), false)
  };
}

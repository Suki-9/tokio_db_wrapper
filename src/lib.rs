mod client;

pub fn run_query(query: &str, panic: bool) {
  println!("[DB Service] Run query\n  ->{query:}");
  match futures::executor::block_on((&client::DB_CLIENNT).batch_execute(query)) {
    Ok(_) => {},
    Err(e) => if panic { panic!("{e:}") } else { print!("{e:}") },
  }
}

pub fn run_query2(query: &str) -> Option<Vec<tokio_postgres::Row>> {
  println!("[DB Service] Run query\n  -> {query:}");
  match futures::executor::block_on((&client::DB_CLIENNT).query(query, &[])) {
    Ok(row) => Some(row),
    Err(e) => { print!("{e:}"); None },
  }
}

//TODO autoincrementとかも使えるように分岐を使って分けるべき。
#[macro_export]
macro_rules! create_table {
  ($t:expr, $( ($k:expr, $tp:ty) ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {}", $k, 
      match std::any::type_name::<$tp>() {
        "i32" | "i16" | "i8" | "isize" => "INTEGER",
        "i64" => "BIGINT",
        "bool" => "BOOL",
        "u32" | "u16" | "u8" | "usize" => "FROAT8",
        "String" | "str" => "TEXT",
        _ => panic!("[DB Service] Type Error!!"),
      }
    ));)*
    $crate::run_query(&format!("CREATE TABLE IF NOT EXISTS {} ({});", $t ,tmp.join(",")), true);
  };

  ($t:expr, $( ($k:expr, $tp:ty, $op:expr) ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} {} {}",
      $k, 
      match std::any::type_name::<$tp>() {
        "i32" | "i16" | "i8" | "isize" => "INTEGER",
        "i64" => "BIGINT",
        "bool" => "BOOL",
        "u32" | "u16" | "u8" | "usize" => "FROAT8",
        "String" | "str" => "TEXT",
        _ => panic!("[DB Service] Type Error!!")},
      $op
    ));)*
    $crate::run_query(&format!("CREATE TABLE IF NOT EXISTS {} ({});", $t ,tmp.join(",")), true);
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
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    $(
      keys.push(format!("{}", $k_t.0));
      values.push(format!("'{}'", $k_t.1));
    )*
    $crate::run_query(&format!("INSERT INTO {} ({}) VALUES({});", $t ,keys.join(","), values.join(",")), false)
  };
}

#[macro_export]
macro_rules! select_record {
  ($t:expr, $( $c:expr ),+) => {{
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{}", $c));)*
    $crate::run_query2(&format!("SELECT {} FROM {};", tmp.join(","), $t))
  }};

  ($t:expr, $op:expr, $( $c:expr ),+) => {{
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{}", $c));)*
    $crate::run_query2(&format!("SELECT {} FROM {} WHERE {};", tmp.join(","), $t, $op))
  }};
}

#[macro_export]
macro_rules! update_record {
  ($t:expr, $( ($k:expr, $v:expr) ),+) => {
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} = '{}'", $k, $v));)*
    $crate::run_query(&format!("UPDATE {} SET {};", $t ,tmp.join(",")), false)
  };

  ($t:expr, $op:expr, $( ($k:expr, $v:expr) ),+) => {{
    let mut tmp: Vec<String> = Vec::new();
    $(tmp.push(format!("{} = '{}'", $k, $v));)*
    $crate::run_query(&format!("UPDATE {} SET {} WHERE {};", $t ,tmp.join(","), $op), false)
  }};
}

#[macro_export]
macro_rules! delete_record {
  ($t:expr, $op:expr) => {
    $crate::run_query(&format!("DELETE FROM {} WHERE {};", $t ,$op), false)
  };
}

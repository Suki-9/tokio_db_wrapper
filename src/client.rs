use futures::executor::block_on;
use once_cell::sync::Lazy;
use tokio_postgres::{
  Client,
  NoTls,
};
use serde_derive::{
  Serialize,
  Deserialize,
};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub HOST: String,
  pub PORT: String,
  pub USER: String,
  pub PASSWORD: String,
  pub NAME: String,
}

pub(crate) static DB_CLIENNT: Lazy<Client> = Lazy::new(|| {
  let config: Config = match serde_yaml::from_str(&std::fs::read_to_string("./default.yaml").unwrap()) {
    Ok(c) => c,
    Err(e) => panic!("config file open faild. {e:}"),
  };
  block_on(init_crient(&config.HOST, &config.PORT, &config.USER, &config.PASSWORD, &config.NAME))
});

async fn init_crient(host: &str, port: &str, user: &str, password: &str, db_name: &str) -> Client {
  let (client, connection) = match tokio_postgres::connect(
    &format!("host={host:} port={port:} user={user:} password={password:} dbname={db_name:}"),
    NoTls,
  ).await {
    Ok((client, connection)) => (client, connection),
    Err(e) => {
      panic!("connection error: {e:}");
    },
  };

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      panic!("connection error: {}", e);
    }
  });

  return client;
}

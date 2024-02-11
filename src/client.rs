use once_cell::sync::Lazy;
use tokio_postgres::{
  Client,
  NoTls,
};

#[allow(non_snake_case)]
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
  pub DB_HOST: String,
  pub DB_PORT: String,
  pub DB_USER: String,
  pub DB_PASSWORD: String,
  pub DB_NAME: String,
}

pub(crate) static DB_CLIENNT: Lazy<Client> = once_cell::sync::Lazy::new(|| {
  let config: Config = match serde_yaml::from_str(&std::fs::read_to_string("./default.yaml").unwrap()) {
    Ok(c) => c,
    Err(e) => panic!("config file open faild. {e:}"),
  };
  futures::executor::block_on(init_crient(&config.DB_HOST, &config.DB_PORT, &config.DB_USER, &config.DB_PASSWORD, &config.DB_NAME))
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

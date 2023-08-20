use mini_redis::Connection;
use mini_redis_client_in_tokio::{RedisDB, RedisError};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("127.0.0.1:6380").await.expect("address is already in use");
  let db = RedisDB::new(10);
  loop {
    let (socket, _) = listener.accept().await.expect("accepct error");
    println!(">>> new connection accepted: {:?}", socket);
    let db = db.clone();
    tokio::spawn(async move {
      process(socket, db).await.expect("unexpected error occured");
    });
  }
}

async fn process(socket: TcpStream, db: RedisDB) -> Result<(), RedisError> {
  let mut connection = Connection::new(socket);
  while let Some(frame) = connection.read_frame().await? {
    let response = db.dispatch(frame)?;
    connection.write_frame(&response).await?;
  }
  Ok(())
}

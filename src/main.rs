use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Command as RedisCommand};
use mini_redis_client_in_tokio::{RedisDB, RedisError};

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("127.0.0.1:6380").await
		.expect("address is already in use");

	let db = RedisDB::new();
	loop {
		let (socket, _) = listener.accept().await.expect("accepct error");

		let db = db.clone();
		tokio::spawn(async move {
			process(socket, db).await
				.expect("unexpected error occured");
		});
	}
}

async fn process(socket: TcpStream, db: RedisDB) -> Result<(), RedisError> {
	let mut connection = Connection::new(socket);
	while let Some(frame) = connection.read_frame().await? {
		let response = match RedisCommand::from_frame(frame)? {
			RedisCommand::Set(cmd) => db.set(cmd),
			RedisCommand::Get(cmd) => db.get(cmd),
			cmd => return Err(RedisError::new(&format!("invalid cmd: {:?}", cmd))),
		};
		connection.write_frame(&response?).await?;
	}
	Ok(())
}


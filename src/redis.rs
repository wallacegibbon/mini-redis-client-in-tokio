use std::sync::{Arc, Mutex, PoisonError};
use std::collections::HashMap;
use mini_redis::cmd::{Get as GetCommand, Set as SetCommand};
use mini_redis::frame as redis_frame;
use core::fmt;
use bytes::Bytes;

pub struct RedisDB {
	db: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl Clone for RedisDB {
	fn clone(&self) -> Self {
		RedisDB { db: self.db.clone() }
	}
}

impl RedisDB {
	pub fn new() -> Self {
		RedisDB { db: Arc::new(Mutex::new(HashMap::new())) }
	}

	pub fn get(&self, command: GetCommand) -> Result<redis_frame::Frame, RedisError> {
		let db = self.db.lock()?;
		if let Some(value) = db.get(command.key()) {
			Ok(redis_frame::Frame::Bulk(value.clone()))
		} else {
			Ok(redis_frame::Frame::Null)
		}
	}

	pub fn set(&self, command: SetCommand) -> Result<redis_frame::Frame, RedisError> {
		let mut db = self.db.lock()?;
		db.insert(command.key().to_string(), command.value().clone());
		Ok(redis_frame::Frame::Simple("OK".to_string()))
	}
}

#[derive(Debug)]
pub struct RedisError {
	info: String,
}

impl RedisError {
	pub fn new(info: &str) -> Self {
		RedisError { info: info.to_string() }
	}
}

impl fmt::Display for RedisError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "redis error; {}", self.info)
	}
}

impl<T> From<PoisonError<T>> for RedisError {
	fn from(cause: PoisonError<T>) -> Self {
		RedisError { info: cause.to_string() }
	}
}

impl From<Box<dyn std::error::Error + Send + Sync>> for RedisError {
	fn from(cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
		RedisError { info: cause.to_string() }
	}
}

impl From<std::io::Error> for RedisError {
	fn from(cause: std::io::Error) -> Self {
		RedisError { info: cause.to_string() }
	}
}


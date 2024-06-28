mod data;

use std::sync::Arc;

use conduit::{Result, Server};
use data::Data;
use database::Database;
use ruma::{OwnedRoomId, OwnedUserId, RoomId, UserId};

pub struct Service {
	db: Data,
}

impl Service {
	pub fn build(_server: &Arc<Server>, db: &Arc<Database>) -> Result<Self> {
		Ok(Self {
			db: Data::new(db),
		})
	}

	pub fn reset_notification_counts(&self, user_id: &UserId, room_id: &RoomId) -> Result<()> {
		self.db.reset_notification_counts(user_id, room_id)
	}

	pub fn notification_count(&self, user_id: &UserId, room_id: &RoomId) -> Result<u64> {
		self.db.notification_count(user_id, room_id)
	}

	pub fn highlight_count(&self, user_id: &UserId, room_id: &RoomId) -> Result<u64> {
		self.db.highlight_count(user_id, room_id)
	}

	pub fn last_notification_read(&self, user_id: &UserId, room_id: &RoomId) -> Result<u64> {
		self.db.last_notification_read(user_id, room_id)
	}

	pub fn associate_token_shortstatehash(&self, room_id: &RoomId, token: u64, shortstatehash: u64) -> Result<()> {
		self.db
			.associate_token_shortstatehash(room_id, token, shortstatehash)
	}

	pub fn get_token_shortstatehash(&self, room_id: &RoomId, token: u64) -> Result<Option<u64>> {
		self.db.get_token_shortstatehash(room_id, token)
	}

	pub fn get_shared_rooms(&self, users: Vec<OwnedUserId>) -> Result<impl Iterator<Item = Result<OwnedRoomId>> + '_> {
		self.db.get_shared_rooms(users)
	}
}

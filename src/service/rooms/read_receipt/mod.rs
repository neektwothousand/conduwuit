mod data;

use std::sync::Arc;

use conduit::{Result, Server};
use data::Data;
use database::Database;
use ruma::{events::receipt::ReceiptEvent, serde::Raw, OwnedUserId, RoomId, UserId};

use crate::services;

pub struct Service {
	db: Data,
}

impl Service {
	pub fn build(_server: &Arc<Server>, db: &Arc<Database>) -> Result<Self> {
		Ok(Self {
			db: Data::new(db),
		})
	}

	/// Replaces the previous read receipt.
	pub fn readreceipt_update(&self, user_id: &UserId, room_id: &RoomId, event: &ReceiptEvent) -> Result<()> {
		self.db.readreceipt_update(user_id, room_id, event)?;
		services().sending.flush_room(room_id)?;

		Ok(())
	}

	/// Returns an iterator over the most recent read_receipts in a room that
	/// happened after the event with id `since`.
	#[tracing::instrument(skip(self))]
	pub fn readreceipts_since<'a>(
		&'a self, room_id: &RoomId, since: u64,
	) -> impl Iterator<Item = Result<(OwnedUserId, u64, Raw<ruma::events::AnySyncEphemeralRoomEvent>)>> + 'a {
		self.db.readreceipts_since(room_id, since)
	}

	/// Sets a private read marker at `count`.
	#[tracing::instrument(skip(self))]
	pub fn private_read_set(&self, room_id: &RoomId, user_id: &UserId, count: u64) -> Result<()> {
		self.db.private_read_set(room_id, user_id, count)
	}

	/// Returns the private read marker.
	#[tracing::instrument(skip(self))]
	pub fn private_read_get(&self, room_id: &RoomId, user_id: &UserId) -> Result<Option<u64>> {
		self.db.private_read_get(room_id, user_id)
	}

	/// Returns the count of the last typing update in this room.
	pub fn last_privateread_update(&self, user_id: &UserId, room_id: &RoomId) -> Result<u64> {
		self.db.last_privateread_update(user_id, room_id)
	}
}

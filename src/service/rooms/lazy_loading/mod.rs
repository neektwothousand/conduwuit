mod data;

use std::{
	collections::{HashMap, HashSet},
	sync::Arc,
};

use conduit::Server;
use data::Data;
use database::Database;
use ruma::{DeviceId, OwnedDeviceId, OwnedRoomId, OwnedUserId, RoomId, UserId};
use tokio::sync::Mutex;

use crate::{PduCount, Result};

pub struct Service {
	db: Data,

	#[allow(clippy::type_complexity)]
	pub lazy_load_waiting: Mutex<HashMap<(OwnedUserId, OwnedDeviceId, OwnedRoomId, PduCount), HashSet<OwnedUserId>>>,
}

impl Service {
	pub fn build(_server: &Arc<Server>, db: &Arc<Database>) -> Result<Self> {
		Ok(Self {
			db: Data::new(db),
			lazy_load_waiting: Mutex::new(HashMap::new()),
		})
	}

	#[tracing::instrument(skip(self))]
	pub fn lazy_load_was_sent_before(
		&self, user_id: &UserId, device_id: &DeviceId, room_id: &RoomId, ll_user: &UserId,
	) -> Result<bool> {
		self.db
			.lazy_load_was_sent_before(user_id, device_id, room_id, ll_user)
	}

	#[tracing::instrument(skip(self))]
	pub async fn lazy_load_mark_sent(
		&self, user_id: &UserId, device_id: &DeviceId, room_id: &RoomId, lazy_load: HashSet<OwnedUserId>,
		count: PduCount,
	) {
		self.lazy_load_waiting
			.lock()
			.await
			.insert((user_id.to_owned(), device_id.to_owned(), room_id.to_owned(), count), lazy_load);
	}

	#[tracing::instrument(skip(self))]
	pub async fn lazy_load_confirm_delivery(
		&self, user_id: &UserId, device_id: &DeviceId, room_id: &RoomId, since: PduCount,
	) -> Result<()> {
		if let Some(user_ids) = self.lazy_load_waiting.lock().await.remove(&(
			user_id.to_owned(),
			device_id.to_owned(),
			room_id.to_owned(),
			since,
		)) {
			self.db
				.lazy_load_confirm_delivery(user_id, device_id, room_id, &mut user_ids.iter().map(|u| &**u))?;
		} else {
			// Ignore
		}

		Ok(())
	}

	#[tracing::instrument(skip(self))]
	pub fn lazy_load_reset(&self, user_id: &UserId, device_id: &DeviceId, room_id: &RoomId) -> Result<()> {
		self.db.lazy_load_reset(user_id, device_id, room_id)
	}
}

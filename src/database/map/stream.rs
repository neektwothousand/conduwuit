use conduwuit::{implement, Result};
use futures::stream::{Stream, StreamExt};
use serde::Deserialize;

use crate::{keyval, keyval::KeyVal, stream, stream::Cursor};

/// Iterate key-value entries in the map from the beginning.
///
/// - Result is deserialized
#[implement(super::Map)]
pub fn stream<'a, K, V>(&'a self) -> impl Stream<Item = Result<KeyVal<'_, K, V>>> + Send
where
	K: Deserialize<'a> + Send,
	V: Deserialize<'a> + Send,
{
	self.raw_stream().map(keyval::result_deserialize::<K, V>)
}

/// Iterate key-value entries in the map from the beginning.
///
/// - Result is raw
#[implement(super::Map)]
#[tracing::instrument(skip(self), fields(%self), level = "trace")]
pub fn raw_stream(&self) -> impl Stream<Item = Result<KeyVal<'_>>> + Send {
	let opts = super::iter_options_default();
	stream::Items::new(&self.db, &self.cf, opts).init(None)
}

#[tracing::instrument(
    name = "cached",
    level = "trace",
    skip_all,
    fields(%map),
)]
pub(super) fn _is_cached<P>(map: &super::Map) -> bool
where
	P: AsRef<[u8]> + ?Sized,
{
	let opts = super::cache_read_options_default();
	let mut state = stream::State::new(&map.db, &map.cf, opts);

	state.seek_fwd();
	!state.is_incomplete()
}

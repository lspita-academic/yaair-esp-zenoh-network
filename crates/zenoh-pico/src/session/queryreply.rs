use std::sync::Arc;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use zenoh_pico_macros::zwrap;

#[zwrap(base(name = "queryable"), zvalue, zown)]
pub(super) struct InternalQueryable;

pub struct Queryable {
    queryable: InternalQueryable,
}

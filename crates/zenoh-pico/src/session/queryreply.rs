use std::sync::Arc;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use zenoh_pico_macros::zwrap;
use zenoh_pico_sys::{
    z_queryable_keyexpr, z_queryable_options_default, z_queryable_options_t, z_undeclare_queryable,
};

use crate::{keyexpr::KeyExpr, query::Query, zoptions::ZOptionsInit, zvalue::ZValue};

#[zwrap(base(name = "queryable"), zvalue, zown(drop_zfn = z_undeclare_queryable))]
pub(super) struct InternalQueryable;

pub struct Queryable {
    pub(super) queryable: InternalQueryable,
    pub(super) signal: Arc<Signal<CriticalSectionRawMutex, Query>>,
}

impl ZOptionsInit for z_queryable_options_t {
    fn zinit(&mut self) {
        unsafe {
            z_queryable_options_default(self);
        }
    }
}

impl Queryable {
    pub async fn recv_async(&self) -> Query {
        self.signal.wait().await
    }

    pub fn keyexpr(&self) -> &KeyExpr {
        KeyExpr::from_ptr(unsafe { z_queryable_keyexpr(self.queryable.zloan()) })
    }
}

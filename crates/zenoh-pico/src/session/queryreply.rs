use std::sync::Arc;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use zenoh_pico_macros::zwrap;
use zenoh_pico_sys::{
    z_querier_get_matching_status, z_querier_get_options_default, z_querier_get_options_t,
    z_querier_get_with_parameters_substr, z_querier_keyexpr, z_querier_options_default,
    z_querier_options_t, z_queryable_keyexpr, z_queryable_options_default, z_queryable_options_t,
    z_undeclare_querier, z_undeclare_queryable,
};

use crate::{
    keyexpr::KeyExpr,
    query::{Query, Reply, ReplyClosure},
    result::{IntoZenohResult, ZenohResult},
    session::matching::MatchingStatus,
    zoptions::{ZOptionsInit, options_ptr_mut},
    zvalue::{ZClosure, ZOwn, ZValue},
};

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

#[zwrap(base(name = "querier"), zvalue, zown(drop_zfn = z_undeclare_querier))]
pub struct Querier;

impl ZOptionsInit for z_querier_options_t {
    fn zinit(&mut self) {
        unsafe {
            z_querier_options_default(self);
        }
    }
}

impl Querier {
    pub fn matching_status(&self) -> ZenohResult<MatchingStatus> {
        let mut matching_status = MatchingStatus::uninitialized();
        unsafe {
            z_querier_get_matching_status(self.zloan(), matching_status.zloan_mut()).into_zresult()
        }?;
        Ok(matching_status)
    }

    pub fn keyexpr(&self) -> &KeyExpr {
        KeyExpr::from_ptr(unsafe { z_querier_keyexpr(self.zloan()) })
    }

    pub fn get(
        &self,
        parameters: Option<&str>,
        mut options: Option<z_querier_get_options_t>,
    ) -> ZenohResult<QuerierGetHandle> {
        let options = options_ptr_mut(options.as_mut());
        let parameters = parameters.unwrap_or_default();
        let signal = Arc::new(Signal::new());
        let reply_closure = ReplyClosure::from_signal(signal.clone())?;

        unsafe {
            z_querier_get_with_parameters_substr(
                self.zloan(),
                parameters.as_ptr(),
                parameters.len(),
                &mut reply_closure.zmove(),
                options,
            )
            .into_zresult()
        }?;
        Ok(QuerierGetHandle { signal })
    }
}

pub struct QuerierGetHandle {
    signal: Arc<Signal<CriticalSectionRawMutex, Reply>>,
}

impl ZOptionsInit for z_querier_get_options_t {
    fn zinit(&mut self) {
        unsafe {
            z_querier_get_options_default(self);
        }
    }
}

impl QuerierGetHandle {
    pub async fn recv_async(&self) -> Reply {
        self.signal.wait().await
    }
}

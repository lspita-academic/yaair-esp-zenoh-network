use zenoh_pico_macros::zwrap;
use zenoh_pico_sys::z_undeclare_matching_listener;

#[zwrap(base(name = "matching_status"), zvalue, zclone)]
pub struct MatchingStatus;

#[zwrap(base(name = "closure_matching_status"), zvalue, zown, zclosure)]
pub struct MatchingStatusClosure;

impl MatchingStatus {
    pub fn is_matching(&self) -> bool {
        self.0.matching
    }
}

#[zwrap(base(name = "matching_listener"), zvalue, zown(drop_zfn = z_undeclare_matching_listener))]
pub struct MatchingListener;

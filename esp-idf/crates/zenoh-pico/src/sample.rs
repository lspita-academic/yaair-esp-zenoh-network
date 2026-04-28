use zenoh_pico_macros::zown;

use crate::zclosure;

#[zown(base = "sample", zloan(mutable), ztake)]
pub struct Sample;

#[zclosure(base = "sample", zloan)]
pub struct SampleClosure;

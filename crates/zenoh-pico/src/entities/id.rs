use std::hash::Hash;

use zenoh_pico_macros::zwrap;
use zenoh_pico_sys::{
    _z_entity_global_id_hash, z_entity_global_id_eid, z_entity_global_id_new,
    z_entity_global_id_zid,
};

use crate::{
    result::{IntoZenohResult, ZenohResult},
    zid::ZId,
    zvalue::ZValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId(u32);

impl From<u32> for EntityId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for EntityId {
    fn into(self) -> u32 {
        self.0
    }
}

impl EntityId {
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[zwrap(base(name = "entity_global_id"), zvalue, zclone)]
pub struct EntityGlobalId;

impl EntityGlobalId {
    pub fn new(zid: &ZId, eid: EntityId) -> ZenohResult<Self> {
        let mut value = EntityGlobalId::uninitialized();
        unsafe {
            z_entity_global_id_new(value.zloan_mut(), zid.zloan(), eid.value()).into_zresult()
        }?;
        Ok(value)
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { z_entity_global_id_eid(self.zloan()) }.into()
    }

    pub fn zid(&self) -> ZId {
        unsafe { z_entity_global_id_zid(self.zloan()) }.into()
    }
}

impl Hash for EntityGlobalId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(unsafe { _z_entity_global_id_hash(self.zloan()) });
    }
}

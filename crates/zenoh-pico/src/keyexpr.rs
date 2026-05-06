use std::{
    fmt::{self, Display},
    str::FromStr,
};

use zenoh_pico_macros::zwrap;
use zenoh_pico_sys::{
    _z_declared_keyexpr_t, z_keyexpr_as_view_string, z_keyexpr_equals, z_keyexpr_from_substr,
    z_keyexpr_from_substr_autocanonize, z_keyexpr_join,
};

use crate::{
    result::{IntoZenohResult, ZenohError, ZenohResult},
    zstring::ZString,
    zvalue::{ZOwn, ZValue, ZView},
};

#[zwrap(base(name = "keyexpr"), zvalue(value_ty = _z_declared_keyexpr_t), zown, zclone)]
pub struct KeyExpr;

impl KeyExpr {
    pub fn new(s: &str) -> ZenohResult<Self> {
        let mut keyexpr = Self::uninitialized();
        keyexpr.with_zowned_mut(|z| unsafe {
            z_keyexpr_from_substr(z, s.as_ptr(), s.len()).into_zresult()
        })?;
        Ok(keyexpr)
    }

    pub fn autocanonize(s: &str) -> ZenohResult<Self> {
        let mut keyexpr = Self::uninitialized();
        keyexpr.with_zowned_mut(|z| unsafe {
            z_keyexpr_from_substr_autocanonize(z, s.as_ptr(), &mut s.len()).into_zresult()
        })?;
        Ok(keyexpr)
    }

    pub fn join_autocanonize(&self, other: &KeyExpr) -> ZenohResult<KeyExpr> {
        let mut keyexpr = Self::uninitialized();
        keyexpr.with_zowned_mut(|z| unsafe {
            z_keyexpr_join(z, self.zloan(), other.zloan()).into_zresult()
        })?;
        Ok(keyexpr)
    }
}

impl Display for KeyExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_view = <ZString as ZView>::ViewValue::default();
        unsafe { z_keyexpr_as_view_string(self.zloan(), &mut string_view).into_zresult() }
            .map_err(|_| fmt::Error)
            .and_then(|_| ZString::from_zview(string_view).fmt(f))
    }
}

impl FromStr for KeyExpr {
    type Err = ZenohError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::autocanonize(s)
    }
}

impl Eq for KeyExpr {}
impl PartialEq for KeyExpr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { z_keyexpr_equals(self.zloan(), other.zloan()) }
    }
}

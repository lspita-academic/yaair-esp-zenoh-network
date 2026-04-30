use std::{
    fmt::{self, Display},
    str::FromStr,
};

use zenoh_pico_core::{
    result::{IntoZenohResult, ZenohError},
    sys::{
        ZP_ENCODING_ZENOH_BYTES, z_encoding_equals, z_encoding_from_substr, z_encoding_to_string,
    },
    zvalue::{ZOwn, ZValue},
};
use zenoh_pico_macros::zwrap;

use crate::zstring::ZString;

#[zwrap(base(name = "encoding"), zvalue, zown)]
pub struct Encoding;

impl Default for Encoding {
    fn default() -> Self {
        Self::from_zowned(unsafe { ZP_ENCODING_ZENOH_BYTES })
    }
}

impl FromStr for Encoding {
    type Err = ZenohError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::uninitialized();
        value
            .inspect_zowned_mut(|z| unsafe {
                z_encoding_from_substr(z, s.as_ptr(), s.len()).into_zresult()
            })
            .map(|_| value)
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = ZString::uninitialized();
        string
            .inspect_zowned_mut(|z| unsafe { z_encoding_to_string(self.zloan(), z).into_zresult() })
            .map_err(|_| fmt::Error)
            .and_then(|_| string.fmt(f))
    }
}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        unsafe { z_encoding_equals(self.zloan(), other.zloan()) }
    }
}

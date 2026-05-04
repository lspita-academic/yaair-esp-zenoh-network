use std::ptr::NonNull;

use ffi_utils::pointer::NonNullExtensions;
use zenoh_pico_macros::zwrap;
use zenoh_pico_sys::{
    z_query_attachment, z_query_encoding, z_query_keyexpr, z_query_parameters, z_query_payload,
};

use crate::{
    keyexpr::KeyExpr,
    message::Encoding,
    zbytes::ZBytes,
    zstring::ZString,
    zvalue::{ZValue, ZView},
};

#[zwrap(base(name = "query", family = "rc"), zvalue, zown)]
pub struct Query;

impl Query {
    pub fn keyexpr(&self) -> &KeyExpr {
        KeyExpr::from_ptr(unsafe { z_query_keyexpr(self.zloan()) })
    }

    pub fn params(&self) -> &ZString {
        let mut view = <ZString as ZView>::ViewValue::default();
        unsafe {
            z_query_parameters(self.zloan(), &mut view);
        }
        ZView::from_zview(view)
    }

    pub fn payload(&self) -> &ZBytes {
        ZBytes::from_ptr(unsafe { z_query_payload(self.zloan()) })
    }

    pub fn encoding(&self) -> &Encoding {
        Encoding::from_ptr(unsafe { z_query_encoding(self.zloan()) })
    }

    pub fn attachment(&self) -> Option<&ZBytes> {
        NonNull::from_ptr(unsafe { z_query_attachment(self.zloan()) })
            .map(|nn| ZBytes::from_ptr(nn.as_ptr()))
    }
}

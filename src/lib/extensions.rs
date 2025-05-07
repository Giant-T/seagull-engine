use std::ffi::{CStr, CString, c_void};

use anyhow::Result;

type GlGetTextureHandleARB = unsafe extern "system" fn(id: u32) -> u64;
type GlMakeTextureResidentARB = unsafe extern "system" fn(handle: u64) -> c_void;
type GlCreateShaderProgramv =
    unsafe extern "system" fn(shader_type: u32, count: i32, code: &CStr) -> u32;

#[derive(Debug)]
pub struct Extensions {
    pub gl_get_texture_handle_arb: GlGetTextureHandleARB,
    pub gl_make_texture_handle_arb: GlMakeTextureResidentARB,
}

impl Extensions {
    pub unsafe fn load_extensions(
        get_proc_address: impl Fn(&CStr) -> *const c_void,
    ) -> Result<Self> {
        let extensions;
        let get_texture_handle_arb_ptr =
            get_proc_address(CString::new("glGetTextureHandleARB")?.as_c_str());
        let make_texture_handle_arb_ptr =
            get_proc_address(CString::new("glMakeTextureHandleResidentARB")?.as_c_str());
        if !get_texture_handle_arb_ptr.is_null() && !make_texture_handle_arb_ptr.is_null() {
            unsafe {
                extensions = Self {
                    gl_get_texture_handle_arb: std::mem::transmute::<_, GlGetTextureHandleARB>(
                        get_texture_handle_arb_ptr,
                    ),
                    gl_make_texture_handle_arb: std::mem::transmute::<_, GlMakeTextureResidentARB>(
                        make_texture_handle_arb_ptr,
                    ),
                };
            }
        } else {
            panic!("An extension function was not available!");
        }

        return Ok(extensions);
    }
}

use std::ffi::{CStr, CString, c_void};

use anyhow::Result;

type GlGetTextureHandleARB = unsafe extern "system" fn(id: u32) -> u64;
type GlMakeTextureHandleResidentARB = unsafe extern "system" fn(handle: u64) -> c_void;
type GlProgramUniform1ui64ARB =
    unsafe extern "system" fn(program: u32, loc: i32, val: u64) -> c_void;

pub struct Extensions {
    pub gl_get_texture_handle_arb: GlGetTextureHandleARB,
    pub gl_make_texture_handle_resident_arb: GlMakeTextureHandleResidentARB,
    pub gl_program_uniform_1ui_arb: GlProgramUniform1ui64ARB,
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
        let program_uniform_1ui_arb =
            get_proc_address(CString::new("glProgramUniform1ui64ARB")?.as_c_str());
        if !get_texture_handle_arb_ptr.is_null()
            && !make_texture_handle_arb_ptr.is_null()
            && !program_uniform_1ui_arb.is_null()
        {
            unsafe {
                extensions = Self {
                    gl_get_texture_handle_arb: std::mem::transmute::<_, GlGetTextureHandleARB>(
                        get_texture_handle_arb_ptr,
                    ),
                    gl_make_texture_handle_resident_arb: std::mem::transmute::<_, GlMakeTextureHandleResidentARB>(
                        make_texture_handle_arb_ptr,
                    ),
                    gl_program_uniform_1ui_arb: std::mem::transmute::<_, GlProgramUniform1ui64ARB>(
                        program_uniform_1ui_arb,
                    ),
                };
            }
        } else {
            panic!("An extension function was not available!");
        }

        return Ok(extensions);
    }
}

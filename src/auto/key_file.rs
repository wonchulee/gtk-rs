// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_50", feature = "dox"))]
use Bytes;
use Char;
use Error;
use GString;
use KeyFileFlags;
use glib_sys;
use std;
use std::mem;
use std::ptr;
use translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct KeyFile(Shared<glib_sys::GKeyFile>);

    match fn {
        ref => |ptr| glib_sys::g_key_file_ref(ptr),
        unref => |ptr| glib_sys::g_key_file_unref(ptr),
        get_type => || glib_sys::g_key_file_get_type(),
    }
}

impl KeyFile {
    pub fn new() -> KeyFile {
        unsafe {
            from_glib_full(glib_sys::g_key_file_new())
        }
    }

    pub fn get_comment(&self, group_name: Option<&str>, key: &str) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_comment(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_double(&self, group_name: &str, key: &str) -> Result<f64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_double(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_double_list(&self, group_name: &str, key: &str) -> Result<Vec<f64>, Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_double_list(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut length, &mut error);
            if error.is_null() { Ok(FromGlibContainer::from_glib_container_num(ret, length as usize)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_groups(&self) -> (Vec<GString>, usize) {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibPtrContainer::from_glib_full(glib_sys::g_key_file_get_groups(self.to_glib_none().0, &mut length));
            (ret, length)
        }
    }

    pub fn get_int64(&self, group_name: &str, key: &str) -> Result<i64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_int64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_integer(&self, group_name: &str, key: &str) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_integer(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_integer_list(&self, group_name: &str, key: &str) -> Result<Vec<i32>, Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_integer_list(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut length, &mut error);
            if error.is_null() { Ok(FromGlibContainer::from_glib_container_num(ret, length as usize)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_keys(&self, group_name: &str) -> Result<(Vec<GString>, usize), Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_keys(self.to_glib_none().0, group_name.to_glib_none().0, &mut length, &mut error);
            if error.is_null() { Ok((FromGlibPtrContainer::from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    pub fn get_locale_for_key(&self, group_name: &str, key: &str, locale: Option<&str>) -> Option<GString> {
        unsafe {
            from_glib_full(glib_sys::g_key_file_get_locale_for_key(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, locale.to_glib_none().0))
        }
    }

    pub fn get_locale_string(&self, group_name: &str, key: &str, locale: Option<&str>) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_locale_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, locale.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_locale_string_list(&self, group_name: &str, key: &str, locale: Option<&str>) -> Result<Vec<GString>, Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_locale_string_list(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, locale.to_glib_none().0, &mut length, &mut error);
            if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(ret, length as usize)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_start_group(&self) -> Option<GString> {
        unsafe {
            from_glib_full(glib_sys::g_key_file_get_start_group(self.to_glib_none().0))
        }
    }

    pub fn get_string(&self, group_name: &str, key: &str) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_string_list(&self, group_name: &str, key: &str) -> Result<Vec<GString>, Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_string_list(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut length, &mut error);
            if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(ret, length as usize)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_uint64(&self, group_name: &str, key: &str) -> Result<u64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_uint64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_value(&self, group_name: &str, key: &str) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = glib_sys::g_key_file_get_value(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(glib_sys::g_key_file_has_group(self.to_glib_none().0, group_name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    pub fn load_from_bytes(&self, bytes: &Bytes, flags: KeyFileFlags) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_load_from_bytes(self.to_glib_none().0, bytes.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn load_from_data(&self, data: &str, flags: KeyFileFlags) -> Result<(), Error> {
        let length = data.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_load_from_data(self.to_glib_none().0, data.to_glib_none().0, length, flags.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn load_from_file<P: AsRef<std::path::Path>>(&self, file: P, flags: KeyFileFlags) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_load_from_file(self.to_glib_none().0, file.as_ref().to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_comment(&self, group_name: Option<&str>, key: Option<&str>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_remove_comment(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_group(&self, group_name: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_remove_group(self.to_glib_none().0, group_name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_key(&self, group_name: &str, key: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_remove_key(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_boolean(&self, group_name: &str, key: &str, value: bool) {
        unsafe {
            glib_sys::g_key_file_set_boolean(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value.to_glib());
        }
    }

    //pub fn set_boolean_list(&self, group_name: &str, key: &str, list: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 1 }) {
    //    unsafe { TODO: call glib_sys:g_key_file_set_boolean_list() }
    //}

    pub fn set_comment(&self, group_name: Option<&str>, key: Option<&str>, comment: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = glib_sys::g_key_file_set_comment(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, comment.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_double(&self, group_name: &str, key: &str, value: f64) {
        unsafe {
            glib_sys::g_key_file_set_double(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_int64(&self, group_name: &str, key: &str, value: i64) {
        unsafe {
            glib_sys::g_key_file_set_int64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_integer(&self, group_name: &str, key: &str, value: i32) {
        unsafe {
            glib_sys::g_key_file_set_integer(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_list_separator(&self, separator: Char) {
        unsafe {
            glib_sys::g_key_file_set_list_separator(self.to_glib_none().0, separator.to_glib());
        }
    }

    pub fn set_locale_string(&self, group_name: &str, key: &str, locale: &str, string: &str) {
        unsafe {
            glib_sys::g_key_file_set_locale_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, locale.to_glib_none().0, string.to_glib_none().0);
        }
    }

    pub fn set_string(&self, group_name: &str, key: &str, string: &str) {
        unsafe {
            glib_sys::g_key_file_set_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, string.to_glib_none().0);
        }
    }

    pub fn set_uint64(&self, group_name: &str, key: &str, value: u64) {
        unsafe {
            glib_sys::g_key_file_set_uint64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_value(&self, group_name: &str, key: &str, value: &str) {
        unsafe {
            glib_sys::g_key_file_set_value(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }
}

impl Default for KeyFile {
    fn default() -> Self {
        Self::new()
    }
}

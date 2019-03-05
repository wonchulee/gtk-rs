// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BindingFlags;
use GString;
use Object;
use gobject_sys;
use std::fmt;
use translate::*;

glib_wrapper! {
    pub struct Binding(Object<gobject_sys::GBinding, BindingClass>);

    match fn {
        get_type => || gobject_sys::g_binding_get_type(),
    }
}

impl Binding {
    pub fn get_flags(&self) -> BindingFlags {
        unsafe {
            from_glib(gobject_sys::g_binding_get_flags(self.to_glib_none().0))
        }
    }

    pub fn get_source(&self) -> Option<Object> {
        unsafe {
            from_glib_none(gobject_sys::g_binding_get_source(self.to_glib_none().0))
        }
    }

    pub fn get_source_property(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gobject_sys::g_binding_get_source_property(self.to_glib_none().0))
        }
    }

    pub fn get_target(&self) -> Option<Object> {
        unsafe {
            from_glib_none(gobject_sys::g_binding_get_target(self.to_glib_none().0))
        }
    }

    pub fn get_target_property(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gobject_sys::g_binding_get_target_property(self.to_glib_none().0))
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gobject_sys::g_binding_unbind(self.to_glib_full());
        }
    }
}

unsafe impl Send for Binding {}
unsafe impl Sync for Binding {}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Binding")
    }
}

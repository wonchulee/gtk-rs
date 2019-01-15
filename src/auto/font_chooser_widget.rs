// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use FontChooser;
use Orientable;
use Widget;
use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct FontChooserWidget(Object<ffi::GtkFontChooserWidget, ffi::GtkFontChooserWidgetClass, FontChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, FontChooser;

    match fn {
        get_type => || ffi::gtk_font_chooser_widget_get_type(),
    }
}

impl FontChooserWidget {
    pub fn new() -> FontChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_chooser_widget_new()).unsafe_cast()
        }
    }
}

impl Default for FontChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FONT_CHOOSER_WIDGET: Option<&FontChooserWidget> = None;

impl fmt::Display for FontChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontChooserWidget")
    }
}

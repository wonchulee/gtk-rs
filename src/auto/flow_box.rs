// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_12", feature = "dox"))]
use Adjustment;
use Buildable;
use Container;
use FlowBoxChild;
use MovementStep;
use Orientable;
use SelectionMode;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FlowBox(Object<ffi::GtkFlowBox, ffi::GtkFlowBoxClass, FlowBoxClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_flow_box_get_type(),
    }
}

impl FlowBox {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn new() -> FlowBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
impl Default for FlowBox {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FLOW_BOX: Option<&FlowBox> = None;

pub trait FlowBoxExt: 'static {
    //#[cfg(any(feature = "v3_18", feature = "dox"))]
    //fn bind_model<'a, P: IsA</*Ignored*/gio::ListModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q, create_widget_func: /*Unknown conversion*//*Unimplemented*/FlowBoxCreateWidgetFunc, user_data_free_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_activate_on_single_click(&self) -> bool;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_child_at_index(&self, idx: i32) -> Option<FlowBoxChild>;

    #[cfg(any(feature = "v3_22_6", feature = "dox"))]
    fn get_child_at_pos(&self, x: i32, y: i32) -> Option<FlowBoxChild>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_column_spacing(&self) -> u32;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_homogeneous(&self) -> bool;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_max_children_per_line(&self) -> u32;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_min_children_per_line(&self) -> u32;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_row_spacing(&self) -> u32;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_selected_children(&self) -> Vec<FlowBoxChild>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_selection_mode(&self) -> SelectionMode;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn insert<P: IsA<Widget>>(&self, widget: &P, position: i32);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn invalidate_filter(&self);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn invalidate_sort(&self);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn select_all(&self);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn select_child<P: IsA<FlowBoxChild>>(&self, child: &P);

    //#[cfg(any(feature = "v3_12", feature = "dox"))]
    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/FlowBoxForeachFunc, data: P);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_activate_on_single_click(&self, single: bool);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_column_spacing(&self, spacing: u32);

    //#[cfg(any(feature = "v3_12", feature = "dox"))]
    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/FlowBoxFilterFunc>>>(&self, filter_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_hadjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_homogeneous(&self, homogeneous: bool);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_max_children_per_line(&self, n_children: u32);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_min_children_per_line(&self, n_children: u32);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_row_spacing(&self, spacing: u32);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_selection_mode(&self, mode: SelectionMode);

    //#[cfg(any(feature = "v3_12", feature = "dox"))]
    //fn set_sort_func<'a, P: Into<Option<&'a /*Unimplemented*/FlowBoxSortFunc>>>(&self, sort_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_vadjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn unselect_all(&self);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn unselect_child<P: IsA<FlowBoxChild>>(&self, child: &P);

    fn get_property_activate_on_single_click(&self) -> bool;

    fn set_property_activate_on_single_click(&self, activate_on_single_click: bool);

    fn get_property_column_spacing(&self) -> u32;

    fn set_property_column_spacing(&self, column_spacing: u32);

    fn get_property_homogeneous(&self) -> bool;

    fn set_property_homogeneous(&self, homogeneous: bool);

    fn get_property_max_children_per_line(&self) -> u32;

    fn set_property_max_children_per_line(&self, max_children_per_line: u32);

    fn get_property_min_children_per_line(&self) -> u32;

    fn set_property_min_children_per_line(&self, min_children_per_line: u32);

    fn get_property_row_spacing(&self) -> u32;

    fn set_property_row_spacing(&self, row_spacing: u32);

    fn get_property_selection_mode(&self) -> SelectionMode;

    fn set_property_selection_mode(&self, selection_mode: SelectionMode);

    fn connect_activate_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_cursor_child(&self);

    fn connect_child_activated<F: Fn(&Self, &FlowBoxChild) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_cursor(&self, step: MovementStep, count: i32) -> bool;

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_all(&self);

    fn connect_selected_children_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toggle_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_cursor_child(&self);

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_unselect_all(&self);

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_children_per_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_children_per_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FlowBox>> FlowBoxExt for O {
    //#[cfg(any(feature = "v3_18", feature = "dox"))]
    //fn bind_model<'a, P: IsA</*Ignored*/gio::ListModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q, create_widget_func: /*Unknown conversion*//*Unimplemented*/FlowBoxCreateWidgetFunc, user_data_free_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_flow_box_bind_model() }
    //}

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_activate_on_single_click(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_index(self.as_ref().to_glib_none().0, idx))
        }
    }

    #[cfg(any(feature = "v3_22_6", feature = "dox"))]
    fn get_child_at_pos(&self, x: i32, y: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_pos(self.as_ref().to_glib_none().0, x, y))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_column_spacing(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_homogeneous(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_max_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_max_children_per_line(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_min_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_min_children_per_line(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_row_spacing(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_selected_children(&self) -> Vec<FlowBoxChild> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_flow_box_get_selected_children(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_selection_mode(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn insert<P: IsA<Widget>>(&self, widget: &P, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_filter(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_sort(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn select_all(&self) {
        unsafe {
            ffi::gtk_flow_box_select_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn select_child<P: IsA<FlowBoxChild>>(&self, child: &P) {
        unsafe {
            ffi::gtk_flow_box_select_child(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v3_12", feature = "dox"))]
    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/FlowBoxForeachFunc, data: P) {
    //    unsafe { TODO: call ffi::gtk_flow_box_selected_foreach() }
    //}

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(self.as_ref().to_glib_none().0, single.to_glib());
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    //#[cfg(any(feature = "v3_12", feature = "dox"))]
    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/FlowBoxFilterFunc>>>(&self, filter_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_flow_box_set_filter_func() }
    //}

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_hadjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(self.as_ref().to_glib_none().0, adjustment.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_flow_box_set_homogeneous(self.as_ref().to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_max_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_max_children_per_line(self.as_ref().to_glib_none().0, n_children);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_min_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_min_children_per_line(self.as_ref().to_glib_none().0, n_children);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_flow_box_set_selection_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    //#[cfg(any(feature = "v3_12", feature = "dox"))]
    //fn set_sort_func<'a, P: Into<Option<&'a /*Unimplemented*/FlowBoxSortFunc>>>(&self, sort_func: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_flow_box_set_sort_func() }
    //}

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_vadjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(self.as_ref().to_glib_none().0, adjustment.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_flow_box_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn unselect_child<P: IsA<FlowBoxChild>>(&self, child: &P) {
        unsafe {
            ffi::gtk_flow_box_unselect_child(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    fn get_property_activate_on_single_click(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"activate-on-single-click\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_activate_on_single_click(&self, activate_on_single_click: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"activate-on-single-click\0".as_ptr() as *const _, Value::from(&activate_on_single_click).to_glib_none().0);
        }
    }

    fn get_property_column_spacing(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"column-spacing\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_column_spacing(&self, column_spacing: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"column-spacing\0".as_ptr() as *const _, Value::from(&column_spacing).to_glib_none().0);
        }
    }

    fn get_property_homogeneous(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"homogeneous\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"homogeneous\0".as_ptr() as *const _, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_property_max_children_per_line(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"max-children-per-line\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_max_children_per_line(&self, max_children_per_line: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"max-children-per-line\0".as_ptr() as *const _, Value::from(&max_children_per_line).to_glib_none().0);
        }
    }

    fn get_property_min_children_per_line(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"min-children-per-line\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_min_children_per_line(&self, min_children_per_line: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"min-children-per-line\0".as_ptr() as *const _, Value::from(&min_children_per_line).to_glib_none().0);
        }
    }

    fn get_property_row_spacing(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"row-spacing\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_row_spacing(&self, row_spacing: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"row-spacing\0".as_ptr() as *const _, Value::from(&row_spacing).to_glib_none().0);
        }
    }

    fn get_property_selection_mode(&self) -> SelectionMode {
        unsafe {
            let mut value = Value::from_type(<SelectionMode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"selection-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_selection_mode(&self, selection_mode: SelectionMode) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"selection-mode\0".as_ptr() as *const _, Value::from(&selection_mode).to_glib_none().0);
        }
    }

    fn connect_activate_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"activate-cursor-child\0".as_ptr() as *const _,
                transmute(activate_cursor_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate_cursor_child(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate-cursor-child", &[]).unwrap() };
    }

    fn connect_child_activated<F: Fn(&Self, &FlowBoxChild) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &FlowBoxChild) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"child-activated\0".as_ptr() as *const _,
                transmute(child_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MovementStep, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"move-cursor\0".as_ptr() as *const _,
                transmute(move_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_cursor(&self, step: MovementStep, count: i32) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-cursor", &[&step, &count]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"select-all\0".as_ptr() as *const _,
                transmute(select_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_select_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("select-all", &[]).unwrap() };
    }

    fn connect_selected_children_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"selected-children-changed\0".as_ptr() as *const _,
                transmute(selected_children_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_toggle_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"toggle-cursor-child\0".as_ptr() as *const _,
                transmute(toggle_cursor_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_toggle_cursor_child(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("toggle-cursor-child", &[]).unwrap() };
    }

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"unselect-all\0".as_ptr() as *const _,
                transmute(unselect_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_unselect_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("unselect-all", &[]).unwrap() };
    }

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::activate-on-single-click\0".as_ptr() as *const _,
                transmute(notify_activate_on_single_click_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::column-spacing\0".as_ptr() as *const _,
                transmute(notify_column_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::homogeneous\0".as_ptr() as *const _,
                transmute(notify_homogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_children_per_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::max-children-per-line\0".as_ptr() as *const _,
                transmute(notify_max_children_per_line_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_min_children_per_line_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::min-children-per-line\0".as_ptr() as *const _,
                transmute(notify_min_children_per_line_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::row-spacing\0".as_ptr() as *const _,
                transmute(notify_row_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-mode\0".as_ptr() as *const _,
                transmute(notify_selection_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_cursor_child_trampoline<P>(this: *mut ffi::GtkFlowBox, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn child_activated_trampoline<P>(this: *mut ffi::GtkFlowBox, child: *mut ffi::GtkFlowBoxChild, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P, &FlowBoxChild) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(child))
}

unsafe extern "C" fn move_cursor_trampoline<P>(this: *mut ffi::GtkFlowBox, step: ffi::GtkMovementStep, count: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<FlowBox> {
    let f: &&(Fn(&P, MovementStep, i32) -> bool + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast(), from_glib(step), count).to_glib()
}

unsafe extern "C" fn select_all_trampoline<P>(this: *mut ffi::GtkFlowBox, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn selected_children_changed_trampoline<P>(this: *mut ffi::GtkFlowBox, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn toggle_cursor_child_trampoline<P>(this: *mut ffi::GtkFlowBox, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn unselect_all_trampoline<P>(this: *mut ffi::GtkFlowBox, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_activate_on_single_click_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_column_spacing_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_homogeneous_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_max_children_per_line_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_min_children_per_line_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_row_spacing_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_mode_trampoline<P>(this: *mut ffi::GtkFlowBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FlowBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FlowBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlowBox")
    }
}

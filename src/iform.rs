use std::mem;

// use crate::gen::{
//     xed_category_enum_t, xed_extension_enum_t, xed_iclass_enum_t, xed_iform_enum_t,
//     xed_iform_info_t, xed_iform_map, xed_isa_set_enum_t,
// };
// include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/names.rs"));
// use crate::names::*;
use self::{
    xed_category_enum_t as category_enum, xed_extension_enum_t as extension_enum,
    xed_iclass_enum_t as iclass_enum, xed_iform_enum_t as iform_enum,
    xed_iform_info_t as iform_info, xed_isa_set_enum_t as isa_set_enum,
};
use crate::gen::{
    xed_category_enum_t, xed_extension_enum_t, xed_iclass_enum_t, xed_iform_enum_t,
    xed_iform_info_t, xed_iform_map, xed_isa_set_enum_t,
};

pub fn iform_map(iform: iform_enum) -> Option<&'static iform_info> {
    unsafe { xed_iform_map(iform).as_ref() }
}

pub fn iform_to_iclass(iform: iform_enum) -> iclass_enum {
    iform_map(iform).map_or(iclass_enum::XED_ICLASS_INVALID, |im| unsafe {
        mem::transmute(im.iclass())
    })
}

pub fn iform_to_category(iform: iform_enum) -> category_enum {
    iform_map(iform).map_or(category_enum::XED_CATEGORY_INVALID, |im| unsafe {
        mem::transmute(im.category())
    })
}

pub fn iform_to_extension(iform: iform_enum) -> extension_enum {
    iform_map(iform).map_or(extension_enum::XED_EXTENSION_INVALID, |im| unsafe {
        mem::transmute(im.extension())
    })
}

pub fn iform_to_isa_set(iform: iform_enum) -> isa_set_enum {
    iform_map(iform).map_or(isa_set_enum::XED_ISA_SET_INVALID, |im| unsafe {
        mem::transmute(im.isa_set())
    })
}

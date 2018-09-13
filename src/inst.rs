use std::mem;

use crate::gen::*;
use crate::iform::*;

/// High-level accessors: xed_inst_t
pub fn inst_iform_enum(inst: &xed_inst_t) -> xed_iform_enum_t {
    unsafe { mem::transmute(i32::from(inst._iform_enum)) }
}

pub fn inst_iclass(inst: &xed_inst_t) -> xed_iclass_enum_t {
    iform_to_iclass(inst_iform_enum(inst))
}

pub fn inst_category(inst: &xed_inst_t) -> xed_category_enum_t {
    iform_to_category(inst_iform_enum(inst))
}

pub fn inst_extension(inst: &xed_inst_t) -> xed_extension_enum_t {
    iform_to_extension(inst_iform_enum(inst))
}

pub fn inst_isa_set(inst: &xed_inst_t) -> xed_isa_set_enum_t {
    iform_to_isa_set(inst_iform_enum(inst))
}

pub fn inst_noperands(inst: &xed_inst_t) -> u8 {
    inst._noperands
}

pub fn inst_get_attribute(inst: &xed_inst_t, attr: xed_attribute_enum_t) -> bool {
    let inst = ref_to_raw_pointer!(inst);
    match unsafe { xed_inst_get_attribute(inst, attr) } {
        1 => true,
        0 => false,
        _ => unreachable!(),
    }
}

pub fn inst_get_attributes(inst: &xed_inst_t) -> xed_attributes_t {
    unsafe { xed_inst_get_attributes(ref_to_raw_pointer!(inst)) }
}

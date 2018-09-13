use std::mem;

use crate::gen::*;
// #[macro_use]
// use crate::utils::ref_to_raw_pointer;

// #[macro_use]
// mod utils;

/// High-level accessors: xed_operand_enum_t

pub fn operand_is_memory_addressing_register(oe: xed_operand_enum_t) -> bool {
    oe == xed_operand_enum_t::XED_OPERAND_BASE0
        || oe == xed_operand_enum_t::XED_OPERAND_INDEX
        || oe == xed_operand_enum_t::XED_OPERAND_SEG0
        || oe == xed_operand_enum_t::XED_OPERAND_BASE1
        || oe == xed_operand_enum_t::XED_OPERAND_SEG1
}

pub fn operand_is_register(oe: xed_operand_enum_t) -> bool {
    use self::xed_operand_enum_t::*;
    oe >= XED_OPERAND_REG0 && oe <= XED_OPERAND_REG8
}

/// High-level accessors: xed_operand_t
pub fn operand_operand_visibility(op: &xed_operand_t) -> xed_operand_visibility_enum_t {
    unsafe { mem::transmute(u32::from(op._operand_visibility)) }
}

pub fn operand_reg(op: &xed_operand_t) -> xed_reg_enum_t {
    match operand_type(op) {
        xed_operand_type_enum_t::XED_OPERAND_TYPE_REG => unsafe { op._u._reg },
        _ => xed_reg_enum_t::XED_REG_INVALID,
    }
}

pub fn operand_rw(op: &xed_operand_t) -> xed_operand_action_enum_t {
    unsafe { mem::transmute(u32::from(op._rw)) }
}

pub fn operand_template_is_register(op: &xed_operand_t) -> bool {
    op._nt != 0 || operand_type(op) == xed_operand_type_enum_t::XED_OPERAND_TYPE_REG
}

pub fn operand_type(op: &xed_operand_t) -> xed_operand_type_enum_t {
    unsafe { mem::transmute(u32::from(op._type)) }
}

pub fn operand_name(op: &xed_operand_t) -> xed_operand_enum_t {
    unsafe { mem::transmute(u32::from(op._name)) }
}

pub fn operand_nonterminal_name(op: &xed_operand_t) -> xed_nonterminal_enum_t {
    if op._nt != 0 {
        unsafe { op._u._nt }
    } else {
        xed_nonterminal_enum_t::XED_NONTERMINAL_INVALID
    }
}

pub fn operand_xtype(op: &xed_operand_t) -> xed_operand_element_xtype_enum_t {
    unsafe { mem::transmute(u32::from(op._xtype)) }
}

pub fn operand_conditional_write(opr: &xed_operand_t) -> bool {
    let cw = unsafe { xed_operand_conditional_write(ref_to_raw_pointer!(opr)) };
    match cw {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

pub fn operand_conditional_read(opr: &xed_operand_t) -> bool {
    let cw = unsafe { xed_operand_conditional_read(ref_to_raw_pointer!(opr)) };
    match cw {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

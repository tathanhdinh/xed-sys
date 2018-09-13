#![feature(crate_visibility_modifier)]

#[macro_use]
mod utils;

mod decoded_inst;
mod gen;
mod general;
mod iform;
mod inst;
mod operand;

use crate::gen::*;

// pub use crate::gen::*;
pub use crate::decoded_inst::*;
pub use crate::general::*;
pub use crate::iform::*;
pub use crate::inst::*;
pub use crate::operand::*;
pub use crate::utils::*;

use self::xed_error_enum_t as error_enum;

/// Printing friendly functions
pub fn error_str(err: error_enum) -> &'static str {
    cstr_to_str!(xed_error_enum_t2str(err))
}

pub fn iclass_str(iclass: xed_iclass_enum_t) -> &'static str {
    cstr_to_str!(xed_iclass_enum_t2str(iclass))
}

pub fn iform_str(iform: xed_iform_enum_t) -> &'static str {
    cstr_to_str!(xed_iform_enum_t2str(iform))
}

pub fn isa_str(isa: xed_isa_set_enum_t) -> &'static str {
    cstr_to_str!(xed_isa_set_enum_t2str(isa))
}

pub fn category_str(cat: xed_category_enum_t) -> &'static str {
    cstr_to_str!(xed_category_enum_t2str(cat))
}

pub fn extension_str(ext: xed_extension_enum_t) -> &'static str {
    cstr_to_str!(xed_extension_enum_t2str(ext))
}

pub fn operand_name_str(opr: xed_operand_enum_t) -> &'static str {
    cstr_to_str!(xed_operand_enum_t2str(opr))
}

pub fn operand_visibility_str(vis: xed_operand_visibility_enum_t) -> &'static str {
    cstr_to_str!(xed_operand_visibility_enum_t2str(vis))
}

pub fn operand_type_str(ot: xed_operand_type_enum_t) -> &'static str {
    cstr_to_str!(xed_operand_type_enum_t2str(ot))
}

pub fn operand_xtype_str(ox: xed_operand_element_xtype_enum_t) -> &'static str {
    cstr_to_str!(xed_operand_element_xtype_enum_t2str(ox))
}

pub fn operand_action_str(oa: xed_operand_action_enum_t) -> &'static str {
    cstr_to_str!(xed_operand_action_enum_t2str(oa))
}

pub fn reg_str(reg: xed_reg_enum_t) -> &'static str {
    cstr_to_str!(xed_reg_enum_t2str(reg))
}

pub fn nonterminal_str(nt: xed_nonterminal_enum_t) -> &'static str {
    cstr_to_str!(xed_nonterminal_enum_t2str(nt))
}

pub fn attribute_str(att: xed_attribute_enum_t) -> &'static str {
    cstr_to_str!(xed_attribute_enum_t2str(att))
}

pub fn attribute_max() -> u32 {
    From::from(unsafe { xed_attribute_max() })
}

pub fn inst_operand(inst: &xed_inst_t, i: u32) -> Option<&'static xed_operand_t> {
    unsafe { xed_inst_operand(ref_to_raw_pointer!(inst), i).as_ref() }
}

// TODO: some tests here
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

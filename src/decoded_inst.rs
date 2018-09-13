use crate::gen::*;

use self::{
    xed_category_enum_t as category_enum, xed_decoded_inst_t as decoded_inst,
    xed_error_enum_t as error_enum, xed_extension_enum_t as extension_enum, xed_inst_t as inst,
    xed_isa_set_enum_t as isa_set_enum, xed_state_t as state,
};
use crate::inst::{inst_category, inst_extension, inst_isa_set, inst_noperands};

pub fn decode(inst: &mut decoded_inst, bytes: &[u8]) -> error_enum {
    unsafe {
        xed_decode(
            ref_to_mut_raw_pointer!(inst),
            bytes.as_ptr(),
            bytes.len() as u32,
        )
    }
}

pub fn decoded_inst_zero_set_mode(inst: &mut decoded_inst, mode: &state) {
    unsafe {
        xed_decoded_inst_zero_set_mode(ref_to_mut_raw_pointer!(inst), ref_to_raw_pointer!(mode))
    }
}

pub fn decoded_inst_inst(inst: &decoded_inst) -> Option<&inst> {
    unsafe { inst._inst.as_ref() }
}

pub fn decoded_inst_valid(inst: &decoded_inst) -> bool {
    decoded_inst_inst(inst).is_some()
}

pub fn decoded_inst_noperands(inst: &decoded_inst) -> Option<u8> {
    decoded_inst_inst(inst).map(|i| inst_noperands(i))
}

pub fn decoded_inst_get_machine_mode_bits(inst: &decoded_inst) -> u8 {
    let mode = inst._operands.smode;
    match mode {
        1 => 32,
        2 => 64,
        _ => 16,
    }
}

pub fn decoded_inst_get_stack_address_mode_bits(inst: &decoded_inst) -> u8 {
    let smode = inst._operands.smode;
    match smode {
        1 => 32,
        2 => 64,
        _ => 16,
    }
}

pub fn decoded_inst_get_category(di: &decoded_inst) -> Option<category_enum> {
    decoded_inst_inst(di).map(|i| inst_category(i))
}

pub fn decoded_inst_get_extenstion(di: &decoded_inst) -> Option<extension_enum> {
    decoded_inst_inst(di).map(|i| inst_extension(i))
}

pub fn decoded_inst_get_isa_set(di: &decoded_inst) -> Option<isa_set_enum> {
    decoded_inst_inst(di).map(|i| inst_isa_set(i))
}

pub fn decoded_inst_get_length(di: &decoded_inst) -> u8 {
    di._decoded_length
}

pub fn decoded_inst_conditionally_writes_registers(di: &decoded_inst) -> bool {
    let ret = unsafe { xed_decoded_inst_conditionally_writes_registers(ref_to_raw_pointer!(di)) };
    match ret {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

pub fn decoded_inst_get_nprefixes(di: &decoded_inst) -> u32 {
    unsafe { xed_decoded_inst_get_nprefixes(ref_to_raw_pointer!(di)) }
}

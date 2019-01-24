use std::fmt;

use crate::gen::*;

pub fn version() -> &'static str {
    cstr_to_str!(xed_get_version())
}

pub fn copyright() -> &'static str {
    cstr_to_str!(xed_get_copyright())
}

pub fn tables_init() {
    unsafe { xed_tables_init() };
}

pub trait FromStr {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: std::marker::Sized;
}

macro_rules! wrap_enum {
    ([$wrapped_enum:ident <= $raw_enum:ident]
    $($wrapped_enum_variant:ident <= $raw_enum_variant:ident;)*
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
        pub enum $wrapped_enum {
            $(
                $wrapped_enum_variant = self::$raw_enum::$raw_enum_variant as isize,
            )*
        }

        impl From<$raw_enum> for $wrapped_enum {
            fn from(other: $raw_enum) -> Self {
                match other {
                    $(
                        self::$raw_enum::$raw_enum_variant => $wrapped_enum::$wrapped_enum_variant,
                    )*
                }
            }
        }

        impl From<$wrapped_enum> for $raw_enum {
            fn from(other: $wrapped_enum) -> Self {
                match other {
                    $(
                        $wrapped_enum::$wrapped_enum_variant => self::$raw_enum::$raw_enum_variant,
                    )*
                }
            }
        }
    }
}

// Early wrapping for xed_machine_mode_enum_t

wrap_enum!(
    [
        MachineMode <= xed_machine_mode_enum_t
    ]
    Invalid <= XED_MACHINE_MODE_INVALID;
    Long64 <= XED_MACHINE_MODE_LONG_64;
    LongCompat32 <= XED_MACHINE_MODE_LONG_COMPAT_32;
    LongCompat16 <= XED_MACHINE_MODE_LONG_COMPAT_16;
    Legacy32 <= XED_MACHINE_MODE_LEGACY_32;
    Legacy16 <= XED_MACHINE_MODE_LEGACY_16;
    Real16 <= XED_MACHINE_MODE_REAL_16;
    Real32 <= XED_MACHINE_MODE_REAL_32;
    Last <= XED_MACHINE_MODE_LAST;
);

impl fmt::Display for MachineMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            cstr_to_str!(self::xed_machine_mode_enum_t2str(From::from(*self)))
        )
    }
}

impl FromStr for MachineMode {
    fn from_str(mm_str: &str) -> Option<MachineMode> {
        let aw = unsafe { self::str2xed_machine_mode_enum_t(str_to_cstr!(mm_str)) };
        match aw {
            self::xed_machine_mode_enum_t::XED_MACHINE_MODE_INVALID => None,
            _ => Some(From::from(aw)),
        }
    }
}

// Early wrapping for xed_address_width_enum_t

wrap_enum!(
    [
        AddressWidth <= xed_address_width_enum_t
    ]
    Invalid <= XED_ADDRESS_WIDTH_INVALID;
    Bit16 <= XED_ADDRESS_WIDTH_16b;
    Bit32 <= XED_ADDRESS_WIDTH_32b;
    Bit64 <= XED_ADDRESS_WIDTH_64b;
    Last <= XED_ADDRESS_WIDTH_LAST;
);

impl fmt::Display for AddressWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            cstr_to_str!(self::xed_address_width_enum_t2str(From::from(*self)))
        )
    }
}

impl FromStr for AddressWidth {
    fn from_str(aw_str: &str) -> Option<AddressWidth> {
        let aw = unsafe { self::str2xed_address_width_enum_t(str_to_cstr!(aw_str)) };
        match aw {
            self::xed_address_width_enum_t::XED_ADDRESS_WIDTH_INVALID => None,
            _ => Some(From::from(aw)),
        }
    }
}

// Early wrapping for xed_attribute_enum_t

wrap_enum!(
    [
        Attribute <= xed_attribute_enum_t
    ]
    Invalid <= XED_ATTRIBUTE_INVALID;
    AmdOnly <= XED_ATTRIBUTE_AMDONLY;
    AttOperandOrderException <= XED_ATTRIBUTE_ATT_OPERAND_ORDER_EXCEPTION;
    BroadcastEnabled <= XED_ATTRIBUTE_BROADCAST_ENABLED;
    ByteOp <= XED_ATTRIBUTE_BYTEOP;
    Disp8EighthMem <= XED_ATTRIBUTE_DISP8_EIGHTHMEM;
    Disp8Full <= XED_ATTRIBUTE_DISP8_FULL;
    Disp8FullMem <= XED_ATTRIBUTE_DISP8_FULLMEM;
    Disp8GprReader <= XED_ATTRIBUTE_DISP8_GPR_READER;
    Disp8GprReaderByte <= XED_ATTRIBUTE_DISP8_GPR_READER_BYTE;
    Disp8GprReaderWord <= XED_ATTRIBUTE_DISP8_GPR_READER_WORD;
    Disp8GprWriterLdopD <= XED_ATTRIBUTE_DISP8_GPR_WRITER_LDOP_D;
    Disp8GprWriterLdopQ <= XED_ATTRIBUTE_DISP8_GPR_WRITER_LDOP_Q;
    Disp8GprWriterStore <= XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE;
    Disp8GprWriterStoreByte <= XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE_BYTE;
    Disp8GprWriterStoreWord <= XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE_WORD;
    Disp8Gscat <= XED_ATTRIBUTE_DISP8_GSCAT;
    Disp8Half <= XED_ATTRIBUTE_DISP8_HALF;
    Disp8HalfMem <= XED_ATTRIBUTE_DISP8_HALFMEM;
    Disp8Mem128 <= XED_ATTRIBUTE_DISP8_MEM128;
    Disp8MovDdup <= XED_ATTRIBUTE_DISP8_MOVDDUP;
    Disp8QuarterMem <= XED_ATTRIBUTE_DISP8_QUARTERMEM;
    Disp8Scalar <= XED_ATTRIBUTE_DISP8_SCALAR;
    Disp8Tuple1 <= XED_ATTRIBUTE_DISP8_TUPLE1;
    Disp8Tuple1_4X <= XED_ATTRIBUTE_DISP8_TUPLE1_4X;
    Disp8Tuple1Byte <= XED_ATTRIBUTE_DISP8_TUPLE1_BYTE;
    Disp8Tuple1Word <= XED_ATTRIBUTE_DISP8_TUPLE1_WORD;
    Disp8Tuple2 <= XED_ATTRIBUTE_DISP8_TUPLE2;
    Disp8Tuple4 <= XED_ATTRIBUTE_DISP8_TUPLE4;
    Disp8Tuple8 <= XED_ATTRIBUTE_DISP8_TUPLE8;
    DoubleWideMemop <= XED_ATTRIBUTE_DOUBLE_WIDE_MEMOP;
    DoubleWideOutput <= XED_ATTRIBUTE_DOUBLE_WIDE_OUTPUT;
    DwordIndices <= XED_ATTRIBUTE_DWORD_INDICES;
    ElementSizeD <= XED_ATTRIBUTE_ELEMENT_SIZE_D;
    ElementSizeQ <= XED_ATTRIBUTE_ELEMENT_SIZE_Q;
    ExceptionBr <= XED_ATTRIBUTE_EXCEPTION_BR;
    FarXfer <= XED_ATTRIBUTE_FAR_XFER;
    FixedBase0 <= XED_ATTRIBUTE_FIXED_BASE0;
    FixedBase1 <= XED_ATTRIBUTE_FIXED_BASE1;
    Gather <= XED_ATTRIBUTE_GATHER;
    HalfWideOutput <= XED_ATTRIBUTE_HALF_WIDE_OUTPUT;
    HleAcqAble <= XED_ATTRIBUTE_HLE_ACQ_ABLE;
    HleRelAble <= XED_ATTRIBUTE_HLE_REL_ABLE;
    IgnoresOSFXSR <= XED_ATTRIBUTE_IGNORES_OSFXSR;
    ImplicitOne <= XED_ATTRIBUTE_IMPLICIT_ONE;
    IndexRegIsPointer <= XED_ATTRIBUTE_INDEX_REG_IS_POINTER;
    IndirectBranch <= XED_ATTRIBUTE_INDIRECT_BRANCH;
    Kmask <= XED_ATTRIBUTE_KMASK;
    Lockable <= XED_ATTRIBUTE_LOCKABLE;
    Locked <= XED_ATTRIBUTE_LOCKED;
    Maskop <= XED_ATTRIBUTE_MASKOP;
    MaskopEvex <= XED_ATTRIBUTE_MASKOP_EVEX;
    MaskAsControl <= XED_ATTRIBUTE_MASK_AS_CONTROL;
    MaskVariableMemop <= XED_ATTRIBUTE_MASK_VARIABLE_MEMOP;
    MemoryFaultSuppression <= XED_ATTRIBUTE_MEMORY_FAULT_SUPPRESSION;
    MmxExcept <= XED_ATTRIBUTE_MMX_EXCEPT;
    MpxPrefixAble <= XED_ATTRIBUTE_MPX_PREFIX_ABLE;
    Multisource4 <= XED_ATTRIBUTE_MULTISOURCE4;
    Mxcsr <= XED_ATTRIBUTE_MXCSR;
    MxcsrRd <= XED_ATTRIBUTE_MXCSR_RD;
    Nontemporal <= XED_ATTRIBUTE_NONTEMPORAL;
    Nop <= XED_ATTRIBUTE_NOP;
    NotSx <= XED_ATTRIBUTE_NOTSX;
    NotSxCond <= XED_ATTRIBUTE_NOTSX_COND;
    RipRel <= XED_ATTRIBUTE_NO_RIP_REL;
    Prefetch <= XED_ATTRIBUTE_PREFETCH;
    ProtectedMode <= XED_ATTRIBUTE_PROTECTED_MODE;
    QwordIndices <= XED_ATTRIBUTE_QWORD_INDICES;
    Rep <= XED_ATTRIBUTE_REP;
    RequiresAlignment <= XED_ATTRIBUTE_REQUIRES_ALIGNMENT;
    Ring0 <= XED_ATTRIBUTE_RING0;
    Scalable <= XED_ATTRIBUTE_SCALABLE;
    Scatter <= XED_ATTRIBUTE_SCATTER;
    SimdScalar <= XED_ATTRIBUTE_SIMD_SCALAR;
    Skiplow32 <= XED_ATTRIBUTE_SKIPLOW32;
    Skiplow64 <= XED_ATTRIBUTE_SKIPLOW64;
    SpecialAgenRequired <= XED_ATTRIBUTE_SPECIAL_AGEN_REQUIRED;
    StackPop0 <= XED_ATTRIBUTE_STACKPOP0;
    StackPop1 <= XED_ATTRIBUTE_STACKPOP1;
    StackPush0 <= XED_ATTRIBUTE_STACKPUSH0;
    StackPush1 <= XED_ATTRIBUTE_STACKPUSH1;
    X87Control <= XED_ATTRIBUTE_X87_CONTROL;
    X87MmxStateCW <= XED_ATTRIBUTE_X87_MMX_STATE_CW;
    X87MmxStateR <= XED_ATTRIBUTE_X87_MMX_STATE_R;
    X87MmxStateW <= XED_ATTRIBUTE_X87_MMX_STATE_W;
    X87Nowait <= XED_ATTRIBUTE_X87_NOWAIT;
    XmmStateCW <= XED_ATTRIBUTE_XMM_STATE_CW;
    XmmStateR <= XED_ATTRIBUTE_XMM_STATE_R;
    XmmStateW <= XED_ATTRIBUTE_XMM_STATE_W;
    Last <= XED_ATTRIBUTE_LAST;
);

// Early wrapping for xed_iclass_enum_t

wrap_enum!(
    [
        IClass <= xed_iclass_enum_t
    ]
    Invalid <= XED_ICLASS_INVALID;
    Aaa <= XED_ICLASS_AAA;
    Aad <= XED_ICLASS_AAD;
    Aam <= XED_ICLASS_AAM;
    Aas <= XED_ICLASS_AAS;
    Adc <= XED_ICLASS_ADC;
    AdcX <= XED_ICLASS_ADCX;
    AdcLock <= XED_ICLASS_ADC_LOCK;
    Add <= XED_ICLASS_ADD;
    AddPD <= XED_ICLASS_ADDPD;
    AddPS <= XED_ICLASS_ADDPS;
    AddSD <= XED_ICLASS_ADDSD;
    AddSS <= XED_ICLASS_ADDSS;
    AddSubPD <= XED_ICLASS_ADDSUBPD;
    AddSubPS <= XED_ICLASS_ADDSUBPS;
    AddLock <= XED_ICLASS_ADD_LOCK;
    AdoX <= XED_ICLASS_ADOX;
    AesDec <= XED_ICLASS_AESDEC;
    AesDecLast <= XED_ICLASS_AESDECLAST;
    AesEnc <= XED_ICLASS_AESENC;
    AesImc <= XED_ICLASS_AESIMC;
    AesKeygenAssist <= XED_ICLASS_AESKEYGENASSIST;
    And <= XED_ICLASS_AND;
    AndN <= XED_ICLASS_ANDN;
    AndNPD <= XED_ICLASS_ANDNPD,
    AndNPS <= XED_ICLASS_ANDNPS,
    AndPD <= XED_ICLASS_ANDPD,
    AndPS <= XED_ICLASS_ANDPS,
    AndLock <= XED_ICLASS_AND_LOCK,
);

// pub fn last_address_width() -> AddressWidth {
//     From::from(unsafe { self::xed_address_width_enum_t_last() })
// }

// pub fn address_width_str(aw: AddressWidth) -> &'static str {
//     cstr_to_str!(self::xed_address_width_enum_t2str(From::from(aw)))
// }

// pub fn address_width_from_str(aw_str: &str) -> AddressWidth {
//     From::from(unsafe { self::str2xed_address_width_enum_t(str_to_cstr!(aw_str)) })
// }

// use self::xed_machine_mode_enum_t::*;
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
// pub enum MachineMode {
//     Invalid = XED_MACHINE_MODE_INVALID as isize,
//     Long64 = XED_MACHINE_MODE_LONG_64 as isize,
//     LongCompat32 = XED_MACHINE_MODE_LONG_COMPAT_32 as isize,
//     LongCompat16 = XED_MACHINE_MODE_LONG_COMPAT_16 as isize,
//     Legacy32 = XED_MACHINE_MODE_LEGACY_32 as isize,
//     Legacy16 = XED_MACHINE_MODE_LEGACY_16 as isize,
//     Real16 = XED_MACHINE_MODE_REAL_16 as isize,
//     Last = XED_MACHINE_MODE_LAST as isize,
// }

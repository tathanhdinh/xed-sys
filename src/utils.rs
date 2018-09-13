#[macro_export]
macro_rules! ref_to_raw_pointer {
    ($ref_v:expr) => {
        $ref_v as *const _
    };
}

#[macro_export]
macro_rules! ref_to_mut_raw_pointer {
    ($ref_v:expr) => {
        $ref_v as *mut _
    };
}

#[macro_export]
macro_rules! raw_pointer_to_ref {
    ($raw_p:expr) => {
        unsafe { &*$raw_p }
    };
}

#[macro_export]
macro_rules! cstr_to_str {
    ($raw_p:expr) => {
        unsafe { std::ffi::CStr::from_ptr($raw_p).to_str().unwrap() }
    };
}

#[macro_export]
macro_rules! str_to_cstr {
    ($str_slice:expr) => {
        $str_slice.as_ptr() as *const i8
    };
}

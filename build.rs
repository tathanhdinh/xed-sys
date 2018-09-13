extern crate bindgen;

use std::{env, fs, path, process};

const XED_C_DIR: &'static str = "xed-c";
const XED_MBUILD_DIR: &'static str = "mbuild";
const XED_C_LIB: &'static str = "xed";

fn build_library(_out_dir_path: &path::Path, xed_c_path: &path::Path) {
    // use it or not?
    let _xed_mbuild_path = {
        let path = path::PathBuf::from(XED_MBUILD_DIR);
        if !path.exists() {
            panic!("Xed mbuild directory not found");
        }
        path
    };

    let build_status = process::Command::new("python")
        .arg("mfile.py")
        // .arg("--shared")
        .arg("--static")
        .arg("--extra-flags=-fPIC")
        .arg("--opt=3")
        .current_dir(xed_c_path)
        .status()
        .expect("failed to execute Xed build script");

    if !build_status.success() {
        panic!("Xed building doesn't succeed")
    }
}

fn generate_binding(out_dir_path: &path::Path, xed_c_path: &path::Path) {
    let xed_public_c_include_path = {
        let path = xed_c_path.join("include").join("public").join("xed");
        if !path.exists() {
            panic!("Xed C public include folder not found")
        }
        path
    };

    let xed_c_header_path = {
        let path = xed_public_c_include_path.join("xed-interface.h");
        if !path.exists() {
            panic!("Xed C header not found")
        }
        path
    };

    let xed_c_obj_path = {
        let path = xed_c_path.join("obj");
        if !path.exists() {
            panic!("Xed build folder not found")
        }
        path
    };

    let xed_binder = bindgen::Builder::default()
        .header(xed_c_header_path.to_string_lossy())
        .clang_arg(format!(
            "-I{}",
            fs::canonicalize(&xed_public_c_include_path)
                .unwrap()
                .to_string_lossy(),
        ))
        .clang_arg(format!(
            "-I{}",
            fs::canonicalize(&xed_c_obj_path).unwrap().to_string_lossy()
        ))
        .prepend_enum_name(false)
        // .derive_partialeq(true)
        .derive_partialord(true)
        // .derive_ord(true)
        // .derive_eq(true)
        // .no_partialeq("xed_flag_enum_s")
        .no_partialord(
            "xed_union16_t__bindgen_ty_1|\
             xed_union32_t__bindgen_ty_1|\
             xed_union32_t__bindgen_ty_2|\
             xed_union64_t__bindgen_ty_1|\
             xed_union64_t__bindgen_ty_2|\
             xed_union64_t__bindgen_ty_3|\
             __fsid_t|\
             xed_iform_info_s|\
             xed_attributes_t|\
             xed_inst_s|\
             xed_flag_set_s__bindgen_ty_1|\
             xed_flag_enum_s|\
             xed_operand_storage_s|\
             xed_chip_features_t|\
             xed_state_s|\
             xed_encoder_iforms_s|\
             xed_enc_displacement_t|\
             xed_memop_t|\
             xed_encoder_operand_t__bindgen_ty_1__bindgen_ty_1|\
             xed_encoder_prefixes_t__bindgen_ty_1|\
             xed_format_options_t|\
             xed_print_info_t|\
             xed_cpuid_rec_t",
        )
        .rustified_enum("*")
        .generate()
        .expect("Unable to generate Xed bindings");

    xed_binder
        .write_to_file(out_dir_path.join("bindings.rs"))
        .expect("Unable to write Xed bindings");
}

fn main() {
    let out_dir_path = {
        let out_dir =
            env::var("OUT_DIR").expect("Unable to get value of OUT_DIR environment variable");
        path::PathBuf::from(out_dir)
    };

    let xed_c_path = {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = path::PathBuf::from(&crate_dir).join(XED_C_DIR);
        if !path.exists() {
            panic!("Xed source directory not found");
        }
        path
    };

    println!("cargo:rerun-if-changed={}", XED_C_DIR);

    build_library(&out_dir_path, &xed_c_path);
    generate_binding(&out_dir_path, &xed_c_path);

    println!("cargo:rustc-link-lib={}={}", "static", XED_C_LIB);
    println!(
        "cargo:rustc-link-search={}={}/obj",
        "native",
        xed_c_path.to_string_lossy()
    );
}

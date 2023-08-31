use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe_ffi)
    generate!("gr::make_top_block")
    generate!("gr::top_block")
    generate!("gr::hier_block2")
    extern_cpp_opaque_type!("gr::basic_block", crate::ffi2::basic_block)
}

#[cxx::bridge(namespace = "gr::blocks")]
mod ffi2 {
    unsafe extern "C++" {
        include!("wrapper.h");

        // Blocks are abstract and break autocxx
        #[namespace = "gr"]
        type basic_block;

        type vector_source_s;
        fn make_vector_source_s(data: &CxxVector<i16>, repeat: bool) -> SharedPtr<vector_source_s>; // autocxx has problems generating this function
        pub fn cast_vector_source_s(arg: SharedPtr<vector_source_s>) -> SharedPtr<basic_block>;

        type file_sink;
        unsafe fn make_file_sink(filename: *const c_char) -> SharedPtr<file_sink>;
        pub fn cast_file_sink(arg: SharedPtr<file_sink>) -> SharedPtr<basic_block>;
    }
}

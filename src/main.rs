use cxx::{let_cxx_string, CxxVector};
use std::ffi::CString;
use std::{ops::Deref, pin::Pin};

use crate::ffi::gr::{hier_block2, top_block};

// Split in a separate file to avoid rerunning slow build-script
include!("ffi.rs");

fn main() {
    let_cxx_string!(s = "Top title");
    let tb = ffi::gr::make_top_block(&s, false);
    let hb: *const hier_block2 = tb.as_ref().unwrap().as_ref();
    let hb = unsafe { Pin::new_unchecked(hb.cast_mut().as_mut().unwrap()) };
    let tb: *const top_block = tb.deref();
    let mut tb = unsafe { Pin::new_unchecked(tb.cast_mut().as_mut().unwrap()) };

    let mut v = CxxVector::new();
    let mut v = v.as_mut().unwrap();
    v.as_mut().push(4);
    v.as_mut().push(2);
    let src = ffi2::make_vector_source_s(&v, false);
    let src = ffi2::cast_vector_source_s(src);
    let filename = CString::new("output.bin").unwrap();
    let sink = unsafe { ffi2::make_file_sink(filename.as_ptr()) };
    let sink = ffi2::cast_file_sink(sink);
    hb.connect1(src, c_int(0), sink, c_int(0));
    tb.as_mut().run(c_int(1000));
    tb.wait();
}

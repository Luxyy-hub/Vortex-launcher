mod io_buf;
pub use io_buf::IoBuf;

mod io_buf_mut;
pub use io_buf_mut::IoBufMut;

mod slice;
pub use slice::Slice;

pub(crate) fn deref(buf: &impl IoBuf) -> &[u8] {
    // implemented correctly.
    unsafe { std::slice::from_raw_parts(buf.stable_ptr(), buf.bytes_init()) }
}

pub(crate) fn deref_mut(buf: &mut impl IoBufMut) -> &mut [u8] {
    // implemented correct.
    unsafe { std::slice::from_raw_parts_mut(buf.stable_mut_ptr(), buf.bytes_init()) }
}

// not important now since the launcher does not contain io_buf but it is better to put it first

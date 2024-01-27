//! Compatibility module for C platform-specific types. Use [`core::ffi`] instead.

#![stable(feature = "raw_os", since = "1.1.0")]

#[cfg(test)]
mod tests;

macro_rules! alias_core_ffi {
    ($($t:ident)*) => {$(
        #[stable(feature = "raw_os", since = "1.1.0")]
        pub type $t = core::ffi::$t;
    )*}
}

alias_core_ffi! {
    c_char c_schar c_uchar
    c_short c_ushort
    c_int c_uint
    c_long c_ulong
    c_longlong c_ulonglong
    c_float
    c_double
    c_void
}

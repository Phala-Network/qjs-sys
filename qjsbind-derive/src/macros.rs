macro_rules! syn_bail {
    ($span:expr, $fmt:literal $(, $($arg:tt)*)?) => {
        return Err(syn::Error::new_spanned(&$span, format_args!($fmt $(, $($arg)*)?)));
    };
}

macro_rules! ensure_none {
    ($option:expr, $span:expr, $fmt:literal $(, $($arg:tt)*)?) => {
        if $option.is_some() {
            syn_bail!($span, $fmt $(, $($arg)*)?);
        }
    };
}

macro_rules! opt_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        }
    };
}

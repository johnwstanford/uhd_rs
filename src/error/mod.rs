
pub fn err_to_string(e: isize) -> &'static str {
    match e {
        0 => "None",
        11 => "uhd::key_error",
        20 => "uhd::not_implemented",
        41 => "uhd::lookup_error",
        43 => "uhd::value",
        100 => "Unknown error at the C level",
        _ => "Unknown error at the Rust level",
    }

}
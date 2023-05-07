#![no_main]

use libfuzzer_sys::fuzz_target;

use stylua_lib::Range;
use stylua_lib::Config;
use stylua_lib::OutputVerification;

use stylua_lib::format_code;

fuzz_target!(|data: &[u8]| {
    let lua = String::from_utf8_lossy(&data);

    let _ = format_code(
        &lua,
        Config::default(),
        None,
        OutputVerification::Full,
    );
});
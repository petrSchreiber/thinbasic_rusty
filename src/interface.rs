/*

    Module functions, basically internal function wrappers doing the parsing.

*/

use thinbasic;
use internal;

pub extern fn rusty_sum_two_longs() -> i32 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_i32();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_i32();
    if parens_present { thinbasic::core::check_close_parens(); }

    if thinbasic::core::error_free() {
        return internal::sum_two_longs(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::Error::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

pub extern fn rusty_sum_two_dwords() -> u32 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_u32();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_u32();    
    if parens_present { thinbasic::core::check_close_parens(); }
    
    if thinbasic::core::error_free() {
        return internal::sum_two_dwords(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::Error::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

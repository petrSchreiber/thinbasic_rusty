/*

    Module functions, basically internal function wrappers doing the parsing.

*/

use thinbasic;
use internal;

pub extern fn rusty_sum_two_integers() -> i16 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_i16();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_i16();
    if parens_present { thinbasic::core::check_close_parens(); }

    if thinbasic::core::error_free() {
        return internal::sum_two_integers(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

pub extern fn rusty_sum_two_longs() -> i32 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_i32();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_i32();
    if parens_present { thinbasic::core::check_close_parens(); }

    if thinbasic::core::error_free() {
        return internal::sum_two_longs(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

pub extern fn rusty_sum_two_quads() -> i64 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_i64();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_i64();
    if parens_present { thinbasic::core::check_close_parens(); }

    if thinbasic::core::error_free() {
        return internal::sum_two_quads(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

pub extern fn rusty_sum_two_bytes() -> u8 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_u8();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_u8();    
    if parens_present { thinbasic::core::check_close_parens(); }
    
    if thinbasic::core::error_free() {
        return internal::sum_two_bytes(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

pub extern fn rusty_sum_two_words() -> u16 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_u16();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_u16();    
    if parens_present { thinbasic::core::check_close_parens(); }
    
    if thinbasic::core::error_free() {
        return internal::sum_two_words(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
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

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0
}

pub extern fn rusty_sum_two_singles() -> f32 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_f32();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_f32();    
    if parens_present { thinbasic::core::check_close_parens(); }
    
    if thinbasic::core::error_free() {
        return internal::sum_two_singles(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0.0
}

pub extern fn rusty_sum_two_doubles() -> f64 {

    let parens_present = thinbasic::core::check_open_parens_optional();
    let parsed_a = thinbasic::core::parse_f64();
    thinbasic::core::check_comma();
    let parsed_b = thinbasic::core::parse_f64();    
    if parens_present { thinbasic::core::check_close_parens(); }
    
    if thinbasic::core::error_free() {
        return internal::sum_two_doubles(parsed_a, parsed_b)
    }

    thinbasic::core::raise_runtime_error(thinbasic::core::RunTimeError::ModuleSpecific, "Something went wrong during the parsing process");
    0.0
}

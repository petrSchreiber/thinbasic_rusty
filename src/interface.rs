/*

    Module functions, basically internal function wrappers doing the parsing.

*/

use thinbasic;
use internal;

pub extern fn rusty_sum_two_longs() -> i32 {

    let parsed_a: i32 = 0;
    let parsed_b: i32 = 0;

    thinbasic::core::parse_i32(&parsed_a);
    thinbasic::core::check_comma_mandatory();
    thinbasic::core::parse_i32(&parsed_b);

	internal::sum_two_longs(parsed_a, parsed_b)
}

pub extern fn rusty_sum_two_dwords() -> u32 {

    let parsed_a: u32 = 0;
    let parsed_b: u32 = 0;

    thinbasic::core::parse_u32(&parsed_a);
    thinbasic::core::check_comma_mandatory();
    thinbasic::core::parse_u32(&parsed_b);

	internal::sum_two_dwords(parsed_a, parsed_b)
}

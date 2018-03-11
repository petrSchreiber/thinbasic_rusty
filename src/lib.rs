/*

    Main code module, where all the functionality gets exposed to thinBasic.

*/

mod thinbasic;	// declaration of thinBasic Core functions
mod internal;	// internal implementation
mod interface;	// thinBasic interface

// LoadLocalSymbols is the function thinBASIC uses to import the functions and other elements
#[no_mangle]
pub extern fn LoadLocalSymbols() -> i32
{
	// exposes the function to thinBasic - please note the return data type is automatically detected
	thinbasic::core::add_function("rusty_sum_two_longs", interface::rusty_sum_two_longs);
	thinbasic::core::add_function("rusty_sum_two_dwords", interface::rusty_sum_two_dwords);

	0
}

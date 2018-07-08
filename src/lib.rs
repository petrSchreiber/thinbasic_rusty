/*

    Main code module, where all the functionality gets exposed to thinBasic.

*/

extern crate thinbasic;	// declaration of thinBasic Core functions
mod internal;	// internal implementation
mod interface;	// thinBasic interface

// LoadLocalSymbols is the function thinBASIC uses to import the functions and other elements
#[no_mangle]
pub extern fn LoadLocalSymbols() -> i32
{
	// exposes the function to thinBasic - please note the return data type is automatically detected
	thinbasic::core::add_function("rusty_sum_two_integers", interface::rusty_sum_two_integers, thinbasic::core::ReturnType::I16);
	thinbasic::core::add_function("rusty_sum_two_longs"   , interface::rusty_sum_two_longs   , thinbasic::core::ReturnType::I32);
	thinbasic::core::add_function("rusty_sum_two_quads"   , interface::rusty_sum_two_quads   , thinbasic::core::ReturnType::I64);

	thinbasic::core::add_function("rusty_sum_two_bytes"   , interface::rusty_sum_two_bytes   , thinbasic::core::ReturnType::U8);
	thinbasic::core::add_function("rusty_sum_two_words"   , interface::rusty_sum_two_words   , thinbasic::core::ReturnType::U16);
	thinbasic::core::add_function("rusty_sum_two_dwords"  , interface::rusty_sum_two_dwords  , thinbasic::core::ReturnType::U32);

	thinbasic::core::add_function("rusty_sum_two_singles" , interface::rusty_sum_two_singles , thinbasic::core::ReturnType::F32);
	thinbasic::core::add_function("rusty_sum_two_doubles" , interface::rusty_sum_two_doubles , thinbasic::core::ReturnType::F64);

	thinbasic::core::add_function("rusty_to_upper_case" , interface::rusty_to_upper_case, thinbasic::core::ReturnType::TBSTR);

	thinbasic::core::add_string_equate("$RUSTY_STRING_EQUATE", "Hello ThinBASIC");

	0
}

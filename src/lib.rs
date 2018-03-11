mod thinbasic;	// declaration of thinBasic Core functions
mod internal;	// internal implementation
mod interface;	// thinBasic interface

#[no_mangle]
pub extern fn LoadLocalSymbols() -> i32
{
	// exposes the function to thinBasic - please not the return data type is automatically detected
	thinbasic::core::add_function("rusty_sum_two_longs", interface::rusty_sum_two_longs);
	thinbasic::core::add_function("rusty_sum_two_dwords", interface::rusty_sum_two_dwords);

	0
}

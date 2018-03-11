pub mod core
{
	extern crate libloading;	// For thinCore
	extern crate winrt;			// For BStr

	#[allow(dead_code)]
	pub enum ReturnType
	{
		U8  =  1,
  		U16 =  3,
  		U32 =  4,		
  		I16 =  2,
  		I32 =  5,
  		I64 =  6,
  		F32 =  7,
  		F64 =  8
	}

	/*

	 Library setup

	*/

	// TODO: Signature
	pub fn add_function<T>(symbol_name: &str, function_ptr: extern fn() -> T) -> i32
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();

			let thinbasic_loadsymbol: libloading::Symbol<unsafe extern fn(symbol_name: winrt::BStr, return_type: i32, function_ptr: extern fn() -> T, force_overwrite: i32) -> i32> = lib.get(b"thinBasic_LoadSymbol").unwrap();

			thinbasic_loadsymbol(winrt::BStr::from(symbol_name), ReturnType::I32 as i32, function_ptr, 1)
		}
	}
	

	/*

	 Parsing

	*/

	pub fn parse_i32(num: *const i32)
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_parselong: libloading::Symbol<unsafe extern fn(*const i32)> = lib.get(b"thinBasic_ParseLong").unwrap();
			thinbasic_parselong(num);
		}
	}

	pub fn parse_u32(num: *const u32)
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_parsedword: libloading::Symbol<unsafe extern fn(*const u32)> = lib.get(b"thinBasic_ParseDWord").unwrap();
			thinbasic_parsedword(num);
		}
	}

	pub fn check_comma_mandatory() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkcomma_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckComma_Mandatory").unwrap();
			let result = thinbasic_checkcomma_mandatory();

			return if result == 0 { false } else { true }
		}
	}
}

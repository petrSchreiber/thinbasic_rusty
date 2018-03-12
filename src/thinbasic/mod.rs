/*

    Binding to thinCore - thinBasic internal engine.
    Functionality present here is used to interact with thinBasic.

*/

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

	#[allow(dead_code)]
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

	#[allow(dead_code)]
	pub fn parse_i32() -> i32
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_parselong: libloading::Symbol<unsafe extern fn(*const i32)> = lib.get(b"thinBasic_ParseLong").unwrap();
			let num: i32 = 0;
			thinbasic_parselong(&num);

			num
		}
	}

	#[allow(dead_code)]
	pub fn parse_u32() -> u32
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_parsedword: libloading::Symbol<unsafe extern fn(*const u32)> = lib.get(b"thinBasic_ParseDWord").unwrap();
			let num: u32 = 0;
			thinbasic_parsedword(&num);

			num
		}
	}

	#[allow(dead_code)]
	pub fn check_comma() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkcomma_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckComma_Mandatory").unwrap();
			let result = thinbasic_checkcomma_mandatory();

			return if result == 0 { false } else { true }
		}
	}

	#[allow(dead_code)]
	pub fn check_comma_optional() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkcomma_optional: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckComma_Optional").unwrap();
			let result = thinbasic_checkcomma_optional();

			return if result == 0 { false } else { true }
		}
	}

	#[allow(dead_code)]
	pub fn check_open_parens() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkopenparens_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckOpenParens_Mandatory").unwrap();
			let result = thinbasic_checkopenparens_mandatory();

			return if result == 0 { false } else { true }
		}
	}

	#[allow(dead_code)]
	pub fn check_open_parens_optional() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkopenparens_optional: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckOpenParens_Optional").unwrap();
			let result = thinbasic_checkopenparens_optional();

			return if result == 0 { false } else { true }
		}
	}

	#[allow(dead_code)]
	pub fn check_close_parens() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkcloseparens_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckCloseParens_Mandatory").unwrap();
			let result = thinbasic_checkcloseparens_mandatory();

			return if result == 0 { false } else { true }
		}
	}

	#[allow(dead_code)]
	pub fn check_close_parens_optional() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_checkcloseparens_optional: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckCloseParens_Optional").unwrap();
			let result = thinbasic_checkcloseparens_optional();

			return if result == 0 { false } else { true }
		}
	}	


	/*

	 Error handling

	*/

	#[allow(dead_code)]
	pub enum Error
	{
		ModuleSpecific = 500
	}	

	pub fn error_free() -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_errorfree: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_ErrorFree").unwrap();
			let result = thinbasic_errorfree();

			return if result == 0 { false } else { true }
		}
	}

	pub fn raise_runtime_error(error_type: Error, description: &str) -> bool
	{
		unsafe
		{
			let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
			let thinbasic_runtimeerror: libloading::Symbol<unsafe extern fn(error_type: i32, error_description: winrt::BStr) -> i32> = lib.get(b"thinBasic_RunTimeError").unwrap();

			let result = thinbasic_runtimeerror(error_type as i32, winrt::BStr::from(description));

			return if result == 0 { false } else { true }
		}
	}	
}

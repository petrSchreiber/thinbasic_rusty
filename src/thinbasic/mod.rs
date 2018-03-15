/*

    Binding to thinCore - thinBasic internal engine.
    Functionality present here is used to interact with thinBasic.

*/

pub mod core
{
    extern crate libloading;    // For thinCore
    extern crate winapi;
    extern crate ascii;

    /*

        Implementation of "ThinBASIC STRING" - TBstr - is tortured code from
        BStr implementation made in https://github.com/contextfree/winrt-rust

    */

    use thinbasic::core::winapi::um::oleauto::{SysAllocStringByteLen, SysFreeString};

    pub struct TBStr(*mut u16);

    impl<'a> From<&'a str> for TBStr {
        
        fn from(str_text: &'a str) -> Self {
            let max_ascii_length = str_text.chars().count();                            // Maximum length of ASCII string will not be bigger than original text
            let mut byte_buffer: Vec<u8> = Vec::with_capacity(max_ascii_length);        // Preallocate to avoid reallocations

            for str_char in str_text.chars() {                                          // For each wide char in str...         
                if str_char.is_ascii()                                                  // If it is representable as ASCII...
                {
                    let ascii_char = ascii::AsciiChar::from(str_char);                  // Convert to byte and push to byte_buffer
                    byte_buffer.push(ascii_char.unwrap().as_byte());
                }
            }

            let first_byte_of_byte_buffer = &byte_buffer[0] as *const _ as *const i8;   // Pointer to first item in our byte buffer, formed in a way SysAllocStringByteLen will love it
            let total_string_data_length =  byte_buffer.len() as u32;                   // Total length of the buffer, just casted in a way SysAllocStringByteLen will love it
            let tbstr = unsafe {
                SysAllocStringByteLen(first_byte_of_byte_buffer,
                                      total_string_data_length)
            };
            
            TBStr(tbstr)                                                                // Filling the TBStr with the value returned by SysAllocStringByteLen
        }
    }

    impl Drop for TBStr {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe { SysFreeString(self.0 as *mut u16) };
        }
    }

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

            let thinbasic_loadsymbol: libloading::Symbol<unsafe extern fn(symbol_name: TBStr, return_type: i32, function_ptr: extern fn() -> T, force_overwrite: i32) -> i32> = lib.get(b"thinBasic_LoadSymbol").unwrap();

            thinbasic_loadsymbol(TBStr::from(symbol_name), ReturnType::I32 as i32, function_ptr, 1)
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
            let thinbasic_runtimeerror: libloading::Symbol<unsafe extern fn(error_type: i32, error_description: TBStr) -> i32> = lib.get(b"thinBasic_RunTimeError").unwrap();

            let result = thinbasic_runtimeerror(error_type as i32, TBStr::from(description));

            return if result == 0 { false } else { true }
        }
    }
}

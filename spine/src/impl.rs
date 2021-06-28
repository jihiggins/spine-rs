use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{BufReader, Read},
    os::raw::{c_char, c_int, c_void},
};

use spine_sys::spAtlasPage;

use super::{error::Error, result::Result};

#[no_mangle]
pub extern "C" fn _spUtil_readFile(path: *const c_char, length: *mut c_int) -> *const c_char {
    #[inline]
    fn read_text_file(path: *const c_char) -> Result<CString> {
        let path = to_str(path)?;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes)?;

        let text = CString::new(bytes)?;

        Ok(text)
    }

    let text = match read_text_file(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("{}", error);
            return std::ptr::null();
        }
    };

    unsafe {
        *length = text.to_bytes().len() as c_int;
        text.into_raw()
    }
}

#[inline]
fn to_str<'a>(s: *const c_char) -> Result<&'a str> {
    let s = unsafe { CStr::from_ptr(s) }
        .to_str()
        .map_err(Error::invalid_input)?;

    Ok(s)
}

use std::borrow::Cow;

#[inline]
pub fn string_from_utf8(v: &[u8]) -> Option<String> {
    match ::core::str::from_utf8(v) {
        Ok(_) => Some(unsafe { String::from_utf8_unchecked(v.to_vec()) }),
        Err(_) => None,
    }
}

#[inline]
pub fn string_from_utf8_cow(v: Cow<'_, [u8]>) -> Option<String> {
    match ::core::str::from_utf8(&v) {
        Ok(_) => Some(unsafe { String::from_utf8_unchecked(v.into_owned()) }),
        Err(_) => None,
    }
}

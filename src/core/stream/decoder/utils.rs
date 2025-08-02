use std::borrow::Cow;

#[allow(private_bounds)]
#[inline]
pub fn string_from_utf8<V: StringFrom>(v: V) -> Option<String> {
    match ::core::str::from_utf8(v.as_bytes()) {
        Ok(_) => Some(unsafe { String::from_utf8_unchecked(v.into_vec()) }),
        Err(_) => None,
    }
}

trait StringFrom: Sized {
    fn as_bytes(&self) -> &[u8];
    fn into_vec(self) -> Vec<u8>;
}

impl StringFrom for &[u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] { *self }
    #[inline(always)]
    fn into_vec(self) -> Vec<u8> { self.to_vec() }
}

impl StringFrom for Cow<'_, [u8]> {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] { self }
    #[inline(always)]
    fn into_vec(self) -> Vec<u8> { self.into_owned() }
}

// mod private {
//     pub trait Sealed: Sized {}

//     impl Sealed for &[u8] {}
//     impl Sealed for super::Cow<'_, [u8]> {}
// }

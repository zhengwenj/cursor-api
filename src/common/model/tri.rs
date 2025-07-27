use ::serde::Serialize;

#[derive(Clone, PartialEq, Default)]
pub enum TriState<T> {
    #[default]
    Undefined, // 未定义/字段不存在
    Null,     // 显式空值
    Value(T), // 包含具体值
}

impl<T> TriState<T> {
    #[inline(always)]
    pub const fn is_undefined(&self) -> bool { matches!(*self, TriState::Undefined) }

    // #[inline(always)]
    // pub const fn is_null(&self) -> bool {
    //     matches!(*self, TriState::Null)
    // }

    // #[inline(always)]
    // pub const fn is_value(&self) -> bool {
    //     matches!(*self, TriState::Value(_))
    // }

    // pub const fn as_value(&self) -> Option<&T> {
    //     match self {
    //         TriState::Value(v) => Some(v),
    //         _ => None,
    //     }
    // }
}

impl<T> Serialize for TriState<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TriState::Undefined => serializer.serialize_none(),
            TriState::Null => serializer.serialize_unit(),
            TriState::Value(value) => value.serialize(serializer),
        }
    }
}

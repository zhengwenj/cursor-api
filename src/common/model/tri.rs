use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub enum TriState<T> {
    None,
    Null,
    Some(T),
}

impl<T> TriState<T> {
    // pub fn is_some(&self) -> bool {
    //     matches!(self, TriState::Some(_))
    // }

    // pub fn is_null(&self) -> bool {
    //     matches!(self, TriState::Null)
    // }

    pub fn is_none(&self) -> bool {
        matches!(self, TriState::None)
    }
}

impl<T> Default for TriState<T> {
    fn default() -> Self {
        TriState::None
    }
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
            TriState::None => serializer.serialize_none(),
            TriState::Null => serializer.serialize_unit(),
            TriState::Some(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for TriState<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt = Option::<T>::deserialize(deserializer);

        match opt {
            Ok(Some(value)) => Ok(TriState::Some(value)),
            Ok(None) => Ok(TriState::Null),
            Err(_) => Ok(TriState::None),
        }
    }
}

impl<T> From<Option<T>> for TriState<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => TriState::Some(value),
            None => TriState::Null,
        }
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct TriStateField<T> {
    #[serde(skip_serializing_if = "TriState::is_none")]
    pub value: TriState<T>,
}

impl<T> From<TriState<T>> for TriStateField<T> {
    fn from(value: TriState<T>) -> Self {
        TriStateField { value }
    }
}

impl<T> From<TriStateField<T>> for TriState<T> {
    fn from(field: TriStateField<T>) -> Self {
        field.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        required: String,
        optional: Option<String>,
        #[serde(skip_serializing_if = "TriState::is_none")]
        tristate: TriState<String>,
    }

    #[test]
    fn test_tristate_serialization() {
        // 创建三个测试结构体，分别包含不同状态的TriState
        let test_none = TestStruct {
            required: "必填字段".to_string(),
            optional: Some("可选字段".to_string()),
            tristate: TriState::None,
        };

        let test_null = TestStruct {
            required: "必填字段".to_string(),
            optional: None,
            tristate: TriState::Null,
        };

        let test_some = TestStruct {
            required: "必填字段".to_string(),
            optional: Some("可选字段".to_string()),
            tristate: TriState::Some("三态字段".to_string()),
        };

        // 序列化并打印结果
        println!("TriState::None 序列化结果:");
        println!("{}", serde_json::to_string_pretty(&test_none).unwrap());
        println!();

        println!("TriState::Null 序列化结果:");
        println!("{}", serde_json::to_string_pretty(&test_null).unwrap());
        println!();

        println!("TriState::Some 序列化结果:");
        println!("{}", serde_json::to_string_pretty(&test_some).unwrap());
        println!();

        // 验证序列化行为
        let json_none = serde_json::to_string(&test_none).unwrap();
        let json_null = serde_json::to_string(&test_null).unwrap();
        let json_some = serde_json::to_string(&test_some).unwrap();

        // TriState::None 不应该在JSON中出现
        assert!(!json_none.contains("tristate"));

        // TriState::Null 应该在JSON中出现为null
        assert!(json_null.contains("\"tristate\":null"));

        // TriState::Some 应该在JSON中出现为具体值
        assert!(json_some.contains("\"tristate\":\"三态字段\""));
    }
}

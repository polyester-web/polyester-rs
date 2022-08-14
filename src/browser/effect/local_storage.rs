use crate::browser;
use crate::browser::effectful_msg::effectful_msg;
use crate::browser::Effect;
use crate::browser::Value;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum LocalStorage {
    GetItem { key: String },
    SetItem { key: String, value: Value },
}

pub fn get_item<Msg, CustomEffect, ToMsg>(key: &str, to_msg: ToMsg) -> Effect<Msg, CustomEffect>
where
    ToMsg: Fn(Value) -> Msg,
{
    let msg = to_msg(Value::empty());
    let effect = Effect::LocalStorage(LocalStorage::GetItem {
        key: key.to_string(),
    });

    effectful_msg(msg, effect)
}

pub fn set_item<Msg, CustomEffect, V>(key: &str, value: V) -> Effect<Msg, CustomEffect>
where
    V: serde::Serialize,
{
    Effect::LocalStorage(LocalStorage::SetItem {
        key: key.to_string(),
        value: browser::to_value(value),
    })
}

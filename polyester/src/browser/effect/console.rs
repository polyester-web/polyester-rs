use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Console {
    Log { message: String },
}

pub fn log<Msg, AppEffect>(s: &str) -> Effect<Msg, AppEffect> {
    Effect::Console(Console::Log {
        message: s.to_string(),
    })
}

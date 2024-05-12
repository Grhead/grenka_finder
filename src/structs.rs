#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum CommandEnum {
    Vec(Vec<String>),
    String(String),
}
#[derive(serde::Deserialize, Debug)]
pub struct ParentStruct {
    pub conf: ConfStruct,
}
#[derive(serde::Deserialize, Debug)]
pub struct ConfStruct {
    pub commands: CommandEnum,
    timeout: Option<i64>,
    rerun_counts: Option<i16>,
    weight: Option<i32>,
    pub path: Option<std::path::PathBuf>,
}

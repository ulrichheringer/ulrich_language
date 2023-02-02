#[derive(Debug)]
pub enum RuntimeVal {
    NullVal { kind: String, value: String },
    NumberVal { kind: String, value: i64 },
    IdentVal { kind: String, value: String },
    TextVal { value: String },
}

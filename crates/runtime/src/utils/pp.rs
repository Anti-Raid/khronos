use mluau::prelude::*;

#[must_use = "pretty_print returns the formatted string, which should probably be used"]
pub fn pretty_print(values: LuaMultiValue) -> String {
    if !values.is_empty() {
        format!(
            "{}",
            values
                .iter()
                .map(|value| {
                    match value {
                        LuaValue::String(s) => format!("{}", s.display()),
                        _ => format!("{value:#?}"),
                    }
                })
                .collect::<Vec<_>>()
                .join("\t")
        )
    } else {
        format!("nil")
    }
}
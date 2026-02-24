use mluau::prelude::*;
pub fn pretty_print(values: LuaMultiValue) {
    if !values.is_empty() {
        println!(
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
        );
    } else {
        println!("nil");
    }
}
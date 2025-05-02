use indexmap::IndexMap;
use mlua::prelude::*;
use rustyline::{
    completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Helper,
};

/// Based on https://github.com/luau-lang/luau/blob/master/CLI/src/Repl.cpp#L538
pub struct LuaStatementCompleter {
    pub runtime: khronos_runtime::rt::KhronosRuntime,
    pub global_tab: LuaTable,
}

impl LuaStatementCompleter {
    /// Remove all until opening parenthesis
    fn prepare_str(line: &str, pos: usize) -> (usize, String) {
        // Look for a opening parenthesis from the position
        // that is what we want to complete
        let mut pos = pos;
        let mut str = String::new();
        while pos > 0 {
            if line.chars().nth(pos - 1).unwrap() == '(' {
                break;
            }
            str.push(line.chars().nth(pos - 1).unwrap());
            pos -= 1;
        }
        str = str.chars().rev().collect(); // Reverse the string

        (pos, str)
    }

    /// Returns the list of candidates for the given line
    pub fn get_candidates(&self, line: &str) -> LuaResult<Vec<String>> {
        let mut line = line;

        let mut complete_only_functions = false;

        // Set the global table to begin the search
        let mut current_value = LuaValue::Table(self.global_tab.clone());

        loop {
            /*
                size_t sep = lookup.find_first_of(".:");
                std::string_view prefix = lookup.substr(0, sep);
            */
            let sep = line.find(['.', ':']);
            let prefix = match sep {
                Some(sep) => &line[..sep],
                None => line,
            };

            /*
               if (sep == std::string_view::npos)
               {
                   completePartialMatches(L, completeOnlyFunctions, editBuffer, prefix, addCompletionCallback);
                   break;
               }
            */

            if sep.is_none() {
                return self.complete_partial_matches(
                    complete_only_functions,
                    prefix,
                    current_value,
                );
            }

            // Try finding the table
            let next = Self::value_get(current_value, prefix)?;

            /*
            if (lua_istable(L, -1) || tryReplaceTopWithIndex(L))
            {
                completeOnlyFunctions = lookup[sep] == ':';
                lookup.remove_prefix(sep + 1);
            }
            else
            {
                // Unable to search for keys, so stop searching
                break;
            }
             */

            // If the table is not found, return an empty list
            if next.is_table() || next.is_userdata() {
                current_value = next;
                complete_only_functions = line.chars().nth(sep.unwrap()).unwrap_or_default() == ':';
                line = &line[sep.unwrap() + 1..];
            } else {
                return Ok(vec![]);
            }
        }
    }

    fn value_metamethod(value: &LuaValue, metamethod: &str) -> Option<LuaTable> {
        match value {
            LuaValue::Table(t) => match t.metatable() {
                Some(mt) => match mt.get::<LuaValue>(metamethod) {
                    Ok(LuaValue::Table(t)) => Some(t),
                    _ => None,
                },
                None => None,
            },
            LuaValue::UserData(ud) => match ud.metatable() {
                Ok(mt) => match mt.get::<LuaValue>(metamethod) {
                    Ok(LuaValue::Table(t)) => Some(t),
                    _ => None,
                },
                Err(_) => None,
            },
            _ => None,
        }
    }

    fn value_get(value: LuaValue, key: impl IntoLua) -> LuaResult<LuaValue> {
        match value {
            LuaValue::UserData(ud) => {
                if let Ok(v) = ud.get::<LuaValue>(key) {
                    return Ok(v);
                }
            }
            LuaValue::Table(t) => {
                if let Ok(v) = t.get::<LuaValue>(key) {
                    return Ok(v);
                }
            }
            _ => {}
        }
        Ok(LuaValue::Nil)
    }

    fn value_iter(value: LuaValue) -> LuaResult<IndexMap<String, LuaValue>> {
        let mut map = IndexMap::new();
        match value {
            LuaValue::UserData(ud) => {
                if let Ok(mt) = ud.metatable() {
                    let Ok(iter) = mt.get::<LuaFunction>(LuaMetaMethod::Iter) else {
                        return Ok(map);
                    };

                    let Ok(iter_func) = iter.call::<LuaFunction>(LuaValue::UserData(ud.clone()))
                    else {
                        return Ok(map);
                    };

                    loop {
                        let Ok((k, v)) = iter_func.call::<(LuaValue, LuaValue)>(()) else {
                            break;
                        };
                        if k.is_nil() {
                            break;
                        }
                        map.insert(k.to_string()?, v);
                    }
                }
            }
            LuaValue::Table(t) => {
                let inner_map = Self::iter_table_to_map(t)?;
                for (k, v) in inner_map {
                    map.insert(k, v);
                }
            }
            _ => {}
        }
        Ok(map)
    }

    fn iter_table_to_map(
        table: LuaTable,
    ) -> LuaResult<std::collections::HashMap<String, LuaValue>> {
        let mut map = std::collections::HashMap::new();
        for entry in table.pairs::<LuaValue, LuaValue>() {
            let (k, v) = entry?;
            map.insert(k.to_string()?, v);
        }
        Ok(map)
    }

    /// completePartialMatches finds keys that match the specified 'prefix'
    fn complete_partial_matches(
        &self,
        complete_only_functions: bool,
        prefix: &str,
        current_value: LuaValue,
    ) -> LuaResult<Vec<String>> {
        let mut candidates = vec![];

        let mut tabs = vec![current_value.clone()];

        if let Some(mt) = Self::value_metamethod(&current_value, "__index") {
            tabs.push(LuaValue::Table(mt));
        }

        if current_value == LuaValue::Table(self.global_tab.clone()) {
            let Some(ref lua) = *self.runtime.lua() else {
                panic!("Lua runtime is not initialized");
            };

            // Add the real global table to the list of tables to search
            tabs.push(LuaValue::Table(lua.globals()));

            if let Some(mt) =
                Self::value_metamethod(&LuaValue::Table(lua.globals()), "__index")
            {
                tabs.push(LuaValue::Table(mt));
            }
        }

        for current_table in tabs {
            for (key, v) in Self::value_iter(current_table)? {
                // If the last separator was a ':' (i.e. a method call) then only functions should be completed.
                // bool requiredValueType = (!completeOnlyFunctions || valueType == LUA_TFUNCTION);
                let required_value_type = !complete_only_functions || v.is_function();

                // if (!key.empty() && requiredValueType && Luau::startsWith(key, prefix))
                if !key.is_empty() && required_value_type && key.starts_with(prefix) {
                    /*
                        std::string completedComponent(key.substr(prefix.size()));
                       std::string completion(editBuffer + completedComponent);
                       if (valueType == LUA_TFUNCTION)
                       {
                           // Add an opening paren for function calls by default.
                           completion += "(";
                       }
                    */

                    let completed_component = key[prefix.len()..].to_string();
                    let mut completion = prefix.to_string() + &completed_component;
                    if v.is_function() {
                        // Add an opening paren for function calls by default.
                        completion += "(";
                    }

                    if candidates.contains(&completion) {
                        continue;
                    }

                    candidates.push(completion);
                }
            }
        }

        Ok(candidates)
    }
}

impl Helper for LuaStatementCompleter {}

impl Hinter for LuaStatementCompleter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        if pos != line.len() || pos == 0 {
            return None;
        }

        // Disable hints until we find a better way to do it using a readline impl that isn't
        // so annoying to work with

        /*let (_pos, mut str) = Self::prepare_str(line, pos);

        let candidates = self.get_candidates(&str).ok()?;
        // Return the first candidate
        if let Some(first) = candidates.into_iter().next() {
            // If there's a dot, find everything after the last dot
            let last_dot = str.rfind('.');
            if let Some(last_dot) = last_dot {
                str = str[last_dot + 1..].to_string();
            }

            let first = first.replace(&str, "");

            if first == "_G" {
                return None; // _G while valid is not a good hint
            }

            return Some(first.clone());
        }*/

        None
    }
}

impl Validator for LuaStatementCompleter {}

impl Highlighter for LuaStatementCompleter {}

impl Completer for LuaStatementCompleter {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let (pos, mut str) = Self::prepare_str(line, pos);

        let mut candidates = self.get_candidates(&str).map_err(|e| {
            rustyline::error::ReadlineError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;

        // Remove everything from the string after the last '.'
        let last_dot = str.rfind('.');
        if let Some(last_dot) = last_dot {
            str = str[..last_dot].to_string();
            candidates = candidates
                .into_iter()
                .map(|c| format!("{}.{}", str, c))
                .collect();
        }

        // Do the same thing for ':'
        let last_colon = str.rfind(':');
        if let Some(last_colon) = last_colon {
            str = str[..last_colon].to_string();
            candidates = candidates
                .into_iter()
                .map(|c| format!("{}:{}", str, c))
                .collect();
        }

        // Then map candidate to the string
        Ok((pos, candidates))
    }
}

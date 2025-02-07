use mlua::prelude::*;
use rustyline::{
    completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Helper,
};

/// Based on https://github.com/luau-lang/luau/blob/master/CLI/src/Repl.cpp#L538
pub struct LuaStatementCompleter {
    pub lua: Lua,
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
        let mut current_table = self.global_tab.clone();

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
                    current_table,
                );
            }

            // Try finding the table
            let next = current_table.get::<LuaValue>(prefix)?;

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
            let mt = self.tab_metamethod(next.clone(), "__index");
            if next.is_table() || mt.is_some() {
                current_table = if let Some(mt) = mt {
                    mt
                } else {
                    next.as_table().unwrap().clone()
                };
                complete_only_functions = line.chars().nth(sep.unwrap()).unwrap_or_default() == ':';
                line = &line[sep.unwrap() + 1..];
            } else {
                return Ok(vec![]);
            }
        }
    }

    fn tab_metamethod(&self, value: LuaValue, metamethod: &str) -> Option<LuaTable> {
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

    /// completePartialMatches finds keys that match the specified 'prefix'
    fn complete_partial_matches(
        &self,
        complete_only_functions: bool,
        prefix: &str,
        current_table: LuaTable,
    ) -> LuaResult<Vec<String>> {
        let mut candidates = vec![];

        let mut tabs = vec![current_table.clone()];

        if let Some(mt) = self.tab_metamethod(LuaValue::Table(current_table.clone()), "__index") {
            tabs.push(mt);
        }

        if current_table == self.global_tab {
            // Add the real global table to the list of tables to search
            tabs.push(self.lua.globals());

            if let Some(mt) = self.tab_metamethod(LuaValue::Table(self.lua.globals()), "__index") {
                tabs.push(mt);
            }
        }

        for current_table in tabs {
            for entry in current_table.pairs::<LuaValue, LuaValue>() {
                let (k, v) = entry?;

                let key = k.to_string()?;

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

        let (_pos, mut str) = Self::prepare_str(line, pos);

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
        }

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

        // Then map candidate to the string
        Ok((pos, candidates))
    }
}

//! The dataset's schema description (data/schema.json from db/tools/webexport.py): tables, fields, the fn::
//! library. Visitors query as a guest and can't run `INFO FOR DB`, so the editor's autocomplete comes from here.

use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;

use crate::bridge;
use crate::kb::fetch_text;

#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Table {
    pub comment: Option<String>,
    pub fields: BTreeMap<String, Field>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: String,
    pub doc: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    pub tables: BTreeMap<String, Table>,
    pub functions: Vec<Function>,
}

const KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "ORDER BY", "GROUP BY", "GROUP ALL", "LIMIT", "START", "VALUE", "ONLY", "OMIT", "FETCH",
    "SPLIT", "AS", "AND", "OR", "NOT", "CONTAINS", "CONTAINSANY", "CONTAINSALL", "INSIDE", "INTERSECTS", "LET", "RETURN",
    "IF", "ELSE", "THEN", "END", "FOR", "IN", "NONE", "NULL", "true", "false", "DESC", "ASC", "count()", "math::sum",
    "math::mean", "math::max", "math::min", "array::distinct", "array::flatten", "array::group", "array::len",
    "array::sort", "string::lowercase", "string::contains", "time::format", "time::year", "geo::distance",
    "search::highlight", "search::score", "type::record", "record::id",
];

/// Fetch schema.json and hand the editor its autocomplete words.
pub async fn load() -> Option<Schema> {
    let schema: Schema = serde_json::from_str(&fetch_text(&format!("data/schema.json?v={}", crate::kb::BUILD)).await.ok()?).ok()?;
    let mut words = vec![];
    for (name, t) in &schema.tables {
        words.push(json!({"label": name, "type": "class", "detail": "table", "info": t.comment}));
        for (f, d) in &t.fields {
            if f.contains('*') || f.contains('[') {
                continue;
            }
            words.push(json!({"label": f, "type": "property", "detail": format!("{name} · {}", d.typ.clone().unwrap_or_default()), "info": d.note}));
        }
    }
    for f in &schema.functions {
        words.push(json!({"label": f.name, "type": "function", "detail": format!("({})", f.params), "info": f.doc,
            "apply": format!("{}()", f.name)}));
    }
    for k in KEYWORDS {
        words.push(json!({"label": k, "type": "keyword"}));
    }
    bridge::set_completions(&serde_json::Value::Array(words).to_string());
    Some(schema)
}

//! Patch validated JSON tokens in place. Borrowing RawValue preserves spelling, whitespace,
//! escaped keys and numeric lexemes throughout the rest of the document.
use std::ops::Range;

use serde::{Deserialize, Deserializer};
use serde_json::{value::RawValue, Value};

use super::TxnError;
use crate::manifest::{DocumentFormat, NewlineStyle};

fn present<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct EditPatch {
    #[serde(default, deserialize_with = "present")]
    #[cfg_attr(feature = "bindings", ts(optional))]
    pub tags: Option<Vec<String>>,
    #[serde(default, deserialize_with = "present")]
    #[cfg_attr(feature = "bindings", ts(optional))]
    pub rating: Option<u8>,
    #[serde(default, deserialize_with = "present")]
    #[cfg_attr(feature = "bindings", ts(optional))]
    pub note: Option<String>,
    #[serde(default, deserialize_with = "present")]
    #[cfg_attr(feature = "bindings", ts(optional))]
    pub source: Option<SourcePatch>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct SourcePatch {
    // Missing "url" is an invalid patch. An explicit null removes just that property.
    #[serde(deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
}

struct Member {
    key: String,
    start: usize,
    key_end: usize,
    value: Range<usize>,
}

fn whitespace(data: &[u8], mut i: usize) -> usize {
    while data.get(i).is_some_and(u8::is_ascii_whitespace) {
        i += 1;
    }
    i
}

fn members(text: &str) -> Result<Vec<Member>, TxnError> {
    let data = text.as_bytes();
    let open = whitespace(data, 0);
    if data.get(open) != Some(&b'{') {
        return Err(TxnError::Invalid("待编辑的字段不是 JSON 对象".into()));
    }
    let mut result = Vec::new();
    let mut i = whitespace(data, open + 1);
    while data.get(i) != Some(&b'}') {
        let start = i;
        let mut keys = serde_json::Deserializer::from_slice(&data[i..]).into_iter::<String>();
        let key = keys
            .next()
            .transpose()?
            .ok_or_else(|| TxnError::Invalid("JSON 对象缺少键".into()))?;
        if result.iter().any(|m: &Member| m.key == key) {
            return Err(TxnError::Invalid(format!("JSON 对象中存在重复键：{key}")));
        }
        let key_end = i + keys.byte_offset();
        let colon = whitespace(data, key_end);
        if data.get(colon) != Some(&b':') {
            return Err(TxnError::Invalid("JSON 对象缺少冒号".into()));
        }
        let value_start = whitespace(data, colon + 1);
        let mut values = serde_json::Deserializer::from_slice(&data[value_start..]).into_iter::<&RawValue>();
        values
            .next()
            .transpose()?
            .ok_or_else(|| TxnError::Invalid("JSON 对象缺少值".into()))?;
        let end = value_start + values.byte_offset();
        result.push(Member {
            key,
            start,
            key_end,
            value: value_start..end,
        });
        i = whitespace(data, end);
        if data.get(i) == Some(&b',') {
            i = whitespace(data, i + 1);
        }
    }
    Ok(result)
}

fn replace_member(
    text: &mut String,
    key: &str,
    value: Option<&Value>,
    format: DocumentFormat,
) -> Result<(), TxnError> {
    let fields = members(text)?;
    let first = fields.first();
    let open = whitespace(text.as_bytes(), 0) + 1;
    let prefix = first.map_or("", |f| &text[open..f.start]);
    let newline = match format.newline {
        NewlineStyle::Crlf => "\r\n",
        NewlineStyle::Mixed { crlf, lf } if crlf > lf => "\r\n",
        _ => "\n",
    };
    let encoded = value
        .map(|v| -> serde_json::Result<String> {
            if let Some((_, indent)) = prefix.rsplit_once('\n') {
                Ok(serde_json::to_string_pretty(v)?.replace('\n', &format!("{newline}{indent}")))
            } else {
                serde_json::to_string(v)
            }
        })
        .transpose()?;
    if let Some(i) = fields.iter().position(|f| f.key == key) {
        let entry = &fields[i];
        if let Some(new_value) = encoded {
            text.replace_range(entry.value.clone(), &new_value);
        } else {
            let range = if let Some(next) = fields.get(i + 1) {
                entry.start..next.start
            } else if i > 0 {
                fields[i - 1].value.end..entry.value.end
            } else {
                entry.start..entry.value.end
            };
            text.replace_range(range, "");
        }
    } else if let Some(new_value) = encoded {
        let separator = first.map_or(": ", |f| &text[f.key_end..f.value.start]);
        let mut member = String::new();
        if let Some(last) = fields.last() {
            member.push(',');
            let gap = if prefix.contains('\n') {
                prefix
            } else {
                fields
                    .get(1)
                    .and_then(|second| text[fields[0].value.end..second.start].split_once(','))
                    .map_or(prefix, |(_, gap)| gap)
            };
            member.push_str(gap);
            member.push_str(&serde_json::to_string(key)?);
            member.push_str(separator);
            member.push_str(&new_value);
            text.insert_str(last.value.end, &member);
        } else {
            member.push_str(&serde_json::to_string(key)?);
            member.push_str(separator);
            member.push_str(&new_value);
            text.insert_str(open, &member);
        }
    }
    Ok(())
}

pub(super) fn apply(
    old: &str,
    original: &Value,
    patch: EditPatch,
    date: &str,
) -> Result<(String, Vec<String>), TxnError> {
    let format = DocumentFormat::detect(old.as_bytes());
    let mut text = old.trim_start_matches('\u{feff}').to_owned();
    let mut expected = original.clone();
    let mut changed = Vec::new();
    for (key, value) in [
        ("tags", patch.tags.map(Value::from)),
        ("rating", patch.rating.map(Value::from)),
        ("note", patch.note.map(Value::from)),
    ] {
        if let Some(value) = value.filter(|v| expected.get(key) != Some(v)) {
            replace_member(&mut text, key, Some(&value), format)?;
            expected[key] = value;
            changed.push(key.to_owned());
        }
    }
    if let Some(source) = patch.source {
        let url = source.url.map(Value::from);
        if expected["source"].get("url") != url.as_ref() {
            if let Some(field) = members(&text)?.into_iter().find(|m| m.key == "source") {
                let mut source_text = text[field.value.clone()].to_owned();
                replace_member(&mut source_text, "url", url.as_ref(), format)?;
                text.replace_range(field.value, &source_text);
            } else {
                replace_member(
                    &mut text,
                    "source",
                    Some(&serde_json::json!({"url": url})),
                    format,
                )?;
            }
            if let Some(url) = url {
                expected["source"]["url"] = url;
            } else if let Some(object) = expected["source"].as_object_mut() {
                object.shift_remove("url");
            }
            changed.push("source.url".to_owned());
        }
    }
    if changed.is_empty() {
        return Err(TxnError::NoChanges);
    }
    let updated = Value::from(date);
    if expected.get("updated") != Some(&updated) {
        replace_member(&mut text, "updated", Some(&updated), format)?;
        expected["updated"] = updated;
        changed.push("updated".into());
    }
    if serde_json::from_str::<Value>(&text)? != expected {
        return Err(TxnError::Invalid("写出结果与已批准的字段修改不符".into()));
    }
    if format.bom {
        text.insert(0, '\u{feff}');
    }
    Ok((text, changed))
}

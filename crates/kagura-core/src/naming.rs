//! 复刻 Python 迁移工具的 Windows 名称和相对路径规则。

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NamingError {
    #[error("目录名为空")]
    EmptyName,
    #[error("非法或不安全的 Windows 相对路径：{0:?}")]
    InvalidRelativePath(String),
}

pub fn utf16_len(text: &str) -> usize {
    text.encode_utf16().count()
}

fn disallowed(ch: char) -> bool {
    matches!(ch, '\0'..='\u{1f}' | '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*')
}

/// 首个点之前的名称决定整段是否为设备名，例如 `LPT3 .txt`。
pub fn is_reserved_device(segment: &str) -> bool {
    let stem = segment
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end_matches(' ');
    if ["CON", "PRN", "AUX", "NUL"]
        .iter()
        .any(|name| stem.eq_ignore_ascii_case(name))
    {
        return true;
    }

    let mut chars = stem.chars();
    let digit = chars.next_back();
    let prefix = chars.as_str();
    matches!(digit, Some('1'..='9' | '¹' | '²' | '³'))
        && (prefix.eq_ignore_ascii_case("COM") || prefix.eq_ignore_ascii_case("LPT"))
}

/// 按 UTF-16 单元限制目录名长度，保留 plan.safe 的设备名截断规则。
pub fn safe(name: &str, limit: usize) -> Result<String, NamingError> {
    let replaced: String = name
        .chars()
        .map(|ch| if disallowed(ch) { '_' } else { ch })
        .collect();
    let mut name = replaced.trim_matches([' ', '.']);

    // 保留能完整放入的最长码点前缀，等价于 Python 逐个删除尾部码点。
    let mut units = 0;
    for (index, ch) in name.char_indices() {
        units += ch.len_utf16();
        if units > limit {
            name = &name[..index];
            break;
        }
    }
    let name = name.trim_end_matches([' ', '.']);
    if is_reserved_device(name) {
        // plan.safe 的第二次截断按码点计数，结果可能比 limit 多一个 UTF-16 单元。
        let mut result = String::from("_");
        result.extend(name.chars().take(limit - 1));
        return Ok(result);
    }
    if name.is_empty() {
        return Err(NamingError::EmptyName);
    }
    Ok(name.to_owned())
}

/// 验证由正斜杠分隔的 Windows 相对路径。
pub fn check_rel(path: &str) -> Result<(), NamingError> {
    for segment in path.split('/') {
        if segment.is_empty()
            || segment.ends_with([' ', '.'])
            || segment.chars().any(disallowed)
            || is_reserved_device(segment)
            || utf16_len(segment) > 255
        {
            return Err(NamingError::InvalidRelativePath(path.to_owned()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_matches_python_edges() {
        // 期望值由 G:\MMD\_design\migration\plan.py 的 safe() 实测得到。
        let cases = [
            ("CON", 40, Ok("_CON")),
            ("con.txt", 40, Ok("_con.txt")),
            ("com1", 40, Ok("_com1")),
            ("COM¹", 40, Ok("_COM¹")),
            ("LPT².txt", 40, Ok("_LPT².txt")),
            ("COM³", 40, Ok("_COM³")),
            ("LPT3 .x", 40, Ok("_LPT3 .x")),
            (" .tail.  ", 40, Ok("tail")),
            ("　a　", 40, Ok("　a　")),
            ("<>:\"/\\|?*", 40, Ok("_________")),
            ("\0\u{1f}", 40, Ok("__")),
            ("a😀b", 2, Ok("a")),
            ("a😀b", 3, Ok("a😀")),
            ("𠮷a", 1, Err(NamingError::EmptyName)),
            ("𠮷a", 2, Ok("𠮷")),
            ("a𠮷", 3, Ok("a𠮷")),
            ("ab.cd", 3, Ok("ab")),
            ("ab cd", 3, Ok("ab")),
            (" ... ", 40, Err(NamingError::EmptyName)),
            ("x", 0, Err(NamingError::EmptyName)),
            ("CON", 3, Ok("_CO")),
            ("CON.😀", 6, Ok("_CON.😀")),
        ];
        for (name, limit, expected) in cases {
            assert_eq!(
                safe(name, limit),
                expected.map(str::to_owned),
                "{name:?}, limit={limit}"
            );
        }
    }

    #[test]
    fn reserved_device_stems() {
        for name in ["prn.log", "aux", "NuL", "COM9", "lpt¹", "COM²", "LPT³ .txt"] {
            assert!(is_reserved_device(name), "{name:?}");
        }
        for name in ["COM0", "COM10", "LPT⁴", "COM１", "ＣＯＮ", "not.CON", " CON"] {
            assert!(!is_reserved_device(name), "{name:?}");
        }
    }

    #[test]
    fn relative_paths() {
        for path in ["ok/sub.txt", "　", "ok/.hidden", "ＣＯＮ/COM10"] {
            assert_eq!(check_rel(path), Ok(()), "{path:?}");
        }
        for path in [
            "", ".", "..", "./x", "a/../b", "a//b", "/a", "a/", "a\\b", "C:/x", "a/\0b",
            "sub/x.", "sub/x ", "sub/CON.txt", "ok/LPT3 .x", "COM¹.txt",
        ] {
            assert!(check_rel(path).is_err(), "{path:?}");
        }

        let emoji = "😀".repeat(127);
        let longest = format!("{emoji}a");
        assert_eq!(utf16_len(&longest), 255);
        assert!(check_rel(&format!("{longest}/{longest}")).is_ok());
        assert!(check_rel(&format!("{emoji}𠮷")).is_err());
    }
}

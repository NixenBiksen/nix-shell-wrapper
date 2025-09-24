use unicode_segmentation::UnicodeSegmentation;

// maximum length of exprs we want in our shell
const MAX_EXPR_LEN: usize = 18;

// By default we will try to split the expression by words,
// but if it becomes too small when we do that, we will include partial words
const MIN_EXPR_LEN: usize = 12;

fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

fn trim_and_replace_whitespace(s: &str) -> String {
    s.trim()
        .graphemes(true)
        .map(|g| if g.trim().is_empty() { "·" } else { g.trim() })
        .collect::<String>()
}

pub fn truncate_string(s: &str) -> String {
    let s = s.trim();

    let attempt = trim_and_replace_whitespace(s);
    if grapheme_count(&attempt) <= MAX_EXPR_LEN {
        return attempt;
    }

    for (i, w) in s.unicode_word_indices().rev() {
        let mut s = trim_and_replace_whitespace(&format!("{}{}", &s[..i], w));
        s.push('…');
        if (MIN_EXPR_LEN..=MAX_EXPR_LEN).contains(&grapheme_count(&s)) {
            return s;
        }
    }

    for (i, g) in s.grapheme_indices(true).rev() {
        let mut s = trim_and_replace_whitespace(&format!("{}{}", &s[..i], g));
        s.push('…');
        if (MIN_EXPR_LEN..=MAX_EXPR_LEN).contains(&grapheme_count(&s)) {
            return s;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello"), "hello");
        assert_eq!(truncate_string(" hello "), "hello");
        assert_eq!(truncate_string(" hello world "), "hello·world");
        assert_eq!(truncate_string("a brave boy!!!!!!!!"), "a·brave·boy…");
        assert_eq!(truncate_string("a whole wooooooorld"), "a·whole·wooooooor…");
        assert_eq!(truncate_string("a whole new wooorld"), "a·whole·new…");
        assert_eq!(
            truncate_string(" a b c d e f g h i j "),
            "a·b·c·d·e·f·g·h·i…"
        );
    }
}

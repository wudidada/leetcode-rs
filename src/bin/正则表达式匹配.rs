/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        is_match(s.as_bytes(), p.as_bytes())
    }

    pub fn is_match1(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        is_match_bytes(s, p)
    }
}

fn is_match(s: &[u8], p: &[u8]) -> bool {
    match parse(p) {
        (Pattern::Empty, _) => s.is_empty(),
        (Pattern::Single(c), subp) => is_match_single(s, c, subp),
        (Pattern::Repeat(c), subp) => is_match_single(s, c, p) || is_match(s, subp),
    }
}

fn is_match_single(s: &[u8], to_match: u8, p: &[u8]) -> bool {
    match s.split_first() {
        Some((c, s)) if to_match == b'.' || *c == to_match => is_match(s, p),
        _ => false,
    }
}

fn parse(p: &[u8]) -> (Pattern, &[u8]) {
    match p.split_first() {
        None => (Pattern::Empty, p),
        Some((c, p)) => match p.split_first() {
            Some((b'*', p)) => (Pattern::Repeat(*c), p),
            _ => (Pattern::Single(*c), p),
        },
    }
}

enum Pattern {
    Empty,
    Single(u8),
    Repeat(u8),
}

fn is_match_bytes(s: &[u8], p: &[u8]) -> bool {
    // println!(
    //     "{} <-> {}",
    //     String::from_utf8(s.to_vec()).unwrap(),
    //     String::from_utf8(p.to_vec()).unwrap()
    // );
    if s.len() == 0 && p.len() == 0 {
        return true;
    }

    if p.len() == 0 {
        return false;
    }

    match p[0] as char {
        '.' => match (p.len(), s.len()) {
            (n, 0) => {
                if n == 1 || p[1] as char != '*' {
                    return false;
                } else {
                    return is_match_bytes(s, &p[2..]);
                }
            }
            (1, 1) => return true,
            (1, _n) => return false,
            (_n, 1) => {
                if p[1] as char == '*' {
                    return is_match_bytes(&s[1..], &p[2..]) || is_match_bytes(s, &p[2..]);
                } else {
                    return is_match_bytes(&s[1..], &p[1..]);
                }
            }
            (_n1, n2) => match p[1] as char {
                '*' => {
                    for i in 0..=n2 {
                        if is_match_bytes(&s[n2 - i..n2], &p[2..]) {
                            return true;
                        }
                    }
                    return false;
                }
                _a => return is_match_bytes(&s[1..], &p[1..]),
            },
        },
        _a => match (p.len(), s.len()) {
            (n, 0) => {
                if n == 1 || p[1] as char != '*' {
                    return false;
                } else {
                    return is_match_bytes(s, &p[2..]);
                }
            }
            (1, 1) => return s[0] == p[0],
            (1, _n) => return false,
            (_n, 1) => {
                if p[1] as char == '*' {
                    if s[0] == p[0] {
                        return is_match_bytes(&s[1..], &p[2..]) || is_match_bytes(s, &p[2..]);
                    } else {
                        return is_match_bytes(s, &p[2..]);
                    }
                } else {
                    if s[0] == p[0] {
                        return is_match_bytes(&s[1..], &p[1..]);
                    }
                    return false;
                }
            }
            (_n1, _n2) => match p[1] as char {
                '*' => {
                    if s[0] != p[0] {
                        return is_match_bytes(s, &p[2..]);
                    } else {
                        return is_match_bytes(s, &p[2..]) || is_match_bytes(&s[1..], p);
                    }
                }
                _b => {
                    if s[0] != p[0] {
                        return false;
                    } else {
                        return is_match_bytes(&s[1..], &p[1..]);
                    }
                }
            },
        },
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(
        Solution::is_match("ab".to_string(), ".*c".to_string()),
        false
    );
    assert_eq!(
        Solution::is_match("aaa".to_string(), "a*a".to_string()),
        true
    );
    assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
    assert_eq!(
        Solution::is_match("ab".to_string(), ".*..c*".to_string()),
        true
    );
    assert_eq!(Solution::is_match("a".to_string(), ".*.".to_string()), true);
}

use std::fs;

pub fn count(input: &str) -> (usize, usize, usize) {
    let nwords = input.split_whitespace().count();
    let nchars = input.len();
    let nlines = input.split('\n').count() - 1;
    (nwords, nchars, nlines)
}

pub fn from_file(filename: &str) -> (usize, usize, usize) {
    let input = fs::read_to_string(filename).unwrap();
    count(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings() {
        let test_input = [
            ("This is a test string", (5, 21, 0)),
            ("A string with a unicode chracter ✔", (7, 36, 0)),
            ("   Trailing spaces and mixed with \ttabs.", (6, 40, 0)),
            ("This is a\nmultiple line\nstring.", (6, 31, 2)),
        ];

        for (input, expected) in test_input {
            assert_eq!(count(input), expected);
        }
    }

    #[test]
    fn test_files() {
        let inputs = [("testdata/test1.txt", (7, 34, 0))];

        for (filename, expected) in inputs {
            assert_eq!(from_file(filename), expected);
        }
    }
}

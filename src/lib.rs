
pub struct ErrorReport {
    display : String,
    line : usize,
    column : usize,
}

pub fn report( input : &str, start : usize, end : usize ) -> ErrorReport {
    let len = input.len();
    assert!( start < len, "Encountered start index longer than input" );
    assert!( end < len, "Encountered end index longer than input" );
    assert!( end >= start, "Encountered end smaller than the start" );

    let lines = input.split(|c| c == '\n' || c == '\r').collect::<Vec<&str>>();
   
    let mut i = 0;
    let mut before = None;
    let mut current = None;
    let mut after = None; 
    let mut pointer = None;
    let mut line = 0;
    for l in lines {
        line += 1;
        if !matches!(current, None) {
            after = Some(l);
            break;
        }
        let dash_len = start - i;
        let arrow_len = 1 + end - start;
        i += l.len() + 1; 
        if i > end {
            current = Some(l); 
            pointer = Some(format!( "{}{}", "-".repeat(dash_len), "^".repeat(arrow_len)));
        }
        else {
            before = Some(l);
        }
    }

    let p = pointer.expect("pointer was not assiged in error reporter");

    let display = match (before, current, after) {
        (None, Some(c), None) => format!( "{}\n{}\n", c, p ),
        (None, Some(c), Some(a)) => format!( "{}\n{}\n{}\n", c, p, a ),
        (Some(b), Some(c), None) => format!( "{}\n{}\n{}\n", b, c, p ),
        (Some(b), Some(c), Some(a)) => format!( "{}\n{}\n{}\n{}\n", b, c, p, a ),
        _ => panic!("Enountered start and end outside of input range in error reporter: {} {}", start, end),
    };

    ErrorReport { display, line, column: dash_len + 1 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_display_single_character_error() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( input, 21, 21 );

        assert_eq!( output, r#"line one
line two
-^
line three
"# );
    }

    #[test]
    fn should_display_multi_character_error() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( input, 21, 24 );

        assert_eq!( output, r#"line one
line two
-^^^^
line three
"# );
    }

    #[test]
    fn should_display_single_character_error_at_start_of_line() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( input, 20, 20 );

        assert_eq!( output, r#"line one
line two
^
line three
"# );
    }

    #[test]
    fn should_display_single_character_error_at_end_of_line() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( input, 27, 27 );

        assert_eq!( output, r#"line one
line two
-------^
line three
"# );
    }

    #[test]
    fn should_display_full_line_error() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( input, 20, 27 );

        assert_eq!( output, r#"line one
line two
^^^^^^^^
line three
"# );
    }

    #[test]
    fn should_display_single_character_error_at_end_of_file() {
        let input = r#"
line zero
line one
line two
line three
line four"#;

        let output = report( input, 48, 48 );

        assert_eq!( output, r#"line three
line four
--------^
"# );
    }

    #[test]
    fn should_display_single_character_error_at_beginning_of_file() {
        let input = r#"line zero
line one
line two
line three
line four
"#;

        let output = report( input, 0, 0 );

        assert_eq!( output, r#"line zero
^
line one
"# );
    }

    #[test]
    fn should_display_single_character_for_single_line() {
        let input = r#"line zero"#;

        let output = report( input, 0, 0 );

        assert_eq!( output, r#"line zero
^
"# );
    }

    #[test]
    fn should_display_multi_character_a_few_characters_away_from_line_start() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( input, 23, 25 );

        assert_eq!( output, r#"line one
line two
---^^^
line three
"# );
    }
}


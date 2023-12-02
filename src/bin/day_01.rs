#![allow(dead_code)]

fn parse_line_part_01( line: &str ) -> u32
{
    let deca_digit = | c: &char | -> bool { c.is_digit( 10 ) };
    let first_digit = match line.chars().find( deca_digit )
    {
        Some( i ) => i as u32 - '0' as u32,
        None => 0u32,
    };
    let last_digit = match line.chars().rev().find( deca_digit )
    {
        Some( i ) => i as u32 - '0' as u32,
        None => 0u32,
    };

    first_digit * 10 + last_digit
}

#[test]
fn test_parse_line_part_01()
{
    assert_eq!( parse_line_part_01( "Hello1" ), 11 );
    assert_eq!( parse_line_part_01( "5Hello1" ), 51 );
    assert_eq!( parse_line_part_01( "Hello" ), 0 );
    assert_eq!( parse_line_part_01( "He5llo11o" ), 51 );
}

fn forward_search( line: &str, patterns: &[&str] ) -> u32
{
    for ( i, _ ) in line.char_indices()
    {
        let substr = &line[i..];
        for ( pat_index, pattern ) in patterns.iter().enumerate()
        {
            if substr.starts_with( pattern )
            {
                return pat_index as u32 / 2 + 1;
            }
        }
    }
    return 0;
}

fn backward_search( line: &str, patterns: &[&str] ) -> u32
{
    for ( i, _ ) in line.char_indices().rev()
    {
        let substr = &line[ .. ( i + 1 ) ];
        for ( pat_index, pattern ) in patterns.iter().enumerate()
        {
            if substr.ends_with( pattern )
            {
                return pat_index as u32 / 2 + 1;
            }
        }
    }
    return 0;
}


const PATTERNS : &[&str] = &[ "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8", "eight", "9", "nine" ];

fn parse_line_part_02( line: &str, patterns: &[&str] ) -> u32
{
    let first_digit = forward_search ( line, patterns );
    let last_digit  = backward_search( line, patterns );

    first_digit * 10 + last_digit
}

#[test]
fn test_parse_line_part_02()
{
    assert_eq!( parse_line_part_02( "Hello1"   , PATTERNS ), 11 );
    assert_eq!( parse_line_part_02( "5Hello1"  , PATTERNS ), 51 );
    assert_eq!( parse_line_part_02( "Hello"    , PATTERNS ), 0  );
    assert_eq!( parse_line_part_02( "He5llo11o", PATTERNS ), 51 );

    assert_eq!( parse_line_part_02( "two7eight"  , PATTERNS ), 28 );
    assert_eq!( parse_line_part_02( "Hello1three", PATTERNS ), 13 );
}

fn main() {
    let args: Vec< String > = std::env::args().collect();

    let file_path = &args[1];
    println!( "File path is {}", file_path );
    let input = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut sum: u32 = 0;
    for line in input.lines()
    {
        sum += parse_line_part_02( line, PATTERNS );
    }

    println!( "Total sum: {}", sum );
}

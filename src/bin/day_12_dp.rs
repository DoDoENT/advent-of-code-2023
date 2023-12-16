use std::collections::HashMap;

fn parse_line( line: &str ) -> ( String, Vec< usize > )
{
    let ( pattern, groups ) = line.split_once( ' ' ).unwrap();
    let expected_runchains = groups.split(',')
        .map( str::parse::< usize > )
        .map( Result::unwrap )
        .collect::< Vec< _ > >();

    ( pattern.to_string(), expected_runchains )
}

// based on https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
// beacuse I suck at DP

#[derive( Clone, Hash, PartialEq, Eq )]
struct State
{
    char_index : usize,
    chain_index: usize,
    last_block_size: usize,
}

type Cache = HashMap< State, usize >;

fn solve( pattern: &str, expected_runchain: &[ usize ], cache: &mut Cache, state: State ) -> usize
{
    if let Some( cached_result ) = cache.get( &state )
    {
        return *cached_result;
    }
    // reached end of pattern - check if we have solution
    if state.char_index == pattern.len()
    {
        if state.chain_index == expected_runchain.len() && state.last_block_size == 0
        {
            return 1;
        }
        else if state.chain_index == expected_runchain.len() - 1 && state.last_block_size == expected_runchain[ state.chain_index ]
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }

    let mut num_solutions_for_input_state = 0usize;

    for c in vec![ '.', '#' ]
    {
        let p = pattern.chars().nth( state.char_index ).unwrap();

        if p == c || p == '?'
        {
            if c == '.' && state.last_block_size == 0
            {
                num_solutions_for_input_state += solve
                (
                    &pattern,
                    &expected_runchain,
                    cache,
                    State
                    {
                        char_index: state.char_index + 1,
                        chain_index: state.chain_index,
                        last_block_size: 0,
                    }
                );
            }
            else if c == '.' && state.last_block_size > 0 && state.chain_index < expected_runchain.len() && expected_runchain[ state.chain_index ] == state.last_block_size
            {
                num_solutions_for_input_state += solve
                (
                    &pattern,
                    &expected_runchain,
                    cache,
                    State
                    {
                        char_index: state.char_index + 1,
                        chain_index: state.chain_index + 1,
                        last_block_size: 0,
                    }
                );
            }
            else if c == '#'
            {
                num_solutions_for_input_state += solve
                (
                    &pattern,
                    &expected_runchain,
                    cache,
                    State
                    {
                        char_index: state.char_index + 1,
                        chain_index: state.chain_index,
                        last_block_size: state.last_block_size + 1,
                    }
                );
            }
        }
    }

    cache.insert( state, num_solutions_for_input_state );

    num_solutions_for_input_state
}

fn solve_line( line: &str, expand: usize ) -> usize
{
    let ( mut pattern, expected_runchain ) = parse_line( line );

    if expand > 1
    {
        pattern = vec![ pattern; expand ].join( "?" );
    }
    let expected_runchain = expected_runchain.repeat( expand );

    let mut cache = Cache::new();

    solve
    (
        &pattern,
        &expected_runchain,
        &mut cache,
        State
        {
            char_index: 0,
            chain_index: 0,
            last_block_size: 0,
        }
    )
}

#[test]
fn test_part1_solutions()
{
    assert_eq!( 1 , solve_line( "???.### 1,1,3"            , 1 ) );
    assert_eq!( 4 , solve_line( ".??..??...?##. 1,1,3"     , 1 ) );
    assert_eq!( 1 , solve_line( "?#?#?#?#?#?#?#? 1,3,1,6"  , 1 ) );
    assert_eq!( 1 , solve_line( "????.#...#... 4,1,1"      , 1 ) );
    assert_eq!( 4 , solve_line( "????.######..#####. 1,6,5", 1 ) );
    assert_eq!( 10, solve_line( "?###???????? 3,2,1"       , 1 ) );
    assert_eq!( 8 , solve_line( "..?????#??.?##? 1,1,2,3"  , 1 ) );
    assert_eq!( 13, solve_line( "?????.???? 2,2"           , 1 ) );
}

#[test]
fn test_part2_solutions()
{
    assert_eq!( 1     , solve_line( "???.### 1,1,3"            , 5 ) );
    assert_eq!( 16384 , solve_line( ".??..??...?##. 1,1,3"     , 5 ) );
    assert_eq!( 1     , solve_line( "?#?#?#?#?#?#?#? 1,3,1,6"  , 5 ) );
    assert_eq!( 16    , solve_line( "????.#...#... 4,1,1"      , 5 ) );
    assert_eq!( 2500  , solve_line( "????.######..#####. 1,6,5", 5 ) );
    assert_eq!( 506250, solve_line( "?###???????? 3,2,1"       , 5 ) );

    assert_eq!( 1, solve_line( "? 1", 5 ) );
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_01 = 0usize;
    let mut part_02 = 0usize;

    for line in input.lines()
    {
        part_01 += solve_line( line, 1 );
        part_02 += solve_line( line, 5 );
    }

    println!( "Part 01: {}", part_01 );
    println!( "Part 02: {}", part_02 );
}

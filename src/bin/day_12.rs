fn parse_line( line: &str ) -> ( String, Vec< usize > )
{
    let ( pattern, groups ) = line.split_once( ' ' ).unwrap();
    let expected_runchains = groups.split(',')
        .map( str::parse::< usize > )
        .map( Result::unwrap )
        .collect::< Vec< _ > >();

    ( pattern.to_string(), expected_runchains )
}

#[derive( Clone )]
struct State
{
    char_index : usize,
    chain_index: usize,
    runchain   : Vec< usize >,

    #[cfg(test)]
    current_solution: String,
}

struct NoMoreBlocks;

impl State
{
    fn next_block( &mut self ) -> Result< (), NoMoreBlocks >
    {
        // avoid twice growing in case next_block is called
        // consecutive (i.e. in "..." case)
        if self.runchain[ self.chain_index ] != 0
        {
            if self.chain_index < self.runchain.len() - 1
            {
                self.chain_index += 1;

                #[cfg(test)]
                {
                    self.current_solution += ".";
                }
                return Ok( () );
            }
            else
            {
                return Err( NoMoreBlocks );
            }
        }
        #[cfg(test)]
        {
            self.current_solution += ".";
        }
        // not grown
        Ok( () )
    }

    fn next_char ( &mut self )
    {
        self.char_index  += 1;
    }

    fn grow_block( &mut self )
    {
        self.runchain[ self.chain_index ] += 1;

        #[cfg(test)]
        {
            self.current_solution += "#";
        }
    }

    fn impossible( &self, expected: &Vec< usize > ) -> bool
    {
        let mut prev_match = true;
        if self.chain_index > 0
        {
            prev_match = self.runchain[ self.chain_index - 1 ] == expected[ self.chain_index - 1 ];
        }

        !prev_match || ( self.chain_index < self.runchain.len() && self.runchain[ self.chain_index ] > expected[ self.chain_index ] )
    }

    fn is_solution( &self, expected: &Vec< usize >, pattern: &str ) -> bool
    {
        let mut remaining_damaged = false;
        if self.char_index < pattern.len() - 1 {
            remaining_damaged = pattern[ self.char_index + 1 .. ].find( '#' ).is_some();
        }
        !remaining_damaged && self.chain_index == expected.len() - 1 && self.runchain[ self.chain_index ] == expected[ self.chain_index ]
    }

    #[cfg(test)]
    fn print_solution( &self, line_length: usize )
    {
        print!( "Solution: {}", self.current_solution );
        for _ in self.current_solution.len() .. line_length
        {
            print!( "." );
        }
        println!();
    }
}

fn solve_line( line: &str, expand: usize ) -> usize
{
    let ( mut pattern, expected_runchain ) = parse_line( line );

    if expand > 1
    {
        pattern = vec![ pattern; expand ].join( "?" );
    }
    let expected_runchain = expected_runchain.repeat( expand );

    let mut valid_combinations = 0usize;

    let mut backtrack_list: Vec< State > = Vec::new();
    let mut current_state = State
    {
        char_index: 0,
        chain_index: 0,
        runchain: vec![ 0; expected_runchain.len() ],

        #[cfg(test)]
        current_solution: String::new(),
    };

    let mut done = false;

    while !done
    {
        let mut drain_backtrack = false;

        if let Some( c ) = pattern.chars().nth( current_state.char_index )
        {
            if c == '.'
            {
                if let Err( NoMoreBlocks ) = current_state.next_block()
                {
                    drain_backtrack = true;
                }
            }
            else if c == '#'
            {
                current_state.grow_block();
            }
            else if c == '?'
            {
                backtrack_list.push( current_state.clone() );
                // assume ? is #
                current_state.grow_block();
            }

            if !drain_backtrack
            {
                if current_state.is_solution( &expected_runchain, &pattern )
                {
                    valid_combinations += 1;

                    drain_backtrack = true;

                    #[cfg(test)]
                    current_state.print_solution( pattern.len() );
                }

                // check if current runchain is still possible
                if current_state.impossible( &expected_runchain )
                {
                    drain_backtrack = true;
                }
            }

            if !drain_backtrack
            {
                current_state.next_char();
            }
        }
        else
        {
            drain_backtrack = true;
        }

        if drain_backtrack
        {
            done = true;
            while let Some( state ) = backtrack_list.pop()
            {
                current_state = state;
                if let Ok( _ ) = current_state.next_block() // assume ? is .
                {
                    current_state.next_char(); // advance to next char
                    if current_state.is_solution( &expected_runchain, &pattern )
                    {
                        valid_combinations += 1;

                        #[cfg(test)]
                        current_state.print_solution( pattern.len() );
                    }
                    if !current_state.impossible( &expected_runchain )
                    {
                        // break the drain loop and continue from backtraced state
                        done = false;
                        break;
                    }
                }
            }
        }
    }

    #[cfg(test)]
    println!( "Line:     {}, total combinations: {}", line, valid_combinations );

    valid_combinations
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

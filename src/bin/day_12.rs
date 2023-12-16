fn parse_line( line: &str ) -> ( &str, Vec< usize > )
{
    let ( pattern, groups ) = line.split_once( ' ' ).unwrap();
    let expected_runchains = groups.split(',')
        .map( str::parse::< usize > )
        .map( Result::unwrap )
        .collect::< Vec< _ > >();

    ( pattern, expected_runchains )
}

#[derive( Clone )]
struct State
{
    char_index : usize,
    chain_index: usize,
    runchain   : Vec< usize >,
}

struct NoMoreBlocks;

impl State
{
    fn next_block( &mut self ) -> Result< (), NoMoreBlocks >
    {
        if self.chain_index < self.runchain.len() - 1
        {
            // avoid twice growing in case next_block is called
            // consecutive (i.e. in "..." case)
            if self.runchain[ self.chain_index ] != 0
            {
                self.chain_index += 1;
            }
            Ok( () )
        }
        else
        {
            Err( NoMoreBlocks )
        }
    }

    fn next_char ( &mut self )
    {
        self.char_index  += 1;
    }

    fn grow_block( &mut self )
    {
        self.runchain[ self.chain_index ] += 1;
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

    fn is_solution( &self, expected: &Vec< usize > ) -> bool
    {
        self.chain_index == expected.len() - 1 && self.runchain[ self.chain_index ] == expected[ self.chain_index ]
    }
}

fn solve_line( line: &str ) -> usize
{
    let ( pattern, expected_runchain ) = parse_line( line );

    let mut valid_combinations = 0usize;

    let mut backtrack_list: Vec< State > = Vec::new();
    let mut current_state = State{ char_index: 0, chain_index: 0, runchain: vec![ 0; expected_runchain.len() ] };

    let mut done = false;

    while !done
    {
        let mut drain_backtrack = false;

        if let Some( c ) = pattern.chars().nth( current_state.char_index )
        {
            if c == '.'
            {
                let _ = current_state.next_block();
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

            if current_state.is_solution( &expected_runchain )
            {
                valid_combinations += 1;

                // drain backtrack list
                drain_backtrack = true;

            }

            // check if current runchain is still possible
            if current_state.impossible( &expected_runchain )
            {
                drain_backtrack = true;
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
                    if current_state.is_solution( &expected_runchain )
                    {
                        valid_combinations += 1;
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

    valid_combinations
}

#[test]
fn test_part1_solutions()
{
    assert_eq!( 1, solve_line( "???.### 1,1,3" ) );
    assert_eq!( 4, solve_line( ".??..??...?##. 1,1,3" ) );
    assert_eq!( 1, solve_line( "?#?#?#?#?#?#?#? 1,3,1,6" ) );
    assert_eq!( 1, solve_line( "????.#...#... 4,1,1" ) );
    assert_eq!( 4, solve_line( "????.######..#####. 1,6,5" ) );
    assert_eq!( 10, solve_line( "?###???????? 3,2,1" ) );
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_01 = 0usize;

    for line in input.lines()
    {
        part_01 += solve_line( line )
    }

    println!( "Part 01: {}", part_01 );
}

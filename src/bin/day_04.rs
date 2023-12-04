use std::collections::{HashSet, VecDeque};

fn to_set( number_list: &str ) -> HashSet< usize >
{
    number_list.trim().split_whitespace().map
    (
        | strnum: &str | -> usize
        {
            strnum.trim().parse().unwrap()
        }
    ).collect()
}

fn get_new_scratchcards( line: &str ) -> usize
{
    let     points_desc = line.split( ':' ).nth( 1 ).unwrap();
    let mut numbers_desc = points_desc.split( '|' );

    let winning_numbers = numbers_desc.next().unwrap().trim();
    let my_numbers      = numbers_desc.next().unwrap().trim();

    let winning_numbers_set = to_set( winning_numbers );
    let my_numbers_set      = to_set( my_numbers      );

    let intersection = winning_numbers_set.intersection( &my_numbers_set );

    intersection.count()
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut total_points       = 0;
    let mut total_scratchcards = 0;

    let mut pending_copies:  VecDeque< usize > = VecDeque::new();

    for game in input.lines()
    {
        let my_copies = 1 + match pending_copies.pop_front()
        {
            None => { 0 },
            Some( x ) => { x },
        };

        total_scratchcards += my_copies;
        let new_scratchcards = get_new_scratchcards( game );

        if new_scratchcards > 0
        {
            total_points += 2_usize.pow( ( new_scratchcards - 1 ) as u32 );
        }

        // update pendine copies
        let mut to_update = new_scratchcards;

        for element in pending_copies.iter_mut()
        {
            if to_update == 0 { break; }
            *element += my_copies;
            to_update -= 1;
        }

        // add new copies to queue
        for _ in 0 .. to_update
        {
            pending_copies.push_back( my_copies );
        }
    }

    println!( "Total points: {}", total_points );
    println!( "Total scratchcards: {}", total_scratchcards );
}


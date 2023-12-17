use std::{u8, collections::HashMap};

#[ derive( Clone, Eq, PartialEq, Hash ) ]
struct Platform
{
    data: Vec< u8 >,
    width: usize,
    height: usize,
}

impl Platform
{
    fn new( input: &str ) -> Platform
    {
        let mut data: Vec< u8 > = Vec::new();

        let mut width  = 0usize;
        let mut height = 0usize;

        for line in input.lines()
        {
            width = line.len();
            height += 1;
            data.extend( line.as_bytes() );
        }

        Platform
        {
            data,
            width,
            height,
        }
    }

    #[allow(dead_code)]
    fn print( &self )
    {
        for row in 0 .. self.height
        {
            for col in 0 .. self.width
            {
                print!( "{}", *self.at( row, col ) as char );
            }
            println!();
        }
    }

    fn at( &self, row: usize, col: usize ) -> &u8
    {
        let index = row * self.width + col;
        return &self.data[ index ];
    }

    fn mut_at( &mut self, row: usize, col: usize ) -> &mut u8
    {
        let index = row * self.width + col;
        return &mut self.data[ index ];
    }

    fn tilt_north( &mut self )
    {
        for col in 0 .. self.width
        {
            for row in 0 .. self.height
            {
                if *self.at( row, col ) == 'O' as u8
                {
                    let mut dest_row = row as isize - 1;
                    while dest_row >= 0 && *self.at( dest_row as usize, col ) == '.' as u8
                    {
                        dest_row -= 1;
                    }
                    // correct for one-off
                    dest_row += 1;
                    if dest_row as usize != row
                    {
                        // ...mrmlj... borrow checker does not allow this
                        // std::mem::swap( self.mut_at( dest_row as usize, col ), self.mut_at( row, col ) );

                        let tmp = *self.at( dest_row as usize, col );
                        *self.mut_at( dest_row as usize, col ) = *self.at( row, col );
                        *self.mut_at( row              , col ) = tmp;
                    }
                }
            }
        }
    }

    fn tilt_south( &mut self )
    {
        for col in 0 .. self.width
        {
            for row in ( 0 .. self.height ).rev()
            {
                if *self.at( row, col ) == 'O' as u8
                {
                    let mut dest_row = row + 1;
                    while dest_row < self.height && *self.at( dest_row as usize, col ) == '.' as u8
                    {
                        dest_row += 1;
                    }
                    // correct for one-off
                    dest_row -= 1;
                    if dest_row as usize != row
                    {
                        // ...mrmlj... borrow checker does not allow this
                        // std::mem::swap( self.mut_at( dest_row as usize, col ), self.mut_at( row, col ) );

                        let tmp = *self.at( dest_row as usize, col );
                        *self.mut_at( dest_row as usize, col ) = *self.at( row, col );
                        *self.mut_at( row              , col ) = tmp;
                    }
                }
            }
        }
    }

    fn tilt_west( &mut self )
    {
        for row in 0 .. self.height
        {
            for col in 0 .. self.width
            {
                if *self.at( row, col ) == 'O' as u8
                {
                    let mut dest_col = col as isize - 1;
                    while dest_col >= 0 && *self.at( row, dest_col as usize ) == '.' as u8
                    {
                        dest_col -= 1;
                    }
                    // correct for one-off
                    dest_col += 1;
                    if dest_col as usize != col
                    {
                        let tmp = *self.at( row, dest_col as usize );
                        *self.mut_at( row, dest_col as usize ) = *self.at( row, col );
                        *self.mut_at( row, col               ) = tmp;
                    }
                }
            }
        }
    }

    fn tilt_east( &mut self )
    {
        for row in 0 .. self.height
        {
            for col in ( 0 .. self.width ).rev()
            {
                if *self.at( row, col ) == 'O' as u8
                {
                    let mut dest_col = col + 1;
                    while dest_col < self.width && *self.at( row, dest_col as usize ) == '.' as u8
                    {
                        dest_col += 1;
                    }
                    // correct for one-off
                    dest_col -= 1;
                    if dest_col as usize != col
                    {
                        let tmp = *self.at( row, dest_col as usize );
                        *self.mut_at( row, dest_col as usize ) = *self.at( row, col );
                        *self.mut_at( row, col               ) = tmp;
                    }
                }
            }
        }
    }

    fn north_load( &self ) -> usize
    {
        let mut load = 0usize;

        for r in 0 .. self.height
        {
            let row_factor = self.height - r;

            for c in 0 .. self.width
            {
                if *self.at( r, c ) == 'O' as u8
                {
                    load += row_factor;
                }
            }
        }

        load
    }
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut platform = Platform::new( &input );

    platform.tilt_north();

    println!( "Part 01 north load: {}", platform.north_load() );

    // rest of first cycle
    platform.tilt_west();
    platform.tilt_south();
    platform.tilt_east();

    let start_time = std::time::Instant::now();

    let mut seen_states: HashMap< Platform, usize > = HashMap::new();

    const CYCLES: usize = 1000000000;

    let mut period = 0usize;
    let mut brute_forced_cycles = 0usize;

    for cycle in 1 .. CYCLES
    {
        seen_states.insert( platform.clone(), cycle );

        // println!( "After cycle {}", cycle );
        // platform.print();
        // println!( "North load: {}", platform.north_load() );

        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();

        if let Some( prev_cycle ) = seen_states.get( &platform )
        {
            println!( "After cycle {}, found same state as after cycle {}", cycle + 1, prev_cycle );

            period = cycle + 1 - prev_cycle;
            brute_forced_cycles = cycle + 1;

            println!( "Period is {}", period );

            break;
        }
    }

    let leftover_cycles = CYCLES - brute_forced_cycles;

    let leftover_iterations = leftover_cycles % period;

    for _ in 0 .. leftover_iterations
    {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
    }

    let elapsed_time = start_time.elapsed();
    println!( "Elapsed time: {:?}", elapsed_time );

    println!( "After {} leftover cycles and {} leftover iterations:", leftover_cycles, leftover_iterations );
    // println!( "After all {} cycles", CYCLES );
    // platform.print();

    println!( "Part 02 north load: {}", platform.north_load() );
}

use std::u8;

#[ derive( Clone, Eq, PartialEq ) ]
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

    println!( "After first cycle: " );
    platform.print();

    let start_time = std::time::Instant::now();

    const CYCLES: usize = 100000;

    let mut total_cycles = 1usize;
    for _ in 0 .. CYCLES - 1
    {
        let before = platform.clone();

        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();

        total_cycles += 1;

        if platform == before
        {
            break;
        }
    }

    println!( "After {} cycles:", total_cycles );
    platform.print();

    let elapsed_time = start_time.elapsed();
    println!( "{:?} cycles elapsed time: {:?}", total_cycles, elapsed_time );
}

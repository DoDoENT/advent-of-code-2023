#[ derive( Copy, Clone ) ]
struct Pos( i64, i64 );

struct Map
{
    layout   : Vec< char >,
    width    : usize,
    height   : usize,
    start_pos: Pos,
}

#[ derive( PartialEq, Debug ) ]
enum Direction
{
    North,
    East,
    South,
    West,
}

impl Map
{
    fn new( input: String ) -> Map
    {
        let mut layout: Vec< char > = Vec::new();
        let mut width               = 0;
        let mut start_pos           = Pos( 0, 0 );
        let     height              = input.lines().count();

        for ( line_num, line ) in input.lines().enumerate()
        {
            width = line.len();

            if let Some( x ) =  line.find( 'S' )
            {
                start_pos = Pos( line_num as i64, x as i64 );
            }
            layout.extend( line.chars() );
        }

        Map { layout, width, height, start_pos }
    }

    fn at_pos( &self, pos: Pos ) -> Option< char >
    {
        let lin_pos = pos.0 * self.width as i64 + pos.1;

        self.layout.get( lin_pos as usize ).copied()
    }

    fn find_connection( &self, pos: Pos, start_direction: Direction ) -> ( Pos, Direction )
    {
        let try_north = || -> Option< ( Pos, Direction ) >
        {
            let candidate = Pos( pos.0 - 1, pos.1 );
            if let Some( x ) = self.at_pos( candidate )
            {
                if x == '|' || x == '7' || x == 'F' || x == 'S'
                {
                    return Some( ( candidate, Direction::North ) );
                }
            }
            None
        };

        let try_east = || -> Option< ( Pos, Direction ) >
        {
            let candidate = Pos( pos.0, pos.1 + 1 );
            if let Some( x ) = self.at_pos( candidate )
            {
                if x == '-' || x == '7' || x == 'J' || x == 'S'
                {
                    return Some( ( candidate, Direction::East ) );
                }
            }
            None
        };

        let try_south = || -> Option< ( Pos, Direction ) >
        {
            let candidate = Pos( pos.0 + 1, pos.1 );
            if let Some( x ) = self.at_pos( candidate )
            {
                if x == '|' || x == 'J' || x == 'L' || x == 'S'
                {
                    return Some( ( candidate, Direction::South ) );
                }
            }
            None
        };

        let try_west = || -> Option< ( Pos, Direction ) >
        {
            let candidate = Pos( pos.0, pos.1 - 1 );
            if let Some( x ) = self.at_pos( candidate )
            {
                if x == '-' || x == 'F' || x == 'L' || x == 'S'
                {
                    return Some( ( candidate, Direction::West ) );
                }
            }
            None
        };

        match self.at_pos( pos ).unwrap()
        {
            'S' => { try_north().or( try_east() ).or( try_south() ).or( try_west() ).unwrap() }
            '|' =>
            {
                if start_direction == Direction::South
                {
                    // coming from North, so don't go back there
                    try_south().unwrap()
                }
                else
                {
                    try_north().unwrap()
                }
            },
            '-' =>
            {
                if start_direction == Direction::East { try_east().unwrap() }
                else                                  { try_west().unwrap() }
            },
            'J' =>
            {
                if start_direction == Direction::South { try_west ().unwrap() }
                else                                   { try_north().unwrap() }
            },
            'L' =>
            {
                if start_direction == Direction::South { try_east ().unwrap() }
                else                                   { try_north().unwrap() }
            },
            'F' =>
            {
                if start_direction == Direction::North { try_east ().unwrap() }
                else                                   { try_south().unwrap() }
            },
            '7' =>
            {
                if start_direction == Direction::East { try_south().unwrap() }
                else                                  { try_west ().unwrap() }
            },
            _ => { panic!( "Unknown marking!" ) }
        }
    }

    fn find_loop( &self ) -> Vec< Pos >
    {
        let mut lp = vec![ self.start_pos ];

        let mut cur_pos = self.find_connection( self.start_pos, Direction::North );

        // println!( "Start: ({}, {})", self.start_pos.0, self.start_pos.1 );

        while self.at_pos( cur_pos.0 ) != Some( 'S' )
        {
            // println!( "Gone {:?} to pos: ({}, {}), mark: {}", cur_pos.1, cur_pos.0.0, cur_pos.0.1, self.at_pos( cur_pos.0 ).unwrap() );
            lp.push( cur_pos.0 );

            cur_pos = self.find_connection( cur_pos.0, cur_pos.1 );
        }

        lp
    }
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let map = Map::new( input );

    let lp = map.find_loop();

    let part_01_solution = lp.len() / 2;

    println!( "Part 01 solution: {}", part_01_solution );
}


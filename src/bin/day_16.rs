use std::fmt::Display;
use std::collections::HashSet;

#[ derive( Clone ) ]
struct Matrix< T >
{
    data: Vec< T >,
    width: usize,
    height: usize,
}

impl< T > Matrix< T > where T: Clone + Copy + Display
{
    fn new( width: usize, height: usize, fill: T ) -> Matrix< T >
    {
        Matrix
        {
            data: vec![ fill; height * width ],
            width,
            height
        }
    }

    fn at( &self, row: usize, col: usize ) -> &T
    {
        let index = row * self.width + col;
        return &self.data[ index ];
    }

    fn mut_at( &mut self, row: usize, col: usize ) -> &mut T
    {
        let index = row * self.width + col;
        return &mut self.data[ index ];
    }

    #[allow(dead_code)]
    fn print( &self )
    {
        for row in 0 .. self.height
        {
            for col in 0 .. self.width
            {
                print!( "{} ", self.at( row, col ) );
            }
            println!();
        }
    }
}

#[ derive( Copy, Clone, PartialEq, Eq, Hash ) ]
#[ repr( u8 )]
enum Direction
{
    Right,
    Left,
    Up,
    Down,
}

#[ derive( Copy, Clone, Hash, PartialEq, Eq ) ]
struct Beam
{
    row: isize,
    col: isize,
    dir: Direction,
}

impl Beam
{
    fn same_direction( &self ) -> Beam
    {
        match self.dir
        {
            Direction::Up    => { Beam{ row: self.row - 1, col: self.col    , dir: self.dir } },
            Direction::Down  => { Beam{ row: self.row + 1, col: self.col    , dir: self.dir } },
            Direction::Left  => { Beam{ row: self.row    , col: self.col - 1, dir: self.dir } },
            Direction::Right => { Beam{ row: self.row    , col: self.col + 1, dir: self.dir } },
        }
    }

    fn rotate_left( &self ) -> Beam
    {
        match self.dir
        {
            Direction::Up    => { Beam{ row: self.row    , col: self.col - 1, dir: Direction::Left  } },
            Direction::Down  => { Beam{ row: self.row    , col: self.col + 1, dir: Direction::Right } },
            Direction::Left  => { Beam{ row: self.row + 1, col: self.col    , dir: Direction::Down  } },
            Direction::Right => { Beam{ row: self.row - 1, col: self.col    , dir: Direction::Up    } },
        }
    }

    fn rotate_right( &self ) -> Beam
    {
        match self.dir
        {
            Direction::Up    => { Beam{ row: self.row    , col: self.col + 1, dir: Direction::Right } },
            Direction::Down  => { Beam{ row: self.row    , col: self.col - 1, dir: Direction::Left  } },
            Direction::Left  => { Beam{ row: self.row - 1, col: self.col    , dir: Direction::Up    } },
            Direction::Right => { Beam{ row: self.row + 1, col: self.col    , dir: Direction::Down  } },
        }
    }
}

fn solve_part_01( tiles: &Matrix< u8 > ) -> usize
{
    let mut energized_tiles: Matrix< bool > = Matrix::new
    (
        tiles.width,
        tiles.height,
        false,
    );

    let mut beams = vec![ Beam{ row: 0, col: 0, dir: Direction::Right } ];

    let mut seen_positions: HashSet< Beam > = HashSet::new();

    while let Some( beam ) = beams.pop()
    {
        // check if beam is out of bounds
        if beam.row < 0                                ||
           beam.row >= energized_tiles.height as isize ||
           beam.col < 0                                ||
           beam.col >= energized_tiles.width as isize
        {
            continue;
        }

        // if we've already seen this position, skip it
        if seen_positions.contains( &beam )
        {
            continue;
        }

        // energize current tile
        *energized_tiles.mut_at( beam.row as usize, beam.col as usize ) = true;
        seen_positions.insert( beam );

        // reflect or split the beam
        match tiles.at( beam.row as usize, beam.col as usize )
        {
            b'.' => { beams.push( beam.same_direction() ); }
            b'\\' =>
            {
                match beam.dir
                {
                    Direction::Up   | Direction::Down  => { beams.push( beam.rotate_left () ); }
                    Direction::Left | Direction::Right => { beams.push( beam.rotate_right() ); }
                }
            },
            b'/' =>
            {
                match beam.dir
                {
                    Direction::Up   | Direction::Down  => { beams.push( beam.rotate_right() ); }
                    Direction::Left | Direction::Right => { beams.push( beam.rotate_left () ); }
                }
            },
            b'-' =>
            {
                match beam.dir
                {
                    Direction::Left | Direction::Right => { beams.push( beam.same_direction() ); }
                    Direction::Up   | Direction::Down  =>
                    {
                        beams.push( beam.rotate_right() );
                        beams.push( beam.rotate_left () );
                    }
                }
            },
            b'|' =>
            {
                match beam.dir
                {
                    Direction::Up   | Direction::Down  => { beams.push( beam.same_direction() ); }
                    Direction::Left | Direction::Right =>
                    {
                        beams.push( beam.rotate_right() );
                        beams.push( beam.rotate_left () );
                    }
                }
            }
            _ => { panic!( "Unknown char at input!" ); }
        }
    }

    // now count energized tiles
    energized_tiles
        .data
        .iter()
        .filter( | x | **x )
        .count()
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut tiles: Matrix< u8 > = Matrix
    {
        data: Vec::new(),
        width: 0,
        height: 0,
    };

    for line in input.lines()
    {
        tiles.data.extend( line.as_bytes() );
        tiles.width = line.len();
        tiles.height += 1;
    }

    println!( "Part 01: {}", solve_part_01( &tiles ) );
}

use std::collections::HashSet;

use utils::matrix::Matrix;

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

fn energize( tiles: &Matrix< u8 >, start_beam: Beam ) -> usize
{
    let mut energized_tiles: Matrix< bool > = Matrix::new
    (
        tiles.width,
        tiles.height,
        false,
    );

    let mut beams = vec![ start_beam ];

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

    println!( "Part 01: {}", energize( &tiles, Beam{ row: 0, col: 0, dir: Direction::Right } ) );

    let mut best_energy = 0usize;

    for col in 0 .. tiles.width
    {
        let e = energize( &tiles, Beam{ row: 0, col: col as isize, dir: Direction::Down } );
        best_energy = std::cmp::max( e, best_energy );

        let e = energize( &tiles, Beam{ row: tiles.height as isize - 1, col: col as isize, dir: Direction::Up } );
        best_energy = std::cmp::max( e, best_energy );
    }

    for row in 0 .. tiles.height
    {
        let e = energize( &tiles, Beam{ row: row as isize, col: 0, dir: Direction::Right } );
        best_energy = std::cmp::max( e, best_energy );

        let e = energize( &tiles, Beam{ row: row as isize, col: tiles.width as isize - 1, dir: Direction::Left } );
        best_energy = std::cmp::max( e, best_energy );
    }

    println!( "Part 02: {}", best_energy );
}

#[ derive( Debug ) ]
#[ repr( u8 ) ]
enum Direction
{
    Up,
    Right,
    Down,
    Left,
}

#[ derive( Debug ) ]
struct Instruction
{
    direction: Direction,
    steps: usize,
}

#[ derive( Clone, Copy, Debug ) ]
struct Vertex( isize, isize );

fn parse_instructions( input: &str ) -> ( Vec< Instruction >, Vec< Instruction > )
{
    let mut result = ( Vec::new(), Vec::new() );

    for line in input.lines()
    {
        let mut parts = line.split_ascii_whitespace();

        let direction = match parts.next()
        {
            Some( "R" ) => Direction::Right,
            Some( "L" ) => Direction::Left,
            Some( "U" ) => Direction::Up,
            Some( "D" ) => Direction::Down,
            _ => { panic!( "Uknown direction!" ); }
        };

        let steps: usize = parts.next().unwrap().parse().unwrap();

        let color = parts.next().unwrap();

        let big_steps = usize::from_str_radix( &color[ 2 .. 7 ], 16 ).unwrap();
        let big_direction = match color.chars().nth( 7 ).unwrap()
        {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => { panic!( "Unknown direction!" ); }
        };

        result.0.push( Instruction{ direction, steps } );
        result.1.push( Instruction{ direction: big_direction, steps: big_steps } );
    }

    result
}

fn to_poligon( instructions: &[ Instruction ] ) -> Vec< Vertex >
{
    let mut polygon = Vec::with_capacity( instructions.iter().count() );

    let mut current_vertex = Vertex( 0, 0 );

    polygon.push(current_vertex);

    for inst in instructions
    {
        match inst.direction
        {
            Direction::Right => { current_vertex.0 += inst.steps as isize; }
            Direction::Down  => { current_vertex.1 -= inst.steps as isize; }
            Direction::Left  => { current_vertex.0 -= inst.steps as isize; }
            Direction::Up    => { current_vertex.1 += inst.steps as isize; }
        }

        polygon.push( current_vertex );
    }

    // println!( "Polygon: {:?}", polygon );

    polygon
}

fn calc_area( instructions: &[ Instruction ] ) -> isize
{
    let polygon = to_poligon( instructions );

    // shoelace formula (based on https://www.wikihow.com/Calculate-the-Area-of-a-Polygon)
    let sum: isize = polygon
        .windows( 2 )
        .map
        (
            | chunks | -> isize
            {
                let a = &chunks[ 0 ];
                let b = &chunks[ 1 ];

                // based on https://www.reddit.com/r/adventofcode/comments/18l0qtr/comment/kegqfjo/?utm_source=share&utm_medium=web2x&context=3
                a.1 * b.0 - a.0 * b.1  // Shoelace formula for inner area
                    + ( b.0 - a.0 + b.1 - a.1 ).abs() // manhattan distance to cover the border length
            }
        )
        .sum();

    sum / 2 + 1
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let instructions = parse_instructions( &input );

    println!( "part 01 area: {}", calc_area( &instructions.0 ) );
    println!( "part 02 area: {}", calc_area( &instructions.1 ) );
}

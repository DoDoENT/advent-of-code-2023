struct GalaxyPos
{
    row: usize,
    col: usize,
}

impl GalaxyPos
{
    fn distance_to( &self, other: &GalaxyPos ) -> usize
    {
        self.col.abs_diff( other.col ) + self.row.abs_diff( other.row )
    }
}

const AGE: usize = 1000000;

fn parse_and_expand( input: &str ) -> Vec< ( GalaxyPos, GalaxyPos ) >
{
    let mut positions: Vec< ( GalaxyPos, GalaxyPos ) > = Vec::new();

    let mut row  = 0usize;
    let mut row2 = 0usize;

    for line in input.lines()
    {
        let mut empty_line = true;

        for ( col, ch ) in line.chars().enumerate()
        {
            if ch == '#' {
                positions.push( ( GalaxyPos{ row, col }, GalaxyPos{ row: row2, col } ) );
                empty_line = false;
            }
        }
        if empty_line {
            row += 1;
            row2 += AGE - 1;
        }

        row += 1;
        row2 += 1;
    }

    positions.sort_by( | x, y | x.0.col.cmp( &y.0.col ) );

    // now fix col positions

    let mut last_non_empty_col = 0usize;
    let mut total_correction   = 0usize;
    let mut total_correction2  = 0usize;

    for position in positions.iter_mut()
    {
        if position.0.col != last_non_empty_col
        {
            let diff_from_last = position.0.col - last_non_empty_col - 1;

            last_non_empty_col = position.0.col;
            total_correction += diff_from_last;
            total_correction2 += diff_from_last * ( AGE - 1 );
        }

        position.0.col += total_correction;
        position.1.col += total_correction2;
    }

    positions
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let galaxies = parse_and_expand( &input );

    let mut part_01_distances = 0usize;
    let mut part_02_distances = 0usize;

    for i in 0 .. galaxies.len()
    {
        for j in i + 1 .. galaxies.len()
        {
            part_01_distances += galaxies[ i ].0.distance_to( &galaxies[ j ].0 );
            part_02_distances += galaxies[ i ].1.distance_to( &galaxies[ j ].1 );
        }
    }

    println!( "Part 01 solution: {}", part_01_distances );
    println!( "Part 02 solution: {}", part_02_distances );

}

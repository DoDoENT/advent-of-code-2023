use utils::matrix::Matrix;

fn find_similar_rows( pattern: &Matrix< u8 > ) -> Matrix< bool >
{
    let mut similarity_matrix: Matrix< bool > = Matrix::new( pattern.height, pattern.height, false );

    for row in 0 .. pattern.height - 1
    {
        // always similar to itself
        *similarity_matrix.mut_at( row, row ) = true;

        let row_val = pattern.row( row );
        for row2 in row + 1 .. pattern.height
        {
            let row2_val = pattern.row( row2 );

            let equality = row_val == row2_val;
            *similarity_matrix.mut_at( row , row2 ) = equality;
        }
    }

    similarity_matrix
}

fn check_ne_diagonal( similarity_matrix: &Matrix< bool >, row: usize, col: usize ) -> bool
{
    let mut irow = row as isize - 1;
    let mut col  = col + 1;
    while irow >= 0 && col < similarity_matrix.width
    {
        if !similarity_matrix.at( irow as usize, col )
        {
            return false;
        }

        irow -= 1;
        col  += 1;
    }

    true
}

fn find_reflection_line( similarity_matrix: &Matrix< bool > ) -> Vec< usize >
{
    let mut reflection_lines: Vec< usize > = Vec::new();

    for col in 0 .. similarity_matrix.width - 1
    {
        // if two consecutive true's in similarity matrix, this is a candidate for
        // a solution
        if *similarity_matrix.at( col, col + 1 )
        {
            // possible candidate - check if from ( col, col + 1 ) north-east diagonal has all true
            if check_ne_diagonal( similarity_matrix, col, col + 1 )
            {
                reflection_lines.push( col );
            }

        }
    }

    reflection_lines
}

fn find_reflections( current_pattern: &Matrix< u8 > ) -> ( Vec< usize >, Vec< usize > )
{
    let similar_rows = find_similar_rows( current_pattern );
    let similar_cols = find_similar_rows( &current_pattern.transposed() );

    // println!( "Similar rows:" ); similar_rows.print();
    // println!();
    // println!( "Similar cols: "); similar_cols.print();

    return ( find_reflection_line( &similar_rows ), find_reflection_line( &similar_cols ) )
}

fn calc_score( reflections: ( Option< usize >, Option< usize > ) ) -> usize
{
    let mut score = 0usize;

    if let Some( row ) = reflections.0
    {
        println!( "Reflection row: {}", row + 1 );
        score += 100 * ( row + 1 );
    }

    if let Some( col ) = reflections.1
    {
        println!( "Reflection col: {}", col + 1 );
        score += col + 1;
    }

    score
}

fn find_reflection_score( current_pattern: &Matrix< u8 > ) -> usize
{
    let reflections = find_reflections( current_pattern );

    calc_score( ( reflections.0.first().copied(), reflections.1.first().copied() ) )
}

fn find_reflection_score_with_smudge( current_pattern: &Matrix< u8 > ) -> usize
{
    // first find original reflections
    let reflections = find_reflections( current_pattern );

    let toggle = | ch: &mut u8 |
    {
        if *ch == '#' as u8
        {
            *ch = '.' as u8;
        }
        else if *ch == '.' as u8
        {
            *ch = '#' as u8;
        }
    };

    // now find smudge
    for r in 0 .. current_pattern.height
    {
        for c in 0 .. current_pattern.width
        {
            let mut modified_pattern = current_pattern.clone();

            toggle( modified_pattern.mut_at( r, c ) );

            let new_reflections = find_reflections( &modified_pattern );

            let mut possible_result: ( Option< usize >, Option< usize > ) = ( None, None );

            for x in new_reflections.0
            {
                if reflections.0.iter().all( | oldx | x != *oldx )
                {
                    possible_result.0 = Some( x );
                }
            }

            for y in new_reflections.1
            {
                if reflections.1.iter().all( | oldy | y != *oldy )
                {
                    possible_result.1 = Some( y );
                }
            }

            match possible_result
            {
                ( Some( _ ), None      ) |
                ( None     , Some( _ ) ) =>
                {
                    println!( "Found smudge at ({}, {})", r, c );
                    return calc_score( possible_result );
                },
                _ => {}
            };
        }
    }

    panic!( "Did not find smudge!" );
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut current_pattern: Matrix< u8 > = Matrix
    {
        data: Vec::new(),
        width: 0,
        height: 0,
    };

    let mut part_01 = 0usize;
    let mut part_02 = 0usize;

    for line in input.lines()
    {
        if !line.is_empty()
        {
            current_pattern.data.extend( line.as_bytes() );
            current_pattern.width = line.len();
            current_pattern.height += 1;
        }
        else
        {
            // current_pattern.print();
            part_01 += find_reflection_score            ( &current_pattern );
            part_02 += find_reflection_score_with_smudge( &current_pattern );

            current_pattern.reset();
            println!();
        }
    }

    // last pattern
    // current_pattern.print();
    part_01 += find_reflection_score            ( &current_pattern );
    part_02 += find_reflection_score_with_smudge( &current_pattern );
    println!();

    println!( "Part 01: {}", part_01 );
    println!( "Part 02: {}", part_02 );
}

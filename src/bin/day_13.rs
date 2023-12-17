use std::fmt::Display;

use transpose::transpose;

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

    fn reset( &mut self )
    {
        self.data.clear();
        self.width = 0;
        self.height = 0;
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

    fn row( &self, row: usize ) -> &[ T ]
    {
        &self.data[ ( row * self.width ) .. ( ( row + 1 ) * self.width ) ]
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

    fn transposed( &self ) -> Matrix< T >
    {
        let mut transposed_data: Vec< T > = Vec::with_capacity( self.width * self.height );

        unsafe
        {
            transposed_data.set_len( self.width * self.height );
        }

        transpose( &self.data, &mut transposed_data, self.width, self.height );

        Matrix
        {
            data: transposed_data,
            width: self.height,
            height: self.width,
        }
    }
}

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

fn find_reflection_line( similarity_matrix: &Matrix< bool > ) -> Option< usize >
{
    for col in 0 .. similarity_matrix.width - 1
    {
        // if two consecutive true's in similarity matrix, this is a candidate for
        // a solution
        if *similarity_matrix.at( col, col + 1 )
        {
            // possible candidate - check if from ( col, col + 1 ) north-east diagonal has all true
            if check_ne_diagonal( similarity_matrix, col, col + 1 )
            {
                return Some( col );
            }

        }
    }
    None
}

fn find_reflection_score( current_pattern: &Matrix< u8 > ) -> usize
{
    let similar_rows = find_similar_rows( current_pattern );
    let similar_cols = find_similar_rows( &current_pattern.transposed() );

    // println!( "Similar rows:" ); similar_rows.print();
    // println!();
    // println!( "Similar cols: "); similar_cols.print();

    let mut score = 0usize;

    if let Some( row ) = find_reflection_line( &similar_rows )
    {
        println!( "Reflection row: {}", row + 1 );
        score += 100 * ( row + 1 );
    }

    if let Some( col ) = find_reflection_line( &similar_cols )
    {
        println!( "Reflection col: {}", col + 1 );
        score += col + 1;
    }

    score
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
            part_01 += find_reflection_score( &current_pattern );

            current_pattern.reset();
            println!();
        }
    }

    // last pattern
    // current_pattern.print();
    part_01 += find_reflection_score( &current_pattern );
    println!();

    println!( "Part 01: {}", part_01 );
}

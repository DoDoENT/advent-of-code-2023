use std::fmt::Display;

use transpose::transpose;

#[ derive( Clone ) ]
pub struct Matrix< T >
{
    pub data: Vec< T >,
    pub width: usize,
    pub height: usize,
}

impl< T > Matrix< T > where T: Clone + Copy + Display
{
    pub fn new( width: usize, height: usize, fill: T ) -> Matrix< T >
    {
        Matrix
        {
            data: vec![ fill; height * width ],
            width,
            height
        }
    }

    pub fn reset( &mut self )
    {
        self.data.clear();
        self.width = 0;
        self.height = 0;
    }

    pub fn at( &self, row: usize, col: usize ) -> &T
    {
        let index = row * self.width + col;
        return &self.data[ index ];
    }

    pub fn mut_at( &mut self, row: usize, col: usize ) -> &mut T
    {
        let index = row * self.width + col;
        return &mut self.data[ index ];
    }

    pub fn row( &self, row: usize ) -> &[ T ]
    {
        &self.data[ ( row * self.width ) .. ( ( row + 1 ) * self.width ) ]
    }

    pub fn print( &self )
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

    pub fn transposed( &self ) -> Matrix< T >
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

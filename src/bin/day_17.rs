use utils::matrix;
use utils::matrix::Matrix;

use pathfinding::directed::astar::astar;

#[ derive( Clone, Copy, Debug, Eq, PartialEq ) ]
#[ repr( u8 ) ]
enum Direction
{
    Up,
    Right,
    Down,
    Left,
}

#[ derive( Clone, Copy, Debug ) ]
struct Pos
{
    row: isize,
    col: isize,
    num_steps_in_same_direction: usize,
    direction: Direction,
}

impl Pos
{
    fn manhattan_distance( &self, other: &Pos ) -> usize
    {
        self.row.abs_diff( other.row ) + self.col.abs_diff( other.col )
    }

    fn successors( &self, map: &Matrix< u8 >, max_steps_in_same_dir: usize ) -> Vec< ( Pos, usize ) >
    {
        let mut succ = Vec::new();

        // right, if possible
        if let Some( right ) = self.next( Direction::Right, max_steps_in_same_dir )
        {
            if right.col < map.width as isize
            {
                succ.push( ( right, ( map.at( right.row as usize, right.col as usize ) - b'0' ) as usize ) );
            }
        }

        // down, if possible
        if let Some( down ) = self.next( Direction::Down, max_steps_in_same_dir )
        {
            if down.row < map.height as isize
            {
                succ.push( ( down, ( map.at( down.row as usize, down.col as usize ) - b'0' ) as usize ) );
            }
        }

        // left, if possible
        if let Some( left ) = self.next( Direction::Down, max_steps_in_same_dir )
        {
            if left.col >= 0
            {
                succ.push( ( left, ( map.at( left.row as usize, left.col as usize ) - b'0' ) as usize ) );
            }
        }

        // up, if possible
        if let Some( up ) = self.next( Direction::Down, max_steps_in_same_dir )
        {
            if up.row >= 0
            {
                succ.push( ( up, ( map.at( up.row as usize, up.col as usize ) - b'0' ) as usize ) );
            }
        }

        succ
    }

    fn next( &self, direction: Direction, max_steps_in_same_dir: usize ) -> Option< Pos >
    {
        // check if we can go in requested direction
        if direction == self.direction
        {
            if self.num_steps_in_same_direction > max_steps_in_same_dir
            {
                return None;
            }
            else
            {
                return Some( self.next_in_direction( direction, self.num_steps_in_same_direction + 1 ) );
            }
        }
        else
        {
            return Some( self.next_in_direction( direction, 1 ) );
        }

    }

    fn next_in_direction( &self, direction: Direction, steps: usize ) -> Pos
    {
        match direction
        {
            Direction::Right =>
            {
                Pos
                {
                    row: self.row,
                    col: self.col + 1,
                    num_steps_in_same_direction: steps,
                    direction
                }
            },
            Direction::Up =>
            {
                Pos
                {
                    row: self.row - 1,
                    col: self.col,
                    num_steps_in_same_direction: steps,
                    direction
                }

            },
            Direction::Down =>
            {
                Pos
                {
                    row: self.row + 1,
                    col: self.col,
                    num_steps_in_same_direction: steps,
                    direction
                }
            },
            Direction::Left =>
            {
                Pos
                {
                    row: self.row,
                    col: self.col - 1,
                    num_steps_in_same_direction: steps,
                    direction
                }
            },
        }
    }
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let city_map = matrix::from_str_input( &input );
}

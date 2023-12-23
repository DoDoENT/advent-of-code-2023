use utils::matrix;
use utils::matrix::Matrix;

use pathfinding::directed::astar::astar;

#[ derive( Clone, Copy, Debug, Eq, PartialEq, Hash ) ]
#[ repr( u8 ) ]
enum Direction
{
    Up,
    Right,
    Down,
    Left,
}

impl Direction
{
    fn opposite( &self ) -> Direction
    {
        match self
        {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[ derive( Clone, Copy, Debug, Eq, Hash ) ]
struct Pos
{
    row: isize,
    col: isize,
    num_steps_in_same_direction: usize,
    direction: Direction,
}

impl PartialEq for Pos
{
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.direction == other.direction
    }
}

impl Pos
{
    fn manhattan_distance( &self, dest_row: usize, dest_col: usize ) -> usize
    {
        self.row.abs_diff( dest_row as isize ) + self.col.abs_diff( dest_col as isize )
    }

    fn successors( &self, map: &Matrix< u8 >, min_steps_in_same_dir: usize, max_steps_in_same_dir: usize ) -> Vec< ( Pos, usize ) >
    {
        let mut succ = Vec::new();

        // right, if possible
        if let Some( right ) = self.next( Direction::Right, min_steps_in_same_dir, max_steps_in_same_dir )
        {
            if right.col < map.width as isize
            {
                succ.push( ( right, ( map.at( right.row as usize, right.col as usize ) - b'0' ) as usize ) );
            }
        }

        // down, if possible
        if let Some( down ) = self.next( Direction::Down, min_steps_in_same_dir, max_steps_in_same_dir )
        {
            if down.row < map.height as isize
            {
                succ.push( ( down, ( map.at( down.row as usize, down.col as usize ) - b'0' ) as usize ) );
            }
        }

        // left, if possible
        if let Some( left ) = self.next( Direction::Left, min_steps_in_same_dir, max_steps_in_same_dir )
        {
            if left.col >= 0
            {
                succ.push( ( left, ( map.at( left.row as usize, left.col as usize ) - b'0' ) as usize ) );
            }
        }

        // up, if possible
        if let Some( up ) = self.next( Direction::Up, min_steps_in_same_dir, max_steps_in_same_dir )
        {
            if up.row >= 0
            {
                succ.push( ( up, ( map.at( up.row as usize, up.col as usize ) - b'0' ) as usize ) );
            }
        }

        succ
    }

    fn next( &self, direction: Direction, min_steps_in_same_dir: usize, max_steps_in_same_dir: usize ) -> Option< Pos >
    {
        if self.num_steps_in_same_direction == 0
        {
            // all directions allowed in the beginning
            return Some( self.next_in_direction( direction, 1 ) );
        }
        // check if we can go in requested direction
        if direction == self.direction.opposite()
        {
            // it's not allowed to return back where you came from
            return None;
        }
        if direction == self.direction
        {
            if self.num_steps_in_same_direction >= max_steps_in_same_dir
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
            if self.num_steps_in_same_direction < min_steps_in_same_dir
            {
                return None;
            }
            else
            {
                return Some( self.next_in_direction( direction, 1 ) );
            }
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

fn draw_result( city_map: &Matrix< u8 >, path: &[ Pos ] )
{
    let mut render_map = city_map.clone();

    for path_element in path
    {
        if path_element.num_steps_in_same_direction == 0
        {
            continue;
        }
        let ( row, col ) = ( path_element.row as usize, path_element.col as usize );
        match path_element.direction
        {
            Direction::Right =>
            {
                *render_map.mut_at( row, col ) = b'>';
            },
            Direction::Left =>
            {
                *render_map.mut_at( row, col ) = b'<';
            },
            Direction::Up =>
            {
                *render_map.mut_at( row, col ) = b'^';
            },
            Direction::Down =>
            {
                *render_map.mut_at( row, col ) = b'v';
            },
        }
    }

    render_map.print_chars();

}

fn min_path( city_map: &Matrix< u8 >, min_steps: usize, max_steps: usize ) -> Option< usize >
{
    let result = astar
    (
        &Pos { row: 0, col: 0, num_steps_in_same_direction: 0, direction: Direction::Up },
        | x: &Pos | x.successors( &city_map, min_steps, max_steps ),
        | x: &Pos | x.manhattan_distance( city_map.width, city_map.height ),
        | x: &Pos | x.col == city_map.width as isize - 1 && x.row == city_map.height as isize - 1 && x.num_steps_in_same_direction >= min_steps
    );

    if let Some( result ) = &result
    {
        draw_result( city_map, &result.0 );
    }

    Some( result?.1 )
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let city_map = matrix::from_str_input( &input );

    if let Some( part_01_result ) = min_path( &city_map, 0, 3 )
    {
        println!( "Part 01 result: {:?}", part_01_result );
    }
    else
    {
        println!( "Failed to find part 01 result!" );
    }

    println!();

    if let Some( part_02_result ) = min_path( &city_map, 4, 10 )
    {
        println!( "Part 02 result: {:?}", part_02_result );
    }
    else
    {
        println!( "Failed to find part 02 result!" );
    }
}

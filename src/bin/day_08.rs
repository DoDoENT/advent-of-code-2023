use std::{collections::HashMap, str::Chars};

use num_integer::lcm;

struct Node< 'a >
{
    left : &'a str,
    right: &'a str,
}

type Map< 'a > = HashMap< &'a str, Node< 'a > >;

fn parse_map< 'a, Iter >( lines: Iter ) -> Option< Map< 'a > >
where
    Iter: Iterator< Item = &'a str >,
{
    let mut result = Map::new();

    for line in lines
    {
        let ( key, value ) = line.split_once( '=' )?;

        let key = key.trim();
        let value = value.trim();

        let ( left, right ) = value[ 1 .. value.len() - 1 ].split_once( ',' )?;

        result.insert( key, Node { left, right: right.trim_start() } );
    }

    Some( result )
}

fn part_01_follow_instructions( instructions: Chars, map: &Map ) -> usize
{
    let mut num_steps = 0usize;

    let mut char_iter = instructions.cycle();
    let mut cur_node  = "AAA";

    if !map.contains_key( cur_node )
    {
        return num_steps;
    }

    while cur_node != "ZZZ"
    {
        num_steps += 1;

        let instruction = char_iter.next().unwrap();

        if instruction == 'L' { cur_node = map[ cur_node ].left ; }
        else                  { cur_node = map[ cur_node ].right; }
    }

    num_steps
}

fn part_02_follow_instructions( instructions: Chars, map: &Map ) -> usize
{
    let current_nodes: Vec< _ > = map.keys().filter
    (
        | x | x.ends_with( 'A' )
    ).collect();

    let mut individual_num_steps: Vec< usize > = Vec::new();

    for node in current_nodes
    {
        let mut num_steps = 0usize;
        let mut char_iter = instructions.clone().cycle();

        let mut cur_node = node;

        while !cur_node.ends_with( 'Z' )
        {
            num_steps += 1;
            let instruction = char_iter.next().unwrap();

            if instruction == 'L' { cur_node = &map[ cur_node ].left; }
            else                  { cur_node = &map[ cur_node ].right; }
        }

        individual_num_steps.push( num_steps );
    }

    least_common_multiple( &individual_num_steps )
}

fn least_common_multiple( nums: &[ usize ] ) -> usize
{
    let mut result = 1usize;

    for num in nums
    {
        result = lcm( result, *num );
    }

    result
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    lines.next();

    let map = parse_map( lines ).unwrap();

    let part_01_result = part_01_follow_instructions( instructions.chars(), &map );

    println!( "Part 01 solution: {}", part_01_result );

    let part_02_result = part_02_follow_instructions( instructions.chars(), &map );

    println!( "Part 02 solution: {}", part_02_result );

}

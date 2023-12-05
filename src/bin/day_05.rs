use std::collections::HashMap;

struct MapRange
{
    start: usize,
    dest : usize,
    len  : usize,
}

impl MapRange
{
    fn map_number( &self, number: usize ) -> Option< usize >
    {
        if number >= self.start && number < self.start + self.len
        {
            let offset = number - self.start;
            Some( self.dest + offset )
        }
        else
        {
            None
        }
    }

    fn new( line: &str ) -> MapRange
    {
        let mut nums = line.splitn(3, ' ');

        let dest  = nums.next().unwrap().parse().unwrap();
        let start = nums.next().unwrap().parse().unwrap();
        let len   = nums.next().unwrap().parse().unwrap();

        MapRange
        {
            start: start,
            dest: dest,
            len: len,
        }
    }
}

struct Map< 'a >
{
    map_ranges: Vec< MapRange >,
    destination: &'a str,
}

impl Map< '_ >
{
    fn new( header_line: &str ) -> ( Map, &str )
    {
        let ( map_name, _ ) = header_line.split_once( ' ' ).unwrap();

        let mut mappings = map_name.splitn( 3, '-' );

        let source = mappings.next().unwrap();
                     mappings.next(); // consume "to"
        let dest   = mappings.next().unwrap();

        let map = Map { map_ranges: Vec::new(), destination: dest };

        ( map, source )
    }

    fn map_number( &self, number: usize ) -> usize
    {
        for range in &self.map_ranges
        {
            let maybe_mapped = range.map_number( number );
            if maybe_mapped.is_some()
            {
                return maybe_mapped.unwrap();
            }
        }

        number
    }

    fn map_numbers( &self, numbers: &[usize] ) -> Vec< usize >
    {
        let number_mapper = | x: &usize | { self.map_number( *x ) };
        numbers.iter().map( number_mapper ).collect()
    }
}

struct PuzzleInput< 'a >
{
    seeds: Vec< usize >,
    mappings: HashMap< &'a str, Map< 'a > >,
}

impl PuzzleInput< '_ >
{
    fn new( input: &str ) -> PuzzleInput
    {
        let mut lines = input.lines();

        // first line, seeds
        let seed_line = lines.next().unwrap();

        let ( _, seed_desc ) = seed_line.split_once( ':' ).unwrap();

        let mut parsed_input = PuzzleInput
        {
            seeds: seed_desc.split_whitespace().map( str::parse::< usize > ).map( Result::unwrap ).collect(),
            mappings: HashMap::new()
        };

        let mut current_line = lines.next();

        while let Some( cur_lin ) = current_line
        {
            if cur_lin.is_empty()
            {
                current_line = lines.next();
                continue;
            }

            // parse mappings
            let ( mut map, source ) = Map::new( cur_lin );

            // parse map ranges
            current_line = lines.next();

            while let Some( cur_lin ) = current_line
            {
                if cur_lin.is_empty()
                {
                    break;
                }

                map.map_ranges.push( MapRange::new( cur_lin ) );
                current_line = lines.next();
            }

            parsed_input.mappings.insert( source, map );
        }

        parsed_input
    }
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let puzzle_input = PuzzleInput::new( &input );

    // part 1
    let mut category = "seed";
    let mut values   = puzzle_input.seeds;

    while category != "location"
    {
        let category_mapping = &puzzle_input.mappings[ category ];

        values   = category_mapping.map_numbers( &values );
        category = category_mapping.destination;
    }

    let min_location = values.iter().min().unwrap();

    println!( "Part 1 minimum location is: {}", min_location );
}

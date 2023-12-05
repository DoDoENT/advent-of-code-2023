use std::collections::{HashMap, VecDeque};

struct Range
{
    start: usize,
    len  : usize,
}

struct MapRange
{
    start: usize,
    dest : usize,
    len  : usize,
}

struct MappedRange
{
    mapped  : Option< Range >,
    unmapped: Option< Vec< Range > >,
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

    fn map_range( &self, range: Range ) -> MappedRange
    {
        // range is fully left or fully right of mapping range o
        if range.start + range.len < self.start || range.start >= self.start + self.len
        {
            return MappedRange
            {
                mapped: None,
                unmapped: Some( vec![ range ] ),
            };
        }
        // range has left overlap with mapping range
        else if range.start < self.start && range.start + range.len >= self.start && range.start + range.len < self.start + self.len 
        {
            return MappedRange
            {
                mapped: Some( Range{ start: self.map_number( self.start ).unwrap(), len: range.start + range.len - self.start } ),
                unmapped: Some( vec![ Range{ start: range.start, len: self.start - range.start } ] ),
            };  
        }
        // range is fully withing mapping range
        else if range.start >= self.start && range.start + range.len < self.start + self.len
        {
            return MappedRange
            {
                mapped: Some( Range{ start: self.map_number( range.start ).unwrap(), len: range.len } ),
                unmapped: None,
            };
        }
        // range has right overlap with mapping range
        else if range.start >= self.start && range.start < self.start + self.len && range.start + range.len >= self.start + self.len
        {
            return MappedRange
            {
                mapped: Some( Range{ start: self.map_number( range.start ).unwrap(), len: self.start + self.len - range.start } ),
                unmapped: Some( vec![ Range{ start: self.start + self.len, len: range.start + range.len - self.start - self.len } ] ),
            };
        }
        // range is larger than mapping range
        else if range.start < self.start && range.start + range.len >= self.start + self.len
        {
            return MappedRange
            {
                mapped: Some( Range{ start: self.dest, len: self.len } ),
                unmapped: Some
                (
                    vec!
                    [
                        Range{ start: range.start, len: self.start - range.start },
                        Range{ start: self.start + self.len, len: range.start + range.len - self.start - self.len },
                    ]
                ),
            };
        }

        panic!( "unhandled case!" );
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
            if let Some( x ) = maybe_mapped
            {
                return x;
            }
        }

        number
    }

    fn map_numbers( &self, numbers: &[usize] ) -> Vec< usize >
    {
        let number_mapper = | x: &usize | { self.map_number( *x ) };
        numbers.iter().map( number_mapper ).collect()
    }

    fn map_range( &self, range: Range ) -> Vec< Range >
    {
        let mut result: Vec< Range > = Vec::new();

        let mut unmapped: VecDeque< Range > = VecDeque::new();
        unmapped.push_back( range );

        for map_range in &self.map_ranges
        {
            let mut new_unmapped: VecDeque< Range > = VecDeque::new();
            while let Some( unmapped_range ) = unmapped.pop_front()
            {
                let mapped_range = map_range.map_range( unmapped_range );

                if let Some( mapped_range ) = mapped_range.mapped
                {
                    result.push( mapped_range );
                }
                if let Some( unmapped_ranges ) = mapped_range.unmapped
                {
                    for unmapped_range in unmapped_ranges
                    {
                        new_unmapped.push_back( unmapped_range );
                    }
                }
            }
            unmapped = new_unmapped;
        }
        
        // no mapping for those
        for unmapped_range in unmapped
        {
            result.push( unmapped_range );
        }

        result 
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
    let mut values   = puzzle_input.seeds.clone();

    while category != "location"
    {
        let category_mapping = &puzzle_input.mappings[ category ];

        values   = category_mapping.map_numbers( &values );
        category = category_mapping.destination;
    }

    let min_location = values.iter().min().unwrap();

    println!( "Part 1 minimum location is: {}", min_location );

    // part 2
    let mut ranges: Vec< Range > = puzzle_input.seeds.chunks_exact( 2 ).map
    (
        | chunk: &[usize] |
        {
            Range{ start: chunk[ 0 ], len: chunk[ 1 ] }
        }
    ).collect();
    
    let mut category = "seed";
    while category != "location"
    {
        let category_mapping = &puzzle_input.mappings[ category ];

        let mut new_ranges: Vec< Range > = Vec::new();

        for range in ranges
        {
            new_ranges.extend( category_mapping.map_range( range ) );
        }

        category = category_mapping.destination;
        ranges = new_ranges;
    }

    let min_location = ranges.iter().min_by( | x, y | { x.start.cmp( &y.start ) } ).unwrap().start;

    println!( "Part 2 minimum location is: {}", min_location );
}

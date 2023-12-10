fn extrapolate( numbers: &[ i64 ] ) -> ( i64, i64 )
{
    let mut diffs: Vec< Vec< i64 > > = Vec::new();

    {
        let mut prev = numbers;

        while !prev.iter().all( |x| { *x == 0 } )
        {
            let diff = prev.windows( 2 ).map ( | x | { x[ 1 ] - x[ 0 ] }).collect();

            diffs.push( diff );
            prev = diffs.last().unwrap();
        }
    }

    let mut added_number_begin = 0i64;
    let mut added_number_end   = 0i64;

    for diff in diffs.iter().rev()
    {
        added_number_end = added_number_end + diff.last().unwrap();
        added_number_begin = diff.first().unwrap() - added_number_begin;
    }

    added_number_end = added_number_end + numbers.last().unwrap();
    added_number_begin = numbers.first().unwrap() - added_number_begin;

    ( added_number_begin, added_number_end )
}


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_01_solution = 0i64;
    let mut part_02_solution = 0i64;

    for line in input.lines()
    {
        let numbers: Vec< _ > = line.split_whitespace().map( str::parse::< i64 > ).map( Result::unwrap ).collect();

        let extrapolation = extrapolate( &numbers );

        println!( "{} {}", extrapolation.0, extrapolation.1 );

        part_01_solution += extrapolation.1;
        part_02_solution += extrapolation.0;
    }

    println!( "Part 01 solution: {}", part_01_solution );
    println!( "Part 02 solution: {}", part_02_solution );
}

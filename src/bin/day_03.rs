fn is_part_number( line_index: usize, lines: &[ &str ], first_digit_index: usize, number_length: usize ) -> bool
{
    let sanitized_pos_begin = if first_digit_index > 0 { first_digit_index - 1 } else { first_digit_index };

    let lines_length = lines[ line_index ].len(); // assume all lines are equal length

    let sanitized_pos_end = if first_digit_index + number_length >= lines_length { first_digit_index + number_length } else { first_digit_index + number_length + 1 };

    let not_symbol = | c: char | { c == '.' || c.is_digit( 10 ) };

    let no_symbols = | slice: &str | { slice.chars().all( not_symbol ) };

    if first_digit_index > 0 && !not_symbol( lines[ line_index ].chars().nth( first_digit_index - 1 ).unwrap() )
    {
        return true;
    }

    if first_digit_index + number_length < lines_length - 1 && !not_symbol( lines[ line_index ].chars().nth( first_digit_index + number_length  ).unwrap() )
    {
        return true;
    }

    if line_index > 0
    {
        // check previous line
        let slice = &lines[ line_index - 1 ][ sanitized_pos_begin .. sanitized_pos_end ];

        if !no_symbols( slice )
        {
            return true
        }
    }

    if line_index < lines.len() - 1
    {
        // check next line

        let slice = &lines[ line_index + 1][ sanitized_pos_begin .. sanitized_pos_end ];

        if !no_symbols( slice )
        {
            return true;
        }
    }

    false
}

fn deca_digit    ( c: char ) -> bool {  c.is_digit( 10 ) }
fn not_deca_digit( c: char ) -> bool { !c.is_digit( 10 ) }

fn sum_part_numbers_from_line( line_index: usize, lines: &[ &str ] ) -> usize
{
    let line = lines[ line_index ];

    let mut part_numbers_sum: usize = 0;

    let mut search_start_index: usize = 0;

    loop
    {
        let number_begin = line[ search_start_index .. ].find( deca_digit );

        match number_begin
        {
            None => { break; },
            Some( x ) =>
            {
                let first_digit_index = search_start_index + x;
                let number_end = line[ first_digit_index .. ].find( not_deca_digit );

                let number_length = match number_end
                {
                    None => { line[ first_digit_index .. ].len() },
                    Some( y ) => { y },
                };

                let part_number_candidate: usize = line[ first_digit_index .. ( first_digit_index + number_length ) ].parse().unwrap();

                if is_part_number( line_index, lines, first_digit_index, number_length )
                {
                    part_numbers_sum += part_number_candidate;
                }

                if first_digit_index + number_length < line.len() - 1
                {
                    search_start_index = first_digit_index + number_length + 1;
                }
                else
                {
                    break;
                }
            }
        };
    }

    part_numbers_sum
}

enum AdjacentNumbers
{
    Single{ num: usize },
    Dual{ left: Option< usize >, right: Option< usize > },
}

fn adjacent_number_in_line( line: &str, gear_position: usize ) -> AdjacentNumbers
{
    let center = line.chars().nth( gear_position ).unwrap();

    let find_left_begin = | end_pos: usize |
    {
        match line[ .. end_pos ].rfind( not_deca_digit )
        {
            None => { 0 },
            Some( x ) => { x + 1 },
        }
    };

    let find_right_end = | start_pos: usize |
    {
        match line[ start_pos .. ].find( not_deca_digit )
        {
            None => { line.len() },
            Some( x ) => { start_pos + x },
        }
    };

    // digit above or below gear - parse a single number
    if center.is_digit( 10 )
    {
        let number_begin = find_left_begin( gear_position );
        let number_end   = find_right_end ( gear_position );

        return AdjacentNumbers::Single{ num: line[ number_begin .. number_end ].parse().unwrap() };
    }
    else  // may have two numbers (left and right)
    {
        let mut left_number : Option< usize > = None;
        let mut right_number: Option< usize > = None;

        // left
        if gear_position > 0 && deca_digit( line.chars().nth( gear_position - 1 ).unwrap() )
        {
            let number_begin = find_left_begin( gear_position - 1 );

            left_number = Some( line[ number_begin .. gear_position ].parse().unwrap() );
        }

        // right
        if gear_position < line.len() - 1 && deca_digit( line.chars().nth( gear_position + 1 ).unwrap() )
        {
            let number_end = find_right_end( gear_position + 1 );

            right_number = Some( line[ ( gear_position + 1 ) .. number_end ].parse().unwrap() );
        }

        return AdjacentNumbers::Dual { left: left_number, right: right_number };
    }
}

fn get_gear_numbers( line_index: usize, lines: &[&str], gear_position: usize ) -> ( Option< usize >, Option< usize > )
{
    let mut numbers: [ Option< usize >; 2 ] = [ None, None ];

    let mut write_pos = 0;

    let mut analyze_line = | line_index: usize, write_pos: &mut usize |
    {
        let mut write_number = | x: Option< usize > |
        {
            match x
            {
                None => {},
                Some( _ ) =>
                {
                    numbers[ *write_pos ] = x;
                    *write_pos += 1;
                }
            }
        };

        match adjacent_number_in_line( lines[ line_index ], gear_position )
        {
            AdjacentNumbers::Single { num } => { write_number( Some( num ) ); }
            AdjacentNumbers::Dual { left, right } =>
            {
                write_number( left  );
                write_number( right );
            }
        }
    };

    // line above
    if line_index > 0
    {
        analyze_line( line_index - 1, &mut write_pos );
    }
    if write_pos < 2
    {
        analyze_line( line_index, &mut write_pos );
    }
    if write_pos < 2 && line_index < lines.len() - 1
    {
        analyze_line( line_index + 1, &mut write_pos );
    }

    ( numbers[ 0 ], numbers[ 1 ] )
}

fn sum_gear_ratios_from_line( line_index: usize, lines: &[&str] ) -> usize
{
    let mut gear_sum: usize = 0;

    let line = lines[ line_index ];

    let mut gear_search_index: usize = 0;

    loop
    {
        let gear_candidate = line[ gear_search_index .. ].find( '*' );

        match gear_candidate
        {
            None => { break; },
            Some( x ) =>
            {
                let gear_absolute_index = x + gear_search_index;

                gear_sum += match get_gear_numbers( line_index, lines, gear_absolute_index )
                {
                    ( Some( x ), Some( y ) ) => { x * y },
                    _ => { 0 },
                };

                if gear_absolute_index < line.len() - 1
                {
                    gear_search_index = gear_absolute_index + 1;
                }
                else
                {
                    break;
                }
            }
        }
    }

    gear_sum
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_number_sum: usize = 0;
    let mut gear_ratio_sum: usize = 0;
    let lines: Vec< &str > = input.lines().collect();
    for line_index in 0 .. lines.len()
    {
        part_number_sum += sum_part_numbers_from_line( line_index, &lines );
        gear_ratio_sum  += sum_gear_ratios_from_line ( line_index, &lines );
    }

    println!( "Total sum of part numbers: {}", part_number_sum );
    println!( "Total sum of gear ratios: {}", gear_ratio_sum );
}

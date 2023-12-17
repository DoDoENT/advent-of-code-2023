fn hash( input: &str ) -> usize
{
    let mut cur_val = 0usize;

    for c in input.as_bytes()
    {
        if *c == '\n' as u8
        {
            continue;
        }
        cur_val += *c as usize;
        cur_val *= 17;
        cur_val = cur_val % 256;
    }

    cur_val
}

struct Lens< 'a >
{
    label: &'a str,
    focal_length: u8,
}

enum Instruction< 'a >
{
    Add( Lens< 'a > ),
    Remove( &'a str ),
}

fn parse_instruction( input: &str ) -> Instruction
{
    let non_ascii_pos = input
        .find( | x: char | !x.is_ascii_alphabetic() )
        .unwrap();

    let label = &input[ 0 .. non_ascii_pos ];

    if input.chars().nth( non_ascii_pos ).unwrap() == '-'
    {
        return Instruction::Remove( label );
    }
    else
    {
        let focal_len = input.chars().nth( non_ascii_pos + 1 ).unwrap() as u8 - '0' as u8;

        return Instruction::Add( Lens{ label, focal_length: focal_len } );
    }
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_01_sum = 0usize;

    let mut boxes: Vec< Vec< Lens > > = Vec::with_capacity( 256 );
    boxes.resize_with( 256, Vec::new );

    for slice in input.split( ',' )
    {
        let h = hash( slice );
        println!( "{} becomes {}", slice, h );
        part_01_sum += h;

        match parse_instruction( slice )
        {
            Instruction::Remove( label ) =>
            {
                let h = hash( label );

                let vec = &mut boxes[ h ];

                if let Some( pos ) = vec.iter().position( | elem | elem.label == label )
                {
                    vec.remove( pos );
                }
            }
            Instruction::Add( lens ) =>
            {
                let h = hash( lens.label );

                let vec = &mut boxes[ h ];

                if let Some( old_lens ) = vec.iter_mut().find( | elem | elem.label == lens.label )
                {
                    *old_lens = lens;
                }
                else
                {
                    vec.push( lens );
                }
            }
        }
    }

    println!( "Part 01: {}", part_01_sum );

    // now calculate the focusing power

    let mut total_focus_power = 0usize;

    for ( box_index, box_contents ) in boxes.iter().enumerate()
    {
        for ( lens_index, lens ) in box_contents.iter().enumerate()
        {
            let focus_power = ( box_index + 1 ) * ( lens_index + 1 ) * lens.focal_length as usize;

            total_focus_power += focus_power;

            println!( "{}: {}", lens.label, focus_power );
        }
    }

    println!( "Part 02: {}", total_focus_power );
}

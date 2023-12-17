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


fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_01_sum = 0usize;

    for slice in input.split( ',' )
    {
        let h = hash( slice );
        println!( "{} becomes {}", slice, h );
        part_01_sum += h;
    }

    println!( "Part 01: {}", part_01_sum );
}


fn num_solutions( time: usize, record_distance: usize ) -> usize
{
    let ftime = time as f64;
    let fdist = record_distance as f64;
    let lower_bound = ( ftime - ( ( ftime * ftime - 4f64 * fdist ) as f64 ).sqrt() ) / 2f64;
    let upper_bound = ( ftime + ( ( ftime * ftime - 4f64 * fdist ) as f64 ).sqrt() ) / 2f64;

    let lower_bound = if lower_bound.ceil() == lower_bound { lower_bound + 1f64 } else { lower_bound.ceil() } as usize;
    let upper_bound = if upper_bound.floor() == upper_bound { upper_bound - 1f64 } else { upper_bound.floor() } as usize;

    println!( "For time={}, record={}, num_solutions={}", time, record_distance, upper_bound - lower_bound + 1 );

    upper_bound - lower_bound + 1
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut lines = input.lines();

    let ( _, times ) = lines.next().unwrap().split_once( ':' ).unwrap();
    let ( _, distances ) = lines.next().unwrap().split_once( ':' ).unwrap();

    {
        let times = times.split_whitespace().map( str::parse::< usize > ).map( Result::unwrap );
        let distances = distances.split_whitespace().map( str::parse::< usize > ).map( Result::unwrap );

        let mut part_01_solution: usize = 1;

        for ( time, distance ) in times.zip( distances )
        {
            let num_races = num_solutions( time, distance );
            part_01_solution *= num_races;
        }

        println!( "Part 01 solution: {}", part_01_solution );
    }

    {
        let single_time: usize = times.replace( " ", "" ).parse().unwrap();
        let single_dist: usize = distances.replace( " ", "" ).parse().unwrap();

        let part_02_solution = num_solutions( single_time, single_dist );

        println!( "Part 02 solution: {}", part_02_solution );
    }
}

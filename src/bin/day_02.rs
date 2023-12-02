#[derive( Debug, PartialEq, Clone )]
struct CubeSet
{
    red  : u32,
    green: u32,
    blue : u32
}

impl CubeSet {
    fn max( &mut self, other: CubeSet )
    {
        self.red   = self.red  .max( other.red   );
        self.green = self.green.max( other.green );
        self.blue  = self.blue .max( other.blue  );
    }

    fn power( &self ) -> u32
    {
        self.red * self.green * self.blue
    }
}

fn parse_set( set_description: &str ) -> CubeSet
{
    let mut cubes = CubeSet { red: 0, green: 0, blue: 0 };

    for desc in set_description.split( ',' )
    {
        let desc        = desc.trim();
        let number: u32 = desc.split( ' ' ).next().unwrap().parse().unwrap();

        let cube_type = desc.split(' ').nth( 1 ).unwrap();

        match cube_type {
            "red"   => { cubes.red   += number; },
            "green" => { cubes.green += number; },
            "blue"  => { cubes.blue  += number; },
            &_      => {                        },
        }
    }

    cubes
}

#[test]
fn test_set_parsing()
{
    assert_eq!( parse_set( "4 blue, 5 red"   ), CubeSet{ red: 5, green:  0, blue: 4 } );
    assert_eq!( parse_set( "19 green, 1 red" ), CubeSet{ red: 1, green: 19, blue: 0 } );
}

fn is_game_possible( game: CubeSet, bag_contents: &CubeSet ) -> bool
{
    game.red   <= bag_contents.red &&
    game.green <= bag_contents.green &&
    game.blue  <= bag_contents.blue
}

#[derive( Debug, Clone )]
struct Analysis
{
    game_id      : u32,
    game_possible: bool,
    min_game_set : CubeSet,
}

fn analyze_line( line: &str, bag_contents: &CubeSet ) -> Analysis
{
    const GAME: &str = "Game ";

    let game_id_begin = &line[ GAME.len() .. ];
    let game_id_end   = game_id_begin.find( ':' ).unwrap();

    let mut result = Analysis
    {
        game_id      : game_id_begin[ .. game_id_end ].parse().unwrap(),
        game_possible: true,
        min_game_set : CubeSet{ red: 0, green: 0, blue: 0 },
    };

    let game_description = &game_id_begin[ ( game_id_end + 1 ) .. ];

    for game_set in game_description.split( ';' )
    {
        let cube_set = parse_set( game_set );

        result.game_possible = result.game_possible && is_game_possible( cube_set.clone(), bag_contents );
        result.min_game_set.max( cube_set );
    }

    result
}

const BAG_CONTENTS: &CubeSet = &CubeSet{ red: 12, green: 13, blue: 14 };

#[test]
fn test_line_analysis()
{
    assert_eq!(  analyze_line( "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"        , BAG_CONTENTS ).game_id, 2    );
    assert!   ( !analyze_line( "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", BAG_CONTENTS ).game_possible );
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut part_01_sum: u32 = 0;
    let mut part_02_sum: u32 = 0;

    for line in input.lines()
    {
        let analysis = analyze_line( line, BAG_CONTENTS );

        if analysis.game_possible
        {
            part_01_sum += analysis.game_id;
        }

        part_02_sum += analysis.min_game_set.power();
    }

    println!( "Total sum (part 01): {}", part_01_sum );
    println!( "Total sum (part 02): {}", part_02_sum );
}

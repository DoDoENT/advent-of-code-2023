use std::collections::{ HashMap, VecDeque };

#[ derive( Debug ) ]
struct Part
{
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Part
{
    fn rating( &self ) -> isize
    {
        self.x + self.m + self.a + self.s
    }
}

#[ derive( Debug, Clone, Copy ) ]
struct Parts
{
    x: ( isize, isize ),
    m: ( isize, isize ),
    a: ( isize, isize ),
    s: ( isize, isize ),
}

impl Parts
{
    fn split_x( &self, operator: char, compare: isize ) -> ( Parts, Parts )
    {
        let splitted = split_interval( operator, compare, self.x );
        (
            Parts{ x: splitted.0, m: self.m, a: self.a, s: self.s },
            Parts{ x: splitted.1, m: self.m, a: self.a, s: self.s },
        )
    }

    fn split_m( &self, operator: char, compare: isize ) -> ( Parts, Parts )
    {
        let splitted = split_interval( operator, compare, self.m );
        (
            Parts{ x: self.x, m: splitted.0, a: self.a, s: self.s },
            Parts{ x: self.x, m: splitted.1, a: self.a, s: self.s },
        )
    }

    fn split_a( &self, operator: char, compare: isize ) -> ( Parts, Parts )
    {
        let splitted = split_interval( operator, compare, self.a );
        (
            Parts{ x: self.x, m: self.m, a: splitted.0, s: self.s },
            Parts{ x: self.x, m: self.m, a: splitted.1, s: self.s },
        )
    }

    fn split_s( &self, operator: char, compare: isize ) -> ( Parts, Parts )
    {
        let splitted = split_interval( operator, compare, self.s );
        (
            Parts{ x: self.x, m: self.m, a: self.a, s: splitted.0 },
            Parts{ x: self.x, m: self.m, a: self.a, s: splitted.1 },
        )
    }

    fn num_combinations( &self ) -> isize
    {
        ( self.x.1 - self.x.0 + 1 ) * ( self.m.1 - self.m.0 + 1 ) * ( self.a.1 - self.a.0 + 1 ) * ( self.s.1 - self.s.0 + 1 )
    }
}

#[ derive( Debug ) ]
struct Rule< 'a >
{
    condition: Option< &'a str >,
    destination: &'a str,
}

fn condition_satisfied( operator: char, compare: isize, value: isize ) -> bool
{
    match operator
    {
        '<' => { value < compare },
        '>' => { value > compare },
        _   => { panic!( "Unknown operator!" ); }
    }
}

// first element of the result tuple satisfies condition, second does not
fn split_interval( operator: char, compare: isize, value: ( isize, isize ) ) -> ( ( isize, isize ), ( isize, isize ) )
{
    assert!( compare >= value.0 && compare <= value.1 );
    match operator
    {
        '<' => { ( ( value.0, compare - 1 ), ( compare, value.1 ) ) },
        '>' => { ( ( compare + 1, value.1 ), ( value.0, compare ) ) },
        _   => { panic!( "Unknown operator!" ); }
    }
}

impl Rule< '_ >
{
    fn apply( &self, part: &Part ) -> Option< &str >
    {
        if let Some( condition ) = self.condition
        {
            let compare: isize = condition[ 2 .. ].parse().unwrap();
            let property = condition.chars().nth( 0 ).unwrap();
            let operator = condition.chars().nth( 1 ).unwrap();

            let satisfied = match property
            {
                'x' => condition_satisfied( operator, compare, part.x ),
                'm' => condition_satisfied( operator, compare, part.m ),
                'a' => condition_satisfied( operator, compare, part.a ),
                's' => condition_satisfied( operator, compare, part.s ),
                _ => { panic!( "Unkown part category!" ); }
            };

            if satisfied
            {
                Some( self.destination )
            }
            else
            {
                None
            }
        }
        else
        {
            Some( self.destination )
        }
    }

    fn apply_multiple( &self, parts: Parts ) -> ( ( &str, Parts ), Option< Parts > )
    {
        if let Some( condition ) = self.condition
        {
            let compare: isize = condition[ 2 .. ].parse().unwrap();
            let property = condition.chars().nth( 0 ).unwrap();
            let operator = condition.chars().nth( 1 ).unwrap();

            let ( applied, non_applied ) = match property
            {
                'x' => parts.split_x( operator, compare ),
                'm' => parts.split_m( operator, compare ),
                'a' => parts.split_a( operator, compare ),
                's' => parts.split_s( operator, compare ),
                _ => { panic!( "Unknown category!" ); }
            };

            ( ( self.destination, applied ), Some( non_applied ) )
        }
        else
        {
            ( ( self.destination, parts ), None )
        }
    }
}

fn parse_workflow( line: &str ) -> ( &str, Vec< Rule > )
{
    let brace_pos = line.find( '{' ).unwrap();

    let name = &line[ 0 .. brace_pos ];

    let rules_str = &line[ brace_pos + 1 .. line.len() - 1 ];
    let rules_split = rules_str.split( ',' );

    let mut rules = Vec::new();

    for rule_str in rules_split
    {
        if let Some( ( cond, dest ) ) = rule_str.split_once( ':' )
        {
            rules.push( Rule{ condition: Some( cond ), destination: dest } );
        }
        else
        {
            // no condition
            rules.push( Rule{ condition: None, destination: rule_str } );
        }
    }

    ( name, rules )
}

fn parse_part( line: &str ) -> Part
{
    let mut xmas = line[ 1 .. line.len() - 1 ].split( ',' );

    let x: isize = xmas.next().unwrap().split( '=' ).nth( 1 ).unwrap().parse().unwrap();
    let m: isize = xmas.next().unwrap().split( '=' ).nth( 1 ).unwrap().parse().unwrap();
    let a: isize = xmas.next().unwrap().split( '=' ).nth( 1 ).unwrap().parse().unwrap();
    let s: isize = xmas.next().unwrap().split( '=' ).nth( 1 ).unwrap().parse().unwrap();

    Part{ x, m, a, s }
}

fn process_part( part: &Part, workflows: &HashMap< &str, Vec< Rule > > ) -> bool
{
    let mut workflow = &workflows[ "in" ];

    print!( "in => " );

    loop
    {
        for rule in workflow
        {
            if let Some( dest ) = rule.apply( part )
            {
                match dest
                {
                    "A" =>
                    {
                        println!( "A" );
                        return true;
                    },
                    "R" =>
                    {
                        println!( "R" );
                        return false;
                    },
                    next_workflow =>
                    {
                        workflow = &workflows[ next_workflow ];
                        print!( "{} => ", next_workflow );
                        break;
                    }
                }
            }
        }
    }
}

fn solve_part01( parts: &[ Part ], workflows: &HashMap< &str, Vec< Rule > > )
{
    let mut ratings_sum = 0isize;

    for part in parts
    {
        println!( "Processing part {:?}", part );
        if process_part( part, workflows )
        {
            ratings_sum += part.rating();
        }
    }

    println!( "Part 01: {}", ratings_sum );
}

fn solve_part02( workflows: &HashMap< &str, Vec< Rule > > )
{
    let mut total_accepted = 0isize;

    let all_parts = Parts
    {
        x: ( 1, 4000 ),
        m: ( 1, 4000 ),
        a: ( 1, 4000 ),
        s: ( 1, 4000 ),
    };

    let mut process_queue: VecDeque< ( &str, Parts ) > = VecDeque::new();
    process_queue.push_back( ( "in", all_parts ) );

    while let Some( task ) = process_queue.pop_front()
    {
        let     workflow = &workflows[ task.0 ];
        let mut parts    = task.1;

        for rule in workflow
        {
            let ( applied, rest ) = rule.apply_multiple( parts );

            if let Some( pts ) = rest
            {
                parts = pts;
            }

            match applied.0
            {
                "A" => total_accepted += applied.1.num_combinations(),
                "R" => {},
                next_workflow =>
                {
                    process_queue.push_back( ( next_workflow, applied.1 ) );
                }
            }
        }
    }

    println!( "Total combinations accepted: {}", total_accepted );
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut workflows: HashMap< &str, Vec< Rule > > = HashMap::new();
    let mut parts: Vec< Part > = Vec::new();

    let mut parsing_workflows = true;

    for line in input.lines()
    {
        if parsing_workflows
        {
            if line.is_empty()
            {
                parsing_workflows = false;
                continue;
            }
            else
            {
                let ( name, rules ) = parse_workflow( line );

                workflows.insert( name, rules );
            }
        }
        else
        {
            parts.push( parse_part( line ) );
        }
    }

    solve_part01( &parts, &workflows );
    solve_part02( &workflows );
}

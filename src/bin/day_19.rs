use std::collections::HashMap;

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
}

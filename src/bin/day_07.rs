use core::panic;
use std::cmp::Ordering;

enum HandType
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType 
{
    fn as_num( &self ) -> usize
    {
        match &self
        {
            HandType::HighCard => { 1 },
            HandType::OnePair => { 2 },
            HandType::TwoPair => { 3 },
            HandType::ThreeOfAKind => { 4 },
            HandType::FullHouse => { 5 },
            HandType::FourOfAKind => { 6 },
            HandType::FiveOfAKind => { 7 },
        }
    }
}

struct Hand
{
    cards     : Vec< char >,
    bid       : usize,
    hand_type : HandType,
    num_jokers: usize,
}

fn card_index( card: char ) -> usize
{
    match card
    {
        '2' ..= '9' => { card as usize - '2' as usize },
        'T' => { 8 },
        'J' => { 9 },
        'Q' => { 10 },
        'K' => { 11 },
        'A' => { 12 },
        _   => { panic!( "unknown card" ) }
    }
}

fn part2_card_index( card: char ) -> isize 
{
    let index = card_index( card );
    if index == card_index( 'J' )
    {
        return -1;
    }
    else
    {
        return index as isize;
    }
}

impl Hand
{
    fn new( cards: Vec< char >, bid: usize ) -> Hand
    {
        let mut card_counts = [ 0usize; 13 ];

        for card in cards.iter()
        {
            card_counts[ card_index( *card ) ] += 1;  
        }

        let max_repeats = card_counts.iter().max().unwrap();
        let num_jokers  = card_counts[ card_index( 'J' ) ];

        let hand_type = match max_repeats
        {
            5 => { HandType::FiveOfAKind },
            4 => { HandType::FourOfAKind },
            3 =>
            {
                if card_counts.iter().find( | &&x | x == 2 ).is_some()
                {
                    HandType::FullHouse
                }
                else
                {
                    HandType::ThreeOfAKind
                }
            },
            2 =>
            {
                if card_counts.iter().filter( | &&x | x == 2 ).count() == 2
                {
                    HandType::TwoPair
                }
                else
                {
                    HandType::OnePair   
                }
            },
            _ => { HandType::HighCard }
        };

        Hand { cards, bid, hand_type, num_jokers }
    }
}

fn parse_hand( line: &str ) -> Hand
{
    let ( cards, bid ) = line.split_once( ' ' ).unwrap();

    assert_eq!( cards.len(), 5 );

    Hand::new( cards.chars().collect(), bid.parse().unwrap() )
}

fn compare_hands( first: &Hand, second: &Hand ) -> Ordering
{
    let val_first  = first.hand_type.as_num();
    let val_second = second.hand_type.as_num();

    let comparison = val_first.cmp( &val_second );
    if comparison == Ordering::Equal
    {
        for ( f, s ) in first.cards.iter().zip( second.cards.iter() )
        {
            let c = card_index( *f ).cmp( &card_index( *s ) );
            if c != Ordering::Equal { return c; }
        }
        return Ordering::Equal;
    }
    else
    {
        return comparison;
    }
}

fn compare_hands2( first: &Hand, second: &Hand ) -> Ordering
{
    let val_first  = first.hand_type.as_num() + first.num_jokers;
    let val_second = second.hand_type.as_num() + second.num_jokers;

    let comparison = val_first.cmp( &val_second );
    if comparison == Ordering::Equal
    {
        // better is the one with less jokers
        if first.num_jokers < second.num_jokers { return Ordering::Greater; }
        else if first.num_jokers > second.num_jokers { return Ordering::Less; }
        else
        {
            // same number of jokers, do the high-card, but treat J as the weakest
            for ( f, s ) in first.cards.iter().zip( second.cards.iter() )
            {
                let c = part2_card_index( *f ).cmp( &part2_card_index( *s ) );
                if c != Ordering::Equal { return c; }
            }
            return Ordering::Equal;
        }
    }
    else
    {
        return comparison;
    }
}

fn main()
{
    let file_path = std::env::args().nth( 1 ).unwrap();
    let input     = std::fs::read_to_string( file_path ).expect( "Failed to read file" );

    let mut hands: Vec< _ > = input.lines().map( | line | { parse_hand( line ) } ).collect();

    {
        let mut part_01_solution = 0usize;

        hands.sort_by( compare_hands );

        for ( index, hand ) in hands.iter().enumerate()
        {
            part_01_solution += ( index + 1 ) * hand.bid;
        }

        println!( "Part 01 solution: {}", part_01_solution );
    }

    {
        let mut part_02_solution = 0usize;

        hands.sort_by( compare_hands2 );

        for ( index, hand ) in hands.iter().enumerate()
        {
            part_02_solution += ( index + 1 ) * hand.bid;
        }

        println!( "Part 02 solution: {}", part_02_solution );
    }
}

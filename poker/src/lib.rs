use std::collections::HashSet;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
struct Card {
    value: u32,
    house: char,
} 
impl Card {
    fn new(input: &str) -> Card {
        let chars = &mut input.chars();
        let char_count = input.chars().count();
        if char_count == 3 {
            let value: u32 = 10;
            let house = chars.nth(2).unwrap();
            Card {value, house}
        } else if char_count == 2 {
            let num_char: char = chars.next().unwrap();
            let value: u32 = match num_char {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14, //Why can this also be 1!? Pain
                _ => 0,
            };
            let house = chars.next().unwrap();
            Card {value, house}
        }
        else {
            println!("You shouldn't be here");
            Card {value: 0, house: 'D'}
        }
    }
}

fn score_hand(mut hand: Vec<Card>) -> Vec<u32> {

    //Sort hand - this sorts by value then by house
    hand.sort();

    //Check if the houses match
    let mut hand_iter = hand.iter();
    let first_house = &hand_iter.next().unwrap().house; //Will panic on empty hand
    
    let same_house: bool = loop {
        let card = hand_iter.next();
        if card.is_some() { 
            if card.unwrap().house != *first_house { 
                break false; 
            }
        }
        if card.is_none() {
            break true 
        }
    };

    //Check for Straight
    let mut hand_iter = hand.iter();
    let first = hand_iter.next().unwrap().value;
    let mut index: u32 = 0;

    let mut is_sequential: bool = loop {
        index += 1;
        let card = hand_iter.next();
        if card.is_some() { 
            if card.unwrap().value != first + index { 
                break false; 
            }
        }
        if card.is_none() {
            break true 
        }
    };
    
    //This garbage handles the one case in which Ace is 1.
    if hand.get(0).unwrap().value == 2 &&
        hand.get(1).unwrap().value == 3 &&
        hand.get(2).unwrap().value == 4 &&
        hand.get(3).unwrap().value == 5 &&
        hand.get(4).unwrap().value == 14 {
        is_sequential = true;
        hand.remove(4); //No 14 for you!
        hand.insert(0, Card{value: 1, house: 'D'}); //This placeholder house is fine, becuase houses are already checked, and the hand strings are returned by index, not reconstructed
    }

    //Count duplicates
    let mut pairs: Vec<u32> = Vec::new();
    let mut three_of: Vec<u32> = Vec::new();
    let mut four_of: Vec<u32> = Vec::new();
    
    let hand_iter = hand.iter(); //You need to make new iters, because they get consumed on use
    let mut set: HashSet<u32> = HashSet::new();
    for card in hand_iter {
        if !set.insert(card.value) {
            if three_of.contains(&card.value) {
                three_of.retain(|x| *x != card.value);
                four_of.push(card.value);
            }
            if pairs.contains(&card.value) {
                pairs.retain(|x| *x != card.value);
                three_of.push(card.value);
            } else {
                pairs.push(card.value);
            } 
        }
    }

    //Scoring Section ------------------------------

    //The scores returned are a vec, containing the pattern (0-8) followed by each tiebraking value in prevelence order
    
    //8: Straight Flush
    //Poker rant: royal flush is listed seperately in poker, but it isn't a real pattern, It's just the highest value straight flush
    if is_sequential && same_house {
        return vec!(8, hand.last().unwrap().value);
    }

    //7: Quartet
    if four_of.len() == 1 {
        let mut hand_iter = hand.iter();
        let tiebraker = loop {
            let temp = hand_iter.next().unwrap().value;
            if temp != *four_of.get(0).unwrap() {
                break temp
            }
        };
        return vec!(7, *four_of.get(0).unwrap(), tiebraker)
    }

    //6: Full House
    if three_of.len() == 1 && pairs.len() == 1 {
        return vec!(6, *three_of.get(0).unwrap(), *pairs.get(0).unwrap()); 
        //Yes, this is the correct tiebraking order based on Poker rules (although you can only tiebrake this way if playing with more than one deck)
        //The test cases for this challenge are so thorough, this is actually tested
    }

    //5: Flush
    if same_house {
        hand.reverse();
        let mut returnval: Vec<u32> = Vec::new();
        returnval.push(5);
        for card in hand.iter() {
            returnval.push(card.value);
        }
        return returnval;
    }

    //4: Straight
    if is_sequential {
        return vec!(4, hand.last().unwrap().value);
    }

    //3: Three of
    if three_of.len() == 1 {
        let mut tiebrakers: Vec<u32> = Vec::new();
        for card in hand.iter() {
            if card.value != *three_of.get(0).unwrap() {
                tiebrakers.push(card.value);
            }
        };
        tiebrakers.sort();
        tiebrakers.reverse();
        let mut returnval: Vec<u32> = Vec::new();
        returnval.push(3);
        returnval.push(*three_of.get(0).unwrap());
        for tiebraker in tiebrakers.iter() {
            returnval.push(*tiebraker);
        }
        return returnval;
    }

    //2: Two pairs
    if pairs.len() == 2 {
        let mut tiebraker: u32 = 0;
        for card in hand.iter() {
            if card.value != *pairs.get(0).unwrap() && card.value != *pairs.get(1).unwrap() {
                tiebraker = card.value;
            }
        }
        if *pairs.get(0).unwrap() > *pairs.get(1).unwrap() {
            return vec!(2, *pairs.get(0).unwrap(), *pairs.get(1).unwrap(), tiebraker);
        } else {
            return vec!(2, *pairs.get(1).unwrap(), *pairs.get(0).unwrap(), tiebraker);
        }
    }

    //1: Pair
    if pairs.len() == 1 {
        let mut tiebrakers: Vec<u32> = Vec::new();
        for card in hand.iter() {
            if card.value != *pairs.get(0).unwrap() {
                tiebrakers.push(card.value);
            }
        };
        tiebrakers.sort();
        tiebrakers.reverse();
        let mut returnval: Vec<u32> = Vec::new();
        returnval.push(1);
        returnval.push(*pairs.get(0).unwrap());
        for tiebraker in tiebrakers.iter() {
            returnval.push(*tiebraker);
        }
        return returnval;
    }

    //0: No pattern
    hand.reverse();
    let mut returnval: Vec<u32> = Vec::new();
    returnval.push(0);
    for card in hand.iter() {
        returnval.push(card.value);
    }
    return returnval;
}
    

pub fn winning_hands<'a>(hand_strings: &[&'a str]) -> Vec<&'a str> { //Returns multiple hands on ties
    let mut hands: Vec<Vec<Card>> = Vec::new();
    for hand_string in hand_strings {
        let card_strings: Vec<&str> = hand_string.split_whitespace().collect();
        let mut hand: Vec<Card> = Vec::new();
        for card_string in card_strings {
            hand.push(Card::new(card_string));
        }
        hands.push(hand);
    }

    let mut highest_scoring_hands: Vec<usize> = Vec::new(); //Now stored by index
    let mut highscore: Vec<u32> = vec!(0,0,0,0,0);

    for (hand_index, hand) in hands.iter().enumerate() {
        let hand_copy = hand.clone();
        let score: Vec<u32> = score_hand(hand_copy);
        let score2 = score.clone();

        for (score_index, score_part) in score.iter().enumerate() {
            if score_part < highscore.get(score_index).unwrap() {
                break;
            } else if score_part > highscore.get(score_index).unwrap() { //New high score case
                highscore = score2;
                highest_scoring_hands.clear();
                highest_scoring_hands.push(hand_index);
                break;
            } else if score_index == highscore.len() - 1 { //True tie case
                highest_scoring_hands.push(hand_index);
                break;
            }
        }
    }

    let mut return_strings: Vec<&'a str> = Vec::new();
    for hand_index in highest_scoring_hands {
        return_strings.push(hand_strings.get(hand_index).unwrap());
        println!("{:?}", hand_strings.get(hand_index).unwrap());
    }

    return return_strings;
}

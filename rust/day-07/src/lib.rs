use std::cmp::Ordering;

use itertools::Itertools;

pub mod parser;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Card {
    Joker(Box<Card>) = 1,
    Tw,
    Th,
    Fo,
    Fi,
    Si,
    Se,
    Ei,
    Ni,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    pub fn new(label: char) -> Option<Card> {
        match label {
            '2' => Some(Card::Tw),
            '3' => Some(Card::Th),
            '4' => Some(Card::Fo),
            '5' => Some(Card::Fi),
            '6' => Some(Card::Si),
            '7' => Some(Card::Se),
            '8' => Some(Card::Ei),
            '9' => Some(Card::Ni),
            'T' => Some(Card::T),
            'J' => Some(Card::J),
            'Q' => Some(Card::Q),
            'K' => Some(Card::K),
            'A' => Some(Card::A),
            _ => None,
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: usize) -> Result<Hand, &'static str> {
        match Hand::get_hand_type(&cards) {
            Some(hand_type) => Ok(Hand {
                hand_type,
                cards,
                bid,
            }),
            None => Err("Unknown hand type."),
        }
    }

    pub fn get_winnings(&self, rank: usize) -> usize {
        self.bid * rank
    }

    fn get_hand_type(cards: &[Card; 5]) -> Option<HandType> {
        let counts = cards
            .into_iter()
            .filter(|card| match card {
                Card::Joker(_) => false,
                _ => true,
            })
            .counts();

        match counts.values().sorted().collect_vec()[..] {
            [5] | [4] | [3] | [2] | [1] | [] => Some(HandType::FiveOfAKind),
            [1, 4] | [1, 3] | [1, 2] | [1, 1] => Some(HandType::FourOfAKind),
            [2, 3] | [2, 2] => Some(HandType::FullHouse),
            [1, 1, 3] | [1, 1, 2] | [1, 1, 1] => Some(HandType::ThreeOfAKind),
            [1, 2, 2] => Some(HandType::TwoPair),
            [1, 1, 1, 2] | [1, 1, 1, 1] => Some(HandType::OnePair),
            [1, 1, 1, 1, 1] => Some(HandType::HighCard),
            _ => None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            cmp => cmp,
        }
    }
}

pub struct Camel {
    hands: Vec<Hand>,
}

impl Camel {
    pub fn new(draws: Vec<([Card; 5], usize)>, joker: Option<Card>) -> Result<Camel, &'static str> {
        let hands = draws
            .into_iter()
            .map(|(cards, bid)| {
                Hand::new(
                    cards.map(|card| match &joker {
                        Some(joker) if joker == &card => Card::Joker(Box::new(card)),
                        _ => card,
                    }),
                    bid,
                )
            })
            .collect::<Result<Vec<Hand>, &str>>()?;
        Ok(Camel { hands })
    }

    pub fn play(&mut self) -> usize {
        self.hands.sort();
        self.hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.get_winnings(idx + 1))
            .sum()
    }
}

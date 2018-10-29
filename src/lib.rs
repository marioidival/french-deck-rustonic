use std::ops::{Index, Range};

#[derive(Clone, Debug, PartialEq)]
struct Card {
    rank: String,
    suit: String,
}

struct FrenchDeck<T> {
    cards: Vec<T>,
    position: usize,
}

impl<'a, T> IntoIterator for &'a FrenchDeck<T> {
    type Item = &'a T;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

impl FrenchDeck<Card> {
    fn new() -> FrenchDeck<Card> {
        let ranks = vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ];
        let suits = vec!["spades", "diamonds", "clubs", "hearts"];
        let mut cards: Vec<Card> = Vec::new();

        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards.push(Card {
                    rank: format!("{}", rank),
                    suit: format!("{}", suit),
                })
            }
        }

        FrenchDeck { cards, position: 0 }
    }

    fn len(&self) -> usize {
        self.cards.len()
    }
}

impl<T> Index<usize> for FrenchDeck<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl<T> Index<Range<usize>> for FrenchDeck<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.cards.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize_deck() {
        let deck = FrenchDeck::new();
        assert_eq!(52, deck.len());
    }

    #[test]
    fn index_deck() {
        let deck = FrenchDeck::new();
        assert_eq!(
            Card {
                rank: "2".into(),
                suit: "spades".into()
            },
            deck[0]
        )
    }

    #[test]
    fn index_range_deck() {
        let deck = FrenchDeck::new();
        assert_eq!(
            [
                Card {
                    rank: "2".into(),
                    suit: "spades".into()
                },
                Card {
                    rank: "3".into(),
                    suit: "spades".into()
                }
            ],
            deck[0..2]
        )
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,

    Unknown,
}

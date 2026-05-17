// ALDS1_2_C: Stable Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_C
fn solve<'a>(cards: &'a [&'a str]) -> (Vec<&'a str>, bool, Vec<&'a str>, bool) {
    let cards: Vec<Card> = cards
        .iter()
        .map(|card| {
            let mut chars = card.chars();
            Card {
                raw: card,
                suit: chars.next().unwrap(),
                number: chars.next().unwrap().to_digit(10).unwrap() as usize,
            }
        })
        .collect();

    let sorted_bubble = bubble_sort(cards.clone());
    // bubble sort は安定
    let is_stable_bubble = true;

    let sorted_selection = selection_sort(cards.clone());
    let is_stable_selection = is_stable(&sorted_bubble, &sorted_selection);

    // 戻り値の型に変換
    let sorted_bubble: Vec<&'a str> = sorted_bubble.iter().map(|card| card.raw).collect();
    let sorted_selection = sorted_selection.iter().map(|card| card.raw).collect();

    (
        sorted_bubble,
        is_stable_bubble,
        sorted_selection,
        is_stable_selection,
    )
}

#[derive(Clone, PartialEq)]
struct Card<'a> {
    raw: &'a str,
    suit: char,
    number: usize,
}

fn bubble_sort(mut cards: Vec<Card>) -> Vec<Card> {
    for i in 0..(cards.len() - 1) {
        // 交換がなくなったタイミングで早期に break するため
        let mut was_swapped = false;
        for j in 0..(cards.len() - (i + 1)) {
            if cards[j].number > cards[j + 1].number {
                // swap method 便利だな
                cards.swap(j, j + 1);

                was_swapped = true;
            }
        }

        if !was_swapped {
            break;
        }
    }

    cards
}

fn selection_sort(mut cards: Vec<Card>) -> Vec<Card> {
    for i in 0..(cards.len() - 1) {
        let mut min_index = i;
        for j in i..cards.len() {
            min_index = if cards[j].number < cards[min_index].number {
                j
            } else {
                min_index
            };
        }

        if i != min_index {
            cards.swap(i, min_index);
        }
    }

    cards
}

// &[Card] にすることで、Vec 自体の機能を使っていない関数の一般性を上げることができる。
fn is_stable(sorted_bubble: &[Card], sorted_selection: &[Card]) -> bool {
    sorted_bubble.iter().eq(sorted_selection.iter())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(
            solve(&["H4", "C9", "S4", "D2", "C3"]),
            (
                vec!["D2", "C3", "H4", "S4", "C9"],
                true,
                vec!["D2", "C3", "S4", "H4", "C9"],
                false,
            )
        );
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            solve(&["S1", "H1"]),
            (vec!["S1", "H1"], true, vec!["S1", "H1"], true)
        );
    }
}

use std::collections::HashMap;

pub fn map_to_ranks<T: Eq + Ord>(map: &HashMap<String, T>) -> HashMap<String, u32> {
    map_to_ranks_with_sort(map, |a, b| b.cmp(a))
}

pub fn map_to_ranks_with_sort<T: Eq + Ord>(
    map: &HashMap<String, T>,
    sort_by: impl Fn(&T, &T) -> std::cmp::Ordering,
) -> HashMap<String, u32> {
    let mut sorted: Vec<_> = map.iter().collect();

    sorted.sort_by(|a, b| sort_by(a.1, b.1));

    let mut ranks = HashMap::new();

    for (rank, (word, _)) in sorted.iter().enumerate() {
        ranks.insert(word.to_string(), rank as u32 + 1);
    }

    ranks
}

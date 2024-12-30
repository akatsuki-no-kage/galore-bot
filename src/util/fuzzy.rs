use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::iter::ParallelIterator;

pub async fn fuzzy(data: impl ParallelIterator<Item = &String>, name: &str) -> Vec<String> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<_> = data
        .flat_map(|key| {
            matcher
                .fuzzy_match(key, name)
                .map(|score| (key.to_string(), score))
        })
        .collect();
    matches.sort_by(|(_, a), (_, b)| b.cmp(a));
    matches.into_iter().map(|(key, _)| key).collect()
}

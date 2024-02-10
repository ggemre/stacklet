/// Return `true` if s1 contains s2, and `false` otherwise.
pub fn exact_match(s1: &str, s2: &str) -> bool {
    s1.contains(s2)
}

/// Calculate the Levenshtein distance between strings s1 and s2.
///
/// Copied from Wikipedia:
///   > Informally, the Levenshtein distance between two words is the minimum number of
///   > single-character edits (insertions, deletions or substitutions) required to change
///   > one word into the other.
/// https://en.wikipedia.org/wiki/Levenshtein_distance
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    // initialize n x m matrix for storing intermediate distances
    // where n is length of s1 and m is length of s2
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    // populate the matrix according to the 2 strings
    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                matrix[i][j] = j;
            } else if j == 0 {
                matrix[i][j] = i;
            } else {
                // cost of editing string at this position, (i, j)
                let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                    0
                } else {
                    1
                };

                // update matrix with minimum cost
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
    }

    // return bottom right cell, (final Levenshtein distance)
    matrix[len1][len2]
}

/// Return `true` if s1 fuzzy matches s2, and `false` otherwise.
///
/// Uses the Levenshtein distance to calculate the editing distance between the two strings
/// and returns a boolean based on if the editing distance ratio is greater than or equal to
/// 0.5, (the ratio of editing distance to string length is less then 50%)
pub fn fuzzy_match(s1: &str, s2: &str) -> bool {
    let threshold: f64 = 0.5;

    let distance = levenshtein_distance(s1, s2);
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let ratio = 1.0 - (distance as f64 / (len1.max(len2) as f64));

    ratio >= threshold
}

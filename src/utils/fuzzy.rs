fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                matrix[i][j] = j;
            } else if j == 0 {
                matrix[i][j] = i;
            } else {
                let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                    0
                } else {
                    1
                };

                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
    }

    matrix[len1][len2]
}

pub fn fuzzy_match(s1: &str, s2: &str) -> bool {
    let threshold: f64 = 0.5;

    let distance = levenshtein_distance(s1, s2);
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    let ratio = 1.0 - (distance as f64 / (len1.max(len2) as f64));

    ratio >= threshold
}

use std::cmp::min;

pub fn edit_distance<T: Eq>(s: &[T], t: &[T]) -> usize {
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![n + m + 1; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            if i < n {
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j] + 1);
            }
            if j < m {
                dp[i][j + 1] = min(dp[i][j + 1], dp[i][j] + 1);
            }
            if i < n && j < m {
                dp[i + 1][j + 1] = min(
                    dp[i + 1][j + 1],
                    dp[i][j] + if s[i] != t[j] { 1 } else { 0 },
                );
            }
        }
    }
    dp[n][m]
}

pub fn z_algorithm<B: AsRef<[u8]>>(s: B) -> Vec<usize> {
    let s = s.as_ref();
    let mut z = vec![0; s.len()];
    let mut j = 0;
    for i in 1..s.len() {
        if i + z[i - j] < j + z[j] {
            z[i] = z[i - j];
        } else {
            let mut k = if j + z[j] > i { j + z[j] - i } else { 0 };
            while i + k < s.len() && s[k] == s[i + k] {
                k += 1;
            }
            z[i] = k;
            j = i;
        }
    }
    z[0] = s.len();
    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance_string() {
        let s = "biting".chars().collect::<Vec<char>>();
        let t = "whiten".chars().collect::<Vec<char>>();
        assert_eq!(edit_distance(&s, &t), 4);
        let s = String::from("icpc").chars().collect::<Vec<char>>();
        let t = String::from("icpc").chars().collect::<Vec<char>>();
        assert_eq!(edit_distance(&s, &t), 0);
    }

    #[test]
    fn test_edit_distance_vec_abc185e() {
        let s = vec![1, 3, 2, 4];
        let t = vec![1, 5, 2, 6, 4, 3];
        assert_eq!(edit_distance(&s, &t), 3);
    }

    #[test]
    fn test_z_algorithm() {
        let s = "ababa";
        let z = z_algorithm(&s);
        assert_eq!(z, vec![5, 0, 3, 0, 1]);
    }
}

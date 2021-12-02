pub fn depth_increase(soundings: &[u32], window_size: usize) -> usize {
    let sums: Vec<u32> = soundings
        .windows(window_size)
        .map(|group| group.iter().sum())
        .collect();

    sums.iter()
        .zip(sums.iter().skip(1))
        .filter(|(first, second)| first < second)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn depth_increase_count_matches_sample() {
        assert_eq!(depth_increase(&SAMPLE_DATA, 1), 7);
    }

    #[test]
    fn depth_increase_with_window_matches_sample() {
        assert_eq!(depth_increase(&SAMPLE_DATA, 3), 5);
    }
}

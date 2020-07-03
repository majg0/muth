use muth::Subdivision;

pub struct Config {
    subdivisions: Vec<Subdivision>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            subdivisions: vec![
                // 1
                vec![1],
                // 2
                vec![1, 1],
                // 3
                vec![1, 1, 1],
                vec![2, 1],
                vec![1, 2],
                // 4
                vec![1, 1, 1, 1],
                vec![2, 1, 1],
                vec![1, 2, 1],
                vec![1, 1, 2],
                vec![3, 1],
                vec![1, 3],
            ]
            .iter()
            .map(Subdivision::new)
            .collect(),
        }
    }
}

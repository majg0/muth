use super::Subdivision;

impl Subdivision {
    pub fn new(subs: &Vec<u64>) -> Subdivision {
        Subdivision {
            total: subs.iter().sum(),
            subs: subs.clone(),
        }
    }
}

/// Implement the trait IntoTab
///
use core::Core;

pub trait IntoTab {
    fn into_tab(&self) -> Vec<Vec<Core>>;
}

impl IntoTab for Vec<Core> {
    fn into_tab(&self) -> Vec<Vec<Core>> {
        let mut res = Vec::new();
        res.push(self.to_vec());
        res
    }
}

impl IntoTab for Vec<Vec<Core>> {
    fn into_tab(&self) -> Vec<Vec<Core>> {
        self.to_vec()
    }
}



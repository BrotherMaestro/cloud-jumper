use crate::region::Region;
use rand::Rng;

#[derive(Default)]
pub struct RegionSet {
    set: Vec<Region>,
}
impl RegionSet {
    // We have the sorted EXCLUSIONS, we need to establish an initial inclusive region...
    // Then we need to exclude each remaining non-overlapping region
    pub fn with_sorted(lower_bound: f32, upper_bound: f32, sorted: Vec<Region>) -> Self {
        // Maximum number of possible regions occurs when every exclusion causes a split
        let mut set = Vec::with_capacity(sorted.len() + 1);
        set.push(Region {
            lower: lower_bound,
            upper: upper_bound,
        });
        let mut new_set = RegionSet { set };
        for exclusion in sorted {
            new_set.apply_exclusion_to_last(exclusion);
        }
        new_set
    }

    // Return a random region in which we can populate a platform...
    // Maybe add width parameter for minimum width...
    // AND maybe loop until an appropriate region is found?
    pub fn random(&self) -> Option<Region> {
        let len = self.set.len();
        if len > 0 {
            let random_index = rand::rng().random_range(0..len);
            unsafe { Some(*self.set.get_unchecked(random_index)) }
        } else {
            None
        }
    }

    fn exclusion_pair(&self, exclusion: Region) -> (Option<Region>, Option<Region>) {
        if let Some(last) = self.set.last() {
            (
                last.exclude_above_point(exclusion.lower),
                last.exclude_below_point(exclusion.upper),
            )
        } else {
            (None, None)
        }
    }

    fn apply_exclusion_to_last(&mut self, exclusion: Region) {
        // NOTE: last WILL exist IFF exclusion_pair returns at least 1 Some variant
        match self.exclusion_pair(exclusion) {
            (Some(lower), Some(upper)) => {
                self.set.last_mut().unwrap().upper = lower.upper;
                self.set.push(upper);
            }
            (Some(lower), None) => {
                self.set.last_mut().unwrap().upper = lower.upper;
            }
            (None, Some(upper)) => {
                self.set.last_mut().unwrap().lower = upper.lower;
            }
            (None, None) => {
                self.set.pop();
            }
        }
    }
}

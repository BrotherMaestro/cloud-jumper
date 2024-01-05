struct Region {
    lower: f64,
    upper: f64,
}

impl Region {
    // Compare existing region with new lower bound, return intersection
    // Return None for invalid regions
    fn intersect_with_lower(&self, lower: f64) -> Option<Region> {
        let inner_boundary = lower.max(self.lower);
        if inner_boundary < self.upper {
            Some(Region {
                lower: inner_boundary,
                upper: self.upper,
            })
        } else {
            None
        }
    }

    // Compare existing region with new upper bound, return intersection
    // Return None for invalid regions
    fn intersect_with_upper(&self, upper: f64) -> Option<Region> {
        let inner_boundary = upper.min(self.upper);
        if inner_boundary > self.lower {
            Some(Region {
                lower: self.lower,
                upper: inner_boundary,
            })
        } else {
            None
        }
    }
}

// TODO: Implement some form of height balancing (difference of 1 or something)
struct RegionNode {
    root: Region,
    left: Option<Box<RegionNode>>,
    right: Option<Box<RegionNode>>,
}

pub struct RegionSet {
    head: Option<RegionNode>,
}

impl RegionSet {
    // Describe a new 1-dimensional region of real numbers between the lower and upper bounds
    pub fn new(lower: f64, upper: f64) -> Self {
        if upper < lower {
            panic!(
                "Region did not satisfy prerequisite! Lower ({lower}) must be less than upper ({upper}).")
        }
        Self {
            head: Some(RegionNode {
                root: Region { lower, upper },
                left: None,
                right: None,
            }),
        }
    }

    pub fn exclude_region(&mut self, lower_exclusion: f64, upper_exclusion: f64) {
        // implement iterator for RegionNode's....
        if let Some(firstNode) = &self.head {
            let lower_region = firstNode.root.intersect_with_upper(lower_exclusion);
            let upper_region = firstNode.root.intersect_with_lower(upper_exclusion);

            // Now try to interpret this result...
            // PREREQUISITE: Valid regions never overlap (only ever shrink, split or become excluded)
            // IF BOTH intersections return regions we have a split,
            // (alternatively we could have an unaffected result BUT, that would imply an invalid exlusion with upper < lower...)
            // IF ONE intersection returns a valid region, we have shrinking, or the origin region (test?)
            // IF NEITHER intersect, the region has been excluded!

            // FOR ONE REGION RETURNED
            // IF lower_region, then 'more right' regions may be effected -> traverse right branch.
            // Similarly, IF right_region, then 'more left' regions may be effected -> traverse left.

            // FOR EXCLUSION
            // Both more left, and right regions may be effected -> traverse both.

            // MATCHES can return values
            // let result = match x {...};
            match (lower_region, upper_region) {
                (None, None) => {
                    // Balance tree (move higher branch to this node)
                    // If no branches, set head to None!
                    // Traverse left + right (queue or recurse)
                }
                (Some(region), None) => {
                    // set this region to be root (it shrunk), and traverse right...
                    firstNode.root = region;
                }
                (None, Some(region)) => {
                    // set this region to be root (it shrunk), and traverse left...
                    firstNode.root = region;
                }
                (Some(lower_region), Some(upper_region)) => {
                    // Inserting... check for lesser branch, it will get the left over node
                    // Meanwhile, set the other region to be this 'root'
                    // firstNode.root = upper_region; // for example...
                }
            }
        }
    }
}

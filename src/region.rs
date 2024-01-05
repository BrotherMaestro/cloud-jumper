// Allow copy of region (allowing swaps via temp)
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Region {
    pub lower: f32,
    pub upper: f32,
}

impl Region {
    // Compare existing region with new lower boundary, return intersection
    // Return None for invalid regions
    pub fn exclude_below_point(&self, point: f32) -> Option<Region> {
        let inner_boundary = point.max(self.lower);
        if inner_boundary < self.upper {
            Some(Region {
                lower: inner_boundary,
                upper: self.upper,
            })
        } else {
            None
        }
    }

    // Compare existing region with new upper boundary, return intersection
    // Return None for invalid regions
    pub fn exclude_above_point(&self, point: f32) -> Option<Region> {
        let inner_boundary = point.min(self.upper);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_exclusion_tests() {
        let positive_region = Region {
            lower: 25.0,
            upper: 50.0,
        };
        let negative_region = Region {
            lower: -300.0,
            upper: -120.0,
        };
        let about_zero_region = Region {
            lower: -20.0,
            upper: 10.0,
        };
        let point_inside = |x: &Region| (x.lower + x.upper) / 2.0;
        let exclude_below_point_below = |x: &Region| x.exclude_below_point(x.lower - 5.0);
        let exclude_below_point_inside = |x: &Region| x.exclude_below_point(point_inside(x));
        let exclude_below_point_above = |x: &Region| x.exclude_below_point(x.upper + 5.0);
        let exclude_above_point_below = |x: &Region| x.exclude_above_point(x.lower - 5.0);
        let exclude_above_point_inside = |x: &Region| x.exclude_above_point(point_inside(x));
        let exclude_above_point_above = |x: &Region| x.exclude_above_point(x.upper + 5.0);

        // test various interesections (and non-intersections) on a positive-only region
        assert_eq!(
            exclude_below_point_inside(&positive_region),
            Some(Region {
                lower: point_inside(&positive_region),
                upper: positive_region.upper,
            })
        );
        assert_eq!(
            exclude_below_point_below(&positive_region),
            Some(positive_region),
        );
        assert_eq!(exclude_below_point_above(&positive_region), None);
        assert_eq!(
            exclude_above_point_inside(&positive_region),
            Some(Region {
                lower: positive_region.lower,
                upper: point_inside(&positive_region),
            })
        );
        assert_eq!(exclude_above_point_below(&positive_region), None);
        assert_eq!(
            exclude_above_point_above(&positive_region),
            Some(positive_region),
        );

        // test various intersections (and non-intersections) on a negative-only region
        assert_eq!(
            exclude_below_point_inside(&negative_region),
            Some(Region {
                lower: point_inside(&negative_region),
                upper: negative_region.upper,
            })
        );
        assert_eq!(
            exclude_below_point_below(&negative_region),
            Some(negative_region)
        );
        assert_eq!(exclude_below_point_above(&negative_region), None);
        assert_eq!(
            exclude_above_point_inside(&negative_region),
            Some(Region {
                lower: negative_region.lower,
                upper: point_inside(&negative_region),
            })
        );
        assert_eq!(exclude_above_point_below(&negative_region), None);
        assert_eq!(
            exclude_above_point_above(&negative_region),
            Some(negative_region),
        );

        // test various intersections (and non-intersections) on a region about zero.
        assert_eq!(
            exclude_below_point_inside(&about_zero_region),
            Some(Region {
                lower: point_inside(&about_zero_region),
                upper: about_zero_region.upper,
            })
        );
        assert_eq!(
            exclude_below_point_below(&about_zero_region),
            Some(about_zero_region)
        );
        assert_eq!(exclude_below_point_above(&about_zero_region), None);
        assert_eq!(
            exclude_above_point_inside(&about_zero_region),
            Some(Region {
                lower: about_zero_region.lower,
                upper: point_inside(&about_zero_region)
            })
        );
        assert_eq!(exclude_above_point_below(&about_zero_region), None);
        assert_eq!(
            exclude_above_point_above(&about_zero_region),
            Some(about_zero_region)
        );
    }
}

use num_traits::{Num, NumCast};

pub struct Triangle<T> {
    uniq_sides: Vec<T>,
}

impl<T> Triangle<T>
where
    T: Num + NumCast + Copy + PartialOrd,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let mut sides = sides.iter().cloned().collect::<Vec<T>>();
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sides[0] + sides[1] <= sides[2] {
            return None;
        }

        let mut uniq_sides = vec![];
        for side in sides.into_iter() {
            if let Some(last_side) = uniq_sides.last() {
                if *last_side == side {
                    continue;
                }
            }
            uniq_sides.push(side);
        }

        Some(Self { uniq_sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.uniq_sides.len() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.uniq_sides.len() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.uniq_sides.len() == 3
    }
}

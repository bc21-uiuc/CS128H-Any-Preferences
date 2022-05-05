struct DivIter {
    value: usize, // usize??? or would u32 B mr appropr?
    divisor: usize,
}
impl DivIter {
    fn new(value: usize, divisor: usize) -> DivIter {
        DivIter { value, divisor }
    }
}
impl Iterator for DivIter {
    type Item = usize; // is this necessary? shouldn't i put this somewhere else?
                       // should I check for that vaue is a power of the divisor?
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            None
        } else {
            let temp = self.value;
            self.value /= self.divisor;
            Some(temp)
        }
    }
}
struct MultIter {
    value: usize,
    multiplier: usize,
    stop: usize,
}
impl MultIter {
    fn new(value: usize, multiplier: usize, stop: usize) -> MultIter {
        MultIter {
            value,
            multiplier,
            stop,
        }
    }
}
impl Iterator for MultIter {
    type Item = usize; // is this necessary? shouldn't i put this somewhere else?
                       // should I check for that vaue is a power of the multiplier?
    fn next(&mut self) -> Option<Self::Item> {
        if self.value >= self.stop {
            None
        } else {
            let temp = self.value;
            self.value *= self.multiplier;
            Some(temp)
        }
    }
}

/// convert a (x, y) coordinate to the corresponding index $d$ of Hilbert curve
/// code "translated" here: https://en.wikipedia.org/wiki/Hilbert_curve
/// dim must be power of 2
/// starts in top left, i.e. traces a U-shape
pub fn hilbert_2_to_1(dim: usize, point: (usize, usize)) -> Result<usize, &'static str> {
    if !dim.is_power_of_two() {
        return Err("dimension must be a power of 2");
    } // what if non-power of 2
    let (mut x, mut y) = point; // this is how to deestrcutre?
    if x >= dim || y >= dim {
        return Err("at least one of the coordinates is larger than the dimension");
    }
    let mut result = 0;
    for curr_dim in DivIter::new(dim / 2, 2) {
        // qx and qy determine the quadrant
        // if coord & curr_dim > 0, then that means
        // it's in the upper quadrant
        // otherwise it's in the lower quadrant
        let qx: bool = (x & curr_dim) > 0;
        let qy: bool = (y & curr_dim) > 0;
        let quadrant = match (qx, qy) {
            (false, false) => 0,
            (false, true) => 1,
            (true, true) => 2,
            (true, false) => 3,
        };
        result += curr_dim * curr_dim * quadrant;
        rot(dim, &mut x, &mut y, qx, qy)
    }
    return Ok(result);
}

/// starts in top left, i.e. U-shape
/// like this: 0 3
///            1 2
pub fn hilbert_1_to_2(dim: usize, value: usize) -> Result<(usize, usize), &'static str> {
    if !dim.is_power_of_two() {
        return Err("dimension must be a power of 2");
    }
    if value > dim * dim {
        return Err("value is larger than the dimension");
    }
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut t: usize = value;
    for curr_dim in MultIter::new(1, 2, dim) {
        let qx = 1 & (t / 2) > 0;
        let qy = (t % 2 == 0) != qx; // wtf is going on
        rot(curr_dim, &mut x, &mut y, qx, qy);
        if qx {
            x += curr_dim;
        }
        if qy {
            y += curr_dim
        } // originally result_x and result_y
        t /= 4;
    }
    return Ok((x, y));
}

fn rot(dim: usize, x: &mut usize, y: &mut usize, qx: bool, qy: bool) {
    // not sure if should be &usize
    // rotate accordingly if it is in the lower left or lower right quadrant
    if !qy {
        // if lower right, rotate 180 deg, and then...
        if qx {
            *x = dim - 1 - *x;
            *y = dim - 1 - *y;
        }
        // if lower right or left, flip across x=y line
        let t: usize = *x;
        *x = *y;
        *y = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn asdf() {
        assert!(true);
    }
}

pub fn order_value(a: i32, b: i32) -> (i32, i32) {
    if a > b { (b, a) } else { (a, b) }
}

pub fn clamp(min: i32, max: i32, value: i32) -> i32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }

    value
}

pub fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

pub fn max(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

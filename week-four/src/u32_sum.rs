pub fn u32_sum(arr: &[u32]) -> Option<u32> {
    //如果溢出返回None
    let mut sum: u32 = 0;
    for i in arr {
        sum = sum.checked_add(*i)?;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_sum() {
        assert_eq!(u32_sum(&[1, 2, 3]), Some(6));
        assert_eq!(u32_sum(&[u32::MAX, 1]), None);
    }
}

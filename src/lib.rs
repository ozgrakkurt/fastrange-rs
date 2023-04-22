/// Given a value "word", produces an integer in [0,p) without division.
#[inline(always)]
pub fn fastrange_32(word: u32, p: u32) -> u32 {
    ((u64::from(word) * u64::from(p)) >> 32) as u32
}

/// Given a value "word", produces an integer in [0,p) without division.
#[inline(always)]
pub fn fastrange_64(word: u64, p: u64) -> u64 {
    ((word as u128 * p as u128) >> 64) as u64
}

/// Given a value "word", produces an integer in [0,p) without division.
#[inline(always)]
pub fn fastrange_usize(word: usize, p: usize) -> usize {
    #[cfg(target_pointer_width = "32")]
    {
        fastrange_32(word as u32, p as u32) as usize
    }

    #[cfg(target_pointer_width = "64")]
    {
        fastrange_64(word as u64, p as u64) as usize
    }

    #[cfg(all(
        not(target_pointer_width = "64"),
        not(target_pointer_width = "32"),
        not(target_pointer_width = "16")
    ))]
    {
        compile_error!("target not supported.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        for x in 0..1_000_000 {
            assert!(fastrange_32(x, 5) < 5);
        }
    }

    #[test]
    fn test_64() {
        for x in 0..1_000_000 {
            assert!(fastrange_64(x, 5) < 5);
        }
    }
}

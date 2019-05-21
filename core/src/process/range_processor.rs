use plasma_utils_range::Range;

pub struct RangeProcessor {}

impl RangeProcessor {
    fn validate_range(start: u64, end: u64) -> bool {
        start < end
    }

    /// Put a range to input_ranges and returns affected output_ranges.
    pub fn put(input_ranges: &[Range], start: u64, end: u64, value: &[u8]) -> Vec<Range> {
        let mut output_ranges = vec![];
        if input_ranges.is_empty() || !Self::validate_range(start, end) {
            return output_ranges;
        }
        if input_ranges[0].get_start() < start {
            output_ranges.push(Range::new(
                input_ranges[0].get_start(),
                start,
                &input_ranges[0].get_value(),
            ));
        }
        let last_range = &input_ranges[input_ranges.len() - 1];
        if end < last_range.get_end() {
            output_ranges.push(Range::new(
                end,
                last_range.get_end(),
                &last_range.get_value(),
            ));
        }
        output_ranges.push(Range::new(start, end, value));
        output_ranges
    }
}

#[cfg(test)]
mod tests {
    use super::Range;
    use super::RangeProcessor;

    #[test]
    fn test_fail_to_put() {
        let value1 = &b"Alice is owner in block 1"[..];
        let value2 = &b"Alice is owner in block 2"[..];
        let range1 = Range::new(0, 100, value1);
        let range2 = Range::new(100, 200, value2);
        let partial_ranges = &[range1, range2];
        let affected_ranges = RangeProcessor::put(partial_ranges, 250, 50, value2);
        assert_eq!(affected_ranges.len(), 0);
    }

    #[test]
    fn test_put() {
        let value1 = &b"Alice is owner in block 1"[..];
        let value2 = &b"Alice is owner in block 2"[..];
        let value3 = &b"Alice is owner is block 3"[..];
        let value4 = &b"Bob is owner is block 4"[..];
        let range1 = Range::new(0, 100, value1);
        let range2 = Range::new(100, 200, value2);
        let range3 = Range::new(200, 300, value3);
        let partial_ranges = &[range1, range2, range3];
        let affected_ranges = RangeProcessor::put(partial_ranges, 50, 250, value4);
        assert_eq!(affected_ranges.len(), 3);
    }
}

use rsomics_common::{Result, RsomicsError};
use std::io::BufRead;

/// Raw sum of `end - start`; overlaps are NOT merged.
pub fn total_bp<R: BufRead>(reader: R) -> Result<u64> {
    let mut total: u64 = 0;
    for line in reader.lines() {
        let line = line.map_err(RsomicsError::Io)?;
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let mut fields = line.splitn(4, '\t');
        let _chrom = fields.next().unwrap_or("");
        let start_str = fields.next().unwrap_or("");
        let end_str = fields.next().unwrap_or("");

        let start: u64 = start_str
            .parse()
            .map_err(|e| RsomicsError::InvalidInput(format!("start: {e}")))?;
        let end: u64 = end_str
            .parse()
            .map_err(|e| RsomicsError::InvalidInput(format!("end: {e}")))?;
        total += end.saturating_sub(start);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn basic() {
        let input = "chr1\t0\t100\nchr2\t200\t300\n";
        assert_eq!(total_bp(Cursor::new(input)).unwrap(), 200);
    }

    #[test]
    fn skip_header() {
        let input = "# comment\nchr1\t0\t50\n";
        assert_eq!(total_bp(Cursor::new(input)).unwrap(), 50);
    }

    #[test]
    fn overlapping_counted_twice() {
        // Total is raw sum, overlaps not merged
        let input = "chr1\t0\t100\nchr1\t50\t150\n";
        assert_eq!(total_bp(Cursor::new(input)).unwrap(), 200);
    }
}

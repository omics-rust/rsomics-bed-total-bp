use rsomics_bed_total_bp::total_bp;
use std::io::Cursor;

#[test]
fn single_interval() {
    let input = "chr1\t0\t1000\n";
    assert_eq!(total_bp(Cursor::new(input)).unwrap(), 1000);
}

#[test]
fn multiple_intervals() {
    let input = "chr1\t0\t100\nchr1\t200\t400\nchr2\t0\t50\n";
    assert_eq!(total_bp(Cursor::new(input)).unwrap(), 350);
}

#[test]
fn header_skipped() {
    let input = "# header\nchr1\t0\t100\n";
    assert_eq!(total_bp(Cursor::new(input)).unwrap(), 100);
}

#[test]
fn empty_input() {
    let input = "";
    assert_eq!(total_bp(Cursor::new(input)).unwrap(), 0);
}

#[test]
fn awk_equiv() {
    // Verify against known awk: awk '{s+=$3-$2} END{print s}'
    let cases: &[(&str, u64)] = &[
        ("chr1\t100\t200\n", 100),
        ("chr1\t0\t100\nchr2\t0\t100\n", 200),
        ("chrX\t999\t1000\n", 1),
    ];
    for (input, expected) in cases {
        let got = total_bp(Cursor::new(*input)).unwrap();
        assert_eq!(got, *expected, "input: {input}");
    }
}

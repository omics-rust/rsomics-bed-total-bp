# rsomics-bed-total-bp

Count the total number of base-pairs spanned by BED intervals (sum of `end - start`). Prints a single integer.

## Usage

```sh
rsomics-bed-total-bp [INPUT]
rsomics-bed-total-bp intervals.bed
cat intervals.bed | rsomics-bed-total-bp
```

## Notes

Overlapping intervals are NOT merged — the raw sum is reported, equivalent to:

```awk
awk '{s+=$3-$2} END{print s}' intervals.bed
```

Use `rsomics-bed-merge` first if you want unique base-pair coverage.

## Origin

Independent Rust implementation.

License: MIT OR Apache-2.0.

#!/usr/bin/env bash

# Take args for year (YYYY) and day number (DD)
# Create a directory src/yearYYYY/dayDD
# Create a mod.rs src/yearYYYY/dayDD/mod.rs from a template scripts/day_template.rs
# Append "pub mod dayDD;" to src/yearYYYY/mod.rs"
# Create a directory resources/yearYYYY/dayDD
# Create files resources/yearYYYY/dayDD/input.txt and resources/yearYYYY/dayDD/test_input.txt

YEAR=$1
DAY=$2

mkdir -p src/year$YEAR/day$DAY
cp scripts/day_template.rs src/year$YEAR/day$DAY/mod.rs
echo "pub mod day$DAY;" >> src/year$YEAR/mod.rs

mkdir -p resources/year$YEAR/day$DAY
touch resources/year$YEAR/day$DAY/input.txt
touch resources/year$YEAR/day$DAY/test_input.txt
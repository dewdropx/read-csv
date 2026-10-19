# Rust CSV Reader

A simple command-line program written in Rust that reads data from a CSV (Comma-Separated Values) file and prints each record to the terminal using the `csv` crate.

## Features

- Reads data from a CSV file.
- Parses CSV records using the Rust `csv` crate.
- Prints each record using debug formatting.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Installation and Usage

1. Clone this repository:

   ```bash
   git clone https://github.com/dewdropx/read-csv.git
   ```

2. Navigate to the project directory:

   ```bash
   cd read-csv
   ```

4. Run the program:

   ```bash
   cargo run
   ```

## Example

Sample `month.csv`:

```csv
Month,Abbreviation,Numeric
January,Jan.,01
February,Feb.,02
March,Mar.,03
```

Example output:

```text
StringRecord(["January", "Jan.", "01"])
StringRecord(["February", "Feb.", "02"])
StringRecord(["March", "Mar.", "03"])
```

The header row is skipped by default when using `reader.records()`.

## Limitations

- Uses a fixed file path (`./month.csv`).
- Does not accept a filename through command-line arguments.
- Stops processing when a file-reading or CSV-parsing error occurs.

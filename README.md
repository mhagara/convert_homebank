# convert_homebank

## Usage
```
Converts Revolut or UniCredit statements to HomeBank CSV format

Usage: convert_homebank [OPTIONS] --input <INPUT> --output <OUTPUT> --conversion <CONVERSION>

Options:
-i, --input <INPUT>
Source from Revolut CSV or UniCredit XLS

-o, --output <OUTPUT>
Destination HomeBank CSV

-c, --conversion <CONVERSION>
Conversion type Revolut or UniCredit

          Possible values:
          - Revolut:   Convert from Revolut CSV
          - UniCredit: Convert from UniCredit XLS

-f, --from <FROM>
Filter transactions from date (inclusive). (yyyy-mm-dd)

-t, --to <TO>
Filter transactions to date (inclusive). (yyyy-mm-dd)

-h, --help
Print help (see a summary with '-h')

-V, --version
Print version
```
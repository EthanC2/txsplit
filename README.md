# txsplit - Split Text


# Options
1. Split by delimiter(s) characters provided as a string (default=all whitespace)
 - Remove empty entries (default=true)
 - Trim whitespace (default=true)
 - Case insensitive? (default=false)
2. Split on size (index)
3. Split into MAX/MIN strings
4. Split on [regular expression](https://crates.io/crates/regex)
5. Output field separator (string, default = "\n" aka `0x0a`)
6. Process lines individually. 

# Future Plans
- Split on strings, not just chars (employ [bpsm](https://www.cs.haifa.ac.il/%7Eoren/Publications/bpsm.pdf))
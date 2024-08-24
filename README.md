# Dwmblocks
Simple and fast modular status bar for dwm written in rust.

## Usage:
Add this library to a project
```
$ cargo add dwmblocks
```

```rust
use dwmblocks::status;

fn main() {
    status!(
        base_path: "./scripts/",
        gap: "",

        // prefix, suffix, interval, script name
        [""      , " "   , 10      , "wifi"     ],
        ["| "    , " "   , 5       , "cpu"      ],
        ["| [ "  , " ] " , 20      , "battery"  ],
        ["| "    , " "   , 10      , "mem"      ],
        ["| "    , " "   , 60      , "hdd"      ],
        ["| "    , ""    , 30      , "date"     ],
    );
}
```

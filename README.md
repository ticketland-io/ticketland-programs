Ticket Land Programs
===

Localnet
===

Install deps:

1. Metaplex

- `cd deps`
- `git clone git@github.com:metaplex-foundation/metaplex-program-library.git`
- `cd metaplex-program-library/token-metadata`
- `cargo build-bpf`
- `cd ../../`
- `cp metaplex-program-library/token-metadata/target/deploy/mpl_token_metadata.so ./`
- `rm -rf metaplex-program-library`


CPI And Entry Point Issue
===

If we use the default `anchor build` command we will end up having `ticket_sale.so` missing the entry point that is needed for the program to accept incoming requests. This results in the following error

> ELF error: Multiple or no text sections, consider removing llc option: -function-sections

The error occurs both when we do testing and when we try to deploy.

There is exactly same as this issue here https://github.com/solana-labs/solana/issues/20761.

All we had to do is update the root `Cargo.toml`

```
[workspace]
members = [
	"programs/*",
	"common",
]
+ edition = "2021"
+ resolver = "2"
```

Ticket Land Programs
===

Localnet
===

Install deps:

1. Metaples

- `cd deps`
- `git clone git@github.com:metaplex-foundation/metaplex-program-library.git`
- `cd metaplex-program-library/token-metadata`
- `cargo build-bpf`
- `cd ../../`
- `cp metaplex-program-library/token-metadata/target/deploy/mpl_token_metadata.so ./`
- `rm -rf metaplex-program-library`

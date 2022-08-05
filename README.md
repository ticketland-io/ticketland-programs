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

Imported Custom Types IDL issue
===

https://github.com/coral-xyz/anchor/issues/1566#issuecomment-1105423938

We need to manually add the type definitions into the degenerated IDL files. These custom types are defined in the `common` crate.

More specifically we need to add the following types to the event_registry.json IDL file

```
 [
    {
      "name": "SaleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Free"
          },
          {
            "name": "FixedPrice",
            "fields": [
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          },
          {
            "name": "Refundable",
            "fields": [
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          },
          {
            "name": "DutchAuction",
            "fields": [
              {
                "name": "start_price",
                "type": "u64"
              },
              {
                "name": "end_price",
                "type": "u64"
              },
              {
                "name": "curve_length",
                "type": "u16"
              },
              {
                "name": "drop_interval",
                "type": "u16"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "TicketType",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "n_tickets",
            "type": "u32"
          },
          {
            "name": "sale_type",
            "type": {
              "defined": "SaleType"
            }
          },
          {
            "name": "sale_start_time",
            "type": "i64"
          },
          {
            "name": "merkle_root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "Currency",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint_account",
            "type": "publicKey"
          },
          {
            "name": "treasury_ata",
            "type": "publicKey"
          },
          {
            "name": "service_fee",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "InitBumps",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "eventNftAuthority",
            "type": "u8"
          },
          {
            "name": "cpiAuthority",
            "type": "u8"
          }
        ]
      }
    }
  ],
	```

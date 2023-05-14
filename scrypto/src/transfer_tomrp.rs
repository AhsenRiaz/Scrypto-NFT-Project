use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct TomrpNFTData {
    pub name: String,
    pub description: String,
    pub key_image_url: String,
    pub owner: String,
}

#[blueprint]
mod transfer_tomrp {
    struct TransferTomrp {
        tomrp_vault: Vault,
        admin_transfer_badge: Vault,
    }

    impl TransferTomrp {
        pub fn new() -> ComponentAddress {
            /*
            @title Admin Transfer badge
            @notice A badge used to identify the admin. Only admin can transfer resources
            */
            let transfer_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Transfer Badge")
                .metadata("description", "Transfer Authority For TOMRP Tokens")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            /*
            @title Tomrp Bucket
            @notice a temporary storage used to store the resources
             */
            let tomrp_bucket = ResourceBuilder::new_integer_non_fungible::<TomrpNFTData>()
                .metadata("name", "TOMRP NFT Collection")
                .metadata("description", "21 piece digital art collection by tomrp")
                .mint_initial_supply([
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#1"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/1.png"),
                            owner: String::from("rdx1qsp8g2ds6pa9ntv3alvvqa6try6quxq00fwfkcgys0rvz7k5rcsh40q9s7q3g")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#2"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/2.png"),
                            owner:String::from("rdx1qspjd2whal3zzy8yjrx9w0867hhpghf7exq22a6pytn9me5ltx442aglxl0er")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#3"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/3.png"),
                            owner:String::from("rdx1qspf7lsfguuuwsghdxygfrz9gex28qn96gmq9agegt24df3cxsc4sjqhx7k9m")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#4"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/4.png"),
                            owner: String::from("rdx1qspah0zysl75x3u8fwwcgxclxcjvew2r7taphj7cf9dqhdjt82y6epsk0u0yt")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#5"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/5.png"),
                            owner: String::from("rdx1qsp2ece7cyn9nypzfl0rhuk5m9hhyvychy3grn7qkwy4zuds0n5mlcg6rnpc5")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#6"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/6.png"),
                            owner: String::from("rdx1qspdzg8svqhft75vty6x44fatr4hypehyx367hn9rrq4yyav983thdg6jtwtw")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#7"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/7.png"),
                            owner: String::from("rdx1qspmjy249qwfnjcfqpl9adkh9nhju5hpdq8zl7chq363qtflc0p09esec04gh")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#8"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/8.png"),
                            owner: String::from("rdx1qsp4zge8j5scfjlhr29rfzr2wcmy8vy87pyvqfwsu24dpmwvzwwjwncj6kndh")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#9"),
                            description: String::from("an NFT"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/9.png"),
                            owner: String::from("rdx1qspdzg8svqhft75vty6x44fatr4hypehyx367hn9rrq4yyav983thdg6jtwtw")
                        },
                    ),
                ]);

            Self {
                tomrp_vault: Vault::with_bucket(tomrp_bucket),
                admin_transfer_badge: Vault::with_bucket(transfer_badge),
            }
            .instantiate()
            .globalize()
        }

        pub fn transfer_resources(&mut self) {
         

        }
    }
}

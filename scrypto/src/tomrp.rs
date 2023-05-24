use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData, Debug)]
pub struct TomrpNFTData {
    pub name: String,
    pub key_image_url: String,
    pub owner: String,
}

#[blueprint]
mod tomrp {

    struct Tomrp {
        tomrp_vault: Vault,
        admin_transfer_badge: Vault,
        owners: Vec<Address>,
    }

    impl Tomrp {
        pub fn instantiate_tomrp() -> ComponentAddress {
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

            let tomrp_bucket = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "TOMRP NFT Collection")
                .metadata("description", "21 piece digital art collection by tomrp")
                .mint_initial_supply([
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#1"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/1.png"),
                            owner: String::from("rdx1qspjd2whal3zzy8yjrx9w0867hhpghf7exq22a6pytn9me5ltx442aglxl0er") 
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(2u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#2"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/2.png"),
                            owner:String::from("rdx1qspjd2whal3zzy8yjrx9w0867hhpghf7exq22a6pytn9me5ltx442aglxl0er")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(3u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#3"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/3.png"),
                            owner:String::from("rdx1qspf7lsfguuuwsghdxygfrz9gex28qn96gmq9agegt24df3cxsc4sjqhx7k9m")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(4u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#4"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/4.png"),
                            owner: String::from("rdx1qspah0zysl75x3u8fwwcgxclxcjvew2r7taphj7cf9dqhdjt82y6epsk0u0yt")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(5u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#5"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/5.png"),
                            owner: String::from("rdx1qsp2ece7cyn9nypzfl0rhuk5m9hhyvychy3grn7qkwy4zuds0n5mlcg6rnpc5")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(6u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#6"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/6.png"),
                            owner: String::from("rdx1qspdzg8svqhft75vty6x44fatr4hypehyx367hn9rrq4yyav983thdg6jtwtw")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(7u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#7"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/7.png"),
                            owner: String::from("rdx1qspmjy249qwfnjcfqpl9adkh9nhju5hpdq8zl7chq363qtflc0p09esec04gh")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(8u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#8"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/8.png"),
                            owner: String::from("rdx1qsp4zge8j5scfjlhr29rfzr2wcmy8vy87pyvqfwsu24dpmwvzwwjwncj6kndh")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(9u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#9"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/9.png"),
                            owner: String::from("rdx1qspdzg8svqhft75vty6x44fatr4hypehyx367hn9rrq4yyav983thdg6jtwtw")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(10u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#10"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/10.png"),
                            owner: String::from("rdx1qsps4xjgfsvx8waknyu78hh47lhwgqr0ntznzcwzew2xg53u28q544gk0hmvg")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(11u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#11"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/11.png"),
                            owner: String::from("rdx1qspdzg8svqhft75vty6x44fatr4hypehyx367hn9rrq4yyav983thdg6jtwtw")
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(12u64),
                        TomrpNFTData {
                            name: String::from("TOMRP#12"),
                            key_image_url: String::from("https://ipfs.io/ipfs/QmTNQDXdWfbRXPHv9MVbcFippv9FUo7Sk3FsrMBxLfF9LW/12.png"),
                            owner: String::from("rdx1qsp3dqdxx4llr4en0u0mdx9sma7nnkzesawm99e2cjuenx3h0ycxsrcav5ldh")
                        },
                    ),
                ]);

            Self {
                tomrp_vault: Vault::with_bucket(tomrp_bucket),
                admin_transfer_badge: Vault::with_bucket(transfer_badge),
                owners: Vec::new(),
            }
            .instantiate()
            .globalize()
        }

        /*
        @notice function to add owners
         */
        pub fn add_owners(&mut self, _owners: Vec<Address>) {
            for owner in _owners {
                self.owners.push(owner);
            }
        }

        /*
        @notice - function to get owners
         */
        pub fn get_owner(&self, _i: u32) -> Address {
            let index = _i.to_usize().unwrap();
            return self.owners[index];
        }

        /*
        @notice - function to get the total supply of vault
         */
        fn get_total_supply(&mut self) -> Decimal {
            let tomrp_resuource_address = self.tomrp_vault.resource_address();

            let total_supply = borrow_resource_manager!(tomrp_resuource_address).total_supply();

            return total_supply;
        }

        /*
        @notice - function to get the non fungible buckets from their ids
         */
        fn get_nfr_bucket_from_id(&mut self, id: &Decimal) -> Bucket {
            let tomrp_nfr_id: u64 = (id.to_string()).parse().unwrap();
            let nfr_bucket = self
                .tomrp_vault
                .take_non_fungible(&NonFungibleLocalId::Integer(
                    IntegerNonFungibleLocalId::new(tomrp_nfr_id),
                ));
            nfr_bucket
        }

        /*
        @notice - function to transfer Non Fungible Resources to their respective owners
         */
        pub fn transfer_resources(&mut self, _recipients: Vec<Address>) {
            let mut i = dec!(1);

            let total_supply = self.get_total_supply();

            while i <= total_supply {
                let _tomrp_nfr_bucket = self.get_nfr_bucket_from_id(&i);

                i += 1;
            }
        }
    }
}

use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct TomrpNFTData {
    pub name: String,
    pub key_image_url: String,
    pub owner: String,
}

#[blueprint]
mod tomrp {

    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
        },
        methods {
            transfer_tomrp => restrict_to: [admin, OWNER];
            set_ids_to_nfts => restrict_to: [admin, OWNER];
        }
    }

    struct Tomrp {
        // a vault that contains all tomrp non-fungibles
        tomrp_vault: Vault,

        // an admin badge resource address
        admin_badge: ResourceAddress,

        // kay value pair of id to nft
        id_to_nft_map: BTreeMap<u32, Global<Account>>,
    }

    impl Tomrp {
        pub fn instantiate_tomrp() -> (Global<Tomrp>, Bucket, Bucket) {
            /*
            @title: Owner Badge
            @notice: A badge used to identify owner
            */
            let owner_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata! {
                    init{
                        "name" => "Owner Badge", locked;
                    }
                })
                .mint_initial_supply(1)
                .into();

            /*
            @title Admin badge
            @notice A badge used to identify the admin. Only admin can transfer resources
            */
            let admin_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(
                require(owner_badge.resource_address())
            )))
            .metadata(metadata!(init{
                "name" => "Admin Badge", locked;
            }))
            .divisibility(DIVISIBILITY_NONE)
            .mint_initial_supply(1)
            .into();

            /*
            @title Tomrp Bucket
            @notice a temporary storage used to store the resources
             */

            let tomrp_bucket = ResourceBuilder::new_integer_non_fungible(OwnerRole::None)
             .metadata(metadata! {
                 init {
                     "name" => "TOMRP NFT Collection", locked;
                     "description" => "21 pieces digital art collection by tomrp", locked;
                 }
             })
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
             ]).into();

            let component = Self {
                tomrp_vault: Vault::with_bucket(tomrp_bucket),
                admin_badge: admin_badge.resource_address(),
                id_to_nft_map: BTreeMap::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
                owner_badge.resource_address()
            ))))
            .roles(roles!(
                admin => rule!(require(admin_badge.resource_address()));
            ))
            .globalize();

            (component, admin_badge, owner_badge)
        }

        pub fn set_ids_to_nfts(&mut self, ids_to_nfts: BTreeMap<u32, Global<Account>>) {
            for (k, v) in ids_to_nfts.into_iter() {
                self.id_to_nft_map.insert(k, v);
            }
        }

        // /*
        // @notice - function to transfer Non Fungible Resources to their respective owners
        //  */
        pub fn transfer_tomrp(&mut self) {
            for (k, mut v) in self.id_to_nft_map.clone() {
                let tomrp_nfr_bucket = self
                    .tomrp_vault
                    .as_non_fungible()
                    .take_non_fungible(&NonFungibleLocalId::Integer(
                        IntegerNonFungibleLocalId::new(k.into()),
                    ))
                    .into();

                v.try_deposit_or_abort(tomrp_nfr_bucket, None);
            }
        }
    }
}

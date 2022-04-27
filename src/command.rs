use std::cmp;
use std::collections::{HashMap, HashSet};

use sunshine_solana::commands::simple::branch;
use sunshine_solana::commands::simple::http_request;
use sunshine_solana::commands::simple::ipfs_nft_upload;
use sunshine_solana::commands::simple::ipfs_upload;
use sunshine_solana::commands::simple::json_extract;
use sunshine_solana::commands::simple::json_insert;
use sunshine_solana::commands::solana;
// use sunshine_solana::commands::solana::add_pubkey;
use sunshine_solana::commands::solana::create_mint_account;
use sunshine_solana::commands::solana::create_token_account;
use sunshine_solana::commands::solana::generate_keypair;
use sunshine_solana::commands::solana::generate_keypair::Arg;
use sunshine_solana::commands::solana::get_balance;
use sunshine_solana::commands::solana::mint_token;
use sunshine_solana::commands::solana::nft;
use sunshine_solana::commands::solana::nft::approve_collection_authority;
use sunshine_solana::commands::solana::nft::approve_use_authority;
use sunshine_solana::commands::solana::nft::arweave_file_upload;
use sunshine_solana::commands::solana::nft::arweave_nft_upload;
use sunshine_solana::commands::solana::nft::create_master_edition;
use sunshine_solana::commands::solana::nft::create_metadata_accounts;
use sunshine_solana::commands::solana::nft::get_left_uses;
use sunshine_solana::commands::solana::nft::sign_metadata;
use sunshine_solana::commands::solana::nft::update_metadata_accounts;
use sunshine_solana::commands::solana::nft::utilize;
use sunshine_solana::commands::solana::nft::verify_collection;
use sunshine_solana::commands::solana::request_airdrop;
use sunshine_solana::commands::solana::transfer_solana;
use sunshine_solana::commands::solana::transfer_token;

use sunshine_solana::{commands::simple::Command as SimpleCommand, CommandConfig};

use crate::model::{NodeDimensions, SolanaNet};

/*
pub struct Commands {
    commands: BTreeMap<&'static str, &'static dyn FnMut(Context<'_>)>,
}

pub struct Context<'a> {
    state: &'a mut Model,
}

impl Commands {
    pub fn new() -> Self {
        /*let commands: &[(&'static str, &'static dyn FnMut(Context<'_>))] =
            &[("print", &Self::print), ("const", &Self::r#const)];
        Self {
            commands: commands.iter().map(|&(key, value)| (key, value)).collect(),
        }*/
        todo!()
    }
}*/

pub const INPUT_SIZE: i64 = 50;
pub const HEADER_SIZE: i64 = 30;

pub const COMMANDS: &'static [&'static dyn DynCommand] = &[
    &PrintCommand,
    &ConstCommand,
    &JsonExtractCommand,
    &JsonInsertCommand,
    &HttpRequestCommand,
    &IpfsUploadCommand,
    &IpfsNftUploadCommand,
    &WaitCommand,
    &BranchCommand,
    // Solana
    &CreateMintAccountCommand,
    // &AddPubkeyCommand,
    &CreateTokenAccountCommand,
    &GenerateKeypairCommand,
    &MintTokenCommand,
    &TransferTokenCommand,
    &TransferSolanaCommand,
    &RequestAirdropCommand,
    &GetBalanceCommand,
    // NFTs
    &CreateMetadataAccountsCommand,
    &CreateMasterEditionCommand,
    &UpdateMetadataAccountsCommand,
    &VerifyCollectionCommand,
    &ApproveCollectionAuthorityCommand,
    &SignMetadataCommand,
    &UtilizeCommand,
    &ApproveUseAuthorityCommand,
    &GetLeftUsesCommand,
    // &ArweaveUploadCommand,
    // &ArweaveNftUploadCommand,
    &ArweaveNftUploadCommand,
    &ArweaveFileUploadCommand,
];

// TODO: Build once on initialization
pub fn commands_map() -> HashMap<&'static str, &'static dyn DynCommand> {
    COMMANDS
        .iter()
        .map(|&command| (command.command_name(), command))
        .collect()
}

// calculate node height based on the max ports
fn calculate_node_height(command: impl DynCommand) -> i64 {
    let max_port_count = cmp::max(command.inputs().len(), command.outputs().len()) as i64;
    (max_port_count * INPUT_SIZE + 30) as i64
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandInput {
    pub name: &'static str,
    pub type_bounds: &'static [TypeBound],
    pub required: &'static bool,
    pub tooltip: &'static str,
    pub default_value: &'static str,
    pub has_default: &'static bool,
}

impl CommandInput {
    pub const fn new(
        name: &'static str,
        type_bounds: &'static [TypeBound],
        required: &'static bool,
        tooltip: &'static str,
        default_value: &'static str,
        has_default: &'static bool,
    ) -> Self {
        Self {
            name,
            type_bounds,
            required,
            tooltip,
            default_value,
            has_default,
        }
    }

    pub fn acceptable_types(&self) -> HashSet<&'static str> {
        let mut type_bounds = self.type_bounds.iter();
        if let Some(type_bound) = type_bounds.next() {
            let mut acceptable_types: HashSet<_> = type_bound.types.iter().copied().collect();
            for type_bound in self.type_bounds {
                let other_acceptable_types: HashSet<_> = type_bound.types.iter().copied().collect();
                acceptable_types = acceptable_types
                    .intersection(&other_acceptable_types)
                    .copied()
                    .collect();
            }
            acceptable_types
        } else {
            HashSet::new()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandOutput {
    pub name: &'static str,
    pub r#type: &'static str,
    pub passthrough: &'static bool,
    pub tooltip: &'static str,
}

impl CommandOutput {
    pub const fn new(
        name: &'static str,
        r#type: &'static str,
        passthrough: &'static bool,
        tooltip: &'static str,
    ) -> Self {
        Self {
            name,
            r#type,
            passthrough,
            tooltip,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeBound {
    pub name: &'static str,
    pub types: &'static [&'static str],
}

pub trait Command {
    const COMMAND_NAME: &'static str;
    const DESCRIPTION: &'static str;
    const WIDGET_NAME: &'static str;
    const INPUTS: &'static [CommandInput];
    const OUTPUTS: &'static [CommandOutput];
    const AVAILABILITY: &'static [SolanaNet];
    fn dimensions() -> NodeDimensions; // TODO: Move to CommandView
    fn config() -> CommandConfig;
}

pub trait DynCommand: std::fmt::Debug {
    fn command_name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn widget_name(&self) -> &'static str;
    fn inputs(&self) -> &'static [CommandInput];
    fn outputs(&self) -> &'static [CommandOutput];
    fn availability(&self) -> &'static [SolanaNet];
    fn dimensions(&self) -> NodeDimensions; // TODO: Move to DynCommandView
    fn config(&self) -> CommandConfig;
}

impl<T: Command + std::fmt::Debug> DynCommand for T {
    fn command_name(&self) -> &'static str {
        T::COMMAND_NAME
    }
    fn description(&self) -> &'static str {
        T::DESCRIPTION
    }

    fn widget_name(&self) -> &'static str {
        T::WIDGET_NAME
    }

    fn inputs(&self) -> &'static [CommandInput] {
        T::INPUTS
    }

    fn outputs(&self) -> &'static [CommandOutput] {
        T::OUTPUTS
    }

    fn availability(&self) -> &'static [SolanaNet] {
        T::AVAILABILITY
    }

    fn dimensions(&self) -> NodeDimensions {
        T::dimensions()
    }

    fn config(&self) -> CommandConfig {
        T::config()
    }
}

// BASIC

#[derive(Copy, Clone, Debug)]
pub struct PrintCommand;

#[derive(Copy, Clone, Debug)]
pub struct ConstCommand;

#[derive(Copy, Clone, Debug)]
pub struct JsonExtractCommand;

#[derive(Copy, Clone, Debug)]
pub struct JsonInsertCommand;

#[derive(Copy, Clone, Debug)]
pub struct HttpRequestCommand;

#[derive(Copy, Clone, Debug)]
pub struct IpfsUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct IpfsNftUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct BranchCommand;

#[derive(Copy, Clone, Debug)]
pub struct WaitCommand;

// SOLANA

#[derive(Copy, Clone, Debug)]
pub struct CreateMintAccountCommand;

#[derive(Copy, Clone, Debug)]
pub struct AddPubkeyCommand;

#[derive(Copy, Clone, Debug)]
pub struct CreateTokenAccountCommand;

#[derive(Copy, Clone, Debug)]
pub struct GenerateKeypairCommand;

#[derive(Copy, Clone, Debug)]
pub struct MintTokenCommand;

#[derive(Copy, Clone, Debug)]
pub struct TransferTokenCommand;

#[derive(Copy, Clone, Debug)]
pub struct TransferSolanaCommand;

#[derive(Copy, Clone, Debug)]
pub struct RequestAirdropCommand;

#[derive(Copy, Clone, Debug)]
pub struct GetBalanceCommand;

// METAPLEX

#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountsCommand;

#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionCommand;

#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountsCommand;

#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionCommand;
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityCommand;
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataCommand;

#[derive(Copy, Clone, Debug)]
pub struct UtilizeCommand;

#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityCommand;

#[derive(Copy, Clone, Debug)]
pub struct GetLeftUsesCommand;

// #[derive(Copy, Clone, Debug)]
// pub struct ArweaveUploadCommand;

// #[derive(Copy, Clone, Debug)]
// pub struct ArweaveNftUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct ArweaveNftUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct ArweaveFileUploadCommand;

impl Command for PrintCommand {
    const COMMAND_NAME: &'static str = "print";
    const WIDGET_NAME: &'static str = "Print";
    const INPUTS: &'static [CommandInput] = &[CommandInput::new(
        "print",
        &[PRINTABLE],
        &true,
        "",
        "",
        &false,
    )];
    const OUTPUTS: &'static [CommandOutput] = &[];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: 150,
            width: 450,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Print)
    }

    const DESCRIPTION: &'static str = "Shows the result of an output";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

/*pub enum Value {
    Integer(i64),
    Keypair(WrappedKeypair),
    String(String),
    NodeId(Uuid),
    DeletedNode(Uuid),
    Pubkey(Pubkey),
    Success(Signature),
    Balance(u64),
    U8(u8),
    U16(u16),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    StringOpt(Option<String>),
    Empty,
    NodeIdOpt(Option<NodeId>),
    NftCreators(Vec<NftCreator>),
    MetadataAccountData(MetadataAccountData),
} */

impl Command for ConstCommand {
    const COMMAND_NAME: &'static str = "const";
    const WIDGET_NAME: &'static str = "Const";
    const INPUTS: &'static [CommandInput] = &[];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new(
        "output",
        "",
        &false,
        "connect to a Print node to display output result",
    )];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: 300,
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Const(sunshine_solana::Value::String(
            "".to_string(),
        )))
    }

    const DESCRIPTION: &'static str =
        "Holds a constant value\n\nSelect appropriate type from dropdown for more help:\nString, Pubkey, JSON, File Picker, Boolean, Operator, Seed Phrase, Numbers, Custom Type";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// e.g. array example
// path:
// json:
// show highlighted
//
impl Command for JsonExtractCommand {
    const COMMAND_NAME: &'static str = "json_extract";
    const WIDGET_NAME: &'static str = "JsonExtract";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "path",
            &[STRING],
            &true,
            "e.g. /data/records/0/fields/url to select the url field value\nnote the /0/ is equivalent to [0], to select the first index in an array",
            "",
            &false,
        ),
        CommandInput::new("json", &[JSON], &true, "",
        "",
        &false,),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("value", "String", &false, "")]; //FIXME any value
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::JsonExtract(json_extract::JsonExtract {
            path: None,
            json: None,
        }))
    }

    const DESCRIPTION: &'static str = "Extracts a field from a JSON";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for JsonInsertCommand {
    const COMMAND_NAME: &'static str = "json_insert";
    const WIDGET_NAME: &'static str = "JsonInsert";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("path", &[STRING], &true, "e.g. /data/records/0/fields/url to insert into url field's value\nnote the /0/ to select [0] in an array",
        "",
        &false,),
        CommandInput::new("json", &[JSON], &true, "",
        "",
        &false,),
        CommandInput::new("value", &[STRING], &false, "if empty, will insert null",
        "",
        &false,),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("json", "Json", &false, "")]; //FIXME any value
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::JsonInsert(json_insert::JsonInsert {
            path: None,
            json: None,
            value: None,
        }))
    }
    const DESCRIPTION: &'static str = "Inserts a field into a JSON";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for HttpRequestCommand {
    const COMMAND_NAME: &'static str = "http_request";
    const WIDGET_NAME: &'static str = "HttpRequest";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "method",
            &[STRING],
            &true,
            "POST, GET, PATCH, etc.",
            "",
            &false,
        ),
        CommandInput::new(
            "url",
            &[STRING],
            &true,
            "https://postman-echo.com/post is useful for testing POST",
            "",
            &false,
        ),
        CommandInput::new("auth_token", &[STRING], &false, "", "", &false),
        CommandInput::new("json_body", &[JSON], &false, "", "", &false),
        CommandInput::new("headers", &[JSON], &false, "", "", &false),
    ];
    const OUTPUTS: &'static [CommandOutput] =
        &[CommandOutput::new("resp_body", "String", &false, "")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::HttpRequest(http_request::HttpRequest {
            method: None,
            url: None,
            auth_token: None,
            json_body: None,
            headers: None,
        }))
    }
    const DESCRIPTION: &'static str = "Makes HTTP requests";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for IpfsUploadCommand {
    const COMMAND_NAME: &'static str = "ipfs_upload";
    const WIDGET_NAME: &'static str = "IpfsUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "pinata_url",
            &[STRING],
            &true,
            "try https://api.pinata.cloud",
            "",
            &false,
        ),
        CommandInput::new(
            "pinata_jwt",
            &[STRING],
            &true,
            "generate on API keys page https://app.pinata.cloud/keys",
            "",
            &false,
        ),
        CommandInput::new(
            "file_path",
            &[STRING],
            &true,
            "path of file to upload",
            "",
            &false,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new(
        "file_cid",
        "String",
        &false,
        "uploaded file's unique id\nto view file, try https://cloudflare-ipfs.com/ipfs/{file_cid}",
    )];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::IpfsUpload(ipfs_upload::IpfsUpload {
            pinata_url: None,
            pinata_jwt: None,
            file_path: None,
        }))
    }
    const DESCRIPTION: &'static str = "Uploads a file to Pinata. https://www.pinata.cloud/";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// metadata json will get updated with the final uploaded file paths
// before and after
impl Command for IpfsNftUploadCommand {
    const COMMAND_NAME: &'static str = "ipfs_nft_upload";
    const WIDGET_NAME: &'static str = "IpfsNftUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "pinata_url",
            &[STRING],
            &true,
            "try https://api.pinata.cloud",
            "",
            &false,
        ),
        CommandInput::new(
            "pinata_jwt",
            &[STRING],
            &true,
            "generate on API keys page https://app.pinata.cloud/keys",
            "",
            &false,
        ),
        CommandInput::new(
            "metadata",
            &[NFT_METADATA],
            &true,
            "Metaplex standard metadata",
            "",
            &false,
        ), // Metaplex Standard Metadata https://docs.metaplex.com/token-metadata/specification
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("metadata_cid", "String", &false, "file's unique id"), // cid: file id
        CommandOutput::new("metadata_url", "String", &false, ""),
        CommandOutput::new(
            "updated_metadata",
            "NftMetadata",
            &false,
            "metadata updated with uploaded files' url",
        ),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::IpfsNftUpload(
            ipfs_nft_upload::IpfsNftUpload {
                pinata_url: None,
                pinata_jwt: None,
                metadata: None,
            },
        ))
    }

    const DESCRIPTION: &'static str = "Uploads a Metaplex metadata and it's files to Pinata.";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}
/// only suppports u64 and f64 comparisons
impl Command for BranchCommand {
    const COMMAND_NAME: &'static str = "branch";
    const WIDGET_NAME: &'static str = "Branch";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "operator",
            &[STRING],
            &true,
            "connect to a Const/Operator node",
            "",
            &false,
        ),
        CommandInput::new(
            "a",
            &[U64, F64],
            &true,
            "a/b must both be either f64 or u64",
            "",
            &false,
        ),
        CommandInput::new(
            "b",
            &[U64, F64],
            &true,
            "a/b must both be either f64 or u64",
            "",
            &false,
        ), //anytype
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("__true_branch", "", &false, ""),
        CommandOutput::new("__false_branch", "", &false, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Branch(branch::Branch {
            a: None,
            b: None,
            operator: None,
        }))
    }

    const DESCRIPTION: &'static str =
        "Branches flow based on the comparison of two values. Currently only works with f64 and u64";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for WaitCommand {
    const COMMAND_NAME: &'static str = "wait";
    const WIDGET_NAME: &'static str = "Wait";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("wait", &[STRING], &true, "node to wait for", "", &false), //
        CommandInput::new("value", &[ANY], &true, "passthrough value", "", &false),  //
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("value", "String", &false, "")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Wait)
    }
    const DESCRIPTION: &'static str = "Waits for a node to complete before continuing";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// Creates a mint account for a custom token
impl Command for CreateMintAccountCommand {
    const COMMAND_NAME: &'static str = "create_mint_account";
    const WIDGET_NAME: &'static str = "CreateMintAccount";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "fee_payer",
            &[KEYPAIR],
            &true,
            "who pays for account rent and transaction fees",
            "",
            &false,
        ),
        CommandInput::new(
            "authority",
            &[KEYPAIR],
            &true,
            "Mint authority - who can mint more tokens",
            "",
            &false,
        ), // Mint authority. Who can mint more tokens?
        CommandInput::new(
            "decimals",
            &[U8],
            &true,
            "NFTs should have decimal = 0\nUS dollars have 2 decimals\nFrom Metaplex documentation:\n'If the token has a master edition it is a NonFungible. If the token has no master edition(ensuring its supply can be > 1) and decimals of 0 it is a FungibleAsset. If the token has no master edition(ensuring its supply can be > 1) and decimals of > 0 it is a Fungible. If the token is a limited edition of a MasterEditon it is a NonFungibleEdition.'",
            "",
            &false,
        ), // Number of decimals a token can have. E.g. USD has 2 decimals. NFT should be 0.
        CommandInput::new("mint_account", &[KEYPAIR], &true, "keypair of the mint account",
        "",
        &false,), //
        CommandInput::new("memo", &[STRING], &false, "additional notes",
        "",
        &false,),             //
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        // passthrough
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("authority", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Keypair", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::CreateMintAccount(
            create_mint_account::CreateMintAccount {
                fee_payer: None,
                decimals: None,
                authority: None,
                mint_account: None,
                memo: None,
            },
        ))
    }

    const DESCRIPTION: &'static str = "Creates a mint account";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// impl Command for AddPubkeyCommand {
//     const COMMAND_NAME: &'static str = "add_pubkey";
//     const WIDGET_NAME: &'static str = "AddPubkey";
//     const INPUTS: &'static [CommandInput] = &[
//         CommandInput::new("name", &[STRING]),
//         CommandInput::new("pubkey", &[PUBKEY]),
//     ];
//     const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("pubkey", "Pubkey")];
//     fn dimensions() -> NodeDimensions {
//         NodeDimensions {
//             height: calculate_node_height(Self),
//             width: 300,
//         }
//     }
//     fn config() -> CommandConfig {
//         CommandConfig::Solana(solana::Kind::AddPubkey(add_pubkey::AddPubkey {
//             name: None,
//             pubkey: None,
//         }))
//     }
// }

/// Create a token account, for storing tokens
impl Command for CreateTokenAccountCommand {
    const COMMAND_NAME: &'static str = "create_token_account";
    const WIDGET_NAME: &'static str = "CreateTokenAccount";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "owner",
            &[PUBKEY],
            &true,
            "pubkey of the account owner",
            "",
            &false,
        ),
        CommandInput::new(
            "fee_payer",
            &[KEYPAIR],
            &true,
            "who pays for rent and transaction fees",
            "",
            &false,
        ), // who pays for the fees. (pays for account rent and transaction fees)
        CommandInput::new(
            "mint_account",
            &[PUBKEY],
            &true,
            "pubkey of the mint account",
            "",
            &false,
        ),
        CommandInput::new(
            "token_account",
            &[KEYPAIR],
            &false,
            "keypair of the account",
            "",
            &false,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //passthrough
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("owner", "Pubkey", &true, ""),
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("token_account", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::CreateTokenAccount(
            create_token_account::CreateTokenAccount {
                owner: None,
                fee_payer: None,
                mint_account: None,
                token_account: None,
            },
        ))
    }

    const DESCRIPTION: &'static str = "Create a token account";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

/// Generate keypair
/// A random keypair is generated if no inputs are passed
impl Command for GenerateKeypairCommand {
    const COMMAND_NAME: &'static str = "generate_keypair";
    const WIDGET_NAME: &'static str = "GenerateKeypair";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "private_key",
            &[STRING],
            &false,
            "pass to load one specific base 58 string, ignores seed/passphrase",
            "",
            &false,
        ), //
        CommandInput::new(
            "seed_phrase",
            &[STRING],
            &false,
            "",
            "12 word BIP39 mnemonic seed phrase",
            &false,
        ), // or
        CommandInput::new("passphrase", &[STRING], &false, "", "", &false),
        // CommandInput::new("save", &[STRING]), //
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("pubkey", "Pubkey", &false, ""),
        CommandOutput::new("keypair", "Keypair", &false, ""),
        // CommandOutput::new("node_id", "NodeId"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::GenerateKeypair(
            generate_keypair::GenerateKeypair {
                seed_phrase: Arg::None,
                passphrase: None,
                save: Arg::None,
                private_key: Arg::None,
            },
        ))
    }

    const DESCRIPTION: &'static str = "Generate or load a keypair and it's pubkey.\n\nWill generate a random keypair every run if no inputs are provided. This is useful for testing purpose.";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// Mint some tokens
impl Command for MintTokenCommand {
    const COMMAND_NAME: &'static str = "mint_token";
    const WIDGET_NAME: &'static str = "MintToken";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "mint_account",
            &[PUBKEY],
            &true,
            "pubkey of the mint account",
            "",
            &false,
        ),
        CommandInput::new(
            "mint_authority",
            &[KEYPAIR],
            &true,
            "keypair of the mint authority",
            "",
            &false,
        ), // mint authority
        CommandInput::new(
            "fee_payer",
            &[KEYPAIR],
            &true,
            "keypair of the fee payer",
            "",
            &false,
        ),
        CommandInput::new(
            "recipient",
            &[PUBKEY],
            &true,
            "pubkey of the mint account",
            "",
            &false,
        ), // pubkey of the mint account
        CommandInput::new(
            "amount",
            &[F64],
            &true,
            "NFTs should have amount = 1",
            "",
            &false,
        ), //f64
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new(
            "signature",
            "String",
            &false,
            "signature of the transaction",
        ),
        //passthrough
        CommandOutput::new(
            "mint_account",
            "Pubkey",
            &true,
            "pubkey of the mint account",
        ),
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("recipient", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::MintToken(mint_token::MintToken {
            mint_account: None,
            recipient: None,
            mint_authority: None,
            amount: None,
            fee_payer: None,
        }))
    }

    const DESCRIPTION: &'static str =
        "Mint tokens. It identifies the token, determines who can mint, and how many";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// TransferTokens tokens from one account to another
impl Command for TransferTokenCommand {
    const COMMAND_NAME: &'static str = "transfer_token";
    const WIDGET_NAME: &'static str = "TransferToken";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "mint_account",
            &[PUBKEY],
            &true,
            "pubkey of the mint account",
            "",
            &false,
        ),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new(
            "sender_owner",
            &[KEYPAIR],
            &true,
            "owner of sender account",
            "",
            &false,
        ), //
        //,
        CommandInput::new(
            "sender_token_account",
            &[PUBKEY],
            &false,
            "sender token account\n if empty, will be derived from sender owner",
            "",
            &false,
        ),
        CommandInput::new(
            "recipient",
            &[PUBKEY],
            &true,
            "token account or will derive a token account",
            "",
            &false,
        ), //
        CommandInput::new("amount", &[NUMBER], &true, "", "", &false),
        CommandInput::new(
            "allow_unfunded",
            &[BOOL],
            &false,
            "allow non token account recipient and derive a token account from recipient",
            "true",
            &true,
        ),
        CommandInput::new(
            "fund_recipient",
            &[BOOL],
            &false,
            "create recipient token account if it doesn't exist.",
            "true",
            &true,
        ),
        CommandInput::new("memo", &[STRING], &false, "", "", &false),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        // passthrough
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("sender_owner", "Pubkey", &true, ""),
        CommandOutput::new("recipient_account", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::TransferToken(transfer_token::TransferToken {
            fee_payer: None,
            mint_account: None,
            amount: None,
            recipient: None,
            sender_token_account: None,
            sender_owner: None,
            allow_unfunded: None,
            fund_recipient: None,
            memo: None,
        }))
    }

    const DESCRIPTION: &'static str = "Transfer a custom token";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for TransferSolanaCommand {
    const COMMAND_NAME: &'static str = "transfer_solana";
    const WIDGET_NAME: &'static str = "TransferSolana";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("sender", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("recipient", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("amount", &[NUMBER], &true, "in SOL", "", &false),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //
        CommandOutput::new("sender", "Keypair", &true, ""),
        CommandOutput::new("recipient", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::TransferSolana(
            transfer_solana::TransferSolana {
                sender: None,
                recipient: None,
                amount: None,
            },
        ))
    }

    const DESCRIPTION: &'static str = "Transfer SOL";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// rate limit, request amount suggestion
impl Command for RequestAirdropCommand {
    const COMMAND_NAME: &'static str = "request_airdrop";
    const WIDGET_NAME: &'static str = "RequestAirdrop";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("pubkey", &[PUBKEY], &true, "Airdrop recipient",
        "",
        &false,),
        CommandInput::new(
            "amount",
            &[U64],
            &true,
            "in lamports, 1 SOL = 1,000,000,000 lamports",
            "",
            &false,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] =
        &[CommandOutput::new("signature", "String", &false, "")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::RequestAirdrop(
            request_airdrop::RequestAirdrop {
                pubkey: None,
                amount: None,
            },
        ))
    }

    const DESCRIPTION: &'static str = "Airdrop SOL for testing purposes\n\nCurrently takes 30s to complete, pass the signature to a wait command";

    const AVAILABILITY: &'static [SolanaNet] = &[SolanaNet::Devnet, SolanaNet::Testnet];
}

impl Command for GetBalanceCommand {
    const COMMAND_NAME: &'static str = "get_balance";
    const WIDGET_NAME: &'static str = "GetBalance";
    const INPUTS: &'static [CommandInput] = &[CommandInput::new(
        "pubkey",
        &[PUBKEY],
        &true,
        "Account to query",
        "",
        &false,
    )];
    const OUTPUTS: &'static [CommandOutput] =
        &[CommandOutput::new("balance", "Number", &false, "")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::GetBalance(get_balance::GetBalance {
            pubkey: None,
        }))
    }

    const DESCRIPTION: &'static str = "Get the balance of the account";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// METAPLEX

impl Command for CreateMetadataAccountsCommand {
    const COMMAND_NAME: &'static str = "create_metadata_accounts";
    const WIDGET_NAME: &'static str = "CreateMetadataAccounts";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("mint_account", &[PUBKEY], &true, "token mint account",
        "",
        &false,),
        CommandInput::new("mint_authority", &[PUBKEY], &true, "",
        "",
        &false,),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "",
        "",
        &false,),
        CommandInput::new(
            "update_authority",
            &[KEYPAIR],
            &true,
            "who can update the on-chain metadata",
            "",
            &false,
        ),
        CommandInput::new(
            "metadata_uri",
            &[STRING],
            &true,
            "uri to the uploaded metadata\nusually stored off-chain, e.g. Arweave",
            "",
            &false,
        ),
        CommandInput::new(
            "metadata",
            &[NFT_METADATA],
            &true,
            "Metaplex standard metadata stored on-chain",
            "",
            &false,
        ),
        CommandInput::new(
            "is_mutable",
            &[BOOL],
            &false,
            "True allows the update authority to update the on-chain metadata",
            "false",
            &true,
        ),
        CommandInput::new(
            "uses",
            &[USES],
            &false,
            "How many and which type of uses each NFT will have.\nUses:\nBurn is a single-time use and is burned after use.\nSingle is a single-time use and does not burn the token.\nMultiple allows up to the specified number of uses",
            "",
            &false,
        ),
        CommandInput::new(
            "collection_mint_account",
            &[PUBKEY],
            &false,
            "",
            "",
            &false,
        ),
        CommandInput::new(
            "creators",
            &[NFT_CREATORS],
            &false,
            "",
            "",
            &false,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        CommandOutput::new("metadata_account", "Pubkey", &false, "metadata PDA account"),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("collection_mint_account", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::CreateMetadataAccounts(
            create_metadata_accounts::CreateMetadataAccounts {
                mint_account: None,
                mint_authority: None,
                fee_payer: None,
                update_authority: None,
                metadata_uri: None,
                metadata: None,
                is_mutable: None,
                uses: None,
                collection_mint_account: None,
                creators: None,
            },
        )))
    }

    const DESCRIPTION: &'static str =
        "Create a metadata  PDA account.\n\nOne account is created per token mint";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// token should already have a metadata account / should wait for Create Metadata Account
impl Command for CreateMasterEditionCommand {
    const COMMAND_NAME: &'static str = "create_master_edition";
    const WIDGET_NAME: &'static str = "CreateMasterEdition";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("mint_account", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("mint_authority", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("update_authority", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new(
            "max_supply",
            &[U64],
            &true,
            "How many copies you can print\n1/1 NFTs should have supply 0",
            "",
            &false,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        CommandOutput::new("metadata_account", "Pubkey", &false, ""),
        CommandOutput::new("master_edition_account", "Pubkey", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::CreateMasterEdition(
            create_master_edition::CreateMasterEdition {
                mint_account: None,
                mint_authority: None,
                fee_payer: None,
                update_authority: None,
                max_supply: solana::nft::create_master_edition::Arg::None,
            },
        )))
    }

    const DESCRIPTION: &'static str =
        "Creates a master edition account.\n\nDetermines whether a NFT can be copied.\nCopies of a master edition are called edition";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for UpdateMetadataAccountsCommand {
    const COMMAND_NAME: &'static str = "update_metadata_accounts";
    const WIDGET_NAME: &'static str = "UpdateMetadataAccounts";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("mint_account", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("update_authority", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("data", &[METADATA_ACCOUNT], &true, "", "", &false),
        CommandInput::new("new_update_authority", &[KEYPAIR], &false, "", "", &false),
        CommandInput::new("primary_sale_happened", &[BOOL], &false, "", "", &false),
        CommandInput::new("is_mutable", &[BOOL], &false, "", "", &false),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("metadata_account", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::UpdateMetadataAccounts(
            update_metadata_accounts::UpdateMetadataAccounts {
                mint_account: None,
                fee_payer: None,
                update_authority: None,
                new_update_authority: None,
                data: None,
                primary_sale_happened: None,
                is_mutable: None,
            },
        )))
    }

    const DESCRIPTION: &'static str = "Update the on-chain metadata";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for VerifyCollectionCommand {
    const COMMAND_NAME: &'static str = "verify_collection";
    const WIDGET_NAME: &'static str = "VerifyCollection";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("mint_account", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("collection_authority", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("collection_mint_account", &[PUBKEY], &true, "", "", &false),
        CommandInput::new(
            "collection_authority_is_delegated",
            &[BOOL],
            &false,
            "",
            "false",
            &true,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("collection_authority", "Keypair", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::VerifyCollection(
            verify_collection::VerifyCollection {
                mint_account: None,
                fee_payer: None,
                collection_authority: None,
                collection_mint_account: None,
                collection_authority_is_delegated: None,
            },
        )))
    }

    const DESCRIPTION: &'static str = "Verifies a collection\n\nhttp://docs.metaplex.com/token-metadata/specification#verifying-a-collection";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for SignMetadataCommand {
    const COMMAND_NAME: &'static str = "sign_metadata";
    const WIDGET_NAME: &'static str = "SignMetadata";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("mint_account", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("creator", &[KEYPAIR], &true, "", "", &false),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("creator", "Keypair", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::SignMetadata(
            sign_metadata::SignMetadata {
                fee_payer: None,
                mint_account: None,
                creator: None,
            },
        )))
    }

    const DESCRIPTION: &'static str = "Creator signing NFT";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for ApproveCollectionAuthorityCommand {
    const COMMAND_NAME: &'static str = "approve_collection_authority";
    const WIDGET_NAME: &'static str = "ApproveCollectionAuthority";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("new_collection_authority", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("update_authority", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("mint_account", &[PUBKEY], &true, "", "", &false),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("collection_authority", "Keypair", &true, ""),
        CommandOutput::new("update_authority", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Keypair", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ApproveCollectionAuthority(
            approve_collection_authority::ApproveCollectionAuthority {
                new_collection_authority: None,
                update_authority: None,
                fee_payer: None,
                mint_account: None,
            },
        )))
    }

    const DESCRIPTION: &'static str = "Who can verify collections";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for UtilizeCommand {
    const COMMAND_NAME: &'static str = "utilize";
    const WIDGET_NAME: &'static str = "Utilize";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "mint_account",
            &[PUBKEY],
            &true,
            "pubkey of the NFT",
            "",
            &false,
        ),
        // CommandInput::new("use_authority_record_pda", &[PUBKEY], &true, ""),
        CommandInput::new(
            "use_authority",
            &[KEYPAIR],
            &true,
            "e.g. the ticket holder",
            "",
            &false,
        ),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("owner", &[PUBKEY], &true, "", "", &false),
        CommandInput::new("account", &[PUBKEY], &false, "", "", &false),
        CommandInput::new(
            "burner",
            &[PUBKEY],
            &true,
            "",
            "Burner account to use",
            &false,
        ),
        CommandInput::new("number_of_uses", &[U64], &true, "", "", &false), //u64
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("use_authority", "Keypair", &true, ""),
        CommandOutput::new("owner", "Pubkey", &true, ""),
        CommandOutput::new("account", "Pubkey", &true, ""),
        CommandOutput::new("burner", "Pubkey", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::Utilize(utilize::Utilize {
            account: None,
            mint_account: None,
            // use_authority_record_pda: None,
            use_authority: None,
            fee_payer: None,
            owner: None,
            burner: None,
            number_of_uses: None,
        })))
    }

    const DESCRIPTION: &'static str =
        "Uses a token.\n\ne.g. allow a ticket booth to scan your ticket";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for ApproveUseAuthorityCommand {
    const COMMAND_NAME: &'static str = "approve_use_authority";
    const WIDGET_NAME: &'static str = "ApproveUseAuthority";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new(
            "mint_account",
            &[PUBKEY],
            &true,
            "NFT with uses",
            "",
            &false,
        ),
        CommandInput::new("owner", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new(
            "token_account",
            &[PUBKEY],
            &false,
            "owner token account.\nderived associated token address if empty",
            "",
            &false,
        ),
        CommandInput::new(
            "use_authority",
            &[PUBKEY],
            &true,
            "the delegated authority that can use the token\nwho receives the ticket",
            "",
            &false,
        ),
        CommandInput::new(
            "burner",
            &[PUBKEY],
            &true,
            "Where burned tokens are sent",
            "",
            &false,
        ),
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new(
            "number_of_uses",
            &[U64],
            &true,
            "If uses is Multiple, number of uses available",
            "",
            &false,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String", &false, ""),
        CommandOutput::new(
            "use_authority_record",
            "Pubkey",
            &false,
            "PDA of use authority record",
        ),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
        CommandOutput::new("mint_account", "Pubkey", &true, ""),
        CommandOutput::new("use_authority", "Pubkey", &true, ""),
        CommandOutput::new("owner", "Keypair", &true, ""),
        CommandOutput::new("token_account", "Pubkey", &true, ""),
        CommandOutput::new(
            "burner",
            "Pubkey",
            &true,
            "accounts burned NFTs are sent to",
        ),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ApproveUseAuthority(
            approve_use_authority::ApproveUseAuthority {
                use_authority: None,
                owner: None,
                fee_payer: None,
                token_account: None,
                mint_account: None,
                burner: None,
                number_of_uses: None,
            },
        )))
    }

    const DESCRIPTION: &'static str =
        "Allows a program to utilize a token.\n\ne.g. distribute a ticket";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

impl Command for GetLeftUsesCommand {
    const COMMAND_NAME: &'static str = "get_left_uses";
    const WIDGET_NAME: &'static str = "GetLeftUses";
    const INPUTS: &'static [CommandInput] = &[CommandInput::new(
        "mint_account",
        &[PUBKEY],
        &true,
        "",
        "",
        &false,
    )];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("left_uses", "U64", &false, "")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::GetLeftUses(
            get_left_uses::GetLeftUses { mint_account: None },
        )))
    }

    const DESCRIPTION: &'static str =
        "Gets remaining uses for a token\ne.g. how many tickets are available?";

    const AVAILABILITY: &'static [SolanaNet] =
        &[SolanaNet::Devnet, SolanaNet::Testnet, SolanaNet::Mainnet];
}

// impl Command for ArweaveUploadCommand {
//     const COMMAND_NAME: &'static str = "arweave_upload";
//     const WIDGET_NAME: &'static str = "ArweaveUpload";
//     const INPUTS: &'static [CommandInput] = &[
//         CommandInput::new("fee_payer", &[KEYPAIR]),
//         CommandInput::new("reward_mult", &[NUMBER]), //f32
//         CommandInput::new("file_path", &[STRING]),
//         CommandInput::new("arweave_key_path", &[STRING]),
//         CommandInput::new("pay_with_solana", &[BOOL]),
//     ];
//     const OUTPUTS: &'static [CommandOutput] = &[
//         CommandOutput::new("fee_payer", "Keypair"),
//         CommandOutput::new("file_uri", "String"),
//     ];
//     fn dimensions() -> NodeDimensions {
//         NodeDimensions {
//             height: calculate_node_height(Self),
//             width: 300,
//         }
//     }
//     fn config() -> CommandConfig {
//         CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveUpload(
//             arweave_upload::ArweaveUpload {
//                 fee_payer: None,
//                 reward_mult: None,
//                 file_path: None,
//                 arweave_key_path: None,
//                 pay_with_solana: None,
//             },
//         )))
//     }
// }

// impl Command for ArweaveNftUploadCommand {
//     const COMMAND_NAME: &'static str = "arweave_nft_upload";
//     const WIDGET_NAME: &'static str = "ArweaveNftUpload";
//     const INPUTS: &'static [CommandInput] = &[
//         CommandInput::new("fee_payer", &[KEYPAIR]),
//         CommandInput::new("reward_mult", &[NUMBER]), //f32
//         CommandInput::new("arweave_key_path", &[STRING]),
//         CommandInput::new("metadata", &[NFT_METADATA]),
//         CommandInput::new("pay_with_solana", &[BOOL]),
//     ];
//     const OUTPUTS: &'static [CommandOutput] = &[
//         CommandOutput::new("metadata_url", "String"),
//         CommandOutput::new("metadata", "String"),
//         CommandOutput::new("fee_payer", "Keypair"),
//     ];
//     fn dimensions() -> NodeDimensions {
//         NodeDimensions {
//             height: calculate_node_height(Self),
//             width: 300,
//         }
//     }
//     fn config() -> CommandConfig {
//         CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveNftUpload(
//             arweave_nft_upload::ArweaveNftUpload {
//                 fee_payer: None,
//                 reward_mult: None,
//                 arweave_key_path: None,
//                 metadata: None,
//                 pay_with_solana: None,
//             },
//         )))
//     }
// }
impl Command for ArweaveNftUploadCommand {
    const COMMAND_NAME: &'static str = "arweave_nft_upload";
    const WIDGET_NAME: &'static str = "ArweaveNftUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new(
            "metadata",
            &[NFT_METADATA],
            &true,
            "Metaplex standard metadata",
            "",
            &false,
        ),
        CommandInput::new(
            "fund_bundlr",
            &[BOOL],
            &false,
            "estimate upload size and fund bundlr node from fee payer",
            "true",
            &true,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("metadata_url", "String", &false, ""),
        CommandOutput::new("updated_metadata", "Nft Metadata", &false, ""),
        //
        CommandOutput::new("fee_payer", "Keypair", &true, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveNftUpload(
            arweave_nft_upload::ArweaveNftUpload {
                fee_payer: None,
                metadata: None,
                fund_bundlr: None,
            },
        )))
    }

    const DESCRIPTION: &'static str = "Uploads a Metaplex standard metadata and it's files to Arweave using the Bundlr Network.\n\nExperiment without paying on Devnet.\nAutomatically funds Bundlr node for upload unless overridden.\nGet approximate costs from \nhttps://www.feecalc.live/\nhttps://arweavefees.com/";

    const AVAILABILITY: &'static [SolanaNet] = &[SolanaNet::Devnet, SolanaNet::Mainnet];
}

impl Command for ArweaveFileUploadCommand {
    const COMMAND_NAME: &'static str = "arweave_file_upload";
    const WIDGET_NAME: &'static str = "ArweaveFileUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR], &true, "", "", &false),
        CommandInput::new("file_path", &[STRING], &true, "", "", &false),
        CommandInput::new(
            "fund_bundlr",
            &[BOOL],
            &false,
            "estimate upload size and fund bundlr node from fee payer",
            "true",
            &true,
        ),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("file_url", "String", &false, ""),
        CommandOutput::new("fee_payer", "Keypair", &false, ""),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveFileUpload(
            arweave_file_upload::ArweaveFileUpload {
                fee_payer: None,
                file_path: None,
                fund_bundlr: None,
            },
        )))
    }

    const DESCRIPTION: &'static str =
        "Uploads a single file to Arweave using the Bundlr Network.\n\nExperiment without paying on Devnet.\nAutomatically funds Bundlr node for upload unless you pass 'bool: false' the 'fund bundlr' input.\nGet approximate costs https://arweavefees.com/";

    const AVAILABILITY: &'static [SolanaNet] = &[SolanaNet::Devnet, SolanaNet::Mainnet];
}

const PRINTABLE: TypeBound = TypeBound {
    name: "Printable",
    types: &[
        "String",
        "Number",
        "Bool",
        "Pubkey",
        "Keypair",
        "NftCreators",
        "NftCollection",
        "Nft_Metadata",
        "Uses",
        "Metadata_Account",
        "Json",
        "U8",
    ],
};

const ANY: TypeBound = TypeBound {
    name: "Any",
    types: &[
        "String",
        "Number",
        "Bool",
        "Pubkey",
        "Keypair",
        "NftCreators",
        "NftCollection",
        "Nft_Metadata",
        "Uses",
        "Metadata_Account",
        "Json",
        "U8",
    ],
};

const PUBKEY: TypeBound = TypeBound {
    name: "Pubkey",
    types: &["Pubkey", "Keypair"],
};

const KEYPAIR: TypeBound = TypeBound {
    name: "Keypair",
    types: &["Keypair"],
};

const STRING: TypeBound = TypeBound {
    name: "String",
    types: &["String"],
};

const NUMBER: TypeBound = TypeBound {
    name: "Number",
    types: &["Number"],
};

const U8: TypeBound = TypeBound {
    name: "u8",
    types: &["u8"],
};
const U64: TypeBound = TypeBound {
    name: "u64",
    types: &["u64"],
};

const F64: TypeBound = TypeBound {
    name: "f64",
    types: &["f64"],
};

const BOOL: TypeBound = TypeBound {
    name: "Bool",
    types: &["Bool"],
};

const NFT_CREATORS: TypeBound = TypeBound {
    name: "NftCreators",
    types: &["NftCreators"],
};

const NFT_COLLECTION: TypeBound = TypeBound {
    name: "NftCollection",
    types: &["NftCollection"],
};

const USES: TypeBound = TypeBound {
    name: "Uses",
    types: &["Uses"],
};

const NFT_METADATA: TypeBound = TypeBound {
    name: "Nft_Metadata",
    types: &["Nft_Metadata"],
};

const METADATA_ACCOUNT: TypeBound = TypeBound {
    name: "Metadata_Account",
    types: &["Metadata_Account"],
};

const JSON: TypeBound = TypeBound {
    name: "Json",
    types: &["Json"],
};

/*
    const INTEGER: TypeBound = TypeBound {
        name: "Printable",
        types: &[
            "u8",
            "u16",
            "u32",
        ],
    };
    ...
*/

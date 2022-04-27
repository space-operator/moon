import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:tuple/tuple.dart';

final templates = Provider<Map<int, Tuple4<String, String, String, String>>>(
  (ref) {
    return const {
      // Name, Description, Relevant nodes used, Filename
      1: Tuple4(
        "NFT",
        "Create a 1/1 NFT",
        "Create Metadata Accounts, Create Master Edition, Arweave Nft Upload",
        "nft",
      ),
      2: Tuple4(
        "NFT Collection",
        "Create an NFT collection",
        "Create Metadata Accounts, Create Master Edition, Arweave Nft Upload, Sign Metadata",
        "nft_collection",
      ),
      3: Tuple4(
        "Create an NFT in a Collection",
        "Create a 1/1 NFT as part of a Collection",
        "Create Metadata Accounts, Create Master Edition, Arweave Nft Upload, Verify Collection",
        "nft_in_collection",
      ),
      4: Tuple4(
        "Update an NFT",
        "Update metadata of a mutable NFT",
        "Arweave Nft Upload, Json Insert, Update Metadata Accounts",
        "nft_update",
      ),
      5: Tuple4(
        "NFT Ticket",
        "Assigning an NFT ticket and the receiver using it",
        "Approve Use Authority, Get Left Uses, Wait, Utilize",
        "utilize",
      ),
      6: Tuple4(
        "Create, Mint, and Tranfer Token",
        "Create a custom token, mint tokens, and transfer some",
        "Airdrop, Wait, Create Mint, Create Token Account, Mint Token, Transfer Token",
        "create_mint_transfer",
      ),
      7: Tuple4(
        "Transfer Solana",
        "Transfer SOL from one account to another",
        "Transfer Solana",
        "transfer_solana",
      ),
      8: Tuple4(
        "Arweave File Upload",
        "Upload a file to Arweave",
        "Arweave File Upload",
        "arweave",
      ),
      9: Tuple4(
        "IPFS Upload",
        "Upload a file to pinata.cloud",
        "IPFS File Upload",
        "ipfs",
      ),
      10: Tuple4(
        "JSON Operations",
        "Extract and insert a value into a JSON",
        "JSON Extract, JSON Insert",
        "json",
      ),
      11: Tuple4(
        "Keypair",
        "Generate a keypair with a seed and passphrase",
        "Generate Keypair",
        "keypair",
      ),
      12: Tuple4(
        "HTTP Request",
        "Make an HTTP POST request",
        "HTTP Request",
        "http",
      ),
      13: Tuple4(
        "Branch and Wait",
        "Branch flow based on number comparison",
        "Branch, Wait",
        "branch",
      ),
    };
  },
);

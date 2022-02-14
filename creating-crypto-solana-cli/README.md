# Creating Cryptocurrency using Solana CLI

## Resources
- https://openquest.xyz/quest/create_crypto_with_solana_cli
- https://spl.solana.com/token
- https://github.com/solana-labs/solana-program-library

## Docs

### Installation
The Solana blockchain offers something called as programs, which are functions that reside on the Solana blockchain that help in performing complex transactions over the blockchain. Solana natively has a set of Programs deployed for usage. These set of on-chain programs are collectively known as the Solana Program Library or the SPL. To interact with these programs, we’ll be using the solana-cli.

```
sh -c "$(curl -sSfL https://release.solana.com/v1.7.12/install)"
solana --version

which cargo
cargo install spl-token-cli
```

### Creating a wallet

```
solana-keygen new
```
<img width="540" alt="image" src="https://user-images.githubusercontent.com/15700993/153786216-4f9d86cd-ff28-491a-98b3-2adbfb8efd1f.png">

```
solana airdrop 2 HDBNcroJgtFGKpcCoUW6u4rx4iYkqq5qYLuPBBrq5SHr
solana balance
```
<img width="824" alt="image" src="https://user-images.githubusercontent.com/15700993/153786328-69dfff02-0802-4b5b-9e06-0c36c7228d0f.png">

```
solana config get
```
<img width="388" alt="image" src="https://user-images.githubusercontent.com/15700993/153786295-121deff5-280d-407c-ad6a-4da4595a9877.png">


### Creating your own tokens
Before we move onto the next section, we must first discuss a bit about tokens and cryptocurrencies. Cryptocurrency refers to the native assets of a blockchain, i.e. the currency that is used to perform transactions in the blockchain. Popular examples include BTC for Bitcoin, ETH for ethereum and SOL for Solana. On the other hand, tokens are built on top of an existing blockchain and are implemented using smart contracts. To put it simply, they’re a tradeable asset that reside in the blockchain, and may or may not have a value.

Solana gives us many programs in it’s library by default for many of the standard tasks that we’d usually do on the Solana Blockchain. This is called the SPL library. One of the most powerful features of Solana is that it has native support for creating a token over its blockchain. Our token is actually created by a smart contract (program) in the SPL library. This program is known as the “SPL token program” and is the program that is used to create our own tokens. We’ll be using the spl-token CLI to interact with this SPL token program.

```
spl-token create-token
```

<img width="713" alt="image" src="https://user-images.githubusercontent.com/15700993/153786417-474d7259-c94a-475f-bcbf-8ae7f0624e3e.png">

We have now created our very first token! As mentioned earlier, tokens are created using the SPL smart contracts. These smart contracts have a unique address in the blockchain which is a string of alpha-numeric characters. The address of this smart contract that holds the logic for the token’s behaviour is known as the “token address”. For all the actions that we’ll be performing for our tokens such as minting, burning etc, we’ll be interacting with this smart contract and hence we need to note down the address of this smart contract.

### Minting new tokens

```
spl-token mint E6DFYL2aEKk65TkuMcta9wb3Uc8NAqwLEzovSpr7s71Q 1000
```

<img width="696" alt="image" src="https://user-images.githubusercontent.com/15700993/153786526-850d34b0-31f0-4e43-a00b-dcd70053ee87.png">

```
spl-token balance E6DFYL2aEKk65TkuMcta9wb3Uc8NAqwLEzovSpr7s71Q
```
<img width="60" alt="image" src="https://user-images.githubusercontent.com/15700993/153786772-d758e3c2-2e67-41a2-9cdc-0246f561688a.png">

```
spl-token supply E6DFYL2aEKk65TkuMcta9wb3Uc8NAqwLEzovSpr7s71Q
```
<img width="60" alt="image" src="https://user-images.githubusercontent.com/15700993/153786775-6fde5f1a-c800-49e1-bc27-15c588b81dd4.png">

### Limiting Total Supply and Burning our tokens

Solana provides us the ability to disable our minting authority, and never enable it back. We can do so by running the following command.

```
spl-token authorize E6DFYL2aEKk65TkuMcta9wb3Uc8NAqwLEzovSpr7s71Q mint --disable
```
<img width="725" alt="image" src="https://user-images.githubusercontent.com/15700993/153787034-ff4e6a7a-9a84-40b4-a630-a4b8996b5e31.png">

```
 spl-token mint E6DFYL2aEKk65TkuMcta9wb3Uc8NAqwLEzovSpr7s71Q 1000
```
<img width="955" alt="image" src="https://user-images.githubusercontent.com/15700993/153787150-7d6aec56-d0b6-474c-b600-274bd1dbd99c.png">

```
spl-token burn HysXUsjTanoNChEA22QCzGcBKYnX8H57XXXtYa3GvShm 200
```

<img width="707" alt="image" src="https://user-images.githubusercontent.com/15700993/153787499-5a8f4730-ea8b-4e33-bf3a-8740305c04e1.png">



### Naming your Tokens


In Phantom wallet, we see that our token is listed as an “Unknown Token”. If we want our token to have a widely recognised name, symbol and image, we need to send a pull request to the solana-labs/token-list: https://github.com/solana-labs/token-list


Add an entry to the “solana.tokenlist.json” file in the “src/tokens” directory. The JSON entry would have the following structure.


```
{
   "chainId":101,
   "address":"your token address",
   "symbol":"your token symbol",
   "name":"Token Name",
   "decimals":"<number of decimals your token supports>",
   "logoURI":"the CDN link to your token's image",
   "tags":[
      "tags_if_any"
   ],
   "extensions":{

   }
}

```

Once the PR gets approved, you’ll be able to view the token image and name on every wallet in the Solana ecosystem.


### Transaction History for this exercise can be found here
https://explorer.solana.com/address/HDBNcroJgtFGKpcCoUW6u4rx4iYkqq5qYLuPBBrq5SHr?cluster=devnet

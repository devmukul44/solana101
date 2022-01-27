const {
    Connection,
    PublicKey,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL,
    Transaction,
    Account,
} = require("@solana/web3.js");

// STEP-1 Generating a new wallet keypair
// The Keypair class that we just imported allows us to create a new wallet.
const newPair = new Keypair();
console.log(newPair);

// STEP-2 Storing the public and private key
// extracting the public key from accountInfo and storing it in a new variable called myPublicKey which is of type string
const publicKey = new PublicKey(newPair._keypair.publicKey).toString();
const secretKey = newPair._keypair.secretKey
console.log(publicKey);
console.log(secretKey);

// STEP-3 Getting the wallet Balance
// Web3.js allows us to view the balance using the getBalance method inside the connection class that we had imported
const getWalletBalance = async () => {
    try {
        // Apart from the main network (called mainnet), Solana also maintains clusters called devnet and testnet
        // clusterApiUrl provides us the URL for devnet that we’ll be passing to create our connection object so that we get details of devnet
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        // create a wallet object from the secretKey
        const myWallet = await Keypair.fromSecretKey(secretKey);
        //  querying the balance of this wallet.
        const walletBalance = await connection.getBalance(
            new PublicKey(myWallet.publicKey)
        );
        console.log(`=> For wallet address ${publicKey}`);
        console.log(`   Wallet balance: ${parseInt(walletBalance)/LAMPORTS_PER_SOL}SOL`);
    } catch (err) {
        console.log(err);
    }
};

// STEP-4 Air dropping SOL (in terms of LAMPORTS)
// function that’ll airdrop some SOL into our wallet
const airDropSol = async () => {
    try {
        // create a connection object and a walletKeyPair object for the airdrop functio
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        const walletKeyPair = await Keypair.fromSecretKey(secretKey);
        // you can airdrop at max 2SOL in one transaction
        console.log(`-- Airdropping 2 SOL --`)
        const fromAirDropSignature = await connection.requestAirdrop(
            new PublicKey(walletKeyPair.publicKey),
            2 * LAMPORTS_PER_SOL
        );
        // await a confirmation for the transaction from the network. Add the following lines to do so.
        await connection.confirmTransaction(fromAirDropSignature);
    } catch (err) {
        console.log(err);
    }
};

//STEP-5 Driver function
const driverFunction = async () => {
    await getWalletBalance();
    await airDropSol();
    await getWalletBalance();
}

// execute program
driverFunction();

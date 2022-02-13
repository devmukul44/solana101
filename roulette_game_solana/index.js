const web3 = require("@solana/web3.js");

// Establishing Connection with devnet
const connection=new web3.Connection(web3.clusterApiUrl("devnet"),"confirmed");
// For checking whether the connection is successfully made
// console.log(connection);

// Generating Address and Funds
// We can generate a new wallet pair using the Keypair method.
const from=web3.Keypair.generate();
console.log("from wallet address");
console.log(from);
console.log(new web3.PublicKey(from.publicKey).toString());

const to=web3.Keypair.generate();
console.log("to wallet address");
console.log(to);
console.log(new web3.PublicKey(to.publicKey).toString());

const getWalletBalance=async (pubk)=>{
    try{
        const connection=new web3.Connection(web3.clusterApiUrl("devnet"),"confirmed");
        const balance=await connection.getBalance(new web3.PublicKey(pubk));
        return balance/web3.LAMPORTS_PER_SOL;
    }catch(err){
        console.log(err);
    }
}

// function that’ll airdrop some SOL into our wallet
const airDropSol = async (secretKey) => {
    try {
        // create a connection object and a walletKeyPair object for the airdrop functio
        const connection=new web3.Connection(web3.clusterApiUrl("devnet"),"confirmed");
        const walletKeyPair = await web3.Keypair.fromSecretKey(secretKey);
        // you can airdrop at max 2SOL in one transaction
        console.log(`-- Airdropping 2 SOL --`)
        const fromAirDropSignature = await connection.requestAirdrop(
            new web3.PublicKey(walletKeyPair.publicKey),
            2 * web3.LAMPORTS_PER_SOL
        );
        // await a confirmation for the transaction from the network. Add the following lines to do so.
        await connection.confirmTransaction(fromAirDropSignature);
    } catch (err) {
        console.log(err);
    }
};

const transferSOL=async (from,to,transferAmt)=>{
    try{
        const connection=new web3.Connection(web3.clusterApiUrl("devnet"),"confirmed");
        const transaction=new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey:new web3.PublicKey(from.publicKey.toString()),
                toPubkey:new web3.PublicKey(to.publicKey.toString()),
                lamports:transferAmt*web3.LAMPORTS_PER_SOL
            })
        )
        const signature=await web3.sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        )
        return signature;
    }catch(err){
        console.log(err);
    }
}


const driverFunction = async () => {
    console.log("from wallet balance");
    console.log(await getWalletBalance(from.publicKey))
    console.log("to wallet balance");
    console.log(await getWalletBalance(to.publicKey))


    await airDropSol(from.secretKey)
    console.log("from wallet balance");
    console.log(await getWalletBalance(from.publicKey))
    console.log("to wallet balance");
    console.log(await getWalletBalance(to.publicKey))

    console.log("-- Transferring 1 SOL --")
    await transferSOL(from, to, 1)
    console.log("from wallet balance");
    console.log(await getWalletBalance(from.publicKey))
    console.log("to wallet balance");
    console.log(await getWalletBalance(to.publicKey))
}
driverFunction()
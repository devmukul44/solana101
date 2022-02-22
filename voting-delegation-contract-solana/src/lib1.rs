use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// As a part of the crowdfunding platform,
/// we’ll store the amount requested, the description of the campaign
/// and how much has been fulfilled so far, for each campaign
///
/// We will create an account for each campaign.
/// All these accounts will be managed by the logic we write.
/// Money can be sent to these campaign accounts.
/// That is how we will crowdfund our Campaigns.
/// These are called Program Derived Accounts (PDAs).
///
/// We also need to tell the compiler to make our ProgramData datastructure to be storable.
/// The way to store this data on to the Solana Persistent Storage, is to serialize it.
/// We’ll use a serializer called the BorchSerializer
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignAccount {
    pub campaign_owner: Pubkey,
    pub campaign_amount: u64,
    pub campaign_description: String,
    pub campaign_fulfilled: u64,
}

/// Every program must have an entrypoint
/// that will be called each time some user wants to interact with the code.
entrypoint!(process_instruction);

/// If the function is called to create a new campaign,
/// you need to create a new account and for this campaign and send that as a parameter.
/// If you are calling this function to fund a campaign,
/// the accounts-array will contain the account of the funder and the account of the campaign being funded.
///
/// `program_id`
///     is the identifier for the program itself.
/// `accounts`
///     array of accounts that will be operated upon in this program.
///     Every user on Solana has a unique account.
///     Every code that you write and deploy also has a unique account.
///     An account can store money in a currency called Lamports. We
///     can transfer lamports from one account to another.
///     When you send a user some money, they can spend it however they want.
///     If you send money to the account operated by a program,
///     that program can use that money defined in the logic of the code itself.
///     Every account has a public key and a private key
/// `data`
///     tells what the function needs to do and all the data that is required to execute the logic.
///     We’ll assume that the first byte of this data tells us what instruction to run (operation code)
///     The next 4 bytes will be the amount.
///     The rest of the bytes will be the string that describes this campaign.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    // fetch the accounts one after another, by iterating over the list.
    // The first account in the accounts array will be the campaign we want to operate upon.
    // Either to create a new campaign or to fund an existing campaign.
    // This owner will also be the same person who will be able to withdraw the funds later.
    let campaign_account = next_account_info(accounts_iter)?;

    let (instruction_byte, rest_of_data) = data.split_first().unwrap();

    // The first byte was the operation code.
    // The next 4 bytes will be the amount.
    let amount = rest_of_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap();

    // The rest of the bytes will be the string that describes this campaign.
    let description = String::from_utf8(rest_of_data[9..].to_vec()).unwrap();

    if *instruction_byte == 0 {
        // create campaign
        // To create the campaign, we need to send the account info of the person creating this
        // campaign in our accounts parameter.
        // We will assume that one person can have only one campaign running at any point in time.
        let campaign_owner_account = next_account_info(accounts_iter)?;

        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        campaign_account_data.campaign_owner = *campaign_owner_account.owner;
        campaign_account_data.campaign_amount = amount;
        campaign_account_data.campaign_description = description;
        campaign_account_data.campaign_fulfilled = 0;
        campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
    }

    if *instruction_byte == 1 {
        // get campaign status
        // send money to campaign
        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        msg!("{}",campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);

    }

    if *instruction_byte == 2 {

    }

    if *instruction_byte == 3 {

    }


    Ok(())
}

use {anchor_lang::prelude::*, anchor_lang::solana_program::account_info::AccountInfo};
use anchor_lang::solana_program::sysvar::clock::Clock;
use solana_program::vote::program::ID;

#[program]
pub mod voting {
    use super::*;
    /// initialize fn can initialize base account with public-key
    pub fn initialize(ctx: Context<StartStuffOff>, authority: Pubkey) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.pubkey = authority;

        Ok(())
    }

    /// Add voter publickey to base_account
    pub fn add_voters(ctx: Context<OwnerContext>, voters: Vec<Pubkey>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        for address in voters.iter() {
            // finding address in voters array
            let mut exists = false;

            for (index, voter) in base_account.voters.iter().enumerate() {
                if voter.address == *address {
                    base_account.voters[index].is_voter = true;
                    exists = true;
                    break;
                }
            }

            if !exists {
                base_account.voters.push(Voters {
                    address: *address,
                    is_voter: true,
                });
            }
        }

        Ok(())
    }

    /// create proposal
    ///
    /// ctx         OwnerContext leveraging Anchor Account and Anchor Signer
    /// name        proposal title
    /// desc        proposal description
    /// choices     choice vector for proposal
    /// offset      help determine endtime(current timestamp + offset) of the proposal
    pub fn create_proposal(
        ctx: Context<OwnerContext>,
        name: String,
        desc: String,
        choices: Vec<String>,
        offset: u32,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let next_proposal_id = base_account.next_proposal_id;
        let mut choice_array: Vec<Choice> = vec![];

        for (index, choice) in choices.iter().enumerate() {
            choice_array.push(Choice {
                id: index as u128,
                name: choice.clone(),
                votes: 0,
            });
        }

        base_account.proposals.push(ProposalArray {
            id: next_proposal_id,
            proposal: Proposal {
                id: next_proposal_id,
                proposal: name,
                description: desc,
                end_time_stamp: Clock::get().unwrap().unix_timestamp as u128 + offset as u128,
                choices: choice_array.clone(),
            },
        });

        base_account.next_proposal_id += 1;

        Ok(())
    }

    /// Vote for a proposal and choice
    pub fn vote(ctx: Context<VoteContext>, proposal_id: u32, choice_id: u32) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let msg_sender = ctx.accounts.msg_sender.key();

        // check if voter can vote for the proposal
        for voter in base_account.voters.iter() {
            if voter.address == msg_sender {
                assert!(voter.is_voter, "vote:: only voters can vote");
                break;
            }
        }

        let mut address_index = 0;
        let mut address_exists = false;
        let mut proposal_exists = false;
        let mut votes_index = 0;
        let mut proposal_index = 0;

        for (address_idx, voter) in base_account.votes.iter().enumerate() {
            if voter.address == msg_sender {
                address_exists = true;
                for (votes_idx, votes) in voter.votes_util.iter().enumerate() {
                    if votes.proposal == proposal_id as u128 {
                        address_index = address_idx;
                        votes_index = votes_idx;
                        proposal_exists = true;
                        break;
                    }
                }
                break;
            }
        }

        if !address_exists {
            base_account.votes.push(Votes {
                address: msg_sender,
                votes_util: vec![VotesUtils {
                    proposal: proposal_id as u128,
                    voted: false,
                }],
            });
            address_index = base_account.votes.len() - 1;
        }

        if !proposal_exists {
            base_account.votes[address_index]
                .votes_util
                .push(VotesUtils {
                    proposal: proposal_id as u128,
                    voted: false,
                });
            votes_index = base_account.votes[address_index].votes_util.len() - 1;
        }

        assert!(
            !base_account.votes[address_index].votes_util[votes_index].voted,
            "vote:: voter can only vote once for a ballot"
        );

        for (index, proposal) in base_account.proposals.iter().enumerate() {
            if proposal.id == proposal_id as u128 {
                proposal_index = index;
            }
        }

        assert!(
            (Clock::get().unwrap().unix_timestamp as u128)
                < base_account.proposals[proposal_index]
                .proposal
                .end_time_stamp,
            "vote:: only voters can vote"
        );

        base_account.votes[address_index].votes_util[votes_index].voted = true;
        base_account.proposals[proposal_index].proposal.choices[choice_id as usize].votes += 1;

        Ok(())
    }
}

/// Initializer
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // Anchor Lang Account Initialization
    #[account(init, payer = user, space = 6942)]
    pub base_account: Account<'info, BaseAccount>,
    // anchor lang Signer
    #[account(mut)]
    pub user: Signer<'info>,
    // anchor lang System Program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
}

/// Context for only owner functions
#[derive(Accounts)]
pub struct OwnerContext<'info> {
    #[account(mut, has_one = authority)]
    pub base_account: Account<'info, BaseAccount>,
    pub authority: Signer<'info>,
}

/// Proposal Choice
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Choice {
    pub id: u128,
    pub name: String,
    pub votes: u128,
}

/// Proposal
///
/// id              proposal id
/// proposal        proposal title
/// description     proposal description
/// choices         proposal Choices
/// end_time_stamp  proposal end time
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Proposal {
    pub id: u128,
    pub proposal: String,
    pub description: String,
    pub choices: Vec<Choice>,
    pub end_time_stamp: u128,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Voters {
    pub address: Pubkey,
    pub is_voter: bool,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProposalArray {
    pub id: u128,
    pub proposal: Proposal,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VotesUtils {
    pub proposal: u128,
    pub voted: bool,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Votes {
    pub address: Pubkey,
    pub votes_util: Vec<VotesUtils>,
}

/// BaseAccount
/// Type: Anchor Account
///
/// pubkey              owner PubKey
/// voters              Voters vector
/// proposals           ProposalArray vector
/// next_proposal_id    next_proposal_id
/// votes               Votes vector
#[account]
pub struct BaseAccount {
    // For owner check
    pub pubkey: Pubkey,

    // Contract variables
    pub voters: Vec<Voters>,
    pub proposals: Vec<ProposalArray>,
    pub next_proposal_id: u128,
    pub votes: Vec<Votes>,
}

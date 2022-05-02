use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("AzQWUGXtjN1P6gmYFAJE1WFop15vgMotBTBnxVyz2gcL");

#[program]
pub mod solana_nft_program {
    use super::*;

    
        pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
            let erc = &mut ctx.accounts.erc;
            erc.greeting = init_message;
            Ok(())
        }

        pub fn balance_of(ctx: Context<BalanceOf>,init_balance: u64) -> ProgramResult{
            let erc = &mut ctx.accounts.erc;
            erc.balance = init_balance;
            Ok(())
        }

        pub fn total_supply(ctx: Context<TotalSupply>, total_supply_of_nft: i64) -> ProgramResult{
            let erc = &mut ctx.accounts.erc;
            erc.result = total_supply_of_nft;
            Ok(())
        }

        //safetramsfer_from method
        pub fn safetramsfer_from(ctx: Context<SafeTransferFrom>, address_from: String, address_to: String, _token_id: i64) -> ProgramResult{
            let safetramsfer_from = &mut ctx.accounts.erc;
            safetramsfer_from.from = address_from;
            safetramsfer_from.to = address_to;
            safetramsfer_from.token_id = _token_id;
            Ok(())
        }

        //owner of method
        pub fn owner_of(ctx: Context<OwnerOf>, _token_id: i64) -> ProgramResult{
            let owner = &mut ctx.accounts.erc;
            owner.token_id = _token_id;
            Ok(())
        }

        //approve method
        pub fn approve(ctx: Context<Approve>, _approved: String, _token_id: i64) -> ProgramResult{
            let approve = &mut ctx.accounts.erc;
            approve.token_id = _token_id;
            approve.approved = _approved;
            Ok(())
        }



        // pub fn test(ctx: Context<Test>, from_address: &Pubkey) -> ProgramResult{
        //     let test = &mut ctx.accounts.erc;
        //     test.from1 = from_address
        // }


    #[derive(Accounts)]
    pub struct Create<'info> {
        #[account(init, payer=user, space=264)]
        pub erc: Account<'info, Erc>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>
    }

    // Total supply
    #[derive(Accounts)]
    pub struct TotalSupply<'info>{
        #[account(mut)]
        pub erc: Account<'info, Erc>
    }

    //Balance of
    #[derive(Accounts)]
    pub struct BalanceOf<'info>{
        #[account(mut)]
        pub erc: Account<'info, Erc>,
    }

    //Safe transform 
    #[derive(Accounts)]
    pub struct SafeTransferFrom<'info>{
        #[account(mut)]
        pub erc: Account<'info, Erc>,
        #[account(mut)]
        pub user: Signer<'info>,
    }

    //Owner of
    #[derive(Accounts)]
    pub struct OwnerOf<'info>{
        #[account(mut)]
        pub erc: Account<'info, Erc>,
    }

    #[derive(Accounts)]
    pub struct Approve<'info>{
        #[account(mut)]
        pub erc: Account<'info, Erc>,
    }

    //Test 
    // #[derive(Accounts)]
    // pub struct Test<'info>{
    //     #[account(mut)]
    //     pub erc: Account<'info, Erc>,
    // }

    #[account]
    pub  struct Erc{
        pub greeting: String,
        pub display: String,
        pub result: i64,
        pub remainder: i64,
        pub balance: u64,
        pub from: String,
        pub to: String,
        pub token_id: i64,
        pub approved: String
        // pub from1: &'erc Pubkey
    }
}


// How to define public key data type in solana

// pub fn create_account(
//     from_pubkey: &Pubkey,
//     to_pubkey: &Pubkey,
//     lamports: u64,
//     space: u64,
//     owner: &Pubkey,
// ) -> Instruction {
//     let account_metas = vec![
//         AccountMeta::new(*from_pubkey, true),
//         AccountMeta::new(*to_pubkey, true),
//     ];
//     Instruction::new_with_bincode(
//         system_program::id(),
//         &SystemInstruction::CreateAccount {
//             lamports,
//             space,
//             owner: *owner,
//         },
//         account_metas,
//     )
// }

//Define struct type

// pub struct AccountInfo<'a> {
//     pub key: &'a Pubkey,
//     pub is_signer: bool,
//     pub is_writable: bool,
//     pub lamports: Rc<RefCell<&'a mut u64>>,
//     pub data: Rc<RefCell<&'a mut [u8]>>,
//     pub owner: &'a Pubkey,
//     pub executable: bool,
//     pub rent_epoch: Epoch,
// }
// use solana_program::{
//     account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
// };

mod entrypoint;
mod instruction;
mod processor;
mod state;
mod error;

// entrypoint!(process_instruction);
// fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     msg!(
//         "process_instruction: {}: {} accounts, data={:?}",
//         program_id,
//         accounts.len(),
//         instruction_data
//     );
//     Ok(())
// }
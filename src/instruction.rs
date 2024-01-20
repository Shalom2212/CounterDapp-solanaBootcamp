use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug,BorshDeserialize,BorshSerialize)]

pub struct Args{
    pub value : u32
}

pub enum CounterInstruction{
    Increment(Args),
    Decrement(Args),
    Update(Args),
    Reset,
}

impl CounterInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError>{
        let (&variant,rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => self::CounterInstruction::Increment(Args::try_from_slice(rest).unwrap()),
            1 => self::CounterInstruction::Decrement(Args::try_from_slice(rest).unwrap()),
            2 => self::CounterInstruction::Update(Args::try_from_slice(rest).unwrap()),
            3 => self::CounterInstruction::Reset,
            _ => return Err(ProgramError::InvalidAccountData)

        })
    }
}
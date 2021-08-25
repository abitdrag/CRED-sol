use solana_program::program_error::ProgramError;
pub enum AcctInstruction{



    UID {
    
    uid: u8,
}
Action{
    actionid: u8
}



}
impl AcctInstruction {
    /// Unpacks a byte buffer into a  AcctInstruction
    pub fn unpack(input: &[u8]) -> Result<Self> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::UID {
                uid: Self::unpack_amount(rest)?,
            },
            1 => Self::Action {
                actionid: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

 //Function to unpack instructi
}

use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum StudentInstruction {
    AddStudentGreeting { name: String, message: String },
    UpdateStudentGreeting { message: String },
}

#[derive(BorshDeserialize)]
struct StudentInstructionPayload {
    name: String,
    message: String,
}

impl StudentInstruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Take the first byte as the variant to
        // determine which instruction to execute
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        // Use the temporary payload struct to deserialize
        let payload = StudentInstructionPayload::try_from_slice(rest).unwrap();
        // Match the variant to determine which data struct is expected by
        // the function and return the Struct or an error
        Ok(match variant {
            0 => Self::AddStudentGreeting {
                name: payload.name,
                message: payload.message,
            },
            1 => Self::UpdateStudentGreeting {
                message: payload.message,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

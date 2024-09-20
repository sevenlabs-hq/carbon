use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::instruction::AccountMeta;

use crate::error::{CarbonResult, Error};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Discriminator {
    OneByte(u8),
    TwoBytes([u8; 2]),
    FourBytes([u8; 4]),
    EightBytes([u8; 8]),
    SixteenBytes([u8; 16]),
}

#[allow(dead_code)]
impl Discriminator {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self.clone() {
            Discriminator::OneByte(d) => std::slice::from_ref(&d).to_vec(),
            Discriminator::TwoBytes(d) => d.to_vec(),
            Discriminator::FourBytes(d) => d.to_vec(),
            Discriminator::EightBytes(d) => d.to_vec(),
            Discriminator::SixteenBytes(d) => d.to_vec(),
        }
    }

    pub fn one_byte_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 1 {
            return Err(Error::MissingInstructionData);
        }
        Ok(Discriminator::OneByte(data[0]))
    }

    pub fn two_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 2 {
            return Err(Error::MissingInstructionData);
        }
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&data[..2]);
        Ok(Discriminator::TwoBytes(buf))
    }

    pub fn four_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 4 {
            return Err(Error::MissingInstructionData);
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&data[..4]);
        Ok(Discriminator::FourBytes(buf))
    }

    pub fn eight_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 8 {
            return Err(Error::MissingInstructionData);
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data[..8]);
        Ok(Discriminator::EightBytes(buf))
    }

    pub fn sixteen_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 16 {
            return Err(Error::MissingInstructionData);
        }
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&data[..16]);
        Ok(Discriminator::SixteenBytes(buf))
    }
}

pub trait InstructionAccounts: Sized {
    fn unpack(accounts: &[AccountMeta]) -> CarbonResult<Self>;
}

pub trait InstructionData: Sized + BorshDeserialize + BorshSerialize {
    fn discriminator() -> Discriminator;
    fn unpack(data: &[u8]) -> CarbonResult<Self> {
        let mut data = data;

        let discriminator = match Self::discriminator() {
            Discriminator::OneByte(_) => Discriminator::OneByte(
                match u8::deserialize(&mut data).map_err(|_| Error::InvalidDataLength) {
                    Ok(val) => val,
                    Err(e) => return Err(e),
                },
            ),
            Discriminator::TwoBytes(_) => {
                let mut buf = [0u8; 2];
                if data.len() < 2 {
                    return Err(Error::InvalidDataLength);
                }
                buf.copy_from_slice(&data[..2]);
                data = &data[2..];
                Discriminator::TwoBytes(buf)
            }
            Discriminator::FourBytes(_) => {
                let mut buf = [0u8; 4];
                if data.len() < 4 {
                    return Err(Error::InvalidDataLength);
                }
                buf.copy_from_slice(&data[..4]);
                data = &data[4..];
                Discriminator::FourBytes(buf)
            }
            Discriminator::EightBytes(_) => {
                let mut buf = [0u8; 8];
                if data.len() < 8 {
                    return Err(Error::InvalidDataLength);
                }
                buf.copy_from_slice(&data[..8]);
                data = &data[8..];
                Discriminator::EightBytes(buf)
            }
            Discriminator::SixteenBytes(_) => {
                let mut buf = [0u8; 16];
                if data.len() < 16 {
                    return Err(Error::InvalidDataLength);
                }
                buf.copy_from_slice(&data[..16]);
                data = &data[16..];
                Discriminator::SixteenBytes(buf)
            }
        };
        if discriminator != Self::discriminator() {
            return Err(Error::InvalidInstructionDiscriminator);
        }
        let data =
            Self::deserialize(&mut data).map_err(|_| Error::InvalidInstructionDiscriminator)?;
        Ok(data)
    }
}

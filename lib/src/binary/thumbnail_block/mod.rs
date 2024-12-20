use core::fmt::Display;

use super::{
    block_header::{block_header_parser, BlockHeader},
    compression_type::CompressionType,
};
use nom::{
    bytes::streaming::take,
    combinator::verify,
    number::streaming::{le_u16, le_u32},
    sequence::preceded,
    IResult, InputTake,
};

mod param;
use param::param_parser;
use param::Param;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThumbnailBlock {
    header: BlockHeader,
    pub param: Param,
    /// binary data.
    pub data: Vec<u8>,
    checksum: Option<u32>,
}
impl Display for ThumbnailBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "-------------------------- ThumbnailBlock --------------------------"
        )?;
        writeln!(f)?;
        writeln!(f, "Params")?;
        write!(f, "{}", self.param)?;
        writeln!(f, "DataBlock omitted")?;
        writeln!(f)?;
        write!(f, "-------------------------- ThumbnailBlock ")?;
        match self.checksum {
            Some(checksum) => writeln!(f, "Ckecksum Ox{checksum:X} ---------")?,
            None => writeln!(f, "No checksum")?,
        };
        Ok(())
    }
}

static THUMBNAIL_BLOCK_ID: u16 = 5u16;
pub fn thumbnail_parser_with_checksum(input: &[u8]) -> IResult<&[u8], ThumbnailBlock> {
    let (after_block_header, header) = preceded(
        verify(le_u16, |block_type| {
            log::debug!(
                "Looking for THUMBNAIL_BLOCK_ID {THUMBNAIL_BLOCK_ID} found {block_type} cond {}",
                *block_type == THUMBNAIL_BLOCK_ID
            );
            *block_type == THUMBNAIL_BLOCK_ID
        }),
        block_header_parser,
    )(input)?;

    log::info!("Found thumbnail block id");
    let BlockHeader {
        compression_type,
        uncompressed_size,
        compressed_size,
    } = header.clone();

    let (after_param, param) = param_parser(after_block_header)?;

    // Decompress datablock
    let (after_data, data_raw) = match compression_type {
        CompressionType::None => take(uncompressed_size)(after_param)?,
        CompressionType::Deflate => {
            // Special case extracted data is not a string.
            log::info!("TODO: Must implement decompression");
            todo!()
        }
        CompressionType::HeatShrink11 => {
            let (_remain, _data_compressed) = take(compressed_size.unwrap())(after_param)?;
            log::info!("TODO: Must implement decompression");
            todo!()
        }
        CompressionType::HeatShrink12 => {
            let (_remain, _data_compressed) = take(compressed_size.unwrap())(after_param)?;
            log::info!("TODO: Must implement decompression");
            todo!()
        }
    };

    let data = data_raw.to_vec();

    let (after_checksum, checksum) = le_u32(after_data)?;

    let param_size = 6;
    let block_size = header.size_in_bytes() + param_size + header.payload_size_in_bytes();
    let crc_input: Vec<u8> = input.take(block_size).to_vec();
    let computed_checksum = crc32fast::hash(&crc_input);

    log::debug!("thumbnail checksum 0x{checksum:04x} computed checksum 0x{computed_checksum:04x} ");
    if checksum == computed_checksum {
        log::debug!("checksum match");
    } else {
        log::error!("failed checksum");
        panic!("file metadata block failed checksum");
    }

    Ok((
        after_checksum,
        ThumbnailBlock {
            header,
            param,
            data,
            checksum: Some(checksum),
        },
    ))
}

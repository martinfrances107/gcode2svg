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
    AsBytes, IResult,
};

mod param;
use param::param_parser;
use param::Param;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintMetadataBlock {
    header: BlockHeader,
    param: Param,
    data: String,
    checksum: Option<u32>,
}
impl Display for PrintMetadataBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "-------------------------- PrintMetadataBlock --------------------------"
        )?;
        writeln!(f, "Params")?;
        writeln!(f, "params {:#?}", self.param)?;
        writeln!(f)?;
        writeln!(f, "DataBlock {}", self.data)?;
        writeln!(f)?;
        write!(f, "-------------------------- PrintMetadataBlock ")?;
        match self.checksum {
            Some(checksum) => writeln!(f, "Ckecksum Ox{checksum:X} ---------")?,
            None => writeln!(f, "No checksum")?,
        };
        Ok(())
    }
}

static PRINT_METADATA_BLOCK_ID: u16 = 5u16;
pub fn print_metadata_parser_with_checksum(input: &[u8]) -> IResult<&[u8], PrintMetadataBlock> {
    let (after_block_header, header) = preceded(
        verify(le_u16, |block_type| {
            println!(
                "looking for PRINT_METADATA_BLOCK_ID {PRINT_METADATA_BLOCK_ID} found {block_type} cond {}",
                *block_type == PRINT_METADATA_BLOCK_ID
            );
            *block_type == PRINT_METADATA_BLOCK_ID
        }),
        block_header_parser,
    )(input)?;

    let BlockHeader {
        compression_type,
        uncompressed_size,
        ..
    } = header.clone();
    eprintln!("about to check param ");
    let (after_param, param) = param_parser(after_block_header)?;
    eprintln!("Param value -- {param:#?}");
    eprintln!("uncompressed_size -- {uncompressed_size:#?}");
    // Decompress datablock
    let (after_data, data_raw) = match compression_type {
        CompressionType::None => take(uncompressed_size)(after_param)?,
        CompressionType::Deflate => {
            let (_remain, _data_compressed) = take(uncompressed_size)(after_param)?;
            // Must decompress here
            todo!()
        }
        CompressionType::HeatShrink11 => {
            let (_remain, _data_compressed) = take(uncompressed_size)(after_param)?;
            // Must decompress here
            todo!()
        }
        CompressionType::HeatShrink12 => {
            let (_remain, _data_compressed) = take(uncompressed_size)(after_param)?;
            // Must decompress here
            todo!()
        }
    };

    let data = match param.encoding {
        0 => {
            print!("wtf");
            String::from_utf8(data_raw.to_vec()).expect("raw data error")
        }
        2u16 => String::from("A meatpacked string with comments handling"),
        _ => {
            panic!("bad encoding");
        }
    };

    let (after_checksum, checksum_value) = le_u32(after_data)?;

    Ok((
        after_checksum,
        PrintMetadataBlock {
            header,
            param,
            data,
            checksum: Some(checksum_value),
        },
    ))
}
use byteorder::WriteBytesExt;
use byteorder::{BigEndian, ReadBytesExt};
use ferrumc_general_purpose::data_packing::u32::read_nbit_u32;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::chunk_format::Heightmaps;
use ferrumc_world::chunk_format::{BiomeStates, BlockStates, Chunk, PaletteType, Section};
use std::collections::HashMap;
use std::io::Cursor;

fn encode_direct_section(section: &Section) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    buf.write_u16::<BigEndian>(section.block_states.non_air_blocks)
        .unwrap();
    if let PaletteType::Direct {
        bits_per_block,
        data,
    } = &section.block_states.block_data
    {
        buf.write_u8(*bits_per_block).unwrap();
        for entry in data {
            buf.write_i64::<BigEndian>(*entry).unwrap();
        }
    } else {
        panic!("expected direct palette section");
    }
    buf.write_u8(section.biome_states.bits_per_biome).unwrap();
    if section.biome_states.bits_per_biome == 0 {
        if let Some(biome) = section.biome_states.palette.first() {
            biome.write(&mut buf).unwrap();
        } else {
            VarInt::from(0).write(&mut buf).unwrap();
        }
    } else {
        VarInt::from(section.biome_states.palette.len() as i32)
            .write(&mut buf)
            .unwrap();
        for biome in &section.biome_states.palette {
            biome.write(&mut buf).unwrap();
        }
        for entry in &section.biome_states.data {
            buf.write_i64::<BigEndian>(*entry).unwrap();
        }
    }
    buf.into_inner()
}

#[test]
fn direct_palette_round_trip() {
    // Create a section using direct palette filled with air (id 0)
    let mut counts = HashMap::new();
    counts.insert(BlockId(0), 4096);
    let section = Section {
        y: 0,
        block_states: BlockStates {
            non_air_blocks: 0,
            block_data: PaletteType::Direct {
                bits_per_block: 15,
                data: vec![0; 1024],
            },
            block_counts: counts,
        },
        biome_states: BiomeStates {
            bits_per_biome: 0,
            data: vec![],
            palette: vec![VarInt::from(0)],
        },
        block_light: vec![0; 2048],
        sky_light: vec![0; 2048],
    };
    let mut chunk = Chunk {
        x: 0,
        z: 0,
        dimension: "overworld".to_string(),
        sections: vec![section],
        heightmaps: Heightmaps::default(),
        block_entities: vec![],
    };

    // Set a couple of blocks to non-zero ids
    chunk.set_block(0, 0, 0, BlockId(1)).unwrap();
    chunk.set_block(1, 0, 0, BlockId(2)).unwrap();

    // Ensure get_block works for direct palette
    assert_eq!(chunk.get_block(0, 0, 0).unwrap(), BlockId(1));
    assert_eq!(chunk.get_block(1, 0, 0).unwrap(), BlockId(2));

    // Serialize section using direct palette encoding
    let bytes = encode_direct_section(&chunk.sections[0]);

    // Parse first section's block data from encoded bytes
    let mut cursor = Cursor::new(bytes);
    let non_air = cursor.read_u16::<BigEndian>().unwrap();
    assert_eq!(non_air, 2);
    let bits_per_block = cursor.read_u8().unwrap();
    assert_eq!(bits_per_block, 15);
    let longs = (4096 * bits_per_block as usize).div_ceil(64);
    let mut data = Vec::with_capacity(longs);
    for _ in 0..longs {
        data.push(cursor.read_i64::<BigEndian>().unwrap());
    }

    // Read back the blocks we set
    let read_block = |x: i32, y: i32, z: i32| {
        let index = ((y & 0xf) * 256 + (z & 0xf) * 16 + (x & 0xf)) as usize;
        let blocks_per_i64 = (64f64 / bits_per_block as f64).floor() as usize;
        let i64_index = index / blocks_per_i64;
        let offset = (index % blocks_per_i64) * bits_per_block as usize;
        let val = read_nbit_u32(&data[i64_index], bits_per_block, offset as u32).unwrap();
        BlockId(val)
    };

    assert_eq!(read_block(0, 0, 0), BlockId(1));
    assert_eq!(read_block(1, 0, 0), BlockId(2));
}

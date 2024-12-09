use std::{collections::VecDeque, error::Error, path::is_separator};
use aochelpers::get_daily_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Filled(usize), 
    Empty
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

struct BlockAllocation{ 
    length: usize,
    block_type: BlockType

}
fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(9,2024)?;
    let blocks: VecDeque<BlockAllocation> = VecDeque::from(parse_data(&data));
    let result = defragment(blocks);
    println!("Part 1: {}", result);
    let result = defragment2(parse_data(&data));
    println!("Part 2: {}", result);

    Ok(())
}

fn defragment(mut disk_map: VecDeque<BlockAllocation>) -> usize {

    let mut defragmented_disk = Vec::new();
    let mut available_blocks = 0;
    'outer: while let Some(mut next_file) = disk_map.pop_back() {
        match next_file.block_type {
            BlockType::Filled(_) => {
                if available_blocks >= next_file.length {
                    defragmented_disk.push(next_file);
                    available_blocks -= next_file.length;
                } else {
                    let next_fragment = BlockAllocation{length: available_blocks, block_type: next_file.block_type};
                    defragmented_disk.push(next_fragment);
                    next_file.length -=available_blocks;
                    available_blocks = 0;
                    while next_file.length > 0 {
                        match disk_map.pop_front() {
                            None => {
                                defragmented_disk.push(next_file);
                                break 'outer;
                            }
                            Some(b) if b.block_type == BlockType::Empty => {
                                available_blocks += b.length;
                                if available_blocks >= next_file.length {
                                    available_blocks -= next_file.length;
                                    defragmented_disk.push(next_file);
                                    next_file.length = 0;
                                } else {
                                    let next_fragment = BlockAllocation{length: b.length, block_type: next_file.block_type};
                                    defragmented_disk.push(next_fragment);
                                    next_file.length -= b.length;
                                    available_blocks -= b.length;
                                }
                            },
                            Some(b) => {  
                                defragmented_disk.push(b);
                            }
                        }
                    }
                }
                
            },
            BlockType::Empty => {},
        }
    }
    score_defrag(&defragmented_disk)
}

fn defragment2(mut disk_map: Vec<BlockAllocation>) -> usize {

    let mut unmovable = Vec::new();
    while let Some(next_file) = disk_map.pop() {
        match next_file.block_type {
            BlockType::Filled(n) => {
                let mut next_map: Vec<BlockAllocation> = Vec::new();
                let mut placed = false;
                for mut entry in disk_map.iter().copied() {
                    match entry.block_type {
                        BlockType::Empty if !placed => {
                            if entry.length >= next_file.length{
                                next_map.push(next_file);
                                unmovable.push(BlockAllocation{block_type: BlockType::Empty, length: next_file.length});
                                entry.length -= next_file.length;
                                placed = true;
                            }
                            if entry.length != 0 {
                                next_map.push(entry); 
                            }
                        },
                        _ => {
                            next_map.push(entry);
                        },
                    }
                }
                if !placed {
                    unmovable.push(next_file);
                } 

                disk_map = consolidate_blocks(next_map);
            },
            BlockType::Empty => {
                unmovable.push(next_file);
            },
        }
    }
    unmovable.reverse();
    score_defrag(&unmovable)
}

fn score_defrag(defragmented_disk: &Vec<BlockAllocation>) -> usize{
    let mut position = 0;
    let mut result = 0;
    for item in defragmented_disk {
        if let BlockType::Filled(id) = item.block_type{
            for _ in 0..item.length {
                result += position * id;
                position +=1;
            }
        } else {
            position += item.length
        }
    }

    result
}

fn consolidate_blocks(filesystem: Vec<BlockAllocation>) -> Vec<BlockAllocation>{
    let mut next_fs = Vec::new();
    let mut last_block = BlockAllocation{block_type: BlockType::Empty, length: 0};
    for item in filesystem {
        if item.block_type == BlockType::Empty && last_block.block_type == BlockType::Empty {
            last_block.length += item.length;
        } else {
            next_fs.push(last_block);
            last_block = item;
        }
    }
    next_fs.push(last_block);
    next_fs
}
 
fn parse_data(data: &str) -> Vec<BlockAllocation> {
    let mut result = Vec::new();
    let mut current_block_id = 0;
    for (i, c) in data.chars().enumerate() {
        if i %2 == 0 {
            result.push(BlockAllocation{length: c.to_digit(10).unwrap() as usize, block_type: BlockType::Filled(current_block_id)});
            current_block_id +=1;
        } else {
            result.push(BlockAllocation{length: c.to_digit(10).unwrap() as usize, block_type: BlockType::Empty});
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let expected = vec![BlockAllocation{length:1, block_type:BlockType::Filled(0)},
                                                                    BlockAllocation{length:2, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:3, block_type:BlockType::Filled(1)},
                                                                    BlockAllocation{length:4, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:5, block_type:BlockType::Filled(2)}];
        assert_eq!(parse_data("12345"), expected);

        let expected = vec![BlockAllocation{length:2, block_type:BlockType::Filled(0)},
                                                                    BlockAllocation{length:3, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:3, block_type:BlockType::Filled(1)},
                                                                    BlockAllocation{length:3, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:1, block_type:BlockType::Filled(2)},
                                                                    BlockAllocation{length:3, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:3, block_type:BlockType::Filled(3)},
                                                                    BlockAllocation{length:1, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:2, block_type:BlockType::Filled(4)},
                                                                    BlockAllocation{length:1, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:4, block_type:BlockType::Filled(5)},
                                                                    BlockAllocation{length:1, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:4, block_type:BlockType::Filled(6)},
                                                                    BlockAllocation{length:1, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:3, block_type:BlockType::Filled(7)},
                                                                    BlockAllocation{length:1, block_type:BlockType::Empty},
                                                                    BlockAllocation{length:4, block_type:BlockType::Filled(8)},
                                                                    BlockAllocation{length:0, block_type:BlockType::Empty},

                                                                    BlockAllocation{length:2, block_type:BlockType::Filled(9)},
                                                                    ];
        assert_eq!(parse_data("2333133121414131402"), expected);

    }
    #[test]
    fn test_part1() {

        let data =  VecDeque::from(parse_data("2333133121414131402"));
        assert_eq!(defragment(data), 1928);
    }

    #[test]
    fn test_part2() {
        let data  = parse_data("2333133121414131402");
        assert_eq!(defragment2(data), 2858);

    }
}

// 6256152319960 too low
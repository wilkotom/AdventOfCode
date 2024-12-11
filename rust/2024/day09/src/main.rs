use core::alloc;
use std::{collections::{BinaryHeap, VecDeque}, error::Error};
use aochelpers::{get_daily_input, ScoredItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BlockType {
    Filled(usize), 
    Empty
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

struct BlockAllocation{ 
    length: usize,
    block_type: BlockType,
    position: usize
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
                    let next_fragment = BlockAllocation{length: available_blocks, block_type: next_file.block_type, position: 0};
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
                                    let next_fragment = BlockAllocation{length: b.length, block_type: next_file.block_type, position: 0};
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
    let mut unused_block_fragments =  vec![BinaryHeap::new(); 10];
    for allocation in &disk_map {
        if allocation.block_type == BlockType::Empty {
            unused_block_fragments[allocation.length].push(ScoredItem{cost: allocation.position, item: *allocation});
        }
    }
    let mut total = 0;
    while let Some(mut allocation) = disk_map.pop() {
        if let BlockType::Filled(id) = allocation.block_type {
        let mut length_selection = None;
        let mut best_position = allocation.position;
        for free_length in allocation.length..=9 {
            if let Some(potential_position) = unused_block_fragments[free_length].peek() {
                if best_position > potential_position.item.position {
                    best_position = potential_position.item.position;
                    length_selection = Some(free_length);
                }
            }
        } 
        if let Some(length) = length_selection {
            let mut unallocated: BlockAllocation =  unused_block_fragments[length].pop().unwrap().item;
            allocation.position = unallocated.position;
            unallocated.length -= allocation.length;
            unallocated.position += allocation.length;
            unused_block_fragments[unallocated.length].push(ScoredItem{cost: unallocated.position, item: unallocated});

        }
        total += ((allocation.length * allocation.position) + ((allocation.length-1) * allocation.length) /2) * id
        }
    }
    total
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
 
fn parse_data(data: &str) -> Vec<BlockAllocation> {
    let mut result = Vec::new();
    let mut current_block_id = 0;
    let mut position = 0;
    for (i, c) in data.chars().enumerate() {
        let length = c.to_digit(10).unwrap() as usize;
        if i %2 == 0 {
            result.push(BlockAllocation{length, block_type: BlockType::Filled(current_block_id), position});
            current_block_id +=1;
        } else {
            result.push(BlockAllocation{length, block_type: BlockType::Empty, position});
        }
        position += length;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let expected = vec![BlockAllocation{length:1, block_type:BlockType::Filled(0), position: 0},
                                                                    BlockAllocation{length:2, block_type:BlockType::Empty, position:1},
                                                                    BlockAllocation{length:3, block_type:BlockType::Filled(1), position:3},
                                                                    BlockAllocation{length:4, block_type:BlockType::Empty, position: 6},
                                                                    BlockAllocation{length:5, block_type:BlockType::Filled(2), position: 10}];
        assert_eq!(parse_data("12345"), expected);

        let expected = vec! [BlockAllocation { length: 2, block_type: BlockType::Filled(0), position: 0 }, 
            BlockAllocation { length: 3, block_type: BlockType::Empty, position: 2 }, 
            BlockAllocation { length: 3, block_type: BlockType::Filled(1), position: 5 }, 
            BlockAllocation { length: 3, block_type: BlockType::Empty, position: 8 }, 
            BlockAllocation { length: 1, block_type: BlockType::Filled(2), position: 11 }, 
            BlockAllocation { length: 3, block_type: BlockType::Empty, position: 12 }, 
             BlockAllocation { length: 3, block_type: BlockType::Filled(3), position: 15 }, 
             BlockAllocation { length: 1, block_type: BlockType::Empty, position: 18 }, 
             BlockAllocation { length: 2, block_type: BlockType::Filled(4), position: 19 }, 
             BlockAllocation { length: 1, block_type: BlockType::Empty, position: 21 }, 
             BlockAllocation { length: 4, block_type: BlockType::Filled(5), position: 22 }, 
             BlockAllocation { length: 1, block_type: BlockType::Empty, position: 26 }, 
             BlockAllocation { length: 4, block_type: BlockType::Filled(6), position: 27 }, 
             BlockAllocation { length: 1, block_type: BlockType::Empty, position: 31 }, 
             BlockAllocation { length: 3, block_type: BlockType::Filled(7), position: 32 }, 
             BlockAllocation { length: 1, block_type: BlockType::Empty, position: 35 }, 
             BlockAllocation { length: 4, block_type: BlockType::Filled(8), position: 36 }, 
             BlockAllocation { length: 0, block_type: BlockType::Empty, position: 40 }, 
             BlockAllocation { length: 2, block_type: BlockType::Filled(9), position: 40 }];
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
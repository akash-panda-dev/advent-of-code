use itertools::Itertools;

fn build_disk_map(input: &str) -> Vec<Option<usize>> {
    //This will return the diskmap and a stack with all the file blocks

    let mut file_id: usize = 0;
    let mut diskmap: Vec<Option<usize>> = Vec::new();

    for (index, block) in input.bytes().enumerate() {
        let block_size = block - b'0';
        if index % 2 == 0 {
            diskmap.extend(vec![Some(file_id); block_size.into()]);
            file_id += 1;
        } else {
            if block_size > 0 {
                diskmap.extend(vec![None; block_size.into()]);
            }
        }
    }

    diskmap
}

// fn build_disk_map2(input: &str) -> Vec<(Option<usize>, u64)> {
//     let mut file_id: usize = 0;
//     let mut diskmap: Vec<(Option<usize>, u64)> = Vec::new();
//
//     for (index, block) in input.trim().chars().enumerate() {
//         let block_size = block
//             .to_digit(10)
//             .unwrap_or_else(|| panic!("Invalid digit character: '{}'", block))
//             as usize;
//
//         if index % 2 == 0 {
//             diskmap.extend(vec![(Some(file_id), block_size); block_size as usize]);
//             file_id += 1;
//         } else {
//             diskmap.extend(vec![(None, block_size); block_size as usize]);
//         }
//     }
//     diskmap
// }

pub mod part1 {
    use std::collections::VecDeque;

    use crate::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process_2(input: &str) -> Result<usize> {
        let disk_map = build_disk_map(&input);
        let mut tail = disk_map.len() - 1;
        let mut compact_disk: Vec<usize> = Vec::new();
        // traverse the diskmap and for every None swap it with a Some from the end. The index of
        // tail must be greater head
        for (head, file_id_opt) in disk_map.clone().into_iter().enumerate() {
            if head <= tail {
                if let None = file_id_opt {
                    while head < tail {
                        if disk_map[tail].is_some() {
                            compact_disk.push(disk_map[tail].unwrap());
                            tail -= 1;
                            break;
                        } else {
                            tail -= 1;
                        }
                    }
                } else {
                    compact_disk.push(file_id_opt.unwrap());
                }
            }
        }
        dbg!(&compact_disk);
        Ok(compact_disk
            .into_iter()
            .enumerate()
            .map(|(index, file_id)| index * file_id as usize)
            .sum())
    }

    #[tracing::instrument]
    pub fn process1(input: &str) -> Result<usize> {
        let disk_map = build_disk_map(&input);
        let mut tail = disk_map.len() - 1;
        let mut compact_disk: Vec<usize> = Vec::new();

        // traverse the diskmap and for every None swap it with a Some from the end. The index of
        // tail must be greater head
        let mut head = 0;

        while head <= tail {
            if disk_map[head].is_none() {
                // Find next Some value from the end
                while head < tail && disk_map[tail].is_none() {
                    tail -= 1;
                }
                if disk_map[tail].is_some() {
                    compact_disk.push(disk_map[tail].unwrap());
                    tail -= 1;
                }
            } else {
                compact_disk.push(disk_map[head].unwrap());
            }
            head += 1;
        }

        dbg!(&compact_disk);

        Ok(compact_disk
            .into_iter()
            .enumerate()
            .map(|(index, file_id)| index * file_id as usize)
            .sum())
    }

    pub fn process(input: &str) -> Result<usize> {
        let disk_map = build_disk_map(input);
        let mut deque: VecDeque<Option<usize>> = VecDeque::from(disk_map);
        let mut result = Vec::new();

        while !deque.is_empty() {
            let mut curr: Option<usize> = deque.pop_front().unwrap();
            if curr.is_none() {
                // Keep popping from right until we find a non-None value
                while !deque.is_empty() && curr.is_none() {
                    curr = deque.pop_back().unwrap();
                }
            }

            if let Some(file_id) = curr {
                result.push(file_id);
            }
        }

        Ok(result
            .iter()
            .enumerate()
            .map(|(pos, &file_id)| pos * file_id as usize)
            .sum())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "2333133121414131402";
        assert_eq!(1928, process(input)?);
        Ok(())
    }
}

fn build_disk_map_tuples(input: &str) -> Vec<MemChunk> {
    let mut file_id: usize = 0;
    let mut diskmap: Vec<MemChunk> = Vec::new();

    for (index, block) in input.trim().chars().enumerate() {
        let block_size = block
            .to_digit(10)
            .unwrap_or_else(|| panic!("Invalid digit character: '{}'", block))
            as usize;

        if index % 2 == 0 {
            if block_size == 0 {
                panic!("woah 0 block file!");
            }
            diskmap.push(MemChunk::from(
                Some(vec![file_id; block_size as usize]),
                block_size,
            ));
            file_id += 1;
        } else {
            if block_size > 0 {
                // diskmap.push((None, block_size));
                diskmap.push(MemChunk::from(None, block_size));
            }
        }
    }
    diskmap
}

#[derive(Debug, Clone)]
struct MemChunk {
    file_ids: Option<Vec<usize>>,
    free_space: usize,
    capacity: usize,
}

impl MemChunk {
    fn from(file_ids: Option<Vec<usize>>, capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity can never be zero");
        }
        MemChunk {
            file_ids: file_ids.clone(),
            capacity,
            free_space: file_ids.map_or(capacity, |f_ids| capacity - f_ids.len()),
        }
    }

    fn push_file_ids(&mut self, new_file_ids: &Vec<usize>) -> () {
        let new_files = new_file_ids.clone();
        let new_files_len = new_file_ids.len();
        if self.free_space < new_files_len {
            panic!(
                "Not enough free space. Free: {}, New files len: {}",
                self.free_space, new_files_len
            );
        }

        if let Some(ref mut f_ids) = self.file_ids {
            f_ids.extend(new_files);
        } else {
            self.file_ids = Some(new_files);
        }
        self.free_space -= new_files_len;
    }

    fn empty_chunk(&mut self) -> () {
        if self.file_ids.is_none() {
            panic!("Free space should not be emptied")
        }

        self.file_ids = None;
        self.free_space = self.capacity;
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    fn find_contiguous_space(
        diskmap: &[MemChunk],
        start: usize,
        tail: usize,
        required_space: usize,
    ) -> Option<(usize, usize)> {
        let mut accumulated_space = 0;
        let mut end = start;

        while end < tail && accumulated_space < required_space {
            let free_space = diskmap[end].free_space;
            if free_space == 0 {
                return None;
            }
            accumulated_space += free_space;
            end += 1;
        }

        if accumulated_space >= required_space {
            Some((start, end))
        } else {
            None
        }
    }

    fn move_files(diskmap: &mut [MemChunk], files: &[usize], start_chunk: usize, end_chunk: usize) {
        let mut remaining_files = files.to_vec();
        (start_chunk..end_chunk).for_each(|i| {
            let free = diskmap[i].free_space;
            let num_to_move = free.min(remaining_files.len());
            let moved_files: Vec<_> = remaining_files.drain(..num_to_move).collect();
            diskmap[i].push_file_ids(&moved_files);
        });
    }

    pub fn process(input: &str) -> Result<usize> {
        let mut diskmap = build_disk_map_tuples(&input);
        let mut tail = diskmap.len() - 1;

        while tail > 0 {
            if let Some(tail_files) = &diskmap[tail].file_ids.clone() {
                let total_files = tail_files.len();

                for start_head in 0..tail {
                    if let Some((start_chunk, end_chunk)) =
                        find_contiguous_space(&diskmap, start_head, tail, total_files)
                    {
                        move_files(&mut diskmap, &tail_files, start_chunk, end_chunk);
                        diskmap[tail].empty_chunk();
                        break;
                    }
                }
            }

            tail -= 1;
        }

        Ok(get_checksum(diskmap))
    }

    fn get_checksum(diskmap: Vec<MemChunk>) -> usize {
        let mut index: usize = 0;
        let mut checksum: usize = 0;
        for chunk in diskmap {
            if let Some(files) = chunk.file_ids {
                for f in files {
                    checksum += f * index;
                    index += 1;
                }
            }
            index += chunk.free_space;
        }

        checksum
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "2333133121414131402";
        assert_eq!(2858, process(input)?);
        Ok(())
    }
}

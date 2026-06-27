use std::collections::HashMap;

pub struct FragmentAssembler {
    fragments: HashMap<u16, FragmentState>,
}

struct FragmentState {
    total_size: u32,
    data: Vec<u8>,
    received: u32,
}

impl FragmentAssembler {
    pub fn new() -> Self {
        Self {
            fragments: HashMap::new(),
        }
    }

    pub fn add_fragment(&mut self, sequence: u16, data: &[u8], is_first: bool) -> Option<Vec<u8>> {
        if is_first && data.len() >= 4 {
            let total_size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            let payload = &data[4..];

            if payload.len() as u32 >= total_size {
                return Some(payload[..total_size as usize].to_vec());
            }

            self.fragments.insert(sequence, FragmentState {
                total_size,
                data: payload.to_vec(),
                received: payload.len() as u32,
            });
            None
        } else {
            let first_seq = self.find_first_sequence(sequence);
            let (complete, total_size) = {
                let state = self.fragments.get_mut(&first_seq)?;
                state.data.extend_from_slice(data);
                state.received += data.len() as u32;
                (state.received >= state.total_size, state.total_size)
            };

            if complete {
                let mut result = self.fragments.remove(&first_seq).unwrap().data;
                result.truncate(total_size as usize);
                Some(result)
            } else {
                None
            }
        }
    }

    fn find_first_sequence(&self, current: u16) -> u16 {
        for dist in 1..=100u16 {
            let candidate = current.wrapping_sub(dist);
            if self.fragments.contains_key(&candidate) {
                return candidate;
            }
        }
        current
    }

    pub fn pending_count(&self) -> usize {
        self.fragments.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_fragment_completes() {
        let mut asm = FragmentAssembler::new();
        let mut data = vec![];
        data.extend_from_slice(&5u32.to_be_bytes()); // total_size = 5
        data.extend_from_slice(b"Hello");

        let result = asm.add_fragment(0, &data, true);
        assert_eq!(result, Some(b"Hello".to_vec()));
        assert_eq!(asm.pending_count(), 0);
    }

    #[test]
    fn multi_fragment_assembly() {
        let mut asm = FragmentAssembler::new();

        // First fragment: total_size header + partial data
        let mut first = vec![];
        first.extend_from_slice(&10u32.to_be_bytes()); // total = 10 bytes
        first.extend_from_slice(b"Hello");
        assert!(asm.add_fragment(0, &first, true).is_none());
        assert_eq!(asm.pending_count(), 1);

        // Second fragment: remaining data
        let result = asm.add_fragment(1, b"World", false);
        assert_eq!(result, Some(b"HelloWorld".to_vec()));
    }
}

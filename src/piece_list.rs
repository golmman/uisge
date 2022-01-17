use crate::constants::BitBoard;

const END_OF_LIST: u8 = 0xff;

pub struct PieceListIterator {
    index: usize,
    pieces: [u8; 8],
}

impl PieceListIterator {
    pub fn new(piece_list: PieceList) -> Self {
        Self {
            index: 0,
            pieces: piece_list.to_array(),
        }
    }
}

impl Iterator for PieceListIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let piece = self.pieces[self.index];
        self.index += 1;

        if piece == END_OF_LIST {
            None
        } else {
            Some(piece)
        }
    }
}

#[derive(Clone, Copy)]
pub struct PieceList {
    pieces: u64,
}

impl PieceList {
    pub fn new() -> Self {
        Self { pieces: END_OF_LIST.into() }
    }

    pub fn to_array(&self) -> [u8; 8] {
        self.pieces.to_le_bytes()
    }
}

impl IntoIterator for PieceList {
    type Item = u8;
    type IntoIter = PieceListIterator;

    fn into_iter(self) -> Self::IntoIter {
        PieceListIterator::new(self)
    }
}

impl From<BitBoard> for PieceList {
    fn from(bit_board: BitBoard) -> Self {
        let mut piece_list = PieceList::new();
        piece_list.pieces = bit_board as u64;

        piece_list
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iterate_trivial() {
        let piece_list = PieceList::new();

        let mut piece_vec = Vec::<i32>::new();

        for piece in piece_list {
            println!("{}", piece);
            piece_vec.push(piece as i32);
        }

        assert_eq!(piece_vec, vec![]);
    }

    #[test]
    fn test_iterate_simple() {
        let piece_list = PieceList::from(0xff01020407fe7f);

        let mut piece_vec = Vec::<i32>::new();

        for piece in piece_list {
            println!("{}", piece);
            piece_vec.push(piece as i32);
        }

        assert_eq!(piece_vec, vec![127, 254, 7, 4, 2, 1]);
    }

    #[test]
    fn test_iterate_early_return() {
        let piece_list = PieceList::from(0xff010204ff0708);

        let mut piece_vec = Vec::<i32>::new();

        for piece in piece_list {
            println!("{}", piece);
            piece_vec.push(piece as i32);
        }

        assert_eq!(piece_vec, vec![8, 7]);
    }
}

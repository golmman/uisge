use crate::constants::{BitBoard, BoardIndex};

const END_OF_LIST: BoardIndex = 0xff;

pub struct PieceListIterator {
    index: usize,
    pieces: [BoardIndex; 8],
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
    type Item = BoardIndex;

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
    pieces: BitBoard,
}

impl PieceList {
    pub fn new() -> Self {
        Self {
            pieces: END_OF_LIST.into(),
        }
    }

    pub fn to_array(&self) -> [BoardIndex; 8] {
        let bytes = self.pieces.to_le_bytes();

        *unsafe { &*(&bytes as *const _ as *const [BoardIndex; 8]) }
    }
}

impl IntoIterator for PieceList {
    type Item = BoardIndex;
    type IntoIter = PieceListIterator;

    fn into_iter(self) -> Self::IntoIter {
        PieceListIterator::new(self)
    }
}

impl From<BitBoard> for PieceList {
    fn from(bit_board: BitBoard) -> Self {
        let mut piece_list = PieceList::new();
        piece_list.pieces = bit_board;

        piece_list
    }
}

#[cfg(test)]
mod test {
    use crate::constants::BoardIndex;

    use super::*;
    #[test]
    fn test_iterate_trivial() {
        let piece_list = PieceList::new();

        let mut piece_vec = Vec::<BoardIndex>::new();

        for piece in piece_list {
            piece_vec.push(piece as BoardIndex);
        }

        assert_eq!(piece_vec, vec![]);
    }

    #[test]
    fn test_iterate_simple() {
        let piece_list = PieceList::from(0xff01020407217f);

        let mut piece_vec = Vec::<BoardIndex>::new();

        for piece in piece_list {
            piece_vec.push(piece as BoardIndex);
        }

        assert_eq!(piece_vec, vec![127, 33, 7, 4, 2, 1]);
    }

    #[test]
    fn test_iterate_early_return() {
        let piece_list = PieceList::from(0xff010204ff0708);

        let mut piece_vec = Vec::<BoardIndex>::new();

        for piece in piece_list {
            piece_vec.push(piece as BoardIndex);
        }

        assert_eq!(piece_vec, vec![8, 7]);
    }
}

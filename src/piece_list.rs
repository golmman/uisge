use std::fmt::Display;

use crate::constants::{BitBoard, BoardIndex, BoardIndexList};

const END_OF_LIST: BoardIndex = 0xff;
const EMPTY_PIECE_LIST: BitBoard = 0xffffffffffffffff;

pub struct PieceListIterator {
    index: usize,
    pieces: BoardIndexList,
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
            pieces: EMPTY_PIECE_LIST,
        }
    }

    pub fn to_array(&self) -> BoardIndexList {
        self.pieces.to_le_bytes()
    }

    pub fn push_front(&mut self, value: BoardIndex) {
        self.pieces = (self.pieces << 8) | (value as BitBoard);
    }

    pub fn remove(&mut self, index: usize) -> Option<BoardIndex> {
        let shift = index * 8;

        let lower = if shift == 0 {
            0
        } else {
            (self.pieces << (64 - shift)) >> (64 - shift)
        };

        let result = self.pieces >> shift;
        let upper = result >> 8;

        self.pieces = (lower | (upper << shift)) | 0xff00000000000000;

        if result as u8 == END_OF_LIST {
            return None;
        } else {
            Some(result as u8)
        }
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

impl Display for PieceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_array())
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

    #[test]
    fn test_push_front() {
        let mut piece_list = PieceList::new();

        piece_list.push_front(3);
        piece_list.push_front(7);
        piece_list.push_front(100);
        piece_list.push_front(87);

        assert_eq!(
            piece_list.to_array(),
            &vec![87, 100, 7, 3, 255, 255, 255, 255][..]
        );
    }

    #[test]
    fn test_remove() {
        let mut piece_list = PieceList::from(0xffff060504030201);

        assert_eq!(piece_list.remove(4), Some(5));
        assert_eq!(
            piece_list.to_array(),
            &vec![1, 2, 3, 4, 6, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(4), Some(6));
        assert_eq!(
            piece_list.to_array(),
            &vec![1, 2, 3, 4, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(4), None);
        assert_eq!(
            piece_list.to_array(),
            &vec![1, 2, 3, 4, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(7), None);
        assert_eq!(
            piece_list.to_array(),
            &vec![1, 2, 3, 4, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(1), Some(2));
        assert_eq!(
            piece_list.to_array(),
            &vec![1, 3, 4, 255, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(0), Some(1));
        assert_eq!(
            piece_list.to_array(),
            &vec![3, 4, 255, 255, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(0), Some(3));
        assert_eq!(
            piece_list.to_array(),
            &vec![4, 255, 255, 255, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(0), Some(4));
        assert_eq!(
            piece_list.to_array(),
            &vec![255, 255, 255, 255, 255, 255, 255, 255][..]
        );

        assert_eq!(piece_list.remove(0), None);
        assert_eq!(
            piece_list.to_array(),
            &vec![255, 255, 255, 255, 255, 255, 255, 255][..]
        );
    }
}

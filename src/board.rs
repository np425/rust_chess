use crate::square::Square;

pub struct Board {
    pub squares: [Square; 64],
}

impl Board {
    fn resolve_index(coord: Coord) -> usize {
        coord.y * 8 + coord.x
    }

    pub fn get(&self, coord: Coord) -> Option<&Square> {
        self.squares.get(Self::resolve_index(coord))
    }

    pub fn iter(&self) -> BoardPathIter {
        BoardPathIter {
            board: self,
            current: Coord {x: 0, y: 0},
            increment: Distance {x: 1, y: 1},
        }
    }

    pub fn ranks_len(&self) -> usize {
        8
    }

    pub fn files_len(&self) -> usize {
        8
    }
}

pub struct BoardPathIter<'a> {
    board: &'a Board,
    pub current: Coord,
    pub increment: Distance,
}

impl<'a> Iterator for BoardPathIter<'a> {
    type Item = (Coord, &'a Square);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.current, self.board.get(self.current)?);

        self.current.x = (self.current.x as isize + self.increment.x) as usize;
        self.current.y = (self.current.y as isize + self.increment.x) as usize;

        Some(result)
    }

}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub struct Path {
    pub from: Coord,
    pub to: Coord,
}

impl Path {
    pub fn distance(&self) -> Distance {
        Distance {
            x: self.to.x as isize - self.from.x as isize,
            y: self.to.y as isize - self.from.y as isize
        }
    }
}

#[derive(Clone, Copy)]
pub struct Distance {
    pub x: isize,
    pub y: isize
}


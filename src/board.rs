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

    pub fn iter(&self, from: Coord, incr: Distance) -> BoardPathIter {
        BoardPathIter {
            board: self,
            iter: from,
            incr,
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
    iter: Coord,
    incr: Distance,
}

impl<'a> Iterator for BoardPathIter<'a> {
    type Item = (Coord, &'a Square);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.iter, self.board.get(self.iter)?);

        self.iter.x = (self.iter.x as isize + self.incr.x) as usize;
        self.iter.y = (self.iter.y as isize + self.incr.x) as usize;

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
    pub fn diff(&self) -> Distance {
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


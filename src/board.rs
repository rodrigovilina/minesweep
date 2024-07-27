use {
  crate::{
    cell::{Cell, Status},
    coordinate::Coordinate,
  },
  rand::Rng,
  std::collections::HashSet,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board<const W: usize, const H: usize> {
  pub cells: [[Cell; W]; H],
}

impl<const W: usize, const H: usize> Board<W, H> {
  const CELLS: usize = W * H;

  fn empty() -> Self {
    let cell = Cell {
      status: Status::ClosedClear,
      neighboring_bombs: 0,
    };
    let cells = [[cell; W]; H];
    Self { cells }
  }

  pub fn new(bombs: usize) -> Result<Self, &'static str> {
    if bombs > Self::CELLS {
      return Err("Cannot generate more unique numbers than the range allows");
    }

    let mut rng = rand::thread_rng();
    let mut bomb_indices: HashSet<usize> = HashSet::new();

    while bomb_indices.len() < bombs {
      bomb_indices.insert(rng.gen_range(0..Self::CELLS));
    }

    let mut board = Self::empty();

    bomb_indices.into_iter().for_each(|index| {
      let x = (index % W) as isize;
      let y = (index / W) as isize;
      if let Some(coord) = Coordinate::<W, H>::new(x, y) {
        board.add_bomb(coord);
      }
    });

    Ok(board)
  }

  fn add_bomb(&mut self, coordinate: Coordinate<W, H>) {
    self.cells[coordinate.y][coordinate.x].plant_bomb();
    coordinate.adjacents().into_iter().for_each(|cord| {
      self.cells[cord.y][cord.x].increase_neighboring_bombs();
    })
  }

  pub fn reveal(&mut self, coordinate: Coordinate<W, H>) -> bool {
    let cell = &mut self.cells[coordinate.y][coordinate.x];
    match cell.status {
      Status::ClosedBomb => false,
      Status::OpenClear => true,
      Status::ClosedClear => {
        cell.status = Status::OpenClear;

        if cell.neighboring_bombs == 0 {
          coordinate.adjacents().into_iter().for_each(|coord| {
            self.reveal(coord);
          })
        }
        true
      },
    }
  }
}

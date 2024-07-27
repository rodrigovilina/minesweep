#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinate<const W: usize, const H: usize> {
  pub x: usize,
  pub y: usize,
}

impl<const W: usize, const H: usize> Coordinate<W, H> {
  const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
  ];

  pub fn new(x: isize, y: isize) -> Option<Self> {
    match (x >= 0 && x < W as isize, y >= 0 && y < H as isize) {
      (true, true) => Some(Self {
        x: x as usize,
        y: y as usize,
      }),
      _ => None,
    }
  }

  pub fn adjacents(&self) -> Vec<Self> {
    Self::DELTAS
      .into_iter()
      .filter_map(|(dx, dy)| {
        let new_x = self.x as isize + dx;
        let new_y = self.y as isize + dy;
        Self::new(new_x, new_y)
      })
      .collect()
  }
}

mod board;
mod cell;
mod coordinate;

use {
  board::Board,
  cell::{Cell, Status},
  coordinate::Coordinate,
  std::{
    fmt::Display,
    io::{self, BufRead, Write},
  },
};

impl Display for Cell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let output = match self.status {
      Status::ClosedBomb => " ",
      Status::ClosedClear => " ",
      Status::OpenClear => &format!("{}", self.neighboring_bombs),
    };
    write!(f, "{}", output)
  }
}

impl<const W: usize, const H: usize> Display for Board<W, H> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let row_length = W * 2;
    let separator: String = (0..=row_length)
      .map(|i| if i % 2 == 0 { '+' } else { '-' })
      .collect();
    let _ = writeln!(f, "{}", separator);
    self.cells.into_iter().for_each(|row| {
      let row_str = row
        .into_iter()
        .map(|cell| cell.to_string())
        .collect::<Vec<String>>()
        .join("|");
      let _ = writeln!(f, "|{row_str}|");
      let _ = writeln!(f, "{}", separator);
    });
    Ok(())
  }
}

fn main() {
  const WIDTH: usize = 5;
  const HEIGHT: usize = 5;

  let mut keep_going = true;
  let mut board = Board::<WIDTH, HEIGHT>::new(3).unwrap();

  while keep_going {
    println!("{}", board);
    let mut buffer = String::with_capacity(2048);
    let mut stdin = io::stdin().lock();

    print!("|> ");
    io::stdout().flush().unwrap();
    let _read_result = stdin.read_line(&mut buffer);
    let parts: Vec<&str> = buffer.split_whitespace().collect();
    let coord =
      Coordinate::<WIDTH, HEIGHT>::new(parts[0].parse().unwrap(), parts[1].parse().unwrap())
        .unwrap();

    keep_going = board.reveal(coord);
  }
}

use rand::{thread_rng, Rng};
use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result},
};

pub struct Field {
    x: usize,
    y: usize,
    mine_info: MineInfo,
}

pub enum MineInfo {
    Mine,
    NeighboringMines(u8),
}

pub enum FieldStatus {
    Open,
    Closed,
    Flag,
}

#[derive(Debug)]
pub struct Minesweeper {
    height: usize,
    width: usize,
    
    // open_fields: HashSet<Field>,
    // mines: HashSet<Field>,
    // positions: HashSet,
    // flagged_fields: HashSet<Field>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if self.flagged_fields.contains(&pos) {
                    f.write_str("🚩 ")?;
                } else if !self.open_fields.contains(&pos) {
                    f.write_str("🟪 ")?;
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let neighbor_count = self.neighboring_mines(pos);
                    if neighbor_count == 0 {
                        f.write_str("   ")?;
                    } else {
                        f.write_fmt(format_args!(" {} ", neighbor_count))?;
                    }
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                // Generate random positions for the mines
                // and make sure they are not on the edge
                // of the board
                let mut mines = HashSet::new();
                let mut rng = thread_rng();

                while mines.len() < mine_count {
                    let x = rng.gen_range(0..width);
                    let y = rng.gen_range(0..height);

                    mines.insert((x, y));
                }
                mines
            },
            flagged_fields: HashSet::new(),
        }
    }

    pub fn iter_neighbors(&self, (x, y): Field) -> impl Iterator<Item = Field> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Field) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Field) -> Option<MineInfo> {
        if self.flagged_fields.contains(&pos) {
            return None;
        }

        self.open_fields.insert(pos);

        let is_mine = self.mines.contains(&pos);

        if is_mine {
            Some(MineInfo::Mine)
        } else {
            Some(MineInfo::NeighboringMines(self.neighboring_mines(pos)))
        }
    }

    pub fn toggle_flag(&mut self, pos: Field) {
        if self.open_fields.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);

        ms.open((5, 5));
        ms.toggle_flag((6, 6));

        println!("{}", ms);
    }
}

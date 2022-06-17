use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter, Result},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Field {
    position: Position, // TODO ta bort position härifrån?
    mine_info: MineInfo,
    status: FieldStatus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MineInfo {
    Mine,
    NeighboringMines(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldStatus {
    Open,
    Closed,
    Flag,
}

#[derive(Debug)]
pub struct Minesweeper {
    height: usize,
    width: usize,
    fields: HashMap<Position, Field>,
    // open_fields: HashSet<Field>,
    // mines: HashSet<Field>,
    // positions: HashSet,
    // flagged_fields: HashSet<Field>
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = &Position { x, y };
                let field = self.fields.get(pos).ok_or(Error)?;

                match field.status {
                    FieldStatus::Flag => f.write_str("🚩 ")?,
                    FieldStatus::Closed => f.write_str("🟪 ")?,
                    FieldStatus::Open => match field.mine_info {
                        MineInfo::Mine => f.write_str("💣 ")?,
                        MineInfo::NeighboringMines(0) => f.write_str("   ")?,
                        MineInfo::NeighboringMines(mine_count) => {
                            f.write_fmt(format_args!(" {} ", mine_count))?
                        }
                    },
                };

                // if field.status == FieldStatus::Flag {
                //     f.write_str("🚩 ")?;
                // } else if (field.status == FieldStatus::Closed) {
                //     f.write_str("🟪 ")?;
                // } else if field.mine_info == MineInfo::Mine {
                //     f.write_str("💣 ")?;
                // } else {

                //     // let x = field.mine_info
                // }

                // if self.flagged_fields.contains(&pos) {
                //     f.write_str("🚩 ")?;
                // } else if !self.open_fields.contains(&pos) {
                //     f.write_str("🟪 ")?;
                // } else if self.mines.contains(&pos) {
                //     f.write_str("💣 ")?;
                // } else {
                //     let neighbor_count = self.neighboring_mines(pos);
                //     if neighbor_count == 0 {
                //         f.write_str("   ")?;
                //     } else {
                //         f.write_fmt(format_args!(" {} ", neighbor_count))?;
                //     }
                // }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            height,
            width,
            fields: {
                let mut fields = HashMap::new();

                // Set up empty fields
                for y in 0..height {
                    for x in 0..width {
                        let position = Position { x, y };
                        let field = Field {
                            position,
                            mine_info: MineInfo::NeighboringMines(0),
                            status: FieldStatus::Closed,
                        };
                        fields.insert(position, field);
                    }
                }

                // Add mines
                let mut rng = thread_rng();
                let mut mines_added = 0;
                while mines_added < mine_count {
                    let x = rng.gen_range(0..width);
                    let y = rng.gen_range(0..height);
                    let position = Position { x, y };
                    let field = fields.get_mut(&position).unwrap();

                    match field.mine_info {
                        MineInfo::Mine => {
                            // Already a mine, try again
                            continue;
                        }
                        _ => {
                            field.mine_info = MineInfo::Mine;
                            mines_added += 1;
                        }
                    }
                }

                fields
            },
        }
        // Minesweeper {
        //     width,
        //     height,
        //     open_fields: HashSet::new(),
        //     mines: {
        //         // Generate random positions for the mines
        //         // and make sure they are not on the edge
        //         // of the board
        //         let mut mines = HashSet::new();
        //         let mut rng = thread_rng();

        //         while mines.len() < mine_count {
        //             let x = rng.gen_range(0..width);
        //             let y = rng.gen_range(0..height);

        //             mines.insert((x, y));
        //         }
        //         mines
        //     },
        //     flagged_fields: HashSet::new(),
        // }
    }

    // pub fn iter_neighbors(&self, (x, y): Field) -> impl Iterator<Item = Field> {
    //     let width = self.width;
    //     let height = self.height;

    //     (x.max(1) - 1..=(x + 1).min(width - 1))
    //         .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
    //         .filter(move |&pos| pos != (x, y))
    // }

    // pub fn neighboring_mines(&self, pos: Field) -> u8 {
    //     self.iter_neighbors(pos)
    //         .filter(|pos| self.mines.contains(pos))
    //         .count() as u8
    // }

    // pub fn open(&mut self, pos: Field) -> Option<MineInfo> {
    //     if self.flagged_fields.contains(&pos) {
    //         return None;
    //     }

    //     self.open_fields.insert(pos);

    //     let is_mine = self.mines.contains(&pos);

    //     if is_mine {
    //         Some(MineInfo::Mine)
    //     } else {
    //         Some(MineInfo::NeighboringMines(self.neighboring_mines(pos)))
    //     }
    // }

    // pub fn toggle_flag(&mut self, pos: Field) {
    //     if self.open_fields.contains(&pos) {
    //         return;
    //     }

    //     if self.flagged_fields.contains(&pos) {
    //         self.flagged_fields.remove(&pos);
    //     } else {
    //         self.flagged_fields.insert(pos);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);

        // ms.open((5, 5));
        // ms.toggle_flag((6, 6));
        println!("{}", ms);
    }
}

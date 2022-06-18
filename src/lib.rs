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

#[derive(Debug, PartialEq, Eq, Clone)]
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
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        let mut ms = Minesweeper {
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
        };

        // Set neighboring mines
        for y in 0..height {
            for x in 0..width {
                let position = Position { x, y };
                let neighboring_mines = ms.neighboring_mines(position);
                let mut field = ms.fields.get_mut(&position).unwrap();

                if let MineInfo::NeighboringMines(0) = field.mine_info {
                    field.mine_info = MineInfo::NeighboringMines(neighboring_mines);
                }
            }
        }

        return ms;

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

    pub fn iter_neighbors(&self, pos: Position) -> impl Iterator<Item = Position> {
        let x = pos.x;
        let y = pos.y;
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
            .map(|(x, y)| Position { x, y })
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .flat_map(|pos| self.fields.get(&pos))
            .filter(|&field| field.mine_info == MineInfo::Mine)
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<MineInfo> {
        let mut field = self.fields.get_mut(&pos)?;

        match field.status {
            FieldStatus::Open | FieldStatus::Flag => None,
            FieldStatus::Closed => {
                field.status = FieldStatus::Open;
                Some(field.mine_info.clone())
            }
        }
    }

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
    use crate::{Minesweeper, Position};

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 10);

        ms.open(Position { x: 6, y: 6 });
        // ms.toggle_flag((6, 6));
        println!("{}", ms);
    }
}

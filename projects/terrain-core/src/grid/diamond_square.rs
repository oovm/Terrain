use crate::{DiamondSquare, GridTerrain};
use diamond_square::MidpointDisplacement;

impl From<DiamondSquare> for GridTerrain {
    fn from(value: DiamondSquare) -> Self {
        GridTerrain::diamond_square(&value)
    }
}

impl GridTerrain {
    pub fn midpoint_(config: &MidpointDisplacement) -> Self {
        todo!()
        // let grid = config.generate();
        // let min = grid.iter().fold(f32::MAX, |acc, &x| acc.min(x));
        // let max = grid.iter().fold(f32::MIN, |acc, &x| acc.max(x));
        // GridTerrain {
        //     grid,
        //     range: min..max,
        // }
    }

    pub fn diamond_square(config: &DiamondSquare) -> Self {
        let grid = config.generate();
        let min = grid.iter().fold(f32::MAX, |acc, &x| acc.min(x));
        let max = grid.iter().fold(f32::MIN, |acc, &x| acc.max(x));
        GridTerrain { grid, range: min..max }
    }
}

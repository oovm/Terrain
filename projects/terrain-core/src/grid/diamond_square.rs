use crate::GridTerrain;
use crate::DiamondSquare;

impl From<DiamondSquare> for GridTerrain {
    fn from(value: DiamondSquare) -> Self {
        GridTerrain::diamond_square(&value)
    }
}

impl GridTerrain {
    pub fn diamond_square(config: &DiamondSquare) -> Self {
        let grid = config.generate();
        let min = grid.iter().fold(f32::MAX, |acc, &x| acc.min(x));
        let max = grid.iter().fold(f32::MIN, |acc, &x| acc.max(x));
        GridTerrain {
            grid,
            range: min..max,
        }
    }
}
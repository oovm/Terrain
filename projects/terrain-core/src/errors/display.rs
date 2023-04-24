use super::*;


impl Display for TerrainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl Display for TerrainErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for TerrainError {}

impl Error for TerrainErrorKind {}
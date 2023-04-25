use super::*;

impl Display for TerrainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for TerrainErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TerrainErrorKind::UnknownError => f.write_str("Unknown error"),
            TerrainErrorKind::InvalidRange { message } => {
                writeln!(f, "Invalid range: {}", message)
            }
        }
    }
}

impl Error for TerrainError {}

impl Error for TerrainErrorKind {}

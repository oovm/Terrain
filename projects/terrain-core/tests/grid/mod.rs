use terrain_core::DiamondSquare;

#[test]
fn test() {
    let mut ds = DiamondSquare::default();
    ds.iteration = 1;
    let grid = ds.generate();
    println!("{:?}", grid);
}
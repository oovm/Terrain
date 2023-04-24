use terrain_core::DiamondSquare;

#[test]
fn test() {
    let mut ds = DiamondSquare::default();
    ds.iteration = 0;
    let grid = ds.generate();
    println!("{:?}", grid);
}
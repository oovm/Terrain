use terrain_core::DiamondSquare;

#[test]
fn test() {
    let ds = DiamondSquare::default().with_iteration(2);
    let grid = ds.generate();
    println!("{:?}", grid);
}
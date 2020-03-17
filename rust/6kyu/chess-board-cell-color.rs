// https://www.codewars.com/kata/5894134c8afa3618c9000146/train/rust

fn parse_location(cell: &str) -> (u32, u32) {
    let mut it = cell.chars();
    let x = it.next().unwrap() as u32 - ('A' as u32);
    let y = it.next().unwrap().to_digit(10).unwrap() as u32;
    (x, y)
}

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    let (x1, y1) = parse_location(cell1);
    let (x2, y2) = parse_location(cell2);
    (x1 + y1) % 2 == (x2 + y2) % 2
}

/// Want to make a 2d board and explode it. 9/10/22 18:00
/// that was fun
fn main() {
    const HW: usize = 9;
    let mut board = ["x"; HW.pow(2)];

    make_space_in_the_origin(&mut board, 2);

    let origin = get_pos(&board, 0, 0);
    board[origin] = "Q";

    render(&board, HW);
}

fn get_pos(board: &[&str], x: i32, y: i32) -> usize {
    let hw = (board.len() as f32).sqrt() as i32;
    assert!(hw % 2 == 1);

    let origin = (hw.pow(2) - 1) / 2;
    (origin as i32 + x - (hw * y)) as usize
}

fn make_space_in_the_origin(board: &mut [&str], size: i32) {
    for x in -size..=size {
        for y in -size..=size {
            let pos = get_pos(board, x, y);
            board[pos] = "o";
        }
    }
}

fn render(board: &[&str], hw: usize) {
    let mut line = String::from("");
    for (i, c) in board.iter().enumerate() {
        if i % hw == 0 {
            println!("{}", line);
            line.clear();
        }
        line.push_str(format!("{} ", c).as_str());
    }
    println!("{}", line);
}

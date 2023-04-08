use flow_solver::problem::flow::{self, Board};

fn main() {
    let g: Board<u8> = flow::board!([
        [. . .]
        [. . 0]
        [0 . .]
        [. . .]
    ]);

    println!("{:?}", g);
}

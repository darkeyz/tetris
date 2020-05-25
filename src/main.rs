#[derive(Debug)]
struct Tetromino<'a> {
    shape: &'a str,
}

fn main() {
    let i = Tetromino { 
        shape: "x\n\
                x\n\
                x\n\
                x"
    };
    let o = Tetromino { 
        shape: "oo\n\
                oo"
    };
    let t = Tetromino { 
        shape: "ttt\n t"
    };
    println!("{}", i.shape);
    println!("{}", o.shape);
    println!("{}", t.shape);
}

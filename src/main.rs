use std::fs::*;
use std::io::{Write, BufReader, BufRead, Error};

#[repr(C)]
struct Points{
    x : f32,
    y : f32,
    z : f32,
    r : u8,
    g : u8,
    b : u8
}

fn main() -> Result<(), Error> {
    let bytes = read("data.txt").expect("File not found");

    let points: Vec<Points> = convert_to_points(bytes);

    let mut out = File::create("maybe.obj")?;
    write!(out, "v -0.3 -0.3 -0.3\n    #1a
                v -0.3 0.3 -0.3\n     #2b
                v 0.3 0.3 -0.3\n     #3c
                v 0.3 -0.3 -0.3\n   #4d
                v -0.3 -0.3 0.3\n   #5e
                v -0.3 0.3 0.3\n    #6f
                v 0.3 0.3 0.3\n     #7g
                v 0.3 -0.3 0.3\n    #8h
                f 1 2 3 4\n
                f 1 5 6 2\n
                f 5 6 7 8\n
                f 7 3 4 8\n
                f 1 4 8 5\n
                f 2 6 7 3\n
                "
            )?;

    let input = File::open("cube.txt")?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())

    // now have an array of all points with position and color
    

}

fn convert_to_points(data: Vec<u8>) -> Vec<Points> {
    if data.len() % std::mem::size_of::<Points>() != 0 {
        panic!("MAYDAY!!!");
    }

    let data_ptr = data.as_ptr();

    let num_points = data.len() / std::mem::size_of::<Points>();

    let mut points_vec = Vec::with_capacity(num_points);

    unsafe{
    for i in 0..num_points {
        let point_ptr = data_ptr.offset((i * std::mem::size_of::<Points>()) as isize);
        let point: Points = unsafe { std::ptr::read(point_ptr as *const Points) };
        points_vec.push(point);
    }}

    points_vec
}

use std::error::Error;
use std::fs::*;
use std::io::{Write, BufReader};

#[repr(C)]
struct Points{
    x : f32,
    y : f32,
    z : f32,
    r : u8,
    g : u8,
    b : u8
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "pointcloudvoxel.obj";

    let bytes = read("data.txt").expect("File not found");

    let points: Vec<Points> = convert_to_points(bytes);

    let out = File::create(filepath)?;

    for(i, item) in points.iter().enumerate(){
        create_object(&out, &points[i], i);
    }

    Ok(())

    // now have an array of all points with position and color
}

fn create_object(mut out: &File, data: &Points, mut offset: usize) -> Result<(), Box<dyn Error>>
{
    let size_of_box = 0.3;
    offset *= 8;
    write!(out, "v {} {} {}\n
                v {} {} {}\n
                v {} {} {}\n
                v {} {} {}\n
                v {} {} {}\n
                v {} {} {}\n
                v {} {} {}\n
                v {} {} {}\n
                f {} {} {} {}\n
                f {} {} {} {}\n
                f {} {} {} {}\n
                f {} {} {} {}\n
                f {} {} {} {}\n
                f {} {} {} {}\n",
            data.x-size_of_box, data.y-size_of_box, data.z-size_of_box,
            data.x-size_of_box, data.y+size_of_box, data.z-size_of_box,
            data.x+size_of_box, data.y+size_of_box, data.z-size_of_box,
            data.x+size_of_box, data.y-size_of_box, data.z-size_of_box,
            data.x-size_of_box, data.y-size_of_box, data.z+size_of_box,
            data.x-size_of_box, data.y+size_of_box, data.z+size_of_box,
            data.x+size_of_box, data.y+size_of_box, data.z+size_of_box,
            data.x+size_of_box, data.y-size_of_box, data.z+size_of_box,
            1+offset, 2+offset, 3+offset, 4+offset,
            1+offset, 5+offset, 6+offset, 2+offset,
            5+offset, 6+offset, 7+offset, 8+offset,
            7+offset, 3+offset, 4+offset, 8+offset,
            1+offset, 4+offset, 8+offset, 5+offset,
            2+offset, 6+offset, 7+offset, 3+offset
        )?;

    Result::Ok(())
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

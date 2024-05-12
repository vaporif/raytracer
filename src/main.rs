use indicatif::ProgressBar;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let pb = ProgressBar::new(image_height);
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        pb.inc(1);
        for i in 0..image_width {
            let r = (i as f32) / ((image_width - 1) as f32);
            let g = (j as f32) / ((image_width - 1) as f32);
            let b: f32 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }
    pb.finish_with_message("done");
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

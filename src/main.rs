use std::fs::File;
use std::io::prelude::*;

use rand::Rng;

const WIDTH: i64 = 800;
const HEIGHT: i64 = 600;

static FILE_PATH: &str = "output.ppm";

type Color32 = i64;
type Image = Vec<Vec<Color32>>;
type Seeds = Vec<Point>;

const BRIGHT_RED :Color32    = 0xFF3449FB;
const BRIGHT_GREEN :Color32  = 0xFF26BBB8;
const BRIGHT_YELLOW:Color32  = 0xFF2FBDFA;
const BRIGHT_BLUE :Color32   = 0xFF98A583;
const BRIGHT_PURPLE:Color32  = 0xFF9B86D3;
const BRIGHT_AQUA :Color32   = 0xFF7CC08E;
const BRIGHT_ORANGE :Color32 = 0xFF1980FE;

const COLOR_BACKGROUND: Color32 = 0xFF181818;

const SEEDS_COUNT: u8 = 10;
const SEED_MARKER_RADIUS: i64 = 5;
const SEED_MARKER_COLOR: i64 = 0xFF181818;


#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}

fn generate_random_seeds(seeds: &mut Seeds) {
    let mut rng = rand::thread_rng();

    for seed in seeds.iter_mut() {
        seed.x = rng.gen_range(0..10000000) % WIDTH;
        seed.y = rng.gen_range(0..10000000) % HEIGHT;
    }
}

fn fill_image(image: &mut Image, color: Color32) {
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            image[y][x] = color;
        }
    }
}

fn save_image_as_ppm(image: Image) -> std::io::Result<()> {
   let mut file = File::create(FILE_PATH)?;
   file.write_all(format!("P6\n{} {} 255\n", WIDTH, HEIGHT).as_bytes())?;

   let mut all_bytes: Vec<u8> = Vec::new();

   for y in 0..HEIGHT as usize {
       for x in 0..WIDTH as usize {
           let pixel: i64 = image[y][x];

           // Extract red component
           all_bytes.push(((pixel&0x0000FF) >> 8*0) as u8);

           // Extract green component
           all_bytes.push(((pixel&0x00FF00) >> 8*1) as u8);

           // Extract blue component
           all_bytes.push(((pixel&0xFF0000) >> 8*2) as u8);
       }
   }

   file.write_all(&all_bytes).unwrap();

   Ok(())
}

fn fill_circle(image: &mut Image, cx: i64, cy: i64, radius: i64, color: Color32) {
    let x0: i64 = cx - radius;
    let y0: i64 = cy - radius;
    let x1: i64 = cx + radius;
    let y1: i64 = cy + radius;

    for x in x0..=x1 {
        if 0 <= x && x < WIDTH {
            for y in y0..=y1 {
                if 0 <= y && y < HEIGHT {
                    let dx: i64 = cx - x;
                    let dy: i64 = cy - y;
                    
                    if dx * dx + dy * dy <= radius * radius {
                        image[y as usize][x as usize] = color;
                    }
                }
            }
        }
    }
}

fn render_seed_markers(image: &mut Image, seeds: Seeds) {
    for seed in &seeds {
        fill_circle(image, seed.x, seed.y, SEED_MARKER_RADIUS, SEED_MARKER_COLOR);
    }
}

fn distance_squared(x1: i64, y1: i64, x2: i64, y2: i64) -> i64{
    let dx = x1 - x2;
    let dy = y1 - y2;

    return dx*dx + dy*dy;
}

fn render_voronoi(image: &mut Image, seeds: &Seeds) {
    let color_palette = vec![
        BRIGHT_RED,
        BRIGHT_GREEN,
        BRIGHT_YELLOW,
        BRIGHT_BLUE,
        BRIGHT_PURPLE,
        BRIGHT_AQUA,
        BRIGHT_ORANGE,
    ];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut j = 0;

            for (idx, seed) in seeds.iter().enumerate().skip(1) {
                if distance_squared(seed.x, seed.y, x, y) < distance_squared(seeds[j].x, seeds[j].y, x, y) {
                    j = idx;
                }
            }

            image[y as usize][x as usize] = color_palette[j%color_palette.len()];
        }
    }
}


fn main() {
    let mut image: Image = vec![vec![0_i64; WIDTH as usize]; HEIGHT as usize];
    let mut seeds: Seeds = vec![Point::new(); SEEDS_COUNT as usize];

    // Recipe
    fill_image(&mut image, COLOR_BACKGROUND);
    generate_random_seeds(&mut seeds);
    render_voronoi(&mut image, &seeds);
    render_seed_markers(&mut image, seeds);
    save_image_as_ppm(image).unwrap();
}

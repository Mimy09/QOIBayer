use std::{fs::File, io::Write};

use image::io::Reader as ImageReader;

use qoi_types::*;
mod qoi_types;

use qoi_encode::*;
mod qoi_encode;

// Release Timing
// Elapsed: 535.14µs
// Elapsed: 507.71µs
// Elapsed: 718.38µs
// Elapsed: 523.31µs
// Elapsed: 525.08µs

fn main() {
    let mut encoder = QOIEncoder::default();
    use std::time::Instant;
    let now = Instant::now();

    encoder.open("data/lena_rggb.png");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    encoder.save("data/lena_rggb.qoi");
}

/*

fn main() {
    let img = ImageReader::open("data/lena_rggb.png")
        .unwrap_or_else(|e| panic!("Can't open file {}", e))
        .decode()
        .unwrap_or_else(|e| panic!("Can't decade file {}", e));
    
    let img_raw = img.as_bytes().to_owned();
    
    let mut pixel_seen: [[u8; 4]; 64] = [[0; 4]; 64];
    let mut pixel_data: Vec<QOIType> = Vec::new();

    let mut pixel_run: usize = 0;
    let mut pixel_index: usize = 0;
    while pixel_index < (img_raw.len()) {

        let (r, g, b, a) = (
            img_raw.get(pixel_index + 0).unwrap_or_else(|| panic!("Out of range")).to_owned(),
            img_raw.get(pixel_index + 1).unwrap_or_else(|| panic!("Out of range")).to_owned(),
            img_raw.get(pixel_index + 2).unwrap_or_else(|| panic!("Out of range")).to_owned(),
            img_raw.get(pixel_index + 3).unwrap_or_else(|| panic!("Out of range")).to_owned(),
        );
        let index_position: usize = (((r as u32) * 3 + (g as u32) * 5 + (b as u32) * 7 + (a as u32) * 11) % 64) as usize;

        if pixel_index == 0 {
            pixel_seen[index_position] = [r, g, b, a];
            pixel_data.push(QOIType::RGBA(QOITypeRGBA::new()
                .with_r(r as u8)
                .with_g(g as u8)
                .with_b(b as u8)
                .with_a(a as u8)));

            pixel_index += 4;
            continue;
        }

        let (pr, pg, pb, pa) = (
            img_raw.get((pixel_index - 4) + 0).unwrap_or_else(|| panic!("Out of range")).to_owned(),
            img_raw.get((pixel_index - 4) + 1).unwrap_or_else(|| panic!("Out of range")).to_owned(),
            img_raw.get((pixel_index - 4) + 2).unwrap_or_else(|| panic!("Out of range")).to_owned(),
            img_raw.get((pixel_index - 4) + 3).unwrap_or_else(|| panic!("Out of range")).to_owned(),
        );

        if r==pr && g==pg && b==pb && a==pa {
            pixel_run += 1;

            if pixel_run >= 62 {
                pixel_data.push(QOIType::Run(QOITypeRun::new().with_run(pixel_run as u8)));
                pixel_run = 0;

                pixel_index += 4;
            }
        } else if pixel_run > 0 {
            pixel_data.push(QOIType::Run(QOITypeRun::new().with_run(pixel_run as u8)));
            pixel_run = 0;

            pixel_index += 4;
            continue;
        }
        
        
        if [r, g, b, a] != [0, 0, 0, 0] {
            if pixel_seen[index_position] == [0, 0, 0, 0] {
                pixel_seen[index_position] = [r, g, b, a];
            }
            else if pixel_seen[index_position] == [r, g, b, a] {
                pixel_data.push(QOIType::Index(QOITypeIndex::new().with_index(index_position as u8)));
                
                pixel_index += 4;
                continue;
            }
        }

        let diff_r = pr as i32 - r as i32;
        let diff_g = pg as i32 - g as i32;
        let diff_b = pb as i32 - b as i32;
        let diff_a = pa as i32 - a as i32;

        if diff_r >= -4 && diff_r <= 3 && diff_g >= -4 && diff_g <= 3 &&
           diff_b >= -4 && diff_b <= 3 && diff_a >= -4 && diff_a <= 3 {

            pixel_data.push(QOIType::Diff(QOITypeDiff::new()
                .with_da((diff_r + 4) as u8)
                .with_dg((diff_g + 4) as u8)
                .with_db((diff_b + 4) as u8)
                .with_da((diff_a + 4) as u8)));
            
            pixel_index += 4;
            continue;
        }
        
        pixel_data.push(QOIType::RGBA(QOITypeRGBA::new()
            .with_r(r as u8)
            .with_g(g as u8)
            .with_b(b as u8)
            .with_a(a as u8)));

        pixel_index += 4;
    }

    let mut sum_rgb: u32 = 0;
    let mut sum_rgba: u32 = 0;
    let mut sum_index: u32 = 0;
    let mut sum_diff: u32 = 0;
    let mut sum_luma: u32 = 0;
    let mut sum_run: u32 = 0;
    for pixel in pixel_data.iter() {
        match pixel {
            QOIType::RGB(_) => { sum_rgb += 1; },
            QOIType::RGBA(_) => { sum_rgba += 1; },
            QOIType::Index(_) => { sum_index += 1; },
            QOIType::Diff(_) => { sum_diff += 1; },
            QOIType::Luma(_) => { sum_luma += 1; },
            QOIType::Run(_) => { sum_run += 1; },
        }
    }

    println!("RGB: {}", sum_rgb);
    println!("RGBA: {}", sum_rgba);
    println!("Index: {}", sum_index);
    println!("Diff: {}", sum_diff);
    println!("Luma: {}", sum_luma);
    println!("Run: {}", sum_run);


    let mut file = File::create("data/lena_rggb.qoi")
        .unwrap_or_else(|e| panic!("Failed to create file: {}", e));


    for pixel in pixel_data.iter() {
        match pixel {
            QOIType::RGB(px) => {
                file.write(&[px.tag(), px.r(), px.g(), px.b()]).unwrap();
            },
            QOIType::RGBA(px) => {
                file.write(&[px.tag(), px.r(), px.g(), px.b(), px.a()]).unwrap();
            },
            QOIType::Index(px) => {
                let mut buffer: [u8; 1] = [0; 1];
                buffer[0] |= px.tag() << 7;
                buffer[0] |= px.index() << 5;

                file.write(&buffer).unwrap();
            },
            QOIType::Diff(px) => {
                let mut buffer: [u8; 2] = [0; 2];
                buffer[0] |= px.tag() << 7;
                buffer[0] |= px.dr() << 5;
                buffer[0] |= px.dg() << 2;
                buffer[1] |= px.db() << 7;
                buffer[1] |= px.da() << 4;

                file.write(&buffer).unwrap();
            },
            QOIType::Luma(px) => {
                panic!("Not implemented");
            },
            QOIType::Run(px) => {
                let mut buffer: [u8; 1] = [0; 1];
                buffer[0] |= px.tag() << 7;
                buffer[0] |= px.run() << 5;

                file.write(&buffer).unwrap();
            },
        }
    }
}

 */
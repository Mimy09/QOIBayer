use std::{fs::File, io::Write};
use image::io::Reader as ImageReader;

pub struct QOIEncoder {
    _header: crate::QOIHeader,
    _data: Vec<crate::QOIType>,
    _seen: [[u8; 4]; 64],
    _run: usize,
    _idx: usize,
}

impl Default for QOIEncoder {
    fn default() -> Self {
        Self {
            _header: crate::QOIHeader::default(),
            _data: Vec::new(),
            _seen: [[0; 4]; 64],
            _run: 0,
            _idx: 0
        }
    }
}

fn get_pixel(idx: usize, data: &Vec<u8>) -> (u8, u8, u8, u8) {
    let (r, g, b, a) = (
        data.get(idx + 0).unwrap_or_else(|| panic!("Out of range")).to_owned(),
        data.get(idx + 1).unwrap_or_else(|| panic!("Out of range")).to_owned(),
        data.get(idx + 2).unwrap_or_else(|| panic!("Out of range")).to_owned(),
        data.get(idx + 3).unwrap_or_else(|| panic!("Out of range")).to_owned(),
    );
    (r, g, b, a)
}

impl QOIEncoder {

    fn qoi_buffer_init(&mut self, r: u8, g: u8, b: u8, a: u8) -> bool {
        if self._idx == 0 {
            let index_position: usize = (((r as u32) * 3 + (g as u32) * 5 + (b as u32) * 7 + (a as u32) * 11) % 64) as usize;
            self._seen[index_position] = [r, g, b, a];
            self._data.push(crate::QOIType::RGBA(crate::QOITypeRGBA::new()
                .with_r(r as u8)
                .with_g(g as u8)
                .with_b(b as u8)
                .with_a(a as u8)));

            self._idx += 4;
            return true;
        }
        return false;
    }

    fn qoi_buffer_run(&mut self, r: u8, g: u8, b: u8, a: u8, pr: u8, pg: u8, pb: u8, pa: u8) -> bool {
        if r==pr && g==pg && b==pb && a==pa {
            self._run += 1;

            if self._run >= 62 {
                self._data.push(crate::QOIType::Run(crate::QOITypeRun::new().with_run(self._run as u8)));
                self._run = 0;

                self._idx += 4;
                return true;
            }
            return true;
        } else if self._run > 0 {
            self._data.push(crate::QOIType::Run(crate::QOITypeRun::new().with_run(self._run as u8)));
            self._run = 0;

            self._idx += 4;
            return true;
        }

        return false;
    }
    
    fn qoi_buffer_diff(&mut self, r: u8, g: u8, b: u8, a: u8, pr: u8, pg: u8, pb: u8, pa: u8) -> bool {
        let diff_r = pr as i32 - r as i32;
        let diff_g = pg as i32 - g as i32;
        let diff_b = pb as i32 - b as i32;
        let diff_a = pa as i32 - a as i32;

        if diff_r >= -4 && diff_r <= 3 && diff_g >= -4 && diff_g <= 3 &&
            diff_b >= -4 && diff_b <= 3 && diff_a >= -4 && diff_a <= 3 {

            self._data.push(crate::QOIType::Diff(crate::QOITypeDiff::new()
                .with_da((diff_r + 4) as u8)
                .with_dg((diff_g + 4) as u8)
                .with_db((diff_b + 4) as u8)
                .with_da((diff_a + 4) as u8)));
            
            self._idx += 4;
            return true;
        }

        return false;
    }

    fn qoi_buffer_index(&mut self, r: u8, g: u8, b: u8, a: u8) -> bool {
        if [r, g, b, a] != [0, 0, 0, 0] {
            let index_position: usize = (((r as u32) * 3 + (g as u32) * 5 + (b as u32) * 7 + (a as u32) * 11) % 64) as usize;

            if self._seen[index_position] == [0, 0, 0, 0] {
                self._seen[index_position] = [r, g, b, a];
            }
            else if self._seen[index_position] == [r, g, b, a] {
                self._data.push(crate::QOIType::Index(crate::QOITypeIndex::new().with_index(index_position as u8)));
                
                self._idx += 4;
                return true;
            }
        }

        return false;
    }

    pub fn open(&mut self, path: &'static str) {
        let img = ImageReader::open(path)
            .unwrap_or_else(|e| panic!("Can't open file {}", e))
            .decode()
            .unwrap_or_else(|e| panic!("Can't decade file {}", e));
        
        use std::time::Instant;
        let now = Instant::now();

        self._header = crate::QOIHeader::new(img.width(), img.height());
        
        let img_raw = img.as_bytes().to_owned();

        while self._idx < (img_raw.len()) {

            let (r, g, b, a) = get_pixel(self._idx, &img_raw);
    
            if self.qoi_buffer_init(r, g, b, a) {
                continue;
            }
    
            let (pr, pg, pb, pa) = get_pixel(self._idx - 4, &img_raw);
    
            if self.qoi_buffer_run(r, g, b, a, pr, pg, pb, pa) {
                continue;
            }
            
            if self.qoi_buffer_diff(r, g, b, a, pr, pg, pb, pa) {
                continue;
            }

            if self.qoi_buffer_index(r, g, b, a) {
                continue;
            }
            
            self._data.push(crate::QOIType::RGBA(crate::QOITypeRGBA::new()
                .with_r(r as u8)
                .with_g(g as u8)
                .with_b(b as u8)
                .with_a(a as u8)));
    
            self._idx += 4;
        }


        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

    }

    pub fn save(&mut self, path: &'static str) {
        let mut file = File::create(path)
            .unwrap_or_else(|e| panic!("Failed to create file: {}", e));
        
        file.write(&self._header.magic).unwrap();
        file.write(&self._header.width.to_ne_bytes()).unwrap();
        file.write(&self._header.height.to_ne_bytes()).unwrap();
        file.write(&self._header.channels.to_ne_bytes()).unwrap();
        file.write(&self._header.colorspace.to_ne_bytes()).unwrap();

        for pixel in self._data.iter() {
            match pixel {
                crate::QOIType::RGB(px) => {
                    file.write(&[px.tag(), px.r(), px.g(), px.b()]).unwrap();
                },
                crate::QOIType::RGBA(px) => {
                    file.write(&[px.tag(), px.r(), px.g(), px.b(), px.a()]).unwrap();
                },
                crate::QOIType::Index(px) => {
                    let mut buffer: [u8; 1] = [0; 1];
                    buffer[0] |= px.tag() << 7;
                    buffer[0] |= px.index() << 5;

                    file.write(&buffer).unwrap();
                },
                crate::QOIType::Diff(px) => {
                    let mut buffer: [u8; 2] = [0; 2];
                    buffer[0] |= px.tag() << 7;
                    buffer[0] |= px.dr() << 5;
                    buffer[0] |= px.dg() << 2;
                    buffer[1] |= px.db() << 7;
                    buffer[1] |= px.da() << 4;

                    file.write(&buffer).unwrap();
                },
                crate::QOIType::Luma(_) => {
                    panic!("Not implemented");
                },
                crate::QOIType::Run(px) => {
                    let mut buffer: [u8; 1] = [0; 1];
                    buffer[0] |= px.tag() << 7;
                    buffer[0] |= px.run() << 5;

                    file.write(&buffer).unwrap();
                },
            }
        }
    }
}
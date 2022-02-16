mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn rand_u32(range: u32) -> u32 {
    return ( random() * (range as f64)) as u32;
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    input_buffer: Vec<u8>,
    output_buffer: Vec<u8>,
}

const GrayscaleWeights: [(f64,f64,f64);2] = [
    (0.299_f64, 0.587_f64, 0.114_f64),   //JPEG,BT.601
    (0.2126_f64, 0.7152_f64, 0.0722_f64)  // BT.709
];

#[wasm_bindgen]
impl Universe {

    pub fn new (width: u32, height: u32) -> Universe {
        let buffersize = width * height * 4;
        let input_buffer = (0..buffersize)
                .map(|_| {0})
                .collect();
        let output_buffer = (0..buffersize)
                .map(|_| {0})
                .collect();
        Universe {
            width,
            height,
            input_buffer,
            output_buffer,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }


    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn input_buffer(&self) -> *const u8 {
        self.input_buffer.as_ptr()
    }

    pub fn output_buffer(&self) -> *const u8 {
        self.output_buffer.as_ptr()
    }

    pub fn fillbox(&mut self,color: u32){
        let height = self.height;
        let width = self.width;
        let buf = &mut self.output_buffer;
        // Color model u32 LE (RGBA)  -> u8 BGRA
        let blue: u8 = ((color  >> 16) & 0xff)  as u8; // R = 1.0
        let green: u8  = ((color >> 8) & 0xff) as u8; // G = 1.0
        let red: u8 = ((color >> 0) & 0xff) as u8; // B = 1.0
        let alpha: u8 = 0xff;

        log(&format!("{} {} {}",blue,green,red));

        for y  in 0..height {
            let offset = y * width * 4;
            for x in 0..width {
                let pos :usize = (offset + x * 4) as usize;
                buf[pos] = blue;
                buf[pos + 1] = green;
                buf[pos + 2] = red;
                buf[pos + 3] = alpha;
            }
        }
    }
    
    pub fn fillrandomrect(&mut self){
        let height = self.height;
        let width = self.width;
        let buf = &mut self.output_buffer;

        let startx:u32 = rand_u32(width);
        let starty:u32 = rand_u32(height);
        let endx:u32 = rand_u32(width-startx); 
        let endy:u32 = rand_u32(height-starty);
        let red:u8 = rand_u32(255) as u8;
        let green:u8 = rand_u32(255) as u8;
        let blue:u8 = rand_u32(255) as u8;
        let alpha:u8 = rand_u32(255) as u8;

        for y in starty..endy {
            let offset = y * width * 4;
            for x  in startx..endx {
                let pos :usize= (offset + (x * 4)) as usize;

                buf[pos] = blue;
                buf[pos + 1] = green;
                buf[pos + 2] = red;
                buf[pos + 3] = alpha;
            }
        }
    }

    pub fn fillrect(&mut self,startx :u32, starty :u32, width: u32, height: u32,color: u32){
        let endx = startx + width;
        let endy = starty + height;
        let buf = &mut self.output_buffer;
        // Color model u32 LE (ARGB)  -> u8 BGRA
        let blue: u8 = ((color  >> 16) & 0xff)  as u8; // R = 1.0
        let green: u8  = ((color >> 8) & 0xff) as u8; // G = 1.0
        let red: u8 = ((color >> 0) & 0xff) as u8; // B = 1.0
        let alpha: u8 = 0xff;

        for y in starty..endy {
            let offset = y * width * 4;
            for x  in startx..endx {
                let pos :usize= (offset + (x * 4)) as usize;

                buf[pos] = blue;
                buf[pos + 1] = green;
                buf[pos + 2] = red;
                buf[pos + 3] = alpha;
            }
        }

    }

    pub fn to_grayscale0(&mut self) {
        self.to_grayscale(0);
    }

    pub fn to_grayscale(&mut self, t: usize) {
        let height = self.height;
        let width = self.width;
        let ibuf = &self.input_buffer;
        let buf = &mut self.output_buffer;
        let (wred, wgreen, wblue)  = GrayscaleWeights[t];
        for y in 0..height {
            let offset = y * width * 4;
            for x  in 0..width {
                let pos = (offset + (x * 4)) as usize;
                let red = ibuf[pos + 2] as f64;
                let green  = ibuf[pos + 1] as f64;
                let blue = ibuf[pos] as f64;

                let gray =  (wred * red + wgreen * green  + wblue * blue).round() as u8;
                buf[pos] = gray;
                buf[pos + 1] = gray;
                buf[pos + 2] = gray;
                buf[pos + 3] = 0xff; // alpha
            }
        }
    }
}

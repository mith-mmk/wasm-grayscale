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

const GrayscaleWeights: [(f64,f64,f64);6] = [
    (0.299_f64, 0.587_f64, 0.114_f64),   //JPEG,BT.601
    (0.2126_f64, 0.7152_f64, 0.0722_f64), // BT.709
    (0.3333333_f64,0.3333334_f64,0.3333333_f64), // Avarage
    (1.0_f64,0.0_f64,0.0_f64), // Red
    (0.0_f64,1.0_f64,0.0_f64), // Green
    (0.0_f64,0.0_f64,1.0_f64), // Blue
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

    pub fn clear(&mut self,color: u32){
        self.fillrect(0, 0, self.width, self.height, color);
    }
    
    pub fn fillrect(&mut self,startx :u32, starty :u32, width: u32, height: u32,color: u32){
        let endx = startx + width;
        let endy = starty + height;
        let buf = &mut self.output_buffer;
        // Color model u32 LE (ARGB)  -> u8 BGRA
        let red: u8 = ((color  >> 16) & 0xff)  as u8; 
        let green: u8  = ((color >> 8) & 0xff) as u8; 
        let blue: u8 = ((color >> 0) & 0xff) as u8; 
        let alpha: u8 = 0xff;

        for y in starty..endy {
            let offset = y * width * 4;
            for x  in startx..endx {
                let pos :usize= (offset + (x * 4)) as usize;

                buf[pos] = red;
                buf[pos + 1] = green;
                buf[pos + 2] = blue;
                buf[pos + 3] = alpha;
            }
        }

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
                let blue = ibuf[pos + 2] as f64;
                let green  = ibuf[pos + 1] as f64;
                let red = ibuf[pos] as f64;

                let gray =  (wred * red + wgreen * green  + wblue * blue).round() as u8;
                buf[pos] = gray;     // Red
                buf[pos + 1] = gray; // Green
                buf[pos + 2] = gray; // Blue
                buf[pos + 3] = 0xff; // alpha
            }
        }
    }
}

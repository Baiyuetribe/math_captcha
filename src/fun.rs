use image::{DynamicImage, ImageBuffer, Rgb};
use rand::Rng;
use std::io::Cursor;

use base64::engine::{general_purpose::STANDARD, Engine};
fn gen_math() -> (u32, char, u32) {
    let operators = ['+', '-', '×', '÷'];
    let mut rng = rand::thread_rng();
    let b = rng.gen_range(0..=3);
    let a = rng.gen_range(0..=30);
    let b = operators[b];
    let c = rng.gen_range(0..=30);
    (a, b, c)
}

// fn check_math(a: u32, b: char, c: u32) -> (bool, u32) {
//     match b {
//         '+' => {
//             if a + c <= 40 {
//                 (true, a + c)
//             } else {
//                 (false, 0)
//             }
//         }
//         '-' => {
//             if a >= c {
//                 // a - c <= 40
//                 if a - c <= 40 {
//                     (true, a - c)
//                 } else {
//                     (false, 0)
//                 }
//             } else {
//                 (false, 0)
//             }
//         }
//         '×' => {
//             if a * c <= 40 {
//                 (true, a * c)
//             } else {
//                 (false, 0)
//             }
//         }
//         '÷' => {
//             if c == 0 {
//                 (false, 0)
//             } else {
//                 if a % c == 0 {
//                     // a / c <= 40
//                     if a / c <= 40 {
//                         (true, a / c)
//                     } else {
//                         (false, 0)
//                     }
//                 } else {
//                     (false, 0)
//                 }
//             }
//         }
//         _ => (false, 0),
//     }
// }

fn check_math(a: u32, b: char, c: u32) -> Option<u32> {
    match b {
        '+' => {
            if a + c <= 40 {
                Some(a + c)
            } else {
                None
            }
        }
        '-' => {
            if a >= c && a - c <= 40 {
                Some(a - c)
            } else {
                None
            }
        }
        '×' => {
            if a * c <= 40 {
                Some(a * c)
            } else {
                None
            }
        }
        '÷' => {
            if c != 0 && a % c == 0 && a / c <= 40 {
                Some(a / c)
            } else {
                None
            }
        }
        _ => None,
    }
}
// 返回公式字符串+答案
pub fn rand_math() -> (String, u32) {
    let mut r = gen_math();
    loop {
        if let Some(v) = check_math(r.0, r.1, r.2) {
            return (format!("{} {} {} = ?", r.0, r.1, r.2), v);
        }
        r = gen_math();
    }
}

fn rand_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen_range(0..255);
    let g: u8 = rng.gen_range(0..255);
    let b; // 未定义，可省略mut
    let sum_rg = u16::from(r) + u16::from(g);
    if sum_rg > 400 {
        b = 0;
    } else {
        let tmp_b = 400 - u16::from(r) - u16::from(g);
        if tmp_b > 255 {
            b = 255;
        } else {
            b = tmp_b as u8;
        }
    }
    Rgb([r, g, b])
}
// ref https://github1s.com/mojocn/base64captcha/blob/HEAD/random_math.go
fn rand_light_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen_range(200..=255);
    let g: u8 = rng.gen_range(200..=255);
    let b: u8 = rng.gen_range(200..=255);
    Rgb([r, g, b])
}
// 用来绘制文字
fn rand_deep_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let rand_color = rand_color();
    let increase = rng.gen_range(30..=255);
    // let r = ((rand_color.0[0] as i32) - increase).min(255).abs() as u8;
    // let g = ((rand_color.0[1] as i32) - increase).min(255).abs() as u8;
    // let b = ((rand_color.0[2] as i32) - increase).min(255).abs() as u8;
    let r = rand_color.0[0].wrapping_sub(increase);
    let g = rand_color.0[1].wrapping_sub(increase);
    let b = rand_color.0[2].wrapping_sub(increase);
    Rgb([r, g, b])
}

// 最后效果是，输入宽高，输出答案+base64图像
pub fn draw_text(width: u32, height: u32, text: &str) -> String {
    let mut image = ImageBuffer::new(width, height);
    // 第一步，绘制背景颜色
    let background_color = rand_light_color();
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = background_color;
    }
    // 第二步，绘制干扰线
    let mut rng = rand::thread_rng();
    // for _ in 0..3 {
    //     let x1 = rng.gen_range(0..width);
    //     let y1 = rng.gen_range(0..height);
    //     let x2 = rng.gen_range(0..width);
    //     let y2 = rng.gen_range(0..height);
    //     let line_color = rand_deep_color();
    //     imageproc::drawing::draw_line_segment_mut(&mut image, (x1 as f32, y1 as f32), (x2 as f32, y2 as f32), line_color);
    // }
    // 第三步，绘制验证码文本
    let font = rusttype::Font::try_from_bytes(include_bytes!("OpenSans-Bold.ttf")).unwrap(); // actionj
                                                                                             // let scale = rusttype::Scale::uniform(rng.gen_range(10.0..40.0));
    let font_width = width as f32 / text.len() as f32;
    for (i, c) in text.chars().enumerate() {
        let font_size = &height * (rng.gen_range(0..7) + 7) / 48;
        let x = (font_width * i as f32 + font_width / font_size as f32) as i32;
        let y = (&height / 2 - font_size / 2 - rng.gen_range(0..&height / 3)) as i32;
        let color = rand_deep_color();
        let scale = rusttype::Scale::uniform(rng.gen_range(30.0..42.0));
        imageproc::drawing::draw_text_mut(&mut image, color, x, y, scale, &font, &c.to_string());
    }
    // let _ = image.save(format!("{}.png", text));
    to_base64_str(image)
}

fn to_base64_str(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
    // 将图像缓冲区转换为动态图像
    let dynamic_image: DynamicImage = ImageBuffer::from(image).into();

    // 创建字节缓冲区，用于存储图像数据
    let mut buf = Cursor::new(Vec::new());

    // 将图像数据写入字节缓冲区
    dynamic_image.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();

    // 将字节缓冲区编码为 Base64 字符串
    let base64_string = STANDARD.encode(&buf.get_ref());
    format!("data:image/png;base64,{}", base64_string)
}

// fn main() {
//     let start = std::time::Instant::now();
//     for _ in 0..10 {
//         let z = rand_math();
//         let zz = draw(170, 50, &z.0);
//         println!("{}", zz);
//     }
//     let end = std::time::Instant::now();
//     println!("time: {:?}", end - start);
// }

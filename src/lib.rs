mod fun;
pub struct Captcha {
    pub value: u32,
    pub base64_img: String,
}

impl Captcha {
    pub fn new(width: u32, height: u32) -> Self {
        //生成验证码字符数组
        let ask = fun::rand_math();
        let base64_img = fun::draw_text(width, height, &ask.0);
        //根据验证码字符数组生成图片并且把图片转化成base64字符串
        Captcha {
            value: ask.1,
            base64_img: base64_img,
        }
    }
}

#[test]
fn main_test() {
    let a = Captcha::new(150, 50);
    println!("value:{},base64_img:{}", a.value, a.base64_img);
}

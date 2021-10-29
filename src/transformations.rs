use crate::configuration::ShimmerType;

pub trait StripeTransformation {
    fn new(img_wight: usize);

    fn transform(bytes: Vec<u8>);
}

impl StripeTransformation for ShimmerType {
    fn new(img_wight: usize) {
        println!("new for nstripes")
    }

    fn transform(bytes: Vec<u8>) {
        print!("transform for nstripes");
    }
}


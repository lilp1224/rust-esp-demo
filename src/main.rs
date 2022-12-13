mod led_and_servo;

fn main() {
    esp_idf_sys::link_patches();
    println!("Hello, world! from rust ~");
    // /*led灯和舵机简单demo*/led_and_servo::start();
}

use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use std::net::UdpSocket;
// use std::{thread, time};

struct LightSystem {
    lights: Vec<Color>,
    controller: Controller,
    socket: UdpSocket,
    offset: usize,
}

impl LightSystem {
    fn render(&mut self) {
        let leds = self.controller.leds_mut(0);

        for x in 1..self.lights.len() {
            self.lights[x] = color_wheel((self.offset + x) as u8);
        }

        for (i, x) in leds.iter_mut().enumerate() {
            *x = self.lights[i].led_color();
        }

        // let a: Vec<[u8; 4]> = self.lights[0..leds.len()].into_iter().map(|x| x.led_color()).collect();
        // leds.clone_from_slice(a.as_slice());

        let udp_message: Vec<u8> = self.lights[800..].into_iter().map(|x| x.led_color()).flatten().collect();
        self.socket.send_to(udp_message.as_slice(), "55.55.55.56:41234").expect("couldn't send data");

        // Wait first, since data might still be flowing out of the serial connection
        self.controller.wait().unwrap();
        self.controller.render().unwrap();

        self.offset += 1;
    }
}

#[derive(Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    white: u8,
}

impl Color {
    fn led_color(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.white]
    }

    fn new (red: u8, green: u8, blue: u8) -> Color {
        return Color {
            red: red,
            green: green,
            blue: blue,
            white: 255,
        };
    }
}

fn color_wheel(mut position: u8) -> Color {
    position = position % 255;

    if position < 85 {
        return Color::new(255 - position * 3, 0, position * 3)
    }

    if position < 170 {
        position -= 85;
        return Color::new(0, position * 3, 255 - position * 3);
    }

    position -= 170;
    return Color::new(position * 3, 255 - position * 3, 0);
}

fn main() {
    // Construct a single channel controller. Note that the
    // Controller is initialized by default and is cleaned up on drop

    let controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(18) // GPIO 10 = SPI0 MOSI
                .count(800) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(255) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let socket = UdpSocket::bind("0.0.0.0:34254").expect("couldn't bind to address");
    let lights: Vec<Color> = std::iter::repeat(Color::new(0,0,0)).take(1600).collect(); // Vec::new(1600); 

    let mut system = LightSystem {
        controller: controller,
        socket: socket,
        lights: lights,
        offset: 0,
    };

    loop {
        system.render();
        // let mut test = controller;

        // for led in leds {
        //     *led = [0, 0, 255, 0];
        // }

        // test.render().unwrap();
    }
}

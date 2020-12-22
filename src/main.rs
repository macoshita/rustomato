use std::convert::TryInto;

use figlet_rs::FIGfont;
use msgbox::IconType;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let time: i32 = args[1].parse().unwrap();

    let standard_font = FIGfont::standand().unwrap();
    let colon_start_x = calc_colon_start_x(&standard_font);

    let mut fps = fps_clock::FpsClock::new(1);
    for t in (0..time + 1).rev() {
        fps.tick();
        let h = t / 60;
        let m = t % 60;
        let figure = standard_font
            .convert(&format!("{:02}:{:02}", h, m))
            .unwrap();
        let cc = &figure.characters;
        let start_x: u16 = (colon_start_x - cc[1].width - cc[0].width)
            .try_into()
            .unwrap();
        print!("{}", termion::clear::All);
        for (i, s) in figure.to_string().lines().enumerate() {
            println!(
                "{}{}",
                termion::cursor::Goto(start_x, (1 + i).try_into().unwrap()),
                s
            );
        }
    }

    msgbox::create("RUSTOMATO", "Time is up!", IconType::None).expect("Failed to open modal");
}

fn calc_colon_start_x(font: &FIGfont) -> u32 {
    let timer_width = 80;
    let colon_width = font.convert("25:00").unwrap().characters[2].width;
    (timer_width - colon_width) / 2
}

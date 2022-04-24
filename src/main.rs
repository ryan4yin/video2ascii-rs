use std::io::Error;
use std::io::Write;
use std::convert::TryInto;
use std::{thread, time};
use std::process::{Command, Child};

use opencv::{
    prelude::*,
    core::Size,
    core::Point,
    videoio,
    imgproc,
};

use crossterm::{
    cursor,
    execute, queue, style,
    terminal::{self, ClearType},
};

// 用于生成字符画的像素，越往后视觉上越明显。。这是我自己按感觉排的，你可以随意调整。
const PIXELS: &str = r#" .,-'`:!1+*abcdefghijklmnopqrstuvwxyz<>()\/{}[]?234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ%&@#$"#;


fn video2imgs(video_path: &str, size: (i32, i32), seconds: f64) -> Result<(Vec<Mat>, f64), opencv::Error> {
    let mut img_vec: Vec<Mat> = Vec::new();

    // 从指定文件创建一个 VideoCapture 对象
    #[cfg(not(ocvrs_opencv_branch_32))]
    let mut cam = videoio::VideoCapture::from_file(video_path, videoio::CAP_FFMPEG)?;
    if !cam.is_opened()? {
        panic!("Unable to open the video file: {}", video_path);
    }


    let fps = cam.get(videoio::CAP_PROP_FPS)?;
    let frames_count = (fps * seconds) as i32;

    let mut count = 0;
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width <= 0 {
            break
        }
        count += 1;

        // 转换成灰度图，也可不做这一步，转换成彩色字符视频。
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;


        // resize 图片，保证图片转换成字符画后，能完整地在命令行中显示。
        let mut resized_frame = Mat::default();
        let size = Size::new(size.0, size.1);
        imgproc::resize(&gray_frame, &mut resized_frame, size, 0.0, 0.0, imgproc::INTER_AREA)?;

        img_vec.push(resized_frame);

        if count >= frames_count {
            break
        }
    }
    cam.release()?;

    Ok((img_vec, fps))
}


fn img2chars(frame: &Mat) -> Result<Vec<String>, opencv::Error>{
    let mut result: Vec<String> = Vec::new();

    let size = frame.size()?;
    for row in 0..size.height {
        let mut line = String::new();
        for col in 0..size.width {
            let point = Point::new(col, row);
            // 这里将灰度从 0-255 转换到 0-1 之间
            let percent = frame.at_pt::<u8>(point)?;
            // 将灰度值进一步转换到 0 到 (len(pixels) - 1) 之间，这样就和 pixels 里的字符对应起来了
            let index = (*percent as u16 * (PIXELS.len() - 1) as u16) / 255;
            // 添加字符像素（最后面加一个空格，是因为命令行有行距却没几乎有字符间距，用空格当间距）
            line.push(PIXELS.chars().nth(index.try_into().unwrap()).unwrap());
            line.push(' ');
        }
        result.push(line);
    }

    return Ok(result);
}


fn play_video(char_imgs: Vec<Vec<String>>, fps: f64) -> std::result::Result<(), Error> {
    let wait_millis = time::Duration::from_secs_f64(1.0 / fps);

    let mut w = std::io::stdout();
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    for char_img in &char_imgs {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;

        for line in char_img {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }
        w.flush()?;
        thread::sleep(wait_millis);
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}


fn play_audio(video_path: &str) -> std::io::Result<Child>{
    Command::new("mpv")
    .args(&["--no-video", &video_path])
    .spawn()
}

fn main() {
    let video_path = "./BadApple.mp4";
    let size = (64, 48);
    let seconds = 30.0;

    let (imgs, fps) = video2imgs(video_path, size, seconds).unwrap();
    let mut char_imgs: Vec<Vec<String>> = Vec::new();
    for img in &imgs {
        let char_img = img2chars(img).unwrap();
        char_imgs.push(char_img);
    }

    let mut child = play_audio(video_path).unwrap();
    play_video(char_imgs, fps).unwrap();
    child.kill().unwrap();
}

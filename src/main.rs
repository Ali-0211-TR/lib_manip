use opencv::{
    core::*,
    imgcodecs::*,
    highgui::*, videoio::*, imgproc::gaussian_blur,
    
};

enum short_key_action{
    START,
    QUIT,
    EXIT,
    SAVE,
    NONE,
}

fn i32_to_ska(key: i32) -> short_key_action{
    let key = key as u8;
    let key = key as char;
    match key {
        'q' => short_key_action::QUIT,
        'w' => short_key_action::START,
        'e' => short_key_action::EXIT,
        's' => short_key_action::SAVE,
        _ => short_key_action::NONE,
    }
}

fn main() {
    let api_pref = opencv::videoio::CAP_ANY;
    let mut cap = VideoCapture::new(0, api_pref).unwrap();
    //cap.set(CAP_PROP_BRIGHTNESS, 50.0).unwrap();
    let _shortcads = 'q' as i32;

    if cap.is_opened().unwrap(){
        println!("Camera opened");
    }else{
        println!("Camera not opened");
    }

    let mut img = Mat::default();
    loop {
        cap.read(&mut img).unwrap();       
        
        match cap.is_opened() {
            Ok(c) => {
                if c{
                    imshow("Local Camera", &img).unwrap();
                }

            },
            Err(e) => println!("Error: {:?}", e)
        }
        match i32_to_ska(wait_key(1).unwrap()) {
            short_key_action::QUIT => {break;},
            short_key_action::EXIT => {
                println!("EXIT button pushed!");
                match cap.release(){
                    Ok(a) => {},
                    Err(e) => println!("Error: {:?}", e),
                    _ => {}
                }
            }
            short_key_action::START => {
                match cap.is_opened(){
                    Ok(c) => {
                        if !c{
                            cap = VideoCapture::new(0, api_pref).unwrap();
                        }
                        else{
                            println!("Camera has been opened!");
                        }
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            short_key_action::SAVE => {
                let tmp = Vector::default();
                opencv::imgcodecs::imwrite("Seved_file.jpg", &img, &tmp).unwrap();
            }
            _ => { }
        }
        
    }
    
    println!("Hello, world!{}", _shortcads);
}


fn prt(cap: &VideoCapture){
    println!("{:?}", cap.get_backend_name().unwrap());
    
}

enum CameraError {
    ConnectionFailed,
    OtherError(String),
}

struct MyCamera{
    login: String,
    pass: String,
    host: (String, u32),
    name: String,
    cap: VideoCapture,
}


impl MyCamera {
    fn new(nm: &str, lg: &str, pass: &str, ip: &str, port: u32) -> Result<MyCamera, opencv::Error> {
        let cap = VideoCapture::default()?;

        Ok(MyCamera {
            login: lg.to_string(),
            pass: pass.to_string(),
            host: (ip.to_string(), port),
            name: nm.to_string(),   
            cap: cap,
        })
    }
    fn get_url(&self)->String{
        format!(
            "http://{}:{}@{}/ISAPI/Streaming/channels/102/httpPreview", 
            self.login, 
            self.pass, 
            self.host.0
        )
    }
    fn connect(&mut self) -> Result<(), opencv::Error>{
        self.cap = VideoCapture::from_file(self.get_url().as_str(), CAP_ANY)?;

        Ok(())
    }
}
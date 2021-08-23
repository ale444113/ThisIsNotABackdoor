extern crate camera_capture;

fn getImage() -> Result<camera_capture::ImageIterator, std::io::Error>{
    let cam = camera_capture::create(0).unwrap();
    let cam = cam.fps(5.0).unwrap().start();
    if cam.is_err(){
        return Err(std::io::Error::from(std::io::ErrorKind::NotConnected));
    }
    Ok(cam.unwrap())
}
use opencv::core::{Rect, Size, Vector};
use opencv::imgcodecs::{imread, imwrite, IMREAD_UNCHANGED, IMWRITE_JPEG_QUALITY};
use opencv::imgproc::{cvt_color, resize, COLOR_BGR2GRAY, INTER_AREA};
use opencv::objdetect::{CascadeClassifier, CASCADE_SCALE_IMAGE};
use opencv::prelude::*;
use opencv::types::VectorOfRect;
use opencv::Error;


pub struct Moustacher {
    cascade_face: CascadeClassifier,
    moustache_original: Mat,
}

impl Moustacher {
    pub fn new(cascade_face_path: String, moustache_path: String) -> Result<Self, Error> {
        let stache_img = imread(&moustache_path, IMREAD_UNCHANGED)?;
        println!("faces path: {cascade_face_path}");
        Ok(Self {
            cascade_face: CascadeClassifier::new(&cascade_face_path)?,
            moustache_original: stache_img,
        })
    }

    fn detect_faces(mut self, image: Mat) -> Result<VectorOfRect, Error> {
        let mut faces: Vector<Rect> = Vector::new();
        self.cascade_face.detect_multi_scale(
            &image,
            &mut faces,
            1.1,
            5,
            CASCADE_SCALE_IMAGE,
            Size::new(30, 30),
            Size::default(),
        )?;
        println!("Faces {:?}", faces);
        Ok(faces)

    }


    pub fn add_moustache_to_image(self, source: String, destination: String) -> Result<(), Error> {
        println!("moustache parsed {:?}", self.moustache_original);
        let image = imread(&source, IMREAD_UNCHANGED)?;
        println!("Image parsed {:?}", image);
        let mut image_gray_scale = Mat::default();
        cvt_color(&image, &mut image_gray_scale, COLOR_BGR2GRAY, 0)?;
        let faces = self.detect_faces(image_gray_scale.clone())?;
        println!("Faces {:?}", faces);
        for face_location in faces {
            let _roi_gray = Mat::roi(&image_gray_scale, face_location)?;
            let _roi_color = Mat::roi(&image, face_location)?;
            let mut _noses: Vector<Rect> = Vector::new();
            }
        let params: Vector<i32> = Vector::from_slice(&[IMWRITE_JPEG_QUALITY, 90]);
        imwrite(&destination, &image_gray_scale, &params)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_moustache_initalization() {
        Moustacher::new(
            String::from("/usr/share/opencv4/haarcascades/haarcascade_frontalface_default.xml"),
            String::from("../static/mustache.png"),
        )
        .unwrap();
    }
}

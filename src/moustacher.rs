use opencv::core::{Rect, Size, Vector};
use opencv::imgcodecs::{imread, imwrite, IMREAD_UNCHANGED, IMWRITE_JPEG_QUALITY};
use opencv::imgproc::{cvt_color, resize, COLOR_BGR2GRAY, INTER_AREA};
use opencv::objdetect::{CascadeClassifier, CASCADE_SCALE_IMAGE};
use opencv::prelude::*;
use opencv::Error;

pub struct Moustacher {
    cascade_face: CascadeClassifier,
    cascade_nose: CascadeClassifier,
    moustache_original: Mat,
}

impl Moustacher {
    pub fn new(cascade_face_path: String, cascade_nose_path: String, moustache_path: String) -> Result<Self, Error> {
        let stache_img = imread(&moustache_path, IMREAD_UNCHANGED)?;
        println!("faces path: {cascade_face_path}");
        Ok(Self {
            cascade_face: CascadeClassifier::new(&cascade_face_path)?,
            cascade_nose: CascadeClassifier::new(&cascade_nose_path)?,
            moustache_original: stache_img,
        })
    }

    pub fn add_moustache_to_image(mut self, source: String, destination: String) -> Result<(), Error> {
        println!("{:?}", self.moustache_original);
        let image = imread(&source, IMREAD_UNCHANGED)?;
        println!("{:?}", image);
        let mut image_gray_scale = Mat::default();
        cvt_color(&image, &mut image_gray_scale, COLOR_BGR2GRAY, 0)?;
        let mut faces: Vector<Rect> = Vector::new();
        self.cascade_face.detect_multi_scale(
            &image_gray_scale,
            &mut faces,
            1.1,
            5,
            CASCADE_SCALE_IMAGE,
            Size::new(30, 30),
            Size::default(),
        )?;
        println!("{:?}", faces);
        for face_location in faces {
            let roi_gray = Mat::roi(&image_gray_scale, face_location)?;
            let roi_color = Mat::roi(&image, face_location)?;
            let mut noses: Vector<Rect> = Vector::new();
            self.cascade_nose
                .detect_multi_scale(&roi_gray, &mut noses, 1.1, 3, 0, Size::default(), Size::default())?;
            if noses.is_empty() {
                panic!("NO NOSE");
            }
            println!("{:?}", noses);
            for nose in noses {
                let stache_width = 3 * nose.width;
                let stache_height = stache_width * self.moustache_original.rows() / self.moustache_original.cols();
                let mut x1 = nose.x - (stache_width / 4);
                let mut x2 = nose.x + nose.width + (stache_width / 4);
                let mut y1 = nose.y + nose.height - (stache_height / 4);
                let mut y2 = nose.y + nose.height + (stache_height / 4);
                if x1 < 0 {
                    x1 = 0;
                };
                if y1 < 0 {
                    y1 = 0;
                }
                if x2 > face_location.width {
                    x2 = face_location.width;
                }
                if y2 > face_location.height {
                    y2 = face_location.height;
                }
                let stache_width = x2 - x1;
                let stache_height = y2 - y1;
                let mut final_mustache = Mat::default();
                resize(
                    &self.moustache_original,
                    &mut final_mustache,
                    Size::new(stache_width, stache_height),
                    0.0,
                    0.0,
                    INTER_AREA,
                )?;
            }
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
            String::from("/usr/share/opencv4/haarcascades/haarcascade_mcs_nose.xml"),
            String::from("../static/mustache.png"),
        )
        .unwrap();
    }
}

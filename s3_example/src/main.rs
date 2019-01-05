extern crate rusoto_core;
extern crate rusoto_s3;

use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3, S3Client};

////////////////////////////////////////////////////////////////////////////////
/// Get an object from S3 and saves it into a file
///
/// Gets object identified by key from bucket and writes it into filepath
///
/// `bucket` - an S3 bucket identifier
/// `key` - an S3 object key
/// `region` - an AWS region
/// `filepath` - a path to a file
///
/// Return Ok(filepath), or Err(...)
////////////////////////////////////////////////////////////////////////////////
fn get_s3_object(
    bucket: &str,
    key: &str,
    region: Region,
    filepath: &str,
) -> Result<String, (String)> {
    let client = S3Client::new(region);
    match client
        .get_object(GetObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            ..Default::default()
        })
        .sync() {
        Ok(res) => {
            let mut stream = res.body.unwrap().into_blocking_read();

            // 4k buffer
            let mut buffer = [0; 1 << 12];
            let mut f = File::create(filepath).unwrap();

            // Read into the buffer until the end of the Stream
            loop {
                match stream.read(&mut buffer) {
                    Ok(res) => {
                        if res == 0 {
                            break;
                        }
                    }
                    Err(err) => return Err(err.to_string()),
                }
                f.write(&buffer).unwrap();
            }

        }
        Err(err) => {
            return Err(err.description().to_string());

        }
    };
    return Ok(filepath.to_string());
}

fn main() {
    get_s3_object("juliod", "julio_photo.png", Region::UsWest2, "/tmp/me.png").unwrap();
}

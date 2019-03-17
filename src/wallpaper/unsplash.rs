use hyper::body::Chunk;
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper::Response;
use hyper_tls::HttpsConnector;
use serde_json::{self, Value};
use std::fs::File;
use std::io::Write;

/// url for download unsplash photos (random api)
///
/// https://api.unsplash.com/photos/random?client_id=4febf919fde1d66795ad19ad5c67ca0267bcf5e6783a42f32b74c92ad473dd9c
const UNSPALSH_ACCESS_KEY: &str =
    "4febf919fde1d66795ad19ad5c67ca0267bcf5e6783a42f32b74c92ad473dd9c";

/// get wallpaper download url
pub fn get_wallpaper_url() -> String {
    format!(
        "https://api.unsplash.com/photos/random?client_id={}",
        UNSPALSH_ACCESS_KEY
    )
}

pub fn get_random_image(file: String) {
    rt::run(rt::lazy(|| {
        let https_connector = HttpsConnector::new(2).expect("TLS initialization failed");
        let client = Client::builder()
            .http1_read_buf_exact_size(1024 * 1024)
            .build::<_, hyper::Body>(https_connector);
        let uri = get_wallpaper_url().parse().unwrap();
        client
            .get(uri)
            .and_then(move |res: Response<_>| {
                //let headers = res.headers();
                //println!("status code : {}\n, headers: {:?}", res.status(), headers);
                let chunk: Chunk = res.into_body().concat2().wait().unwrap();
                let v = chunk.to_vec();
                let body = String::from_utf8(v).unwrap();
                //println!("total body: {:?}", body);
                let json = serde_json::from_str::<Value>(&body[..]).unwrap();
                let json = &json["urls"]["full"];
                match json {
                    Value::String(raw_image_url) => {
                        println!("downloading: {}", raw_image_url);
                        Ok(String::from(&raw_image_url[..]))
                    }

                    _ => {
                        println!("not matchecd");
                        Ok(String::from(""))
                    }
                }
            })
            .and_then(move |img_url| {
                client.get(img_url.parse().unwrap()).and_then(|res| {
                    /*
                    let chunk: Chunk = res.into_body().concat2().wait().unwrap();
                    let v = chunk.by();
                    */
                    println!("try to write to {}", file);
                    let mut f = File::create(file).expect("desktop file can't be written");
                    res.into_body()
                        .map_err(|_| (()))
                        .for_each(move |chunk: Chunk| {
                            println!("got chunk: {}", chunk.len());
                            f.write_all(&chunk).map_err(|_| ())
                        })
                        .wait()
                        .unwrap();

                    Ok(())
                })
            })
            .map_err(|err| eprintln!("Error {}", err))
    }));
}

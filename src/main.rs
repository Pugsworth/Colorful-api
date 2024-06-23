use std::io::{Read, Write};

use actix_web::{get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use image::{GenericImage, ImageEncoder, Rgb};
use palette_generator::{ColorBlockLayout, ColorSortOrder};
use rand::Rng;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use serde_qs::actix::QsQuery;
use hex_color::HexColor;

mod palette_generator;


#[get("/")]
async fn hello(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

fn from_csv<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: DeserializeOwned + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    deserializer.deserialize_str(CSVVecVisitor::<T>::default())
}

struct CSVVecVisitor<T: DeserializeOwned + std::str::FromStr>(std::marker::PhantomData<T>);

impl<T: DeserializeOwned + std::str::FromStr> Default for CSVVecVisitor<T> {
    fn default() -> Self {
        CSVVecVisitor(std::marker::PhantomData)
    }
}

impl<'de, T: DeserializeOwned + std::str::FromStr> serde::de::Visitor<'de> for CSVVecVisitor<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug, // handle the parse error in a generic way
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a str")
    }

    fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Treat the comma-separated string as a single record in a CSV.
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(s.as_bytes());

        // Try to get the record and collect its values into a vector.
        let mut output = Vec::new();
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    for field in record.iter() {
                        output.push(
                            field
                                .parse::<T>()
                                .map_err(|_| E::custom("Failed to parse field"))?,
                        );
                    }
                }
                Err(e) => {
                    return Err(E::custom(format!(
                        "could not deserialize sequence value: {:?}",
                        e
                    )));
                }
            }
        }

        Ok(output)
    }
}


#[derive(Debug, Deserialize)]
pub struct ColorParams {
    #[serde(deserialize_with="from_csv")]
    // #[serde(with="hex_color::u24")]
    colors: Vec<HexColor>,            // List of colors
    bs: Option<u16>,                  // block_size
    layout: Option<ColorBlockLayout>, // Layout
    order: Option<ColorSortOrder>,    // Ordering
}


async fn manual_hello(req: HttpRequest, query: QsQuery<ColorParams>) -> impl Responder {
    let colors = &query.colors;
    let block_size = query.bs.unwrap_or(1);
    let layout = query.layout.as_ref().unwrap_or(&ColorBlockLayout::Linear);

    let img = palette_generator::image_from_colors(colors, block_size, *layout);

    // println!("bs: {}", block_size);
    println!("{:?}", query);

    // Encode the raw bytes to PNG and send to client.
    let mut buf = std::io::Cursor::new(Vec::new());
    let _ = img.write_to(&mut buf, image::ImageFormat::Png);
    let png_bytes = buf.into_inner();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(png_bytes)

    // HttpResponse::Ok().body(str)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let img = generate_image(128, 128);
    // img.save("image.png").unwrap();


    let _ = HttpServer::new(||{
        App::new()
            // .service(hello)
            .route("/palette", web::get().to(manual_hello))
            .service(actix_files::Files::new("/", "./public").show_files_listing())
    })
    .bind(("localhost", 8080))?
        .run()
        .await;

    Ok(())
}

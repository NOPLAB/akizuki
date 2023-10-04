use std::{
    fs::{self, File},
    io,
    ops::Add,
};

use scraper::{Html, Selector};

const AKIZUKI_HOME_URL: &str = "https://akizukidenshi.com";
const AKIZUKI_GOODS_URL: &str = "https://akizukidenshi.com/catalog/e/enewall_dT";

#[tokio::main]
async fn main() {
    let html = reqwest::get(AKIZUKI_GOODS_URL)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let doc = Html::parse_document(&html);

    // 商品の一つ一つを収集
    let objects_selector = Selector::parse(".thumbox").unwrap();
    let objects = doc.select(&objects_selector);

    for object in objects {
        // 画像のURLを抽出
        let img_selector = Selector::parse("img").unwrap();
        let img = object.select(&img_selector).collect::<Vec<_>>()[0];
        let img_url = img.value().attr("src").unwrap();
        println!("{:?}", img_url);

        // 画像を収集
        let img_response = reqwest::get(AKIZUKI_HOME_URL.to_string().add(img_url))
            .await
            .unwrap();
        let img_bytes = img_response.bytes().await.unwrap();

        // 画像を保存
        fs::create_dir_all("Akizuki").unwrap();
        let img_filename = img_url.split("/").last().unwrap();
        let mut out = File::create("Akizuki/".to_string().add(img_filename)).unwrap();
        io::copy(&mut img_bytes.as_ref(), &mut out).unwrap();
    }
}

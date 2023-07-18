mod types;

#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://honkai-star-rail.fandom.com/wiki/Character/List")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut a = 0;
    for character in herta::extractor::index_characters(resp) {
        dbg!(character.path);
        dbg!(character.rarity);
        dbg!(character.ctype);
        a += 1;
    }
    println!("{} characters indexed", a)
}

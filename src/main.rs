use reqwest::header;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://down.foodmate.net/standard/down.php?auth=123275";
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;
    // let text = resp.text().await?;
    // let doc = Html::parse_fragment(&text);
    // let selector = Selector::parse("div.bz_listl > ul > a").unwrap();

    // for el in doc.select(&selector) {
    //     let resp = client
    //         .get(el.value().attr("href").unwrap_or("null"))
    //         .send()
    //         .await?;
    //     let headers = resp.headers();
    //     // println!("{}", headers["Connection"].to_str().unwrap());
    //     // println!("{}", headers.get("Connection").unwrap().to_str().unwrap());
    //     let text = resp.text().await?;
    //     let doc = Html::parse_fragment(&text);
    //     let selector = Selector::parse("div.downk > a.telecom").unwrap();

    //     for el in doc.select(&selector) {
    //         let mut headers = HeaderMap::new();
    //         let link = el.value().attr("href").unwrap();
    //         headers.insert(COOKIE, HeaderValue::from_str(r#"__gads=ID=ca195710e1b89df8:T=1648200092:S=ALNI_MYjAMe_-PCFu6PE56uHNTGaN0k8og; bc08_f0d8_saltkey=bZt99dG9; bc08_f0d8_lastvisit=1669597318; bc08_f0d8_lastact=1669690826	api.php	js; Hm_lvt_2aeaa32e7cee3cfa6e2848083235da9f=1668826787,1669190296,1669600919,1669690829; Hm_lvt_d4fdc0f0037bcbb9bf9894ffa5965f5e=1668822035,1669190297,1669600921,1669690830; __51cke__=; u_rdown=1; __gpi=UID=0000056d406a7b7f:T=1652773656:RT=1669690846:S=ALNI_MZYDLN6f5QqoCSt_at5V_figPcoFg; __tins__1484185={"sid#": 1669705608429, "vd": 10, "expires": 1669708093646}; __51laig__=34; Hm_lpvt_d4fdc0f0037bcbb9bf9894ffa5965f5e=1669706294; Hm_lpvt_2aeaa32e7cee3cfa6e2848083235da9f=1669706294"#).unwrap());
    //         headers.insert("Referer", HeaderValue::from_str(link).unwrap());
    //         // println!("{}", el.value().attr("href").unwrap());
    //         headers.insert("User-Agent",HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.56").unwrap());
    //         headers.insert("Host", HeaderValue::from_str("down.foodmate.net").unwrap());
    //         let client = reqwest::Client::new();
    //         let resp = client.get(link).headers(headers).send().await?;
    //         let text = resp.headers();
    //         println!("{}", text.get("location").unwrap().to_str().unwrap(),);
    //     }
    // }

    let mut headers = header::HeaderMap::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
    headers.insert(
        "Accept-Language",
        "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"
            .parse()
            .unwrap(),
    );
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert(header::COOKIE, "__gads=ID=ca195710e1b89df8:T=1648200092:S=ALNI_MYjAMe_-PCFu6PE56uHNTGaN0k8og; bc08_f0d8_saltkey=bZt99dG9; bc08_f0d8_lastvisit=1669597318; bc08_f0d8_lastact=1669690826%09api.php%09js; Hm_lvt_2aeaa32e7cee3cfa6e2848083235da9f=1668826787,1669190296,1669600919,1669690829; Hm_lvt_d4fdc0f0037bcbb9bf9894ffa5965f5e=1668822035,1669190297,1669600921,1669690830; __51cke__=; u_rdown=1; __gpi=UID=0000056d406a7b7f:T=1652773656:RT=1669690846:S=ALNI_MZYDLN6f5QqoCSt_at5V_figPcoFg; Hm_lpvt_d4fdc0f0037bcbb9bf9894ffa5965f5e=1669705798; __tins__1484185=%7B%22sid%22%3A%201669705608429%2C%20%22vd%22%3A%208%2C%20%22expires%22%3A%201669707597775%7D; __51laig__=32; Hm_lpvt_2aeaa32e7cee3cfa6e2848083235da9f=1669705798".parse().unwrap());
    headers.insert(
        "Referer",
        "http://down.foodmate.net/standard/sort/3/123275.html"
            .parse()
            .unwrap(),
    );
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.56".parse().unwrap());

    let resp = client.get(url).headers(headers).send().await?;
    let h = resp.headers().get("location");
    println!("{}",h.unwrap().to_str().unwrap());

    Ok(())
}

// body > div:nth-child(7) > div.fl_rb > div > div.bz_list > ul > li> div.bz_listl > ul > a > b
// body > div:nth-child(8) > div.fl_rb > div:nth-child(4) > div.downk > a.telecom

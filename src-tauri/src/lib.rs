use std::{fs::File, path::Path, io::{Read, Write}};

use serde::ser::{Serialize, SerializeMap, Serializer};
use urlencoding::encode;

#[derive(Debug)]
pub struct Singer {
    pub id: String,
    pub name: String,
}

impl Serialize for Singer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.end()
    }
}

#[derive(Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
}

impl Serialize for Album {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.name.len()))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.end()
    }
}

#[derive(Debug)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub download_url: String,
    pub singers: Vec<Singer>,
    pub albums: Vec<Album>,
}

impl Serialize for Song {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.name.len()))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("image_url", &self.image_url)?;
        map.serialize_entry("download_url", &self.download_url)?;
        map.serialize_entry("singers", &self.singers)?;
        map.serialize_entry("albums", &self.albums)?;
        map.end()
    }
}

#[derive(Debug)]
pub struct Page {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub songs: Vec<Song>,
}

impl Serialize for Page {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.total as usize))?;
        map.serialize_entry("page", &self.page)?;
        map.serialize_entry("page_size", &self.page_size)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("songs", &self.songs)?;
        map.end()
    }
}

pub fn search(
    key_word: &str,
    page_number: u8,
    page_size: u8,
    quality: &str,
) -> Result<Page, String> {
    println!(
        "search {} {} {} {}",
        key_word, page_number, page_size, quality
    );
    let switch =
        r#"{"song":1,"album":0,"singer":0,"tagSong":0,"mvSong":0,"songlist":0,"bestShow":1}"#;
    let url = format!("http://pd.musicapp.migu.cn/MIGUM2.0/v1.0/content/search_all.do?ua=Android_migu&version=5.0.1&pageNo={}&pageSize={}&text={}&searchSwitch={}", page_number, page_size, encode(key_word), encode(switch));
    println!("url {}", url);
    let resp = reqwest::blocking::get(&url).unwrap().text().unwrap();
    println!("response: {:?}", resp);
    let data = json::parse(&resp).unwrap();
    let code = &data["code"].as_str().unwrap().parse::<i32>().unwrap();
    if *code != 0 {
        return Err(format!("code: {}", code));
    }
    if data["songResultData"]["totalCount"].is_null() {
        return Err(format!("not result"));
    }
    let total_count = &data["songResultData"]["totalCount"].as_str().unwrap();
    println!("total_count: {}", total_count);
    let mut songs = Vec::<Song>::new();
    for song_info in data["songResultData"]["result"].members() {
        // albums = [Album(name=a["name"], id=a["id"]) for a in song_info["albums"]] if "albums" in song_info else []
        // let singers = [Singer(name=s['name'], id=s['id']) for s in song_info['singers']]
        let mut singers = Vec::<Singer>::new();
        for singer in song_info["singers"].members() {
            singers.push(Singer {
                id: singer["id"].as_str().unwrap().to_string(),
                name: singer["name"].as_str().unwrap().to_string(),
            });
        }
        let mut albums = Vec::<Album>::new();
        for a in song_info["albums"].members() {
            albums.push(Album {
                id: a["id"].as_str().unwrap().to_string(),
                name: a["name"].as_str().unwrap().to_string(),
            });
        }
        let image_items = &song_info["imgItems"];
        let image_url = if image_items.len() > 0 {
            image_items[0]["img"].as_str().unwrap().to_string()
        } else {
            "".to_string()
        };
        let mut tone_type = "SQ&formatType=SQ&resourceType=E";
        if quality == "HQ" {
            tone_type = "HQ&formatType=HQ&resourceType=2";
        }
        let download_url = format!("http://218.205.239.34/MIGUM2.0/v1.0/content/sub/listenSong.do?toneFlag={}&netType=00&copyrightId=0&&contentId={}&channel=0", tone_type, song_info["contentId"].as_str().unwrap().to_string());
        let song = Song {
            id: song_info["id"].as_str().unwrap().to_string(),
            name: song_info["name"].as_str().unwrap().to_string(),
            image_url,
            download_url,
            singers,
            albums,
        };
        songs.push(song)
    }
    Ok(Page {
        page: page_number as u32,
        page_size: page_size as u32,
        total: total_count.parse::<u32>().unwrap(),
        songs,
    })
}

pub fn download(name: &str, url: &str, path: &str) -> Result<(), String> {
    println!("download {} {} {}", name, url, path);
    let mut resp = reqwest::blocking::get(url).unwrap();
    let content_type = resp.headers().get("Content-Type").unwrap().to_str().unwrap();
    println!("content_type: {}", content_type);
    let mime_sub_type = content_type.split("/").nth(1).unwrap();
    let mut extension = "mp3";
    match mime_sub_type {
        "mpeg" => extension = "mp3",
        "x-flac" => extension = "flac",
        _ => (),
    }
    let file_path = Path::new(path).join(format!("{}.{}", name, extension));
    let mut file = File::create(file_path).unwrap();
    let mut buf = [0; 1024];
    while let Ok(len) = resp.read(&mut buf) {
        if len == 0 {
            break;
        }
        file.write_all(&buf[..len]).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_search_song_successfully() {
        let result = search("许巍", 1, 10, "SQ").unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn should_parse_json_success() {
        let json = r#"
        {
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        }
        "#;
        let data = json::parse(json).unwrap();
        assert_eq!(data["code"], 200);
    }
}

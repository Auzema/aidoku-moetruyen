#![no_std]
extern crate alloc;

use aidoku::{
    error::Result,
    prelude::*,
    std::{
        html::Node,
        net::{HttpMethod, Request},
        String, Vec,
    },
    Chapter, DeepLink, Filter, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Page,
};
use alloc::{string::ToString, format};

const BASE_URL: &str = "https://truyen.moe";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    // Basic search implementation
    let url = format!("{}/manga?page={}", BASE_URL, page);
    let html = Request::new(url, HttpMethod::Get).html()?;
    
    let mut mangas = Vec::new();
    
    for item in html.select("article.manga-card").array() {
        let node = item.as_node()?;
        
        let id_url = node.select("a").attr("href").read();
        let id = id_url.replace("/manga/", "");
        
        let title = node.select(".manga-body h3").text().read();
        let cover = node.select(".cover img").attr("src").read();
        
        mangas.push(Manga {
            id,
            cover,
            title,
            author: node.select(".manga-author").text().read(),
            artist: String::new(),
            description: node.select(".manga-description-preview").text().read(),
            url: format!("{}{}", BASE_URL, id_url),
            categories: Vec::new(),
            status: MangaStatus::Unknown,
            nsfw: MangaContentRating::Safe,
            viewer: MangaViewer::Default,
        });
    }
    
    Ok(MangaPageResult {
        manga: mangas,
        has_more: true, // Need to implement proper pagination check
    })
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
    get_manga_list(Vec::new(), page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
    let url = format!("{}/manga/{}", BASE_URL, id);
    let html = Request::new(url.clone(), HttpMethod::Get).html()?;
    
    let title = html.select("h1.manga-detail-title").text().read();
    let cover = html.select("meta[property=\"og:image\"]").attr("content").read();
    let description = html.select("meta[property=\"og:description\"]").attr("content").read();
    
    let mut authors = Vec::new();
    for author_node in html.select(".manga-detail-meta-line a").array() {
        authors.push(author_node.as_node()?.text().read());
    }
    
    let mut categories = Vec::new();
    for genre in html.select("a.chip").array() {
        categories.push(genre.as_node()?.text().read());
    }
    
    let status_str = html.select(".manga-status-pill").text().read();
    let status = if status_str.contains("Còn tiếp") {
        MangaStatus::Ongoing
    } else if status_str.contains("Đã hoàn thành") {
        MangaStatus::Completed
    } else {
        MangaStatus::Unknown
    };
    
    Ok(Manga {
        id,
        cover,
        title,
        author: authors.join(", "),
        artist: String::new(),
        description,
        url,
        categories,
        status,
        nsfw: MangaContentRating::Safe,
        viewer: MangaViewer::Default,
    })
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
    let url = format!("{}/manga/{}", BASE_URL, id);
    let html = Request::new(url, HttpMethod::Get).html()?;
    
    let mut chapters = Vec::new();
    
    for item in html.select("li.chapter").array() {
        let node = item.as_node()?;
        
        let chapter_id = node.attr("data-chapter-id").read();
        let chapter_url = node.select("a.chapter-link").attr("href").read();
        let chap_num_str = node.select(".chapter-num").text().read().replace("Ch. ", "");
        let title = node.select(".chapter-title").text().read();
        
        let chap_num = chap_num_str.parse::<f32>().unwrap_or(0.0);
        
        chapters.push(Chapter {
            id: chapter_id,
            title,
            volume: -1.0,
            chapter: chap_num,
            date_updated: 0.0,
            scanlator: String::new(),
            url: format!("{}{}", BASE_URL, chapter_url),
            lang: String::from("vi"),
        });
    }
    
    Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
    // TODO: IMGX decryption logic
    Ok(Vec::new())
}

#[modify_image_request]
fn modify_image_request(request: Request) {
    // request.header("Referer", BASE_URL);
}

#[handle_url]
fn handle_url(url: String) -> Result<DeepLink> {
    todo!()
}

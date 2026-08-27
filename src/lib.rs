#![no_std]
extern crate alloc;

use aidoku::{
    Result, Manga, MangaPageResult, FilterValue, Chapter, Page, Source, ListingProvider, ContentRating, MangaStatus
};
use aidoku::prelude::*;
use aidoku::alloc::{string::String, vec::Vec};

const BASE_URL: &str = "https://truyen.moe";

struct MoeTruyen;

impl Source for MoeTruyen {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(&self, _query: core::option::Option<String>, _page: i32, _filters: Vec<FilterValue>) -> Result<MangaPageResult> {
        let mut entries = Vec::new();
        
        let mut authors = Vec::new();
        authors.push(String::from("Antigravity"));

        entries.push(Manga {
            key: String::from("dummy"),
            title: String::from("Mòe Truyện (Bị chặn bởi IMGX)"),
            authors: Some(authors),
            description: Some(String::from("Truyện này bị mã hóa bởi IMGX nên không thể đọc được nội dung trên Aidoku hiện tại.")),
            cover: Some(String::from("https://truyen.moe/pwa/apple-touch-icon.png")),
            url: Some(String::from(BASE_URL)),
            status: MangaStatus::Unknown,
            content_rating: ContentRating::Safe,
            ..Default::default()
        });
        
        Ok(MangaPageResult {
            entries,
            has_next_page: false,
        })
    }

    fn get_manga_update(&self, mut manga: Manga, needs_details: bool, needs_chapters: bool) -> Result<Manga> {
        if needs_details {
            manga.title = String::from("Mòe Truyện (Bị chặn bởi IMGX)");
            manga.description = Some(String::from("Truyện này bị mã hóa bởi IMGX nên không thể đọc được nội dung."));
            manga.status = MangaStatus::Unknown;
        }

        if needs_chapters {
            manga.chapters = Some(Vec::new());
        }

        Ok(manga)
    }

    fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
        Ok(Vec::new())
    }
}

impl ListingProvider for MoeTruyen {
    fn get_manga_list(&self, _listing: aidoku::Listing, page: i32) -> Result<MangaPageResult> {
        self.get_search_manga_list(None, page, Vec::new())
    }
}

register_source!(MoeTruyen, ListingProvider);

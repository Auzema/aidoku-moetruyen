#![no_std]
use aidoku::{Source, ListingProvider, prelude::*, Result, Manga, MangaPageResult, FilterValue, Chapter, Page};
use aidoku::alloc::{String, Vec};

struct MoeTruyen;

impl aidoku::Source for MoeTruyen {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(&self, _query: core::option::Option<String>, _page: i32, _filters: Vec<FilterValue>) -> Result<MangaPageResult> {
        Ok(MangaPageResult::default())
    }

    fn get_manga_update(&self, _manga: Manga, _update_cover: bool, _update_metadata: bool) -> Result<Manga> {
        Ok(Manga::default())
    }

    fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
        Ok(Vec::new())
    }
}

impl ListingProvider for MoeTruyen {
    fn get_manga_list(&self, _listing: aidoku::Listing, _page: i32) -> Result<MangaPageResult> {
        Ok(MangaPageResult::default())
    }
}

register_source!(MoeTruyen, ListingProvider);

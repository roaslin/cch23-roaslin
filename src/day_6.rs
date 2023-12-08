use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ElfCounter {
    pub elf: usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    pub elf_on_a_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    pub shelf_with_no_elf_on_it: usize,
}

pub async fn count_elf(payload: String) -> (StatusCode, Json<ElfCounter>) {
    let elf: Vec<&str> = payload.matches("elf").collect();
    let elf_on_a_shelf: Vec<&str> = payload.matches("elf on a shelf").collect();
    let shelf_with_no_elf_on_it = payload.replace("elf on a shelf", "");
    let shelf_with_no_elf_on_it: Vec<&str> = shelf_with_no_elf_on_it.matches("shelf").collect();

    (
        StatusCode::OK,
        Json(ElfCounter {
            elf: elf.len(),
            elf_on_a_shelf: elf_on_a_shelf.len(),
            shelf_with_no_elf_on_it: shelf_with_no_elf_on_it.len(),
        }),
    )
}

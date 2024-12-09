use constants::COUNTRIES;
use definitions::Country;
mod constants;
mod definitions;
mod tests;
use std::collections::HashSet;

// return array of all country codes
pub fn iso_codes() -> Vec<&'static str> {
    COUNTRIES.iter().map(|country| country.iso_code).collect()
}

// return array of all country names
pub fn country_names() -> Vec<&'static str> {
    COUNTRIES
        .iter()
        .map(|country| country.iso_short_name)
        .collect()
}

pub fn by_iso_code(code: &str) -> Option<Country> {
    COUNTRIES
        .iter()
        .find(|country| country.iso_code == code)
        .map(|country| country.clone())
}

pub fn by_country_name(name: &str) -> Option<Country> {
    COUNTRIES
        .iter()
        .find(|country| country.iso_short_name == name)
        .map(|country| country.clone())
}

pub fn by_country_name_or_code(name_or_code: &str) -> Option<Country> {
    COUNTRIES
        .iter()
        .find(|country| country.iso_short_name == name_or_code || country.iso_code == name_or_code)
        .map(|country| country.clone())
}

pub fn by_country_name_or_code_case_insensitive(name_or_code: &str) -> Option<Country> {
    COUNTRIES
        .iter()
        .find(|country| {
            country.iso_short_name.to_lowercase() == name_or_code.to_lowercase()
                || country.iso_code.to_lowercase() == name_or_code.to_lowercase()
        })
        .map(|country| country.clone())
}

pub fn continents() -> Vec<&'static str> {
    let unique_continents: HashSet<&str> = COUNTRIES.iter().map(|country| country.continent).collect();
    unique_continents.into_iter().collect()
}

pub fn regions() -> Vec<&'static str> {
    let unique_regions: HashSet<&str> = COUNTRIES.iter().map(|country| country.region).collect();
    unique_regions.into_iter().collect()
}

pub fn by_region(region: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.region == region)
        .map(|country| country.clone())
        .collect()
}
pub fn by_continent(continent: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.continent == continent)
        .map(|country| country.clone())
        .collect()
}
pub fn by_subregion(subregion: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.subregion == subregion)
        .map(|country| country.clone())
        .collect()
}

pub fn by_region_or_subregion(region_or_subregion: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| {
            country.region == region_or_subregion || country.subregion == region_or_subregion
        })
        .map(|country| country.clone())
        .collect()
}

pub fn by_region_or_subregion_case_insensitive(region_or_subregion: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| {
            country.region.to_lowercase() == region_or_subregion.to_lowercase()
                || country.subregion.to_lowercase() == region_or_subregion.to_lowercase()
        })
        .map(|country| country.clone())
        .collect()
}

pub fn by_languages_official(language: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.languages_official.contains(&language))
        .map(|country| country.clone())
        .collect()
}

pub fn by_languages_spoken(language: &str) -> Vec<Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.languages_spoken.contains(&language))
        .map(|country| country.clone())
        .collect()
}

pub fn world_regions() -> Vec<&'static str> {
    COUNTRIES
        .iter()
        .map(|country| country.region)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect()
}

pub fn world_subregions() -> Vec<&'static str> {
    COUNTRIES
        .iter()
        .map(|country| country.subregion)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect()
}

//! **Nationify** is a Rust library designed for querying and managing country-related data.
//! It provides an intuitive interface for retrieving information such as ISO codes, names, regions, languages, and geographical data.

#![warn(missing_docs)]

use std::collections::HashSet;

mod constants;
mod definitions;

use constants::COUNTRIES;
#[cfg(feature = "phf")]
use constants::ISO_CODE_INDEX;
pub use definitions::{Bounds, Country, Geo, LatLng};

#[cfg(feature = "iso_code")]
/// Return list of all country codes
pub fn iso_codes() -> Vec<&'static str> {
    COUNTRIES.iter().map(|country| country.iso_code).collect()
}

#[cfg(feature = "iso_short_name")]
/// Return list of all country names
pub fn country_names() -> Vec<&'static str> {
    COUNTRIES
        .iter()
        .map(|country| country.iso_short_name)
        .collect()
}

#[cfg(all(feature = "phf", feature = "iso_code"))]
/// Find a country by its ISO code - using PHF for fast lookups.
pub fn by_iso_code(code: &str) -> Option<&'static Country> {
    ISO_CODE_INDEX.get(code).map(|&i| &COUNTRIES[i])
}

#[cfg(all(feature = "iso_code", not(feature = "phf")))]
/// Find a country by its ISO code - using a linear search.
pub fn by_iso_code(code: &str) -> Option<&'static Country> {
    COUNTRIES.iter().find(|country| country.iso_code == code)
}

#[cfg(feature = "iso_short_name")]
/// Find a country by its name.
pub fn by_country_name(name: &str) -> Option<&'static Country> {
    COUNTRIES
        .iter()
        .find(|country| country.iso_short_name == name)
}

#[cfg(all(feature = "iso_short_name", feature = "iso_code"))]
/// Find a country by its name or ISO code.
pub fn by_country_name_or_code(name_or_code: &str) -> Option<&'static Country> {
    COUNTRIES
        .iter()
        .find(|country| country.iso_short_name == name_or_code || country.iso_code == name_or_code)
}

#[cfg(all(feature = "iso_short_name", feature = "iso_code"))]
/// Find a country by its name or ISO code, case insensitive.
pub fn by_country_name_or_code_case_insensitive(name_or_code: &str) -> Option<&'static Country> {
    COUNTRIES.iter().find(|country| {
        country.iso_short_name.to_lowercase() == name_or_code.to_lowercase()
            || country.iso_code.to_lowercase() == name_or_code.to_lowercase()
    })
}

#[cfg(feature = "continent")]
/// Return list of all continents.
pub fn continents() -> Vec<&'static str> {
    let unique_continents: HashSet<&str> =
        COUNTRIES.iter().map(|country| country.continent).collect();
    unique_continents.into_iter().collect()
}

#[cfg(feature = "region")]
/// Return list of all regions.
pub fn regions() -> Vec<&'static str> {
    let unique_regions: HashSet<&str> = COUNTRIES.iter().map(|country| country.region).collect();
    unique_regions.into_iter().collect()
}

#[cfg(feature = "economic_unions")]
/// Return list of all economic unions.
pub fn economic_unions() -> Vec<&'static str> {
    let mut unions = HashSet::new();
    for country in COUNTRIES.iter() {
        for &union in country.economic_unions {
            unions.insert(union);
        }
    }
    unions.into_iter().collect()
}

#[cfg(feature = "region")]
/// Return list of all countries in a specific region.
pub fn by_region(region: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.region == region)
        .collect()
}

#[cfg(feature = "continent")]
/// Return list of all countries in a specific continent.
pub fn by_continent(continent: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.continent == continent)
        .collect()
}

#[cfg(feature = "subregion")]
/// Return list of all countries in a specific subregion.
pub fn by_subregion(subregion: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.subregion == subregion)
        .collect()
}

#[cfg(all(feature = "region", feature = "subregion"))]
/// Return list of all countries in a specific region or subregion.
pub fn by_region_or_subregion(region_or_subregion: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| {
            country.region == region_or_subregion || country.subregion == region_or_subregion
        })
        .collect()
}

#[cfg(all(feature = "region", feature = "subregion"))]
/// Return list of all countries in a specific region or subregion.
pub fn by_region_or_subregion_case_insensitive(region_or_subregion: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| {
            country.region.to_lowercase() == region_or_subregion.to_lowercase()
                || country.subregion.to_lowercase() == region_or_subregion.to_lowercase()
        })
        .collect()
}

#[cfg(feature = "economic_unions")]
/// Return list of all countries in a specific economic union.
pub fn by_economic_union(union: &str) -> Vec<&'static Country> {
    let union_lower = union.to_lowercase();
    COUNTRIES
        .iter()
        .filter(|country| {
            country
                .economic_unions
                .iter()
                .any(|&u| u.to_lowercase() == union_lower)
        })
        .collect()
}

#[cfg(feature = "languages_official")]
/// Return list of all countries with a specific official language.
pub fn by_languages_official(language: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.languages_official.contains(&language))
        .collect()
}

#[cfg(feature = "languages_spoken")]
/// Return list of all countries with a specific spoken language.
pub fn by_languages_spoken(language: &str) -> Vec<&'static Country> {
    COUNTRIES
        .iter()
        .filter(|country| country.languages_spoken.contains(&language))
        .collect()
}

#[cfg(feature = "region")]
/// Return list of all world regions.
pub fn world_regions() -> Vec<&'static str> {
    COUNTRIES
        .iter()
        .map(|country| country.region)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect()
}

#[cfg(feature = "subregion")]
/// Return list of all world subregions.
pub fn world_subregions() -> Vec<&'static str> {
    COUNTRIES
        .iter()
        .map(|country| country.subregion)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "iso_code")]
    // Test that the `iso_codes` function returns an array of all country codes
    #[test]
    pub fn test_iso_codes() {
        let codes = iso_codes();
        assert_eq!(codes.len(), 249);
    }

    #[cfg(feature = "continent")]
    // Test that the `continent` function returns an array of all continents
    #[test]
    pub fn test_continent() {
        let continents = continents();
        assert_eq!(continents.len(), 7);
    }

    #[cfg(all(feature = "iso_short_name", feature = "iso_code"))]
    // Test that the `by_iso_code` function returns a country by its ISO code
    #[test]
    pub fn test_by_iso_code() {
        let country = by_iso_code("US").unwrap();
        assert_eq!(country.iso_short_name, "United States");
    }

    #[cfg(all(feature = "iso_short_name", feature = "iso_code"))]
    // Test that the `by_country_name` function returns a country by its name
    #[test]
    pub fn test_by_country_name() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.iso_code, "US");
    }

    #[cfg(all(feature = "iso_short_name", feature = "iso_code"))]
    // Test that the `by_country_name_or_code` function returns a country by its name or code
    #[test]
    pub fn test_by_country_name_or_code() {
        let country = by_country_name_or_code("US").unwrap();
        assert_eq!(country.iso_short_name, "United States");
    }

    #[cfg(all(feature = "iso_short_name", feature = "iso_code"))]
    // Test that the `by_country_name_or_code_case_insensitive` function returns a country by its name or code
    #[test]
    pub fn test_by_country_name_or_code_case_insensitive() {
        let country = by_country_name_or_code_case_insensitive("us").unwrap();
        assert_eq!(country.iso_short_name, "United States");
    }

    #[cfg(feature = "region")]
    // Test that the `by_region` function returns an array of countries by region
    #[test]
    pub fn test_by_region() {
        let countries = by_region("Europe");
        assert_eq!(countries.len(), 51);
    }

    #[cfg(all(feature = "region", feature = "subregion"))]
    // Test that the `by_region_or_subregion` function returns an array of countries by region or subregion
    #[test]
    pub fn test_by_region_or_subregion() {
        let countries = by_region_or_subregion("Northern Europe");
        assert_eq!(countries.len(), 16);
    }

    #[cfg(all(feature = "region", feature = "subregion"))]
    // Test that the `by_region_or_subregion_case_insensitive` function returns an array of countries by region or subregion
    #[test]
    pub fn test_by_region_or_subregion_case_insensitive() {
        let countries = by_region_or_subregion_case_insensitive("northern europe");
        assert_eq!(countries.len(), 16);
    }

    #[cfg(feature = "subregion")]
    // Test that the `by_subregion` function returns an array of countries by subregion
    #[test]
    pub fn test_by_subregion() {
        let countries = by_subregion("Northern Europe");
        assert_eq!(countries.len(), 16);
    }

    #[cfg(feature = "iso_short_name")]
    // Test that the `country_names` function returns an array of all country names
    #[test]
    pub fn test_country_names() {
        let names = country_names();
        assert_eq!(names.len(), 249);
    }

    #[cfg(feature = "continent")]
    // Test that the `by_continent` function returns an array of countries by continent
    #[test]
    pub fn test_by_continent() {
        let countries = by_continent("Europe");
        assert_eq!(countries.len(), 52);
    }

    #[cfg(feature = "economic_unions")]
    // Test that the `economic_unions` function returns an array of all economic unions
    #[test]
    pub fn test_economic_unions() {
        let unions = economic_unions();
        assert!(!unions.is_empty());
        assert!(unions.contains(&"European Union"));
    }

    #[cfg(feature = "economic_unions")]
    // Test that the `by_economic_union` function returns an array of countries by economic union
    #[test]
    pub fn test_by_economic_union() {
        let countries = by_economic_union("European Union");
        assert!(!countries.is_empty());
        assert!(countries.iter().any(|c| c.iso_short_name == "Belgium"));
    }

    // testing country functions

    #[cfg(all(feature = "iso_short_name", feature = "languages_official"))]
    // Test that the `is_language_official` function returns true if a language is official
    #[test]
    pub fn test_is_language_official() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_language_official("en"));
        assert!(!country.is_language_official("ar"));
    }

    #[cfg(all(feature = "iso_short_name", feature = "languages_spoken"))]
    #[test]
    pub fn test_is_language_spoken() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_language_spoken("en"));
        assert!(!country.is_language_spoken("ar"));
    }

    #[cfg(all(
        feature = "iso_short_name",
        feature = "languages_official",
        feature = "languages_spoken"
    ))]
    #[test]
    pub fn test_has_language() {
        let country = by_country_name("United States").unwrap();
        assert!(country.has_language("en")); // Official language
        assert!(!country.has_language("ar")); // Official language
    }

    #[cfg(all(feature = "iso_short_name", feature = "unofficial_names"))]
    #[test]
    pub fn test_is_unofficial_name() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_unofficial_name("USA")); // Unofficial name
        assert!(!country.is_unofficial_name("America")); // Not unofficial
        assert!(country.is_unofficial_name("usa")); // Case insensitive
    }

    #[cfg(all(feature = "iso_short_name", feature = "unofficial_names"))]
    #[test]
    pub fn test_has_unofficial_name() {
        let country = by_country_name("United States").unwrap();
        assert!(country.has_unofficial_name("United States")); // Official name
        assert!(country.has_unofficial_name("USA")); // Unofficial name
    }

    #[cfg(all(feature = "iso_short_name", feature = "region"))]
    #[test]
    pub fn test_is_region() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_region("Americas")); // Matching region
        assert!(!country.is_region("Asia")); // Different region
        assert!(country.is_region("AMERICAS")); // Case insensitive
    }

    #[cfg(all(feature = "iso_short_name", feature = "subregion"))]
    #[test]
    pub fn test_is_subregion() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_subregion("Northern America")); // Matching subregion
        assert!(!country.is_subregion("Central America")); // Different subregion
        assert!(country.is_subregion("northern america")); // Case insensitive
    }

    #[cfg(all(feature = "iso_short_name", feature = "region", feature = "subregion"))]
    #[test]
    pub fn test_is_region_or_subregion() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_region_or_subregion("Americas")); // Region
        assert!(country.is_region_or_subregion("Northern America")); // Subregion
        assert!(!country.is_region_or_subregion("Asia")); // Different region
    }

    #[cfg(all(feature = "iso_short_name", feature = "region", feature = "subregion"))]
    #[test]
    pub fn test_is_region_or_subregion_case_insensitive() {
        let country = by_country_name("United States").unwrap();
        assert!(country.is_region_or_subregion_case_insensitive("americas")); // Region (case insensitive)
        assert!(country.is_region_or_subregion_case_insensitive("northern america")); // Subregion (case insensitive)
        assert!(!country.is_region_or_subregion_case_insensitive("asia")); // Different region
    }
}

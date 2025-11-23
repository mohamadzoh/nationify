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

#[cfg(feature = "ports")]
include!(concat!(env!("OUT_DIR"), "/ports.rs"));
#[cfg(feature = "ports")]
pub use definitions::Port;

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

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_all_ports() {
        let ports = all_ports();
        assert!(!ports.is_empty());
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_closest_port() {
        // Test with coordinates near Los Angeles
        let port = closest_port(34.05, -118.25).unwrap();
        // The closest port to these coordinates
        assert!(port.country == "United States" || port.country == "U.S.A.");
        assert!(port.latitude > 33.0 && port.latitude < 35.0);
        assert!(port.longitude > -119.0 && port.longitude < -117.0);
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_by_country() {
        let ports = ports_by_country("Denmark");
        assert!(!ports.is_empty());
        assert!(ports.iter().any(|p| p.city == "Aalborg"));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_closest_ports() {
        // Test with coordinates near Los Angeles
        let ports = closest_ports(34.05, -118.25, 5);
        assert_eq!(ports.len(), 5);
        // Verify we got results
        assert!(!ports.is_empty());
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_by_unloc() {
        let port = port_by_unloc("USNYC");
        assert!(port.is_some());
        if let Some(p) = port {
            assert!(p.country.to_lowercase().contains("united states") || p.country == "U.S.A.");
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_by_code() {
        let ports = all_ports();
        if let Some(port_with_code) = ports.iter().find(|p| !p.port_code.is_empty()) {
            let found = port_by_code(port_with_code.port_code);
            assert!(found.is_some());
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_by_city() {
        let ports = ports_by_city("Dubai");
        assert!(!ports.is_empty());
        assert!(ports.iter().all(|p| p.city.eq_ignore_ascii_case("Dubai")));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_by_timezone() {
        let ports = ports_by_timezone("Asia/Dubai");
        assert!(!ports.is_empty());
        assert!(ports.iter().all(|p| p.timezone == "Asia/Dubai"));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_timezones() {
        let timezones = port_timezones();
        assert!(!timezones.is_empty());
        assert!(timezones.contains(&"Asia/Dubai") || timezones.contains(&"America/New_York"));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_countries() {
        let countries = port_countries();
        assert!(!countries.is_empty());
        assert!(countries.len() > 50); // Should have many countries
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_cities() {
        let cities = port_cities();
        assert!(!cities.is_empty());
        assert!(cities.len() > 100); // Should have many cities
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_search_ports() {
        let results = search_ports("Dubai");
        assert!(!results.is_empty());
        assert!(results.iter().any(|p| p.city.to_lowercase().contains("dubai") || p.name.to_lowercase().contains("dubai")));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_within_radius() {
        // Dubai coordinates
        let ports = ports_within_radius(25.2048, 55.2708, 50.0);
        assert!(!ports.is_empty());
        // All ports should be within 50km
        assert!(ports.iter().all(|p| p.distance_to(25.2048, 55.2708) <= 50.0));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_in_bounding_box() {
        // Bounding box around UAE
        let ports = ports_in_bounding_box(24.0, 26.5, 51.0, 56.5);
        assert!(!ports.is_empty());
        assert!(ports.iter().any(|p| p.country == "United Arab Emirates"));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_count() {
        let count = ports_count();
        assert!(count > 0);
        assert_eq!(count, all_ports().len());
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_ports_count_by_country() {
        let count = ports_count_by_country("United Arab Emirates");
        assert!(count > 0);
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_statistics() {
        let stats = port_statistics();
        assert!(stats.total_ports > 0);
        assert!(stats.total_countries > 0);
        assert!(stats.total_cities > 0);
        assert!(stats.total_timezones > 0);
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_distance_conversions() {
        let ports = all_ports();
        if let Some(port) = ports.first() {
            let dist_km = port.distance_to(0.0, 0.0);
            let dist_nm = port.distance_to_nautical_miles(0.0, 0.0);
            let dist_mi = port.distance_to_miles(0.0, 0.0);
            
            // Verify conversions are reasonable
            assert!(dist_nm > 0.0);
            assert!(dist_mi > 0.0);
            assert!(dist_nm < dist_km); // nautical miles < kilometers
            assert!(dist_mi < dist_km); // miles < kilometers
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_has_unloc() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| !p.unlocs.is_empty()) {
            let unloc = port.unlocs[0];
            assert!(port.has_unloc(unloc));
            assert!(port.has_unloc(&unloc.to_lowercase())); // Case insensitive
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_display_name() {
        let ports = all_ports();
        if let Some(port) = ports.first() {
            let display = port.display_name();
            assert!(display.contains(port.name));
            assert!(display.contains(port.city));
            assert!(display.contains(port.country));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_search() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| p.name == "Dubai") {
            assert!(port.matches_search("Dubai"));
            assert!(port.matches_search("dubai")); // Case insensitive
            assert!(port.matches_search("dub")); // Partial match
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_code() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| !p.port_code.is_empty()) {
            let code = port.port_code;
            assert!(port.matches_code(code));
            assert!(port.matches_code(&code.to_lowercase()));
            assert!(port.matches_code(&code.to_uppercase()));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_city() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| p.city == "Dubai") {
            assert!(port.matches_city("Dubai"));
            assert!(port.matches_city("dubai"));
            assert!(port.matches_city("DUBAI"));
            assert!(!port.matches_city("Singapore"));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_country() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| p.country == "United Arab Emirates") {
            assert!(port.matches_country("United Arab Emirates"));
            assert!(port.matches_country("united arab emirates"));
            assert!(!port.matches_country("United States"));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_state() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| !p.state.is_empty()) {
            let state = port.state;
            assert!(port.matches_state(state));
            assert!(port.matches_state(&state.to_lowercase()));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_name() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| p.name == "Dubai") {
            assert!(port.matches_name("Dubai"));
            assert!(port.matches_name("dubai"));
            assert!(port.matches_name("DUBAI"));
            assert!(!port.matches_name("Abu Dhabi"));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_matches_timezone() {
        let ports = all_ports();
        if let Some(port) = ports.iter().find(|p| p.timezone == "Asia/Dubai") {
            assert!(port.matches_timezone("Asia/Dubai"));
            assert!(port.matches_timezone("asia/dubai"));
            assert!(!port.matches_timezone("America/New_York"));
        }
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_advanced_port_filtering() {
        let all = all_ports();
        
        // Filter by multiple criteria using new methods
        let results: Vec<_> = all
            .iter()
            .filter(|p| p.matches_country("United Arab Emirates") && p.matches_city("Dubai"))
            .collect();
        
        assert!(!results.is_empty());
        assert!(results.iter().all(|p| p.country == "United Arab Emirates"));
        assert!(results.iter().all(|p| p.city == "Dubai"));
    }

    #[cfg(feature = "ports")]
    #[test]
    pub fn test_port_search_by_code_comprehensive() {
        let all = all_ports();
        
        // Find ports with specific codes
        let with_code: Vec<_> = all
            .iter()
            .filter(|p| !p.port_code.is_empty())
            .take(5)
            .collect();
        
        for port in with_code {
            assert!(port.matches_code(port.port_code));
        }
    }
}

// Ports functionality

#[cfg(feature = "ports")]
/// Return list of all ports.
pub fn all_ports() -> Vec<&'static Port> {
    PORTS.iter().collect()
}

#[cfg(feature = "ports")]
/// Find the closest port to the given coordinates.
pub fn closest_port(latitude: f64, longitude: f64) -> Option<&'static Port> {
    PORTS
        .iter()
        .min_by(|a, b| {
            let dist_a = a.distance_to(latitude, longitude);
            let dist_b = b.distance_to(latitude, longitude);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
}

#[cfg(feature = "ports")]
/// Find the N closest ports to the given coordinates.
pub fn closest_ports(latitude: f64, longitude: f64, n: usize) -> Vec<&'static Port> {
    let mut ports_with_distances: Vec<_> = PORTS
        .iter()
        .map(|port| (port, port.distance_to(latitude, longitude)))
        .collect();
    
    ports_with_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    
    ports_with_distances
        .into_iter()
        .take(n)
        .map(|(port, _)| port)
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all ports in a specific country.
pub fn ports_by_country(country: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.country.eq_ignore_ascii_case(country))
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all ports in a specific city.
pub fn ports_by_city(city: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.city.eq_ignore_ascii_case(city))
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all ports matching a specific name.
pub fn ports_by_name(name: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.name.eq_ignore_ascii_case(name))
        .collect()
}

#[cfg(feature = "ports")]
/// Find a port by its UNLOC code.
pub fn port_by_unloc(unloc: &str) -> Option<&'static Port> {
    PORTS.iter().find(|port| port.has_unloc(unloc))
}

#[cfg(feature = "ports")]
/// Find a port by its port code.
pub fn port_by_code(code: &str) -> Option<&'static Port> {
    PORTS
        .iter()
        .find(|port| port.port_code.eq_ignore_ascii_case(code) || port.code.eq_ignore_ascii_case(code))
}

#[cfg(feature = "ports")]
/// Return list of all ports in a specific state/province.
pub fn ports_by_state(state: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.state.eq_ignore_ascii_case(state))
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all ports in a specific timezone.
pub fn ports_by_timezone(timezone: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.timezone.eq_ignore_ascii_case(timezone))
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all unique timezones across all ports.
pub fn port_timezones() -> Vec<&'static str> {
    use std::collections::HashSet;
    PORTS
        .iter()
        .map(|port| port.timezone)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all unique countries with ports.
pub fn port_countries() -> Vec<&'static str> {
    use std::collections::HashSet;
    PORTS
        .iter()
        .map(|port| port.country)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(feature = "ports")]
/// Return list of all unique cities with ports.
pub fn port_cities() -> Vec<&'static str> {
    use std::collections::HashSet;
    PORTS
        .iter()
        .map(|port| port.city)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(feature = "ports")]
/// Search ports by query string (searches name, city, code, unlocs).
pub fn search_ports(query: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.matches_search(query))
        .collect()
}

#[cfg(feature = "ports")]
/// Find ports within a radius (in kilometers) of given coordinates.
pub fn ports_within_radius(latitude: f64, longitude: f64, radius_km: f64) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.distance_to(latitude, longitude) <= radius_km)
        .collect()
}

#[cfg(feature = "ports")]
/// Find ports in a bounding box defined by min/max latitude and longitude.
pub fn ports_in_bounding_box(
    min_lat: f64,
    max_lat: f64,
    min_lng: f64,
    max_lng: f64,
) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| {
            port.latitude >= min_lat
                && port.latitude <= max_lat
                && port.longitude >= min_lng
                && port.longitude <= max_lng
        })
        .collect()
}

#[cfg(feature = "ports")]
/// Find all ports serving a specific region.
pub fn ports_by_region(region: &str) -> Vec<&'static Port> {
    PORTS
        .iter()
        .filter(|port| port.serves_region(region))
        .collect()
}

#[cfg(feature = "ports")]
/// Count the total number of ports.
pub fn ports_count() -> usize {
    PORTS.len()
}

#[cfg(feature = "ports")]
/// Count ports by country.
pub fn ports_count_by_country(country: &str) -> usize {
    PORTS
        .iter()
        .filter(|port| port.country.eq_ignore_ascii_case(country))
        .count()
}

#[cfg(feature = "ports")]
/// Get statistics about ports (total, countries, cities, timezones).
pub fn port_statistics() -> PortStatistics {
    use std::collections::HashSet;
    PortStatistics {
        total_ports: PORTS.len(),
        total_countries: PORTS.iter().map(|p| p.country).collect::<HashSet<_>>().len(),
        total_cities: PORTS.iter().map(|p| p.city).collect::<HashSet<_>>().len(),
        total_timezones: PORTS.iter().map(|p| p.timezone).collect::<HashSet<_>>().len(),
    }
}

#[cfg(feature = "ports")]
/// Statistics about the ports database.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PortStatistics {
    /// Total number of ports.
    pub total_ports: usize,
    /// Total number of countries with ports.
    pub total_countries: usize,
    /// Total number of cities with ports.
    pub total_cities: usize,
    /// Total number of timezones.
    pub total_timezones: usize,
}

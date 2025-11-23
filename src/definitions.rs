//! Definitions for geographic and country-related data structures.

/// Geographic coordinates.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Geo {
    /// The latitude of the geographic point.
    pub latitude: f64,
    /// The longitude of the geographic point.
    pub longitude: f64,
    /// The maximum latitude of the bounding box.
    pub max_latitude: f64,
    /// The maximum longitude of the bounding box.
    pub max_longitude: f64,
    /// The minimum latitude of the bounding box.
    pub min_latitude: f64,
    /// The minimum longitude of the bounding box.
    pub min_longitude: f64,
    /// The bounding box for the geographic point.
    pub bounds: Bounds,
}

/// Geographic bounding box.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Bounds {
    /// The northeast corner of the bounding box.
    pub northeast: LatLng,
    /// The southwest corner of the bounding box.
    pub southwest: LatLng,
}

/// Geographic coordinates.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LatLng {
    /// The latitude of the geographic point.
    pub lat: f64,
    /// The longitude of the geographic point.
    pub lng: f64,
}

/// Country information.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Country {
    #[cfg(feature = "iso_code")]
    /// The ISO 3166-1 alpha-2 code of the country.
    pub iso_code: &'static str,
    #[cfg(feature = "alpha3")]
    /// The ISO 3166-1 alpha-3 code of the country.
    pub alpha3: &'static str,
    #[cfg(feature = "continent")]
    /// The continent the country is located in.
    pub continent: &'static str,
    #[cfg(feature = "continent")]
    /// The continent code.
    pub continent_code: &'static str,
    #[cfg(feature = "country_code")]
    /// The country code.
    pub country_code: &'static str,
    #[cfg(feature = "currency_code")]
    /// The currency code used in the country.
    pub currency_code: &'static str,
    #[cfg(feature = "distance_unit")]
    /// The distance unit used in the country.
    pub distance_unit: &'static str,
    #[cfg(feature = "economic_unions")]
    /// The economic unions the country is part of.
    pub economic_unions: &'static [&'static str],
    #[cfg(feature = "gec")]
    /// The GEC (Geographic Encoding Class) code.
    pub gec: &'static str,
    #[cfg(feature = "geo")]
    /// The geographic information.
    pub geo: Geo,
    #[cfg(feature = "international_prefix")]
    /// The international dialing prefix.
    pub international_prefix: &'static str,
    #[cfg(feature = "ioc")]
    /// The IOC (International Olympic Committee) code.
    pub ioc: &'static str,
    #[cfg(feature = "iso_long_name")]
    /// The long name of the country in ISO format.
    pub iso_long_name: &'static str,
    #[cfg(feature = "iso_short_name")]
    /// The short name of the country in ISO format.
    pub iso_short_name: &'static str,
    #[cfg(feature = "languages_official")]
    /// The official languages of the country.
    pub languages_official: &'static [&'static str],
    #[cfg(feature = "languages_spoken")]
    /// The spoken languages in the country.
    pub languages_spoken: &'static [&'static str],
    #[cfg(feature = "national_destination_code_lengths")]
    /// The national destination code lengths.
    pub national_destination_code_lengths: &'static [u8],
    #[cfg(feature = "national_number_lengths")]
    /// The national number lengths.
    pub national_number_lengths: &'static [u8],
    #[cfg(feature = "national_prefix")]
    /// The national prefix.
    pub national_prefix: Option<&'static str>,
    #[cfg(feature = "nationality")]
    /// The nationality.
    pub nationality: &'static str,
    #[cfg(feature = "number")]
    /// The country number.
    pub number: &'static str,
    #[cfg(feature = "postal_code")]
    /// Whether the country has a postal code.
    pub postal_code: bool,
    #[cfg(feature = "postal_code_format")]
    /// The postal code format.
    pub postal_code_format: &'static str,
    #[cfg(feature = "region")]
    /// The region the country is located in.
    pub region: &'static str,
    #[cfg(feature = "start_of_week")]
    /// The start of the week in the country.
    pub start_of_week: &'static str,
    #[cfg(feature = "subregion")]
    /// The subregion the country is located in.
    pub subregion: &'static str,
    #[cfg(feature = "un_locode")]
    /// The UN/LOCODE of the country.
    pub un_locode: &'static str,
    #[cfg(feature = "unofficial_names")]
    /// The unofficial names of the country.
    pub unofficial_names: &'static [&'static str],
    #[cfg(feature = "world_region")]
    /// The world region the country is located in.
    pub world_region: &'static str,
}

/// Port information.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Port {
    /// The name of the port.
    pub name: &'static str,
    /// The city where the port is located.
    pub city: &'static str,
    /// The state or region where the port is located.
    pub state: &'static str,
    /// The country where the port is located.
    pub country: &'static str,
    /// The latitude of the port.
    pub latitude: f64,
    /// The longitude of the port.
    pub longitude: f64,
    /// The timezone of the port.
    pub timezone: &'static str,
    /// UN/LOCODE codes for the port.
    pub unlocs: &'static [&'static str],
    /// Port code.
    pub code: &'static str,
    /// Port UNLOC code.
    pub port_code: &'static str,
    /// Aliases for the port name.
    pub alias: &'static [&'static str],
    /// Regions the port serves.
    pub regions: &'static [&'static str],
}

impl Port {
    /// Calculate the Haversine distance in kilometers between this port and the given coordinates.
    pub fn distance_to(&self, latitude: f64, longitude: f64) -> f64 {
        haversine_distance(self.latitude, self.longitude, latitude, longitude)
    }

    /// Calculate the Haversine distance in nautical miles between this port and the given coordinates.
    pub fn distance_to_nautical_miles(&self, latitude: f64, longitude: f64) -> f64 {
        self.distance_to(latitude, longitude) * 0.539957
    }

    /// Calculate the Haversine distance in miles between this port and the given coordinates.
    pub fn distance_to_miles(&self, latitude: f64, longitude: f64) -> f64 {
        self.distance_to(latitude, longitude) * 0.621371
    }

    /// Check if the port has a specific UNLOC code.
    pub fn has_unloc(&self, unloc: &str) -> bool {
        self.unlocs.iter().any(|&u| u.eq_ignore_ascii_case(unloc))
    }

    /// Check if the port has any alias matching the given name.
    pub fn has_alias(&self, alias: &str) -> bool {
        self.alias.iter().any(|&a| a.eq_ignore_ascii_case(alias))
    }

    /// Check if the port serves a specific region.
    pub fn serves_region(&self, region: &str) -> bool {
        self.regions.iter().any(|&r| r.eq_ignore_ascii_case(region))
    }

    /// Get the full display name of the port (name, city, country).
    pub fn display_name(&self) -> String {
        format!("{}, {}, {}", self.name, self.city, self.country)
    }

    /// Check if the port matches a search query (searches name, city, code, unlocs).
    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.name.to_lowercase().contains(&query_lower)
            || self.city.to_lowercase().contains(&query_lower)
            || self.code.to_lowercase().contains(&query_lower)
            || self.port_code.to_lowercase().contains(&query_lower)
            || self.unlocs.iter().any(|&u| u.to_lowercase().contains(&query_lower))
    }

    /// Check if the port's code matches the given code (case-insensitive).
    pub fn matches_code(&self, code: &str) -> bool {
        self.code.eq_ignore_ascii_case(code) || self.port_code.eq_ignore_ascii_case(code)
    }

    /// Check if the port's city matches the given city (case-insensitive).
    pub fn matches_city(&self, city: &str) -> bool {
        self.city.eq_ignore_ascii_case(city)
    }

    /// Check if the port's country matches the given country (case-insensitive).
    pub fn matches_country(&self, country: &str) -> bool {
        self.country.eq_ignore_ascii_case(country)
    }

    /// Check if the port's state matches the given state (case-insensitive).
    pub fn matches_state(&self, state: &str) -> bool {
        self.state.eq_ignore_ascii_case(state)
    }

    /// Check if the port's name matches the given name (case-insensitive).
    pub fn matches_name(&self, name: &str) -> bool {
        self.name.eq_ignore_ascii_case(name)
    }

    /// Check if the port's timezone matches the given timezone (case-insensitive).
    pub fn matches_timezone(&self, timezone: &str) -> bool {
        self.timezone.eq_ignore_ascii_case(timezone)
    }
}

/// Calculate the Haversine distance in kilometers between two points on Earth.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}

impl Country {
    #[cfg(feature = "languages_official")]
    /// Checks if a language is an official language of the country.
    pub fn is_language_official(&self, language: &str) -> bool {
        let language_lower = language.to_lowercase();
        self.languages_official
            .iter()
            .any(|&lang| lang.to_lowercase() == language_lower)
    }

    #[cfg(feature = "languages_spoken")]
    /// Checks if a language is a spoken language of the country.
    pub fn is_language_spoken(&self, language: &str) -> bool {
        let language_lower = language.to_lowercase();
        self.languages_spoken
            .iter()
            .any(|&lang| lang.to_lowercase() == language_lower)
    }

    #[cfg(all(feature = "languages_official", feature = "languages_spoken"))]
    /// Checks if a language is either an official or spoken language of the country.
    pub fn has_language(&self, language: &str) -> bool {
        self.is_language_official(language) || self.is_language_spoken(language)
    }

    #[cfg(feature = "unofficial_names")]
    /// Checks if a name is an unofficial name of the country.
    pub fn is_unofficial_name(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.unofficial_names
            .iter()
            .any(|&n| n.to_lowercase() == name_lower)
    }

    #[cfg(all(feature = "iso_short_name", feature = "unofficial_names"))]
    /// Checks if a name is either the short name or an unofficial name of the country.
    pub fn has_unofficial_name(&self, name: &str) -> bool {
        self.iso_short_name.to_lowercase() == name.to_lowercase() || self.is_unofficial_name(name)
    }

    #[cfg(feature = "region")]
    /// Checks if a region is the region the country is located in.
    pub fn is_region(&self, region: &str) -> bool {
        self.region.to_lowercase() == region.to_lowercase()
    }

    #[cfg(feature = "subregion")]
    /// Checks if a subregion is the subregion the country is located in.
    pub fn is_subregion(&self, subregion: &str) -> bool {
        self.subregion.to_lowercase() == subregion.to_lowercase()
    }

    #[cfg(all(feature = "region", feature = "subregion"))]
    /// Checks if a region is either the region or subregion the country is located in.
    pub fn is_region_or_subregion(&self, region_or_subregion: &str) -> bool {
        self.is_region(region_or_subregion) || self.is_subregion(region_or_subregion)
    }

    #[cfg(all(feature = "region", feature = "subregion"))]
    /// Checks if a region is either the region or subregion the country is located in.
    pub fn is_region_or_subregion_case_insensitive(&self, region_or_subregion: &str) -> bool {
        self.region.to_lowercase() == region_or_subregion.to_lowercase()
            || self.subregion.to_lowercase() == region_or_subregion.to_lowercase()
    }

    #[cfg(feature = "economic_unions")]
    /// Checks if the country is part of a specific economic union.
    pub fn is_in_economic_union(&self, economic_union: &str) -> bool {
        self.economic_unions.contains(&economic_union)
    }

    #[cfg(feature = "economic_unions")]
    /// Checks if the country is part of a specific economic union.
    pub fn is_in_economic_union_case_insensitive(&self, economic_union: &str) -> bool {
        let economic_union_lower = economic_union.to_lowercase();
        self.economic_unions
            .iter()
            .any(|&u| u.to_lowercase() == economic_union_lower)
    }
}

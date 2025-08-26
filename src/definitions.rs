//! Definitions for geographic and country-related data structures.

/// Geographic coordinates.
#[derive(Debug, Clone, PartialEq)]
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
pub struct Bounds {
    /// The northeast corner of the bounding box.
    pub northeast: LatLng,
    /// The southwest corner of the bounding box.
    pub southwest: LatLng,
}

/// Geographic coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct LatLng {
    /// The latitude of the geographic point.
    pub lat: f64,
    /// The longitude of the geographic point.
    pub lng: f64,
}

/// Country information.
#[derive(Debug, Clone, PartialEq)]
pub struct Country {
    /// The ISO 3166-1 alpha-2 code of the country.
    pub iso_code: &'static str,
    /// The ISO 3166-1 alpha-3 code of the country.
    pub alpha3: &'static str,
    /// The continent the country is located in.
    pub continent: &'static str,
    /// The country code.
    pub country_code: &'static str,
    /// The currency code used in the country.
    pub currency_code: &'static str,
    /// The distance unit used in the country.
    pub distance_unit: &'static str,
    /// The GEC (Geographic Encoding Class) code.
    pub gec: &'static str,
    /// The geographic information.
    pub geo: Geo,
    /// The international dialing prefix.
    pub international_prefix: &'static str,
    /// The IOC (International Olympic Committee) code.
    pub ioc: &'static str,
    /// The long name of the country in ISO format.
    pub iso_long_name: &'static str,
    /// The short name of the country in ISO format.
    pub iso_short_name: &'static str,
    /// The official languages of the country.
    pub languages_official: &'static [&'static str],
    /// The spoken languages in the country.
    pub languages_spoken: &'static [&'static str],
    /// The national destination code lengths.
    pub national_destination_code_lengths: &'static [u8],
    /// The national number lengths.
    pub national_number_lengths: &'static [u8],
    /// The national prefix.
    pub national_prefix: Option<&'static str>,
    /// The nationality.
    pub nationality: &'static str,
    /// The country number.
    pub number: &'static str,
    /// Whether the country has a postal code.
    pub postal_code: bool,
    /// The postal code format.
    pub postal_code_format: &'static str,
    /// The region the country is located in.
    pub region: &'static str,
    /// The start of the week in the country.
    pub start_of_week: &'static str,
    /// The subregion the country is located in.
    pub subregion: &'static str,
    /// The UN/LOCODE of the country.
    pub un_locode: &'static str,
    /// The unofficial names of the country.
    pub unofficial_names: &'static [&'static str],
    /// The world region the country is located in.
    pub world_region: &'static str,
}

impl Country {
    /// Checks if a language is an official language of the country.
    pub fn is_language_official(&self, language: &str) -> bool {
        let language_lower = language.to_lowercase();
        self.languages_official
            .iter()
            .any(|&lang| lang.to_lowercase() == language_lower)
    }

    /// Checks if a language is a spoken language of the country.
    pub fn is_language_spoken(&self, language: &str) -> bool {
        let language_lower = language.to_lowercase();
        self.languages_spoken
            .iter()
            .any(|&lang| lang.to_lowercase() == language_lower)
    }

    /// Checks if a language is either an official or spoken language of the country.
    pub fn has_language(&self, language: &str) -> bool {
        self.is_language_official(language) || self.is_language_spoken(language)
    }

    /// Checks if a name is an unofficial name of the country.
    pub fn is_unofficial_name(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.unofficial_names
            .iter()
            .any(|&n| n.to_lowercase() == name_lower)
    }

    /// Checks if a name is either the short name or an unofficial name of the country.
    pub fn has_unofficial_name(&self, name: &str) -> bool {
        self.iso_short_name.to_lowercase() == name.to_lowercase() || self.is_unofficial_name(name)
    }

    /// Checks if a region is the region the country is located in.
    pub fn is_region(&self, region: &str) -> bool {
        self.region.to_lowercase() == region.to_lowercase()
    }

    /// Checks if a subregion is the subregion the country is located in.
    pub fn is_subregion(&self, subregion: &str) -> bool {
        self.subregion.to_lowercase() == subregion.to_lowercase()
    }

    /// Checks if a region is either the region or subregion the country is located in.
    pub fn is_region_or_subregion(&self, region_or_subregion: &str) -> bool {
        self.is_region(region_or_subregion) || self.is_subregion(region_or_subregion)
    }

    /// Checks if a region is either the region or subregion the country is located in.
    pub fn is_region_or_subregion_case_insensitive(&self, region_or_subregion: &str) -> bool {
        self.region.to_lowercase() == region_or_subregion.to_lowercase()
            || self.subregion.to_lowercase() == region_or_subregion.to_lowercase()
    }
}

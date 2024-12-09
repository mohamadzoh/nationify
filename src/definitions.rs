#[derive(Debug, Clone, PartialEq)]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
    pub max_latitude: f64,
    pub max_longitude: f64,
    pub min_latitude: f64,
    pub min_longitude: f64,
    pub bounds: Bounds,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub northeast: LatLng,
    pub southwest: LatLng,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Country<'a> {
    pub iso_code: &'a str,
    pub alpha3: &'a str,
    pub continent: &'a str,
    pub country_code: &'a str,
    pub currency_code: &'a str,
    pub distance_unit: &'a str,
    pub gec: &'a str,
    pub geo: Geo,
    pub international_prefix: &'a str,
    pub ioc: &'a str,
    pub iso_long_name: &'a str,
    pub iso_short_name: &'a str,
    pub languages_official: &'a [&'a str],
    pub languages_spoken: &'a [&'a str],
    pub national_destination_code_lengths: &'a [u8],
    pub national_number_lengths: &'a [u8],
    pub national_prefix: Option<&'a str>,
    pub nationality: &'a str,
    pub number: &'a str,
    pub postal_code: bool,
    pub postal_code_format: &'a str,
    pub region: &'a str,
    pub start_of_week: &'a str,
    pub subregion: &'a str,
    pub un_locode: &'a str,
    pub unofficial_names: &'a [&'a str],
    pub world_region: &'a str,
}



impl<'a> Country<'a> {
    pub fn is_language_official(&self, language: &str) -> bool {
        let language_lower = language.to_lowercase();
        self.languages_official.iter().any(|&lang| lang.to_lowercase() == language_lower)
    }
    pub fn is_language_spoken(&self, language: &str) -> bool {
        let language_lower = language.to_lowercase();
        self.languages_spoken.iter().any(|&lang| lang.to_lowercase() == language_lower)
    }

    pub fn has_language(&self, language: &str) -> bool {
        self.is_language_official(language) || self.is_language_spoken(language)
    }

    pub fn is_unofficial_name(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.unofficial_names.iter().any(|&n| n.to_lowercase() == name_lower)
    }

    pub fn has_unofficial_name(&self, name: &str) -> bool {
        self.iso_short_name.to_lowercase() == name.to_lowercase() || self.is_unofficial_name(name)
    }

    pub fn is_region(&self, region: &str) -> bool {
        self.region.to_lowercase() == region.to_lowercase()
    }

    pub fn is_subregion(&self, subregion: &str) -> bool {
        self.subregion.to_lowercase() == subregion.to_lowercase()
    }

    pub fn is_region_or_subregion(&self, region_or_subregion: &str) -> bool {
        self.is_region(region_or_subregion) || self.is_subregion(region_or_subregion)
    }

    pub fn is_region_or_subregion_case_insensitive(&self, region_or_subregion: &str) -> bool {
        self.region.to_lowercase() == region_or_subregion.to_lowercase() || self.subregion.to_lowercase() == region_or_subregion.to_lowercase()
    }
}
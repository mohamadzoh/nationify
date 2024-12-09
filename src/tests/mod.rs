#[cfg(test)]
mod tests {
    use crate::{
        by_continent, by_country_name, by_country_name_or_code,
        by_country_name_or_code_case_insensitive, by_iso_code, by_region, by_region_or_subregion,
        by_region_or_subregion_case_insensitive, by_subregion, continents, country_names,
        iso_codes,
    };
    // Test that the `iso_codes` function returns an array of all country codes
    #[test]
    pub fn test_iso_codes() {
        let codes = iso_codes();
        assert_eq!(codes.len(), 249);
    }

    // Test that the `continent` function returns an array of all continents
    #[test]
    pub fn test_continent() {
        let continents = continents();
        assert_eq!(continents.len(), 7);
    }

    // Test that the `by_iso_code` function returns a country by its ISO code
    #[test]
    pub fn test_by_iso_code() {
        let country = by_iso_code("US").unwrap();
        assert_eq!(country.iso_short_name, "United States");
    }

    // Test that the `by_country_name` function returns a country by its name
    #[test]
    pub fn test_by_country_name() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.iso_code, "US");
    }

    // Test that the `by_country_name_or_code` function returns a country by its name or code
    #[test]
    pub fn test_by_country_name_or_code() {
        let country = by_country_name_or_code("US").unwrap();
        assert_eq!(country.iso_short_name, "United States");
    }

    // Test that the `by_country_name_or_code_case_insensitive` function returns a country by its name or code
    #[test]
    pub fn test_by_country_name_or_code_case_insensitive() {
        let country = by_country_name_or_code_case_insensitive("us").unwrap();
        assert_eq!(country.iso_short_name, "United States");
    }

    // Test that the `by_region` function returns an array of countries by region
    #[test]
    pub fn test_by_region() {
        let countries = by_region("Europe");
        assert_eq!(countries.len(), 51);
    }

    // Test that the `by_region_or_subregion` function returns an array of countries by region or subregion
    #[test]
    pub fn test_by_region_or_subregion() {
        let countries = by_region_or_subregion("Northern Europe");
        assert_eq!(countries.len(), 16);
    }

    // Test that the `by_region_or_subregion_case_insensitive` function returns an array of countries by region or subregion
    #[test]
    pub fn test_by_region_or_subregion_case_insensitive() {
        let countries = by_region_or_subregion_case_insensitive("northern europe");
        assert_eq!(countries.len(), 16);
    }

    // Test that the `by_subregion` function returns an array of countries by subregion
    #[test]
    pub fn test_by_subregion() {
        let countries = by_subregion("Northern Europe");
        assert_eq!(countries.len(), 16);
    }

    // Test that the `country_names` function returns an array of all country names
    #[test]
    pub fn test_country_names() {
        let names = country_names();
        assert_eq!(names.len(), 249);
    }

    // Test that the `by_continent` function returns an array of countries by continent
    #[test]
    pub fn test_by_continent() {
        let countries = by_continent("Europe");
        assert_eq!(countries.len(), 52);
    }




    // testing country functions

    // Test that the `is_language_official` function returns true if a language is official
    #[test]
    pub fn test_is_language_official() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_language_official("en"), true);
        assert_eq!(country.is_language_official("ar"), false);
    }

    #[test]
    pub fn test_is_language_spoken() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_language_spoken("en"), true);
        assert_eq!(country.is_language_spoken("ar"), false);
    }

    #[test]
    pub fn test_has_language() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.has_language("en"), true); // Official language
        assert_eq!(country.has_language("ar"), false); // Official language
    }

    #[test]
    pub fn test_is_unofficial_name() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_unofficial_name("USA"), true); // Unofficial name
        assert_eq!(country.is_unofficial_name("America"), false); // Not unofficial
        assert_eq!(country.is_unofficial_name("usa"), true); // Case insensitive
    }

    #[test]
    pub fn test_has_unofficial_name() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.has_unofficial_name("United States"), true); // Official name
        assert_eq!(country.has_unofficial_name("USA"), true); // Unofficial name
    }

    #[test]
    pub fn test_is_region() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_region("Americas"), true); // Matching region
        assert_eq!(country.is_region("Asia"), false); // Different region
        assert_eq!(country.is_region("AMERICAS"), true); // Case insensitive
    }

    #[test]
    pub fn test_is_subregion() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_subregion("Northern America"), true); // Matching subregion
        assert_eq!(country.is_subregion("Central America"), false); // Different subregion
        assert_eq!(country.is_subregion("northern america"), true); // Case insensitive
    }

    #[test]
    pub fn test_is_region_or_subregion() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_region_or_subregion("Americas"), true); // Region
        assert_eq!(country.is_region_or_subregion("Northern America"), true); // Subregion
        assert_eq!(country.is_region_or_subregion("Asia"), false); // Different region
    }

    #[test]
    pub fn test_is_region_or_subregion_case_insensitive() {
        let country = by_country_name("United States").unwrap();
        assert_eq!(country.is_region_or_subregion_case_insensitive("americas"), true); // Region (case insensitive)
        assert_eq!(country.is_region_or_subregion_case_insensitive("northern america"), true); // Subregion (case insensitive)
        assert_eq!(country.is_region_or_subregion_case_insensitive("asia"), false); // Different region
    }

}

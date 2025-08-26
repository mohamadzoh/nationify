
# Nationify

**Nationify** is a Rust library designed for querying and managing country-related data. It provides an intuitive interface for retrieving information such as ISO codes, names, regions, languages, and geographical data.

---

## Installation

Add `nationify` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
nationify = "0.1.2"
```

---

## Crate Features

The `nationify` crate provides the following features - by default all are enabled:

- **serde** - Enables Serde serialization of definitions.
- **phf** - Enables compile-time country lookup by ISO code.
- **iso_code** - The ISO 3166-1 alpha-2 code of the country.
- **alpha3** - The ISO 3166-1 alpha-3 code of the country.
- **continent** - The continent the country is located in.
- **country_code** - The country code.
- **currency_code** - The currency code used in the country.
- **distance_unit** - The distance unit used in the country.
- **gec** - The GEC (Geographic Encoding Class) code.
- **geo** - The geographic information.
- **international_prefix** - The international dialing prefix.
- **ioc** - The IOC (International Olympic Committee) code.
- **iso_long_name** - The long name of the country in ISO format.
- **iso_short_name** - The short name of the country in ISO format.
- **languages_official** - The official languages of the country.
- **languages_spoken** - The spoken languages in the country.
- **national_destination_code_lengths** - The national destination code lengths.
- **national_number_lengths** - The national number lengths.
- **national_prefix** - The national prefix.
- **nationality** - The nationality.
- **number** - The country number.
- **postal_code** - Whether the country has a postal code.
- **postal_code_format** - The postal code format.
- **region** - The region the country is located in.
- **start_of_week** - The start of the week in the country.
- **subregion** - The subregion the country is located in.
- **un_locode** - The UN/LOCODE of the country.
- **unofficial_names** - The unofficial names of the country.
- **world_region** - The world region the country is located in.

---

## Features & Examples

### 1. Retrieve All ISO Codes

Get a list of all country ISO codes.

```rust
use nationify::iso_codes;

fn main() {
    let codes = iso_codes();
    println!("ISO Codes: {:?}", codes);
}
```

---

### 2. Retrieve All Country Names

Fetch all country names.

```rust
use nationify::country_names;

fn main() {
    let names = country_names();
    println!("Country Names: {:?}", names);
}
```

---

### 3. Search by ISO Code or Country Name

Find a country by its ISO code or name.

```rust
use nationify::{by_iso_code, by_country_name};

fn main() {
    if let Some(country) = by_iso_code("US") {
        println!("Country by ISO Code: {:?}", country);
    }

    if let Some(country) = by_country_name("United States") {
        println!("Country by Name: {:?}", country);
    }
}
```

---

### 4. Case-Insensitive Search

Perform case-insensitive searches for countries.

```rust
use nationify::by_country_name_or_code_case_insensitive;

fn main() {
    if let Some(country) = by_country_name_or_code_case_insensitive("united states") {
        println!("Case-Insensitive Search: {:?}", country);
    }
}
```

---

### 5. Retrieve Continents, Regions, and Subregions

Fetch a list of unique continents, regions, or subregions.

```rust
use nationify::{continents, regions, world_subregions};

fn main() {
    println!("Continents: {:?}", continents());
    println!("Regions: {:?}", regions());
    println!("Subregions: {:?}", world_subregions());
}
```

---

### 6. Filter by Region, Continent, or Subregion

Filter countries by specific geographical areas.

```rust
use nationify::{by_region, by_continent, by_subregion};

fn main() {
    let african_countries = by_region("Africa");
    println!("Countries in Africa: {:?}", african_countries);

    let asian_countries = by_continent("Asia");
    println!("Countries in Asia: {:?}", asian_countries);

    let subregion_countries = by_subregion("Northern Europe");
    println!("Countries in Northern Europe: {:?}", subregion_countries);
}
```

---

### 7. Query by Languages

Search countries based on official or spoken languages.

```rust
use nationify::{by_languages_official, by_languages_spoken};

fn main() {
    let english_official = by_languages_official("English");
    println!("Countries where English is an official language: {:?}", english_official);

    let english_spoken = by_languages_spoken("English");
    println!("Countries where English is spoken: {:?}", english_spoken);
}
```

---

### 8. Additional Country Metadata

Access detailed metadata for countries.

```rust
use nationify::by_country_name;

fn main() {
    if let Some(country) = by_country_name("United States") {
        println!("ISO Code: {}", country.iso_code);
        println!("Continent: {}", country.continent);
        println!("Region: {}", country.region);
        println!("Subregion: {}", country.subregion);
        println!("Official Languages: {:?}", country.languages_official);
        println!("Spoken Languages: {:?}", country.languages_spoken);
    }
}
```

---

## Struct Definitions

### `Country`

The `Country` struct provides comprehensive details for each country.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Country<'a> {
    pub iso_code: &'a str,
    pub alpha3: &'a str,
    pub continent: &'a str,
    pub region: &'a str,
    pub subregion: &'a str,
    pub languages_official: &'a [&'a str],
    pub languages_spoken: &'a [&'a str],
    pub geo: Geo,
    // ... additional fields
}
```

### `Geo` and `Bounds`

Geographical data includes latitude, longitude, and boundary information.

---

## Rusty Rails Project

Rusty Rails is a larger project aiming to bridge the gap between Rust and Ruby/Ruby on Rails. We are actively working on recreating ruby library into rust that seamlessly make working in rust more easy and fun for new developers.

### Contributing

Contributions to the Nationify library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.

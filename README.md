
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

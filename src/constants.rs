use crate::definitions::Country;
#[cfg(feature = "geo")]
use crate::definitions::{Bounds, Geo, LatLng};

pub static COUNTRIES: &[Country] = &[
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AD",
        #[cfg(feature = "alpha3")]
        alpha3: "AND",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "376",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 42.506285,
            longitude: 1.521801,
            max_latitude: 42.655791,
            max_longitude: 1.786639,
            min_latitude: 42.4287488,
            min_longitude: 1.4087052,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 42.655791,
                    lng: 1.786639,
                },
                southwest: LatLng {
                    lat: 42.4287488,
                    lng: 1.4087052,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "AND",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Principality of Andorra",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Andorra",
        #[cfg(feature = "languages_official")]
        languages_official: &["ca"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ca"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Andorran",
        #[cfg(feature = "number")]
        number: "020",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"AD[1-7]0\d",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "AD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Andorre", "Andorra", "アンドラ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AE",
        #[cfg(feature = "alpha3")]
        alpha3: "ARE",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "971",
        #[cfg(feature = "currency_code")]
        currency_code: "AED",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 23.424076,
            longitude: 53.847818,
            max_latitude: 26.0765,
            max_longitude: 56.4395001,
            min_latitude: 22.6315138,
            min_longitude: 51.4723,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 26.0765,
                    lng: 56.4395001,
                },
                southwest: LatLng {
                    lat: 22.6315138,
                    lng: 51.4723,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "UAE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The United Arab Emirates",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "United Arab Emirates",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Emirian",
        #[cfg(feature = "number")]
        number: "784",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "AE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "United Arab Emirates",
            "الإمارات العربية المتحدة",
            "Vereinigte Arabische Emirate",
            "Émirats Arabes Unis",
            "Emiratos Árabes Unidos",
            "アラブ首長国連邦",
            "Verenigde Arabische Emiraten",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AF",
        #[cfg(feature = "alpha3")]
        alpha3: "AFG",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "93",
        #[cfg(feature = "currency_code")]
        currency_code: "AFN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 33.93911,
            longitude: 67.709953,
            max_latitude: 38.49087670000001,
            max_longitude: 74.8898619,
            min_latitude: 29.3772,
            min_longitude: 60.5170005,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 38.49087670000001,
                    lng: 74.8898619,
                },
                southwest: LatLng {
                    lat: 29.3772,
                    lng: 60.5170005,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "AFG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Islamic Republic of Afghanistan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Afghanistan",
        #[cfg(feature = "languages_official")]
        languages_official: &["ps", "uz", "tk"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ps", "uz", "tk"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Afghan",
        #[cfg(feature = "number")]
        number: "004",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "AF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Afghanistan", "Afganistán", "アフガニスタン"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AG",
        #[cfg(feature = "alpha3")]
        alpha3: "ATG",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 17.060816,
            longitude: -61.796428,
            max_latitude: 17.7499946,
            max_longitude: -61.6394,
            min_latitude: 16.9018,
            min_longitude: -62.38100009999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.7499946,
                    lng: -61.6394,
                },
                southwest: LatLng {
                    lat: 16.9018,
                    lng: -62.38100009999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "ANT",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Antigua and Barbuda",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Antigua and Barbuda",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Antiguan, Barbudan",
        #[cfg(feature = "number")]
        number: "028",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "AG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Antigua and Barbuda",
            "Antigua und Barbuda",
            "Antigua et Barbuda",
            "Antigua y Barbuda",
            "アンティグア・バーブーダ",
            "Antigua en Barbuda",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AI",
        #[cfg(feature = "alpha3")]
        alpha3: "AIA",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.220554,
            longitude: -63.06861499999999,
            max_latitude: 18.6332326,
            max_longitude: -62.91999999999999,
            min_latitude: 18.1465043,
            min_longitude: -63.4803,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.6332326,
                    lng: -62.91999999999999,
                },
                southwest: LatLng {
                    lat: 18.1465043,
                    lng: -63.4803,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Anguilla",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Anguilla",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Anguillian",
        #[cfg(feature = "number")]
        number: "660",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:AI-)?2640",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "AI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Anguilla", "アンギラ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AL",
        #[cfg(feature = "alpha3")]
        alpha3: "ALB",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "355",
        #[cfg(feature = "currency_code")]
        currency_code: "ALL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 41.153332,
            longitude: 20.168331,
            max_latitude: 42.6611669,
            max_longitude: 21.0572394,
            min_latitude: 39.6447296,
            min_longitude: 19.1217,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 42.6611669,
                    lng: 21.0572394,
                },
                southwest: LatLng {
                    lat: 39.6447296,
                    lng: 19.1217,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ALB",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Albania",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Albania",
        #[cfg(feature = "languages_official")]
        languages_official: &["sq"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sq"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Albanian",
        #[cfg(feature = "number")]
        number: "008",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "AL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Albania", "Albanien", "Albanie", "アルバニア", "Albanië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AM",
        #[cfg(feature = "alpha3")]
        alpha3: "ARM",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "374",
        #[cfg(feature = "currency_code")]
        currency_code: "AMD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 40.069099,
            longitude: 45.038189,
            max_latitude: 41.300993,
            max_longitude: 46.6342219,
            min_latitude: 38.840244,
            min_longitude: 43.4472601,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 41.300993,
                    lng: 46.6342219,
                },
                southwest: LatLng {
                    lat: 38.840244,
                    lng: 43.4472601,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ARM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Armenia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Armenia",
        #[cfg(feature = "languages_official")]
        languages_official: &["hy"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["hy", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Armenian",
        #[cfg(feature = "number")]
        number: "051",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:37)?\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "AM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Armenia", "Armenien", "Arménie", "アルメニア", "Armenië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AO",
        #[cfg(feature = "alpha3")]
        alpha3: "AGO",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "244",
        #[cfg(feature = "currency_code")]
        currency_code: "AOA",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -11.202692,
            longitude: 17.873887,
            max_latitude: -4.388063300000001,
            max_longitude: 24.0878855,
            min_latitude: -18.0391039,
            min_longitude: 11.4696999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -4.388063300000001,
                    lng: 24.0878855,
                },
                southwest: LatLng {
                    lat: -18.0391039,
                    lng: 11.4696999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ANG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Angola",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Angola",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Angolan",
        #[cfg(feature = "number")]
        number: "024",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "AO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Angola", "アンゴラ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AQ",
        #[cfg(feature = "alpha3")]
        alpha3: "ATA",
        #[cfg(feature = "continent")]
        continent: "Antarctica",
        #[cfg(feature = "country_code")]
        country_code: "672",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -82.862752,
            longitude: 135.0,
            max_latitude: -60.1086999,
            max_longitude: 180.0,
            min_latitude: -90.0,
            min_longitude: -180.0,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -60.1086999,
                    lng: 180.0,
                },
                southwest: LatLng {
                    lat: -90.0,
                    lng: -180.0,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Antarctica",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Antarctica",
        #[cfg(feature = "languages_official")]
        languages_official: &[],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &[],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "",
        #[cfg(feature = "number")]
        number: "010",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "",
        #[cfg(feature = "un_locode")]
        un_locode: "AQ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Antarctica",
            "Antarktis",
            "Antarctique",
            "Antártida",
            "南極",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AR",
        #[cfg(feature = "alpha3")]
        alpha3: "ARG",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "54",
        #[cfg(feature = "currency_code")]
        currency_code: "ARS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -38.416097,
            longitude: -63.61667199999999,
            max_latitude: -21.7810459,
            max_longitude: -53.637481,
            min_latitude: -55.1250224,
            min_longitude: -73.5603601,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -21.7810459,
                    lng: -53.637481,
                },
                southwest: LatLng {
                    lat: -55.1250224,
                    lng: -73.5603601,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ARG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Argentine Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Argentina",
        #[cfg(feature = "languages_official")]
        languages_official: &["es", "gn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "gn", "qu"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Argentinean",
        #[cfg(feature = "number")]
        number: "032",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"((?:[A-HJ-NP-Z])?\d{4})([A-Z]{3})?",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "AR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Argentina",
            "Argentinien",
            "Argentine",
            "アルゼンチン",
            "Argentinië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AS",
        #[cfg(feature = "alpha3")]
        alpha3: "ASM",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "AQ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -14.270972,
            longitude: -170.132217,
            max_latitude: -13.4056506,
            max_longitude: -169.2059326,
            min_latitude: -14.7217608,
            min_longitude: -171.0076904,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -13.4056506,
                    lng: -169.2059326,
                },
                southwest: LatLng {
                    lat: -14.7217608,
                    lng: -171.0076904,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "ASA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of American Samoa",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "American Samoa",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "sm"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "sm"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "American Samoan",
        #[cfg(feature = "number")]
        number: "016",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(96799)(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "AS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "American Samoa",
            "Amerikanisch-Samoa",
            "Samoa américaines",
            "Samoa Americana",
            "アメリカ領サモア",
            "Amerikaans Samoa",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AT",
        #[cfg(feature = "alpha3")]
        alpha3: "AUT",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "43",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 47.516231,
            longitude: 14.550072,
            max_latitude: 49.0206081,
            max_longitude: 17.1607329,
            min_latitude: 46.37233579999999,
            min_longitude: 9.530783399999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 49.0206081,
                    lng: 17.1607329,
                },
                southwest: LatLng {
                    lat: 46.37233579999999,
                    lng: 9.530783399999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "AUT",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Austria",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Austria",
        #[cfg(feature = "languages_official")]
        languages_official: &["de"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["de"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[1, 2, 3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9, 10, 11, 12, 13],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Austrian",
        #[cfg(feature = "number")]
        number: "040",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "AT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Austria",
            "Österreich",
            "Autriche",
            "オーストリア",
            "Oostenrijk",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AU",
        #[cfg(feature = "alpha3")]
        alpha3: "AUS",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "61",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -25.274398,
            longitude: 133.775136,
            max_latitude: -9.187026399999999,
            max_longitude: 159.2872223,
            min_latitude: -54.83376579999999,
            min_longitude: 110.9510339,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -9.187026399999999,
                    lng: 159.2872223,
                },
                southwest: LatLng {
                    lat: -54.83376579999999,
                    lng: 110.9510339,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "0011",
        #[cfg(feature = "ioc")]
        ioc: "AUS",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Commonwealth of Australia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Australia",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Australian",
        #[cfg(feature = "number")]
        number: "036",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Australia and New Zealand",
        #[cfg(feature = "un_locode")]
        un_locode: "AU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Australia",
            "Australien",
            "Australie",
            "オーストラリア",
            "Australië",
            "澳洲",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AW",
        #[cfg(feature = "alpha3")]
        alpha3: "ABW",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "297",
        #[cfg(feature = "currency_code")]
        currency_code: "AWG",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.52111,
            longitude: -69.968338,
            max_latitude: 12.6306179,
            max_longitude: -69.8644638,
            min_latitude: 12.406093,
            min_longitude: -70.070114,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.6306179,
                    lng: -69.8644638,
                },
                southwest: LatLng {
                    lat: 12.406093,
                    lng: -70.070114,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ARU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Aruba",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Aruba",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Aruban",
        #[cfg(feature = "number")]
        number: "533",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "AW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Aruba", "アルバ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AX",
        #[cfg(feature = "alpha3")]
        alpha3: "ALA",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "358",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 60.1785247,
            longitude: 19.9156105,
            max_latitude: 60.8400009,
            max_longitude: 21.4866841,
            min_latitude: 59.6872001,
            min_longitude: 19.2095998,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 60.8400009,
                    lng: 21.4866841,
                },
                southwest: LatLng {
                    lat: 59.6872001,
                    lng: 19.2095998,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Åland",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Åland Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["sv"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sv"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Swedish",
        #[cfg(feature = "number")]
        number: "248",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"22\d{3}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "AX",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Åland Islands", "Åland", "オーランド諸島", "Ålandeilanden"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "AZ",
        #[cfg(feature = "alpha3")]
        alpha3: "AZE",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "994",
        #[cfg(feature = "currency_code")]
        currency_code: "AZN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AJ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 40.143105,
            longitude: 47.576927,
            max_latitude: 41.9594999,
            max_longitude: 50.7458001,
            min_latitude: 38.3922171,
            min_longitude: 44.7632599,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 41.9594999,
                    lng: 50.7458001,
                },
                southwest: LatLng {
                    lat: 38.3922171,
                    lng: 44.7632599,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "AZE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Azerbaijan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Azerbaijan",
        #[cfg(feature = "languages_official")]
        languages_official: &["az", "hy"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["az", "hy"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Azerbaijani",
        #[cfg(feature = "number")]
        number: "031",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "AZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Azerbaijan",
            "Aserbaidschan",
            "Azerbaïdjan",
            "Azerbaiyán",
            "アゼルバイジャン",
            "Azerbeidzjan",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BA",
        #[cfg(feature = "alpha3")]
        alpha3: "BIH",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "387",
        #[cfg(feature = "currency_code")]
        currency_code: "BAM",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 43.915886,
            longitude: 17.679076,
            max_latitude: 45.2766262,
            max_longitude: 19.6237016,
            min_latitude: 42.5564808,
            min_longitude: 15.7223665,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 45.2766262,
                    lng: 19.6237016,
                },
                southwest: LatLng {
                    lat: 42.5564808,
                    lng: 15.7223665,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BIH",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Bosnia and Herzegovina",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bosnia and Herzegovina",
        #[cfg(feature = "languages_official")]
        languages_official: &["bs", "hr", "sr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["bs", "hr", "sr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Bosnian, Herzegovinian",
        #[cfg(feature = "number")]
        number: "070",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "BA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Bosnia and Herzegovina",
            "Bosnien und Herzegowina",
            "Bosnie et Herzégovine",
            "Bosnia y Herzegovina",
            "ボスニア・ヘルツェゴビナ",
            "Bosnië en Herzegovina",
            "Bosnia Herzegovina",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BB",
        #[cfg(feature = "alpha3")]
        alpha3: "BRB",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "BBD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BB",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 13.193887,
            longitude: -59.543198,
            max_latitude: 13.3365093,
            max_longitude: -59.4174957,
            min_latitude: 13.039844,
            min_longitude: -59.6530151,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.3365093,
                    lng: -59.4174957,
                },
                southwest: LatLng {
                    lat: 13.039844,
                    lng: -59.6530151,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "BAR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Barbados",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Barbados",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Barbadian",
        #[cfg(feature = "number")]
        number: "052",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"BB\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "BB",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Barbade", "Barbados", "バルバドス"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BD",
        #[cfg(feature = "alpha3")]
        alpha3: "BGD",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "880",
        #[cfg(feature = "currency_code")]
        currency_code: "BDT",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 23.684994,
            longitude: 90.356331,
            max_latitude: 26.633914,
            max_longitude: 92.6801153,
            min_latitude: 20.3794,
            min_longitude: 88.00861410000002,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 26.633914,
                    lng: 92.6801153,
                },
                southwest: LatLng {
                    lat: 20.3794,
                    lng: 88.00861410000002,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BAN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The People's Republic of Bangladesh",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bangladesh",
        #[cfg(feature = "languages_official")]
        languages_official: &["bn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["bn"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Bangladeshi",
        #[cfg(feature = "number")]
        number: "050",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "BD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Bangladesh", "Bangladesch", "バングラデシュ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BE",
        #[cfg(feature = "alpha3")]
        alpha3: "BEL",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "32",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 50.503887,
            longitude: 4.469936,
            max_latitude: 51.5051449,
            max_longitude: 6.408124099999999,
            min_latitude: 49.497013,
            min_longitude: 2.5240999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 51.5051449,
                    lng: 6.408124099999999,
                },
                southwest: LatLng {
                    lat: 49.497013,
                    lng: 2.5240999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BEL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Belgium",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Belgium",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl", "fr", "de"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl", "fr", "de"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Belgian",
        #[cfg(feature = "number")]
        number: "056",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "BE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Belgium",
            "Belgien",
            "Belgique",
            "Bélgica",
            "ベルギー",
            "België",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BF",
        #[cfg(feature = "alpha3")]
        alpha3: "BFA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "226",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "UV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.238333,
            longitude: -1.561593,
            max_latitude: 15.0840397,
            max_longitude: 2.4043596,
            min_latitude: 9.4104717,
            min_longitude: -5.5132416,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 15.0840397,
                    lng: 2.4043596,
                },
                southwest: LatLng {
                    lat: 9.4104717,
                    lng: -5.5132416,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BUR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Burkina Faso",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Burkina Faso",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "ff"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "ff"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Burkinabe",
        #[cfg(feature = "number")]
        number: "854",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "BF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Burkina Faso", "ブルキナファソ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BG",
        #[cfg(feature = "alpha3")]
        alpha3: "BGR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "359",
        #[cfg(feature = "currency_code")]
        currency_code: "BGN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 42.733883,
            longitude: 25.48583,
            max_latitude: 44.2153059,
            max_longitude: 28.7292001,
            min_latitude: 41.2354469,
            min_longitude: 22.3573446,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 44.2153059,
                    lng: 28.7292001,
                },
                southwest: LatLng {
                    lat: 41.2354469,
                    lng: 22.3573446,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BUL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Bulgaria",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bulgaria",
        #[cfg(feature = "languages_official")]
        languages_official: &["bg"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["bg"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Bulgarian",
        #[cfg(feature = "number")]
        number: "100",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "BG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Bulgaria",
            "България",
            "Bulgarien",
            "Bulgarie",
            "ブルガリア",
            "Bulgarije",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BH",
        #[cfg(feature = "alpha3")]
        alpha3: "BHR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "973",
        #[cfg(feature = "currency_code")]
        currency_code: "BHD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 26.0667,
            longitude: 50.5577,
            max_latitude: 26.3469001,
            max_longitude: 50.8509064,
            min_latitude: 25.5349999,
            min_longitude: 50.324246,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 26.3469001,
                    lng: 50.8509064,
                },
                southwest: LatLng {
                    lat: 25.5349999,
                    lng: 50.324246,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BRN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Bahrain",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bahrain",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Bahraini",
        #[cfg(feature = "number")]
        number: "048",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:\d|1[0-2])\d{2}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "BH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Bahrain", "البحرين", "Bahreïn", "Bahrein", "バーレーン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BI",
        #[cfg(feature = "alpha3")]
        alpha3: "BDI",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "257",
        #[cfg(feature = "currency_code")]
        currency_code: "BIF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -3.373056,
            longitude: 29.918886,
            max_latitude: -2.3097301,
            max_longitude: 30.84954,
            min_latitude: -4.4693288,
            min_longitude: 29.000968,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -2.3097301,
                    lng: 30.84954,
                },
                southwest: LatLng {
                    lat: -4.4693288,
                    lng: 29.000968,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BDI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Burundi",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Burundi",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "rn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "rn"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Burundian",
        #[cfg(feature = "number")]
        number: "108",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "BI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Burundi", "ブルンジ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BJ",
        #[cfg(feature = "alpha3")]
        alpha3: "BEN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "229",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 9.30769,
            longitude: 2.315834,
            max_latitude: 12.4086111,
            max_longitude: 3.8433429,
            min_latitude: 6.2061001,
            min_longitude: 0.7754124000000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.4086111,
                    lng: 3.8433429,
                },
                southwest: LatLng {
                    lat: 6.2061001,
                    lng: 0.7754124000000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BEN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Benin",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Benin",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Beninese",
        #[cfg(feature = "number")]
        number: "204",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "BJ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Benin", "Bénin", "ベナン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BL",
        #[cfg(feature = "alpha3")]
        alpha3: "BLM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "590",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TB",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 17.9,
            longitude: -62.833333,
            max_latitude: 17.978,
            max_longitude: -62.7869,
            min_latitude: 17.8663,
            min_longitude: -62.9559999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.978,
                    lng: -62.7869,
                },
                southwest: LatLng {
                    lat: 17.8663,
                    lng: -62.9559999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Collectivity of Saint-Barthélemy",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Barthélemy",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Saint Barthélemy Islander",
        #[cfg(feature = "number")]
        number: "652",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78][01]\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "BL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Saint Barthélemy", "Saint-Barthélemy", "サン・バルテルミー"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BM",
        #[cfg(feature = "alpha3")]
        alpha3: "BMU",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "BMD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BD",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 32.3078,
            longitude: -64.7505,
            max_latitude: 32.3961,
            max_longitude: -64.6413999,
            min_latitude: 32.2424975,
            min_longitude: -64.89139999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 32.3961,
                    lng: -64.6413999,
                },
                southwest: LatLng {
                    lat: 32.2424975,
                    lng: -64.89139999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "BER",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Bermuda",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bermuda",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Bermudian",
        #[cfg(feature = "number")]
        number: "060",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[A-Z]{2} ?[A-Z0-9]{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern America",
        #[cfg(feature = "un_locode")]
        un_locode: "BM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Bermuda", "Bermudes", "Bermudas", "バミューダ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BN",
        #[cfg(feature = "alpha3")]
        alpha3: "BRN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "673",
        #[cfg(feature = "currency_code")]
        currency_code: "BND",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BX",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 4.535277,
            longitude: 114.727669,
            max_latitude: 5.0978001,
            max_longitude: 115.3639552,
            min_latitude: 4.002460999999999,
            min_longitude: 114.0752,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 5.0978001,
                    lng: 115.3639552,
                },
                southwest: LatLng {
                    lat: 4.002460999999999,
                    lng: 114.0752,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BRU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Nation of Brunei, the Abode of Peace",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Brunei Darussalam",
        #[cfg(feature = "languages_official")]
        languages_official: &["ms"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ms"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Bruneian",
        #[cfg(feature = "number")]
        number: "096",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[A-Z]{2} ?\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "BN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Brunei", "ブルネイ・ダルサラーム"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BO",
        #[cfg(feature = "alpha3")]
        alpha3: "BOL",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "591",
        #[cfg(feature = "currency_code")]
        currency_code: "BOB",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -16.290154,
            longitude: -63.58865299999999,
            max_latitude: -9.669323,
            max_longitude: -57.453803,
            min_latitude: -22.8980899,
            min_longitude: -69.64498999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -9.669323,
                    lng: -57.453803,
                },
                southwest: LatLng {
                    lat: -22.8980899,
                    lng: -69.64498999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "0010",
        #[cfg(feature = "ioc")]
        ioc: "BOL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Plurinational State of Bolivia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bolivia (Plurinational State of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["es", "ay", "qu"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "ay", "qu"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("010"),
        #[cfg(feature = "nationality")]
        nationality: "Bolivian",
        #[cfg(feature = "number")]
        number: "068",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "BO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Bolivia", "Bolivien", "Bolivie", "ボリビア多民族国"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BQ",
        #[cfg(feature = "alpha3")]
        alpha3: "BES",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "599",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.1783611,
            longitude: -68.2385339,
            max_latitude: 17.6606999,
            max_longitude: -62.9228,
            min_latitude: 11.9641,
            min_longitude: -68.5149,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.6606999,
                    lng: -62.9228,
                },
                southwest: LatLng {
                    lat: 11.9641,
                    lng: -68.5149,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Bonaire, Sint Eustatius and Saba",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bonaire, Sint Eustatius and Saba",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Dutch",
        #[cfg(feature = "number")]
        number: "535",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "BQ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Bonaire, Sint Eustatius and Saba",
            "Caribbean Netherlands",
            "Caribisch Nederland",
            "ボネール、シント・ユースタティウスおよびサバ",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BR",
        #[cfg(feature = "alpha3")]
        alpha3: "BRA",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "55",
        #[cfg(feature = "currency_code")]
        currency_code: "BRL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -14.235004,
            longitude: -51.92528,
            max_latitude: 5.2717863,
            max_longitude: -28.650543,
            min_latitude: -34.0891,
            min_longitude: -73.9828169,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 5.2717863,
                    lng: -28.650543,
                },
                southwest: LatLng {
                    lat: -34.0891,
                    lng: -73.9828169,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "0014",
        #[cfg(feature = "ioc")]
        ioc: "BRA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federative Republic of Brazil",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Brazil",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("014"),
        #[cfg(feature = "nationality")]
        nationality: "Brazilian",
        #[cfg(feature = "number")]
        number: "076",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}-?\d{3}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "BR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Brazil",
            "Brasilien",
            "Brésil",
            "Brasil",
            "ブラジル",
            "Brazilië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BS",
        #[cfg(feature = "alpha3")]
        alpha3: "BHS",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "BSD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "BF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 25.03428,
            longitude: -77.39627999999999,
            max_latitude: 27.263412,
            max_longitude: -72.70975390000001,
            min_latitude: 20.9082735,
            min_longitude: -80.4775603,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 27.263412,
                    lng: -72.70975390000001,
                },
                southwest: LatLng {
                    lat: 20.9082735,
                    lng: -80.4775603,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "BAH",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Commonwealth of The Bahamas",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bahamas",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Bahamian",
        #[cfg(feature = "number")]
        number: "044",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "BS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["The Bahamas", "バハマ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BT",
        #[cfg(feature = "alpha3")]
        alpha3: "BTN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "975",
        #[cfg(feature = "currency_code")]
        currency_code: "BTN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 27.514162,
            longitude: 90.433601,
            max_latitude: 28.246987,
            max_longitude: 92.125232,
            min_latitude: 26.702016,
            min_longitude: 88.7464739,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 28.246987,
                    lng: 92.125232,
                },
                southwest: LatLng {
                    lat: 26.702016,
                    lng: 88.7464739,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BHU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Bhutan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bhutan",
        #[cfg(feature = "languages_official")]
        languages_official: &["dz"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["dz"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Bhutanese",
        #[cfg(feature = "number")]
        number: "064",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "BT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Bhutan", "Bhoutan", "Bután", "ブータン"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BV",
        #[cfg(feature = "alpha3")]
        alpha3: "BVT",
        #[cfg(feature = "continent")]
        continent: "Antarctica",
        #[cfg(feature = "country_code")]
        country_code: "47",
        #[cfg(feature = "currency_code")]
        currency_code: "NOK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -54.4207915,
            longitude: 3.3464497,
            max_latitude: -54.3869298,
            max_longitude: 3.4332785,
            min_latitude: -54.4541004,
            min_longitude: 3.2858826,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -54.3869298,
                    lng: 3.4332785,
                },
                southwest: LatLng {
                    lat: -54.4541004,
                    lng: 3.2858826,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Bouvet Island",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Bouvet Island",
        #[cfg(feature = "languages_official")]
        languages_official: &[],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &[],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "",
        #[cfg(feature = "number")]
        number: "074",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "",
        #[cfg(feature = "un_locode")]
        un_locode: "BV",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Bouvet Island", "Bouvetinsel", "ブーベ島", "Bouveteiland"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BW",
        #[cfg(feature = "alpha3")]
        alpha3: "BWA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "267",
        #[cfg(feature = "currency_code")]
        currency_code: "BWP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -22.328474,
            longitude: 24.684866,
            max_latitude: -17.7781369,
            max_longitude: 29.375304,
            min_latitude: -26.9075448,
            min_longitude: 19.998903,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -17.7781369,
                    lng: 29.375304,
                },
                southwest: LatLng {
                    lat: -26.9075448,
                    lng: 19.998903,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BOT",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Botswana",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Botswana",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "tn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "tn"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Motswana",
        #[cfg(feature = "number")]
        number: "072",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "BW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Botswana", "ボツワナ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BY",
        #[cfg(feature = "alpha3")]
        alpha3: "BLR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "375",
        #[cfg(feature = "currency_code")]
        currency_code: "BYN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 53.709807,
            longitude: 27.953389,
            max_latitude: 56.1718719,
            max_longitude: 32.7768202,
            min_latitude: 51.26201100000001,
            min_longitude: 23.1783377,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 56.1718719,
                    lng: 32.7768202,
                },
                southwest: LatLng {
                    lat: 51.26201100000001,
                    lng: 23.1783377,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "BLR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Belarus",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Belarus",
        #[cfg(feature = "languages_official")]
        languages_official: &["be", "ru"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["be", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Belarusian",
        #[cfg(feature = "number")]
        number: "112",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "BY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Belarus",
            "Weißrussland",
            "Biélorussie",
            "Bielorrusia",
            "ベラルーシ",
            "Wit-Rusland",
            "Беларусь",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "BZ",
        #[cfg(feature = "alpha3")]
        alpha3: "BLZ",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "501",
        #[cfg(feature = "currency_code")]
        currency_code: "BZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "BH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 17.189877,
            longitude: -88.49765,
            max_latitude: 18.4959419,
            max_longitude: -87.41269989999999,
            min_latitude: 15.8856189,
            min_longitude: -89.22758789999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.4959419,
                    lng: -87.41269989999999,
                },
                southwest: LatLng {
                    lat: 15.8856189,
                    lng: -89.22758789999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "BIZ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Belize",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Belize",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Belizean",
        #[cfg(feature = "number")]
        number: "084",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "BZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Belize", "Belice", "ベリーズ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CA",
        #[cfg(feature = "alpha3")]
        alpha3: "CAN",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "CAD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 56.130366,
            longitude: -106.346771,
            max_latitude: 83.6381,
            max_longitude: -50.9766,
            min_latitude: 41.6765559,
            min_longitude: -141.00187,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 83.6381,
                    lng: -50.9766,
                },
                southwest: LatLng {
                    lat: 41.6765559,
                    lng: -141.00187,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "CAN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Canada",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Canada",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Canadian",
        #[cfg(feature = "number")]
        number: "124",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJ-NPRSTV-Z] ?\d[ABCEGHJ-NPRSTV-Z]\d",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Northern America",
        #[cfg(feature = "un_locode")]
        un_locode: "CA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Canada", "Kanada", "Canadá", "カナダ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CC",
        #[cfg(feature = "alpha3")]
        alpha3: "CCK",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "61",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -12.164165,
            longitude: 96.87095599999999,
            max_latitude: -11.819973,
            max_longitude: 96.93271639999999,
            min_latitude: -12.2118513,
            min_longitude: 96.8134118,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -11.819973,
                    lng: 96.93271639999999,
                },
                southwest: LatLng {
                    lat: -12.2118513,
                    lng: 96.8134118,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "0011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of Cocos (Keeling) Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cocos (Keeling) Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Cocos Islander",
        #[cfg(feature = "number")]
        number: "166",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"6799",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Australia and New Zealand",
        #[cfg(feature = "un_locode")]
        un_locode: "CC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Cocos (Keeling) Islands",
            "Kokosinseln",
            "ココス（キーリング）諸島",
            "Cocoseilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CD",
        #[cfg(feature = "alpha3")]
        alpha3: "COD",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "243",
        #[cfg(feature = "currency_code")]
        currency_code: "CDF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -4.038333,
            longitude: 21.758664,
            max_latitude: 5.3920029,
            max_longitude: 31.314612,
            min_latitude: -13.4590349,
            min_longitude: 12.1454,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 5.3920029,
                    lng: 31.314612,
                },
                southwest: LatLng {
                    lat: -13.4590349,
                    lng: 12.1454,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "COD",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Democratic Republic of the Congo",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Congo (Democratic Republic of the)",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "ln", "kg", "sw", "lu"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "ln", "kg", "sw", "lu"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Congolese",
        #[cfg(feature = "number")]
        number: "180",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "CD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Congo (Dem. Rep.)",
            "Kongo (Dem. Rep.)",
            "Congo (Rep. Dem.)",
            "コンゴ民主共和国",
            "Congo [DRC]",
            "Congo (The Democratic Republic Of The)",
            "Democratic Republic of the Congo",
            "Congo, Democratic Republic of",
            "Congo (Kinshasa)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CF",
        #[cfg(feature = "alpha3")]
        alpha3: "CAF",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "236",
        #[cfg(feature = "currency_code")]
        currency_code: "XAF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 6.611110999999999,
            longitude: 20.939444,
            max_latitude: 11.0179569,
            max_longitude: 27.4583049,
            min_latitude: 2.2230529,
            min_longitude: 14.4150981,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 11.0179569,
                    lng: 27.4583049,
                },
                southwest: LatLng {
                    lat: 2.2230529,
                    lng: 14.4150981,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CAF",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Central African Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Central African Republic",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "sg"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "sg"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Central African",
        #[cfg(feature = "number")]
        number: "140",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "CF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Central African Republic",
            "Zentralafrikanische Republik",
            "République Centrafricaine",
            "República Centroafricana",
            "中央アフリカ共和国",
            "Centraal-Afrikaanse Republiek",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CG",
        #[cfg(feature = "alpha3")]
        alpha3: "COG",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "242",
        #[cfg(feature = "currency_code")]
        currency_code: "XAF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -0.228021,
            longitude: 15.827659,
            max_latitude: 3.707791,
            max_longitude: 18.650421,
            min_latitude: -5.0964,
            min_longitude: 11.1182001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 3.707791,
                    lng: 18.650421,
                },
                southwest: LatLng {
                    lat: -5.0964,
                    lng: 11.1182001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CGO",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of the Congo",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Congo",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "ln"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "ln"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Congolese",
        #[cfg(feature = "number")]
        number: "178",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "CG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Congo",
            "Kongo",
            "コンゴ共和国",
            "Congo [Republiek]",
            "Congo, Republic of",
            "Congo (Brazzaville)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CH",
        #[cfg(feature = "alpha3")]
        alpha3: "CHE",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "41",
        #[cfg(feature = "currency_code")]
        currency_code: "CHF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 46.818188,
            longitude: 8.227511999999999,
            max_latitude: 47.8084546,
            max_longitude: 10.4923401,
            min_latitude: 45.81792,
            min_longitude: 5.95608,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 47.8084546,
                    lng: 10.4923401,
                },
                southwest: LatLng {
                    lat: 45.81792,
                    lng: 5.95608,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SUI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Swiss Confederation",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Switzerland",
        #[cfg(feature = "languages_official")]
        languages_official: &["de", "fr", "it", "rm"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["de", "fr", "it", "rm"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Swiss",
        #[cfg(feature = "number")]
        number: "756",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "CH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Switzerland",
            "Schweiz",
            "Suisse",
            "Suiza",
            "スイス",
            "Zwitserland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CI",
        #[cfg(feature = "alpha3")]
        alpha3: "CIV",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "225",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.539988999999999,
            longitude: -5.547079999999999,
            max_latitude: 10.7400149,
            max_longitude: -2.493031,
            min_latitude: 4.193,
            min_longitude: -8.6020589,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 10.7400149,
                    lng: -2.493031,
                },
                southwest: LatLng {
                    lat: 4.193,
                    lng: -8.6020589,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CIV",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Côte d'Ivoire",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Côte d'Ivoire",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Ivorian",
        #[cfg(feature = "number")]
        number: "384",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "CI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Côte D'Ivoire",
            "Elfenbeinküste",
            "コートジボワール",
            "Ivoorkust",
            "Cote D'Ivoire (Ivory Coast)",
            "Cote d Ivoire (Ivory Coast)",
            "Ivory Coast",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CK",
        #[cfg(feature = "alpha3")]
        alpha3: "COK",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "682",
        #[cfg(feature = "currency_code")]
        currency_code: "NZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CW",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -21.236736,
            longitude: -159.777671,
            max_latitude: -8.1679932,
            max_longitude: -155.6982422,
            min_latitude: -23.0898384,
            min_longitude: -166.1791992,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -8.1679932,
                    lng: -155.6982422,
                },
                southwest: LatLng {
                    lat: -23.0898384,
                    lng: -166.1791992,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "COK",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Cook Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cook Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("00"),
        #[cfg(feature = "nationality")]
        nationality: "Cook Islander",
        #[cfg(feature = "number")]
        number: "184",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "CK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Cook Islands",
            "Cookinseln",
            "Îles Cook",
            "Islas Cook",
            "クック諸島",
            "Cookeilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CL",
        #[cfg(feature = "alpha3")]
        alpha3: "CHL",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "56",
        #[cfg(feature = "currency_code")]
        currency_code: "CLP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -35.675147,
            longitude: -71.542969,
            max_latitude: -17.4983291,
            max_longitude: -66.3327,
            min_latitude: -56.1455,
            min_longitude: -110.0281,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -17.4983291,
                    lng: -66.3327,
                },
                southwest: LatLng {
                    lat: -56.1455,
                    lng: -110.0281,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CHI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Chile",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Chile",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Chilean",
        #[cfg(feature = "number")]
        number: "152",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{7}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "CL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Chile", "チリ", "Chili"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CM",
        #[cfg(feature = "alpha3")]
        alpha3: "CMR",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "237",
        #[cfg(feature = "currency_code")]
        currency_code: "XAF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.369721999999999,
            longitude: 12.354722,
            max_latitude: 13.083335,
            max_longitude: 16.1944081,
            min_latitude: 1.6559,
            min_longitude: 8.3936001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.083335,
                    lng: 16.1944081,
                },
                southwest: LatLng {
                    lat: 1.6559,
                    lng: 8.3936001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CMR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Cameroon",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cameroon",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Cameroonian",
        #[cfg(feature = "number")]
        number: "120",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "CM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Cameroon",
            "Kamerun",
            "Cameroun",
            "Camerún",
            "カメルーン",
            "Kameroen",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CN",
        #[cfg(feature = "alpha3")]
        alpha3: "CHN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "86",
        #[cfg(feature = "currency_code")]
        currency_code: "CNY",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 35.86166,
            longitude: 104.195397,
            max_latitude: 53.5609739,
            max_longitude: 134.7754563,
            min_latitude: 17.9996,
            min_longitude: 73.4994136,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 53.5609739,
                    lng: 134.7754563,
                },
                southwest: LatLng {
                    lat: 17.9996,
                    lng: 73.4994136,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CHN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The People's Republic of China",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "China",
        #[cfg(feature = "languages_official")]
        languages_official: &["zh"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["zh"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9, 10, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Chinese",
        #[cfg(feature = "number")]
        number: "156",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "CN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["China", "Chine", "中国"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CO",
        #[cfg(feature = "alpha3")]
        alpha3: "COL",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "57",
        #[cfg(feature = "currency_code")]
        currency_code: "COP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 4.570868,
            longitude: -74.297333,
            max_latitude: 13.5177999,
            max_longitude: -66.8463122,
            min_latitude: -4.227109899999999,
            min_longitude: -81.8317,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.5177999,
                    lng: -66.8463122,
                },
                southwest: LatLng {
                    lat: -4.227109899999999,
                    lng: -81.8317,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "005",
        #[cfg(feature = "ioc")]
        ioc: "COL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Colombia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Colombia",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("05"),
        #[cfg(feature = "nationality")]
        nationality: "Colombian",
        #[cfg(feature = "number")]
        number: "170",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "CO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Colombia", "Kolumbien", "Colombie", "コロンビア"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CR",
        #[cfg(feature = "alpha3")]
        alpha3: "CRI",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "506",
        #[cfg(feature = "currency_code")]
        currency_code: "CRC",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 9.748916999999999,
            longitude: -83.753428,
            max_latitude: 11.2196806,
            max_longitude: -82.51830009999999,
            min_latitude: 5.496099999999999,
            min_longitude: -87.09899999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 11.2196806,
                    lng: -82.51830009999999,
                },
                southwest: LatLng {
                    lat: 5.496099999999999,
                    lng: -87.09899999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CRC",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Costa Rica",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Costa Rica",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Costa Rican",
        #[cfg(feature = "number")]
        number: "188",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4,5}|\d{3}-\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "CR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Costa Rica", "コスタリカ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CU",
        #[cfg(feature = "alpha3")]
        alpha3: "CUB",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "53",
        #[cfg(feature = "currency_code")]
        currency_code: "CUP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 21.521757,
            longitude: -77.781167,
            max_latitude: 23.3776001,
            max_longitude: -73.9545,
            min_latitude: 19.6529001,
            min_longitude: -85.1715001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 23.3776001,
                    lng: -73.9545,
                },
                southwest: LatLng {
                    lat: 19.6529001,
                    lng: -85.1715001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "119",
        #[cfg(feature = "ioc")]
        ioc: "CUB",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Cuba",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cuba",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Cuban",
        #[cfg(feature = "number")]
        number: "192",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "CU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Cuba", "Kuba", "キューバ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CV",
        #[cfg(feature = "alpha3")]
        alpha3: "CPV",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "238",
        #[cfg(feature = "currency_code")]
        currency_code: "CVE",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 16.5388,
            longitude: -23.0418,
            max_latitude: 17.3191764,
            max_longitude: -22.5933839,
            min_latitude: 14.7270733,
            min_longitude: -25.383911,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.3191764,
                    lng: -22.5933839,
                },
                southwest: LatLng {
                    lat: 14.7270733,
                    lng: -25.383911,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CPV",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Cabo Verde",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cabo Verde",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Cape Verdian",
        #[cfg(feature = "number")]
        number: "132",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "CV",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Cape Verde",
            "Kap Verde",
            "Cap Vert",
            "Cabo Verde",
            "カーボベルデ",
            "Kaapverdië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CW",
        #[cfg(feature = "alpha3")]
        alpha3: "CUW",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "599",
        #[cfg(feature = "currency_code")]
        currency_code: "ANG",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "UC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.16957,
            longitude: -68.99002,
            max_latitude: 12.4941999,
            max_longitude: -68.5670001,
            min_latitude: 11.9224,
            min_longitude: -69.29899999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.4941999,
                    lng: -68.5670001,
                },
                southwest: LatLng {
                    lat: 11.9224,
                    lng: -69.29899999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Country of Curaçao",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Curaçao",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Dutch",
        #[cfg(feature = "number")]
        number: "531",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "CW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Curaçao", "キュラソー島"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CX",
        #[cfg(feature = "alpha3")]
        alpha3: "CXR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "61",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -10.447525,
            longitude: 105.690449,
            max_latitude: -10.412352,
            max_longitude: 105.7129382,
            min_latitude: -10.5703619,
            min_longitude: 105.5333161,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -10.412352,
                    lng: 105.7129382,
                },
                southwest: LatLng {
                    lat: -10.5703619,
                    lng: 105.5333161,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "0011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of Christmas Island",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Christmas Island",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "zh", "ms"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "zh", "ms"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Christmas Island",
        #[cfg(feature = "number")]
        number: "162",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"6798",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Australia and New Zealand",
        #[cfg(feature = "un_locode")]
        un_locode: "CX",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Christmas Island",
            "Weihnachtsinsel",
            "クリスマス島",
            "Christmaseiland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CY",
        #[cfg(feature = "alpha3")]
        alpha3: "CYP",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "357",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 35.126413,
            longitude: 33.429859,
            max_latitude: 35.7071999,
            max_longitude: 34.60450000000001,
            min_latitude: 34.6304001,
            min_longitude: 32.2459,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 35.7071999,
                    lng: 34.60450000000001,
                },
                southwest: LatLng {
                    lat: 34.6304001,
                    lng: 32.2459,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CYP",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Cyprus",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cyprus",
        #[cfg(feature = "languages_official")]
        languages_official: &["el", "tr", "hy"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["el", "tr", "hy"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Cypriot",
        #[cfg(feature = "number")]
        number: "196",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "CY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Cyprus", "Zypern", "Chypre", "Chipre", "キプロス"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "CZ",
        #[cfg(feature = "alpha3")]
        alpha3: "CZE",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "420",
        #[cfg(feature = "currency_code")]
        currency_code: "CZK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "EZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 49.81749199999999,
            longitude: 15.472962,
            max_latitude: 51.0557185,
            max_longitude: 18.8592361,
            min_latitude: 48.5518081,
            min_longitude: 12.090589,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 51.0557185,
                    lng: 18.8592361,
                },
                southwest: LatLng {
                    lat: 48.5518081,
                    lng: 12.090589,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CZE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Czech Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Czechia",
        #[cfg(feature = "languages_official")]
        languages_official: &["cs"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["cs"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Czech",
        #[cfg(feature = "number")]
        number: "203",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3} ?\d{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "CZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Czech Republic",
            "Tschechische Republik",
            "République Tchèque",
            "República Checa",
            "チェコ",
            "Tsjechië",
            "Czechia",
            "Česká republika",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "DE",
        #[cfg(feature = "alpha3")]
        alpha3: "DEU",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "49",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 51.165691,
            longitude: 10.451526,
            max_latitude: 55.0815,
            max_longitude: 15.0418962,
            min_latitude: 47.2701115,
            min_longitude: 5.8663425,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 55.0815,
                    lng: 15.0418962,
                },
                southwest: LatLng {
                    lat: 47.2701115,
                    lng: 5.8663425,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GER",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federal Republic of Germany",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Germany",
        #[cfg(feature = "languages_official")]
        languages_official: &["de"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["de"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2, 3, 4, 5],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7, 8, 9, 10, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "German",
        #[cfg(feature = "number")]
        number: "276",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "DE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Germany",
            "Deutschland",
            "Allemagne",
            "Alemania",
            "ドイツ",
            "Duitsland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "DJ",
        #[cfg(feature = "alpha3")]
        alpha3: "DJI",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "253",
        #[cfg(feature = "currency_code")]
        currency_code: "DJF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "DJ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 11.825138,
            longitude: 42.590275,
            max_latitude: 12.7136972,
            max_longitude: 43.4839,
            min_latitude: 10.912953,
            min_longitude: 41.77084600000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.7136972,
                    lng: 43.4839,
                },
                southwest: LatLng {
                    lat: 10.912953,
                    lng: 41.77084600000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "DJI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Djibouti",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Djibouti",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Djibouti",
        #[cfg(feature = "number")]
        number: "262",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "DJ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Djibouti", "جيبوتي", "Dschibuti", "ジブチ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "DK",
        #[cfg(feature = "alpha3")]
        alpha3: "DNK",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "45",
        #[cfg(feature = "currency_code")]
        currency_code: "DKK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "DA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 56.26392,
            longitude: 9.501785,
            max_latitude: 58.02846,
            max_longitude: 15.2298,
            min_latitude: 54.4317001,
            min_longitude: 7.855200099999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 58.02846,
                    lng: 15.2298,
                },
                southwest: LatLng {
                    lat: 54.4317001,
                    lng: 7.855200099999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "DEN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Denmark",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Denmark",
        #[cfg(feature = "languages_official")]
        languages_official: &["da"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["da"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Danish",
        #[cfg(feature = "number")]
        number: "208",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "DK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Denmark",
            "Dänemark",
            "Danemark",
            "Dinamarca",
            "デンマーク",
            "Denemarken",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "DM",
        #[cfg(feature = "alpha3")]
        alpha3: "DMA",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "DO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.414999,
            longitude: -61.37097600000001,
            max_latitude: 15.6485199,
            max_longitude: -61.23090180000001,
            min_latitude: 15.2042266,
            min_longitude: -61.484108,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 15.6485199,
                    lng: -61.23090180000001,
                },
                southwest: LatLng {
                    lat: 15.2042266,
                    lng: -61.484108,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "DMA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Commonwealth of Dominica",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Dominica",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Dominican",
        #[cfg(feature = "number")]
        number: "212",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "DM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Dominica", "ドミニカ国"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "DO",
        #[cfg(feature = "alpha3")]
        alpha3: "DOM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "DOP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "DR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.735693,
            longitude: -70.162651,
            max_latitude: 19.9786989,
            max_longitude: -68.25260010000001,
            min_latitude: 17.3611001,
            min_longitude: -72.0075099,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 19.9786989,
                    lng: -68.25260010000001,
                },
                southwest: LatLng {
                    lat: 17.3611001,
                    lng: -72.0075099,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "DOM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Dominican Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Dominican Republic",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Dominican",
        #[cfg(feature = "number")]
        number: "214",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "DO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Dominican Republic",
            "Dominikanische Republik",
            "République Dominicaine",
            "República Dominicana",
            "ドミニカ共和国",
            "Dominicaanse Republiek",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "DZ",
        #[cfg(feature = "alpha3")]
        alpha3: "DZA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "213",
        #[cfg(feature = "currency_code")]
        currency_code: "DZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "AG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 28.033886,
            longitude: 1.659626,
            max_latitude: 37.2216,
            max_longitude: 11.9999992,
            min_latitude: 18.9681469,
            min_longitude: -8.6676111,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 37.2216,
                    lng: 11.9999992,
                },
                southwest: LatLng {
                    lat: 18.9681469,
                    lng: -8.6676111,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ALG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The People's Democratic Republic of Algeria",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Algeria",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("7"),
        #[cfg(feature = "nationality")]
        nationality: "Algerian",
        #[cfg(feature = "number")]
        number: "012",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "DZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Algeria",
            "الجزائر",
            "Algerien",
            "Algérie",
            "Argelia",
            "アルジェリア",
            "Algerije",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "EC",
        #[cfg(feature = "alpha3")]
        alpha3: "ECU",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "593",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "EC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -1.831239,
            longitude: -78.18340599999999,
            max_latitude: 2.2955,
            max_longitude: -75.1887938,
            min_latitude: -5.0143509,
            min_longitude: -92.60379999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 2.2955,
                    lng: -75.1887938,
                },
                southwest: LatLng {
                    lat: -5.0143509,
                    lng: -92.60379999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ECU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Ecuador",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Ecuador",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Ecuadorean",
        #[cfg(feature = "number")]
        number: "218",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "EC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Ecuador", "Équateur", "エクアドル"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "EE",
        #[cfg(feature = "alpha3")]
        alpha3: "EST",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "372",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "EN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 58.595272,
            longitude: 25.0136071,
            max_latitude: 59.7315,
            max_longitude: 28.2101389,
            min_latitude: 57.50931600000001,
            min_longitude: 21.6540999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 59.7315,
                    lng: 28.2101389,
                },
                southwest: LatLng {
                    lat: 57.50931600000001,
                    lng: 21.6540999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "EST",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Estonia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Estonia",
        #[cfg(feature = "languages_official")]
        languages_official: &["et"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["et"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Estonian",
        #[cfg(feature = "number")]
        number: "233",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "EE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Estonia", "Estland", "Estonie", "エストニア"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "EG",
        #[cfg(feature = "alpha3")]
        alpha3: "EGY",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "20",
        #[cfg(feature = "currency_code")]
        currency_code: "EGP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "EG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 26.820553,
            longitude: 30.802498,
            max_latitude: 31.8122,
            max_longitude: 37.0569,
            min_latitude: 21.9999999,
            min_longitude: 24.696775,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 31.8122,
                    lng: 37.0569,
                },
                southwest: LatLng {
                    lat: 21.9999999,
                    lng: 24.696775,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "EGY",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Arab Republic of Egypt",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Egypt",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Egyptian",
        #[cfg(feature = "number")]
        number: "818",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "EG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Egypt",
            "مصر",
            "Ägypten",
            "Égypte",
            "Egipto",
            "エジプト",
            "Egypte",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "EH",
        #[cfg(feature = "alpha3")]
        alpha3: "ESH",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "212",
        #[cfg(feature = "currency_code")]
        currency_code: "MAD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "WI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 24.215527,
            longitude: -12.885834,
            max_latitude: 27.7223999,
            max_longitude: -8.667525,
            min_latitude: 20.427,
            min_longitude: -17.4573001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 27.7223999,
                    lng: -8.667525,
                },
                southwest: LatLng {
                    lat: 20.427,
                    lng: -17.4573001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Sahrawi Arab Democratic Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Western Sahara",
        #[cfg(feature = "languages_official")]
        languages_official: &["es", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Sahrawi",
        #[cfg(feature = "number")]
        number: "732",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "EH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Western Sahara",
            "الصحراء الغربية",
            "Westsahara",
            "Sahara Occidental",
            "西サハラ",
            "Westelijke Sahara",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ER",
        #[cfg(feature = "alpha3")]
        alpha3: "ERI",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "291",
        #[cfg(feature = "currency_code")]
        currency_code: "ERN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ER",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.179384,
            longitude: 39.782334,
            max_latitude: 18.0204137,
            max_longitude: 43.2312,
            min_latitude: 12.354723,
            min_longitude: 36.433348,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.0204137,
                    lng: 43.2312,
                },
                southwest: LatLng {
                    lat: 12.354723,
                    lng: 36.433348,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ERI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The State of Eritrea",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Eritrea",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ar", "ti"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ar", "ti"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Eritrean",
        #[cfg(feature = "number")]
        number: "232",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ER",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Eritrea", "إريتريا", "Érythrée", "エリトリア"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ES",
        #[cfg(feature = "alpha3")]
        alpha3: "ESP",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "34",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 40.46366700000001,
            longitude: -3.74922,
            max_latitude: 43.8504,
            max_longitude: 4.6362,
            min_latitude: 27.4985,
            min_longitude: -18.2648001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.8504,
                    lng: 4.6362,
                },
                southwest: LatLng {
                    lat: 27.4985,
                    lng: -18.2648001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ESP",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Spain",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Spain",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "ca", "gl", "eu", "an", "ast", "fax", "rif", "rmq"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Spanish",
        #[cfg(feature = "number")]
        number: "724",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "ES",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Spain",
            "Spanien",
            "Espagne",
            "España",
            "スペイン",
            "Spanje",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ET",
        #[cfg(feature = "alpha3")]
        alpha3: "ETH",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "251",
        #[cfg(feature = "currency_code")]
        currency_code: "ETB",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ET",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 9.145000000000001,
            longitude: 40.489673,
            max_latitude: 14.8942141,
            max_longitude: 48.0010561,
            min_latitude: 3.4041369,
            min_longitude: 32.997734,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 14.8942141,
                    lng: 48.0010561,
                },
                southwest: LatLng {
                    lat: 3.4041369,
                    lng: 32.997734,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ETH",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federal Democratic Republic of Ethiopia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Ethiopia",
        #[cfg(feature = "languages_official")]
        languages_official: &["am"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["am"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Ethiopian",
        #[cfg(feature = "number")]
        number: "231",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ET",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Ethiopia",
            "Äthiopien",
            "Éthiopie",
            "Etiopía",
            "エチオピア",
            "Ethiopië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "FI",
        #[cfg(feature = "alpha3")]
        alpha3: "FIN",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "358",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 61.92410999999999,
            longitude: 25.7481511,
            max_latitude: 70.0922932,
            max_longitude: 31.5870999,
            min_latitude: 59.693623,
            min_longitude: 20.4565002,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 70.0922932,
                    lng: 31.5870999,
                },
                southwest: LatLng {
                    lat: 59.693623,
                    lng: 20.4565002,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "FIN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Finland",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Finland",
        #[cfg(feature = "languages_official")]
        languages_official: &["fi", "sv"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fi", "sv"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Finnish",
        #[cfg(feature = "number")]
        number: "246",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "FI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Finland",
            "Finnland",
            "Finlande",
            "Finlandia",
            "フィンランド",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "FJ",
        #[cfg(feature = "alpha3")]
        alpha3: "FJI",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "679",
        #[cfg(feature = "currency_code")]
        currency_code: "FJD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FJ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -17.713371,
            longitude: 178.065032,
            max_latitude: -12.2084957,
            max_longitude: -177.8686523,
            min_latitude: -20.8998713,
            min_longitude: 176.7919922,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -12.2084957,
                    lng: -177.8686523,
                },
                southwest: LatLng {
                    lat: -20.8998713,
                    lng: 176.7919922,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "FIJ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Fiji",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Fiji",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "fj", "hi"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "fj", "hi", "ur"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Fijian",
        #[cfg(feature = "number")]
        number: "242",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Melanesia",
        #[cfg(feature = "un_locode")]
        un_locode: "FJ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Fiji", "Fidschi", "Fidji", "フィジー"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "FK",
        #[cfg(feature = "alpha3")]
        alpha3: "FLK",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "500",
        #[cfg(feature = "currency_code")]
        currency_code: "FKP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "FK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -51.796253,
            longitude: -59.523613,
            max_latitude: -50.9809115,
            max_longitude: -57.6768495,
            min_latitude: -52.4744161,
            min_longitude: -61.3792419,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -50.9809115,
                    lng: -57.6768495,
                },
                southwest: LatLng {
                    lat: -52.4744161,
                    lng: -61.3792419,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Falkland Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Falkland Islands (Malvinas)",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Falkland Islander",
        #[cfg(feature = "number")]
        number: "238",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"FIQQ 1ZZ",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "FK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Falkland Islands",
            "Falklandinseln",
            "Îles Malouines",
            "Islas Malvinas",
            "フォークランド（マルビナス）諸島",
            "Falklandeilanden [Islas Malvinas]",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "FM",
        #[cfg(feature = "alpha3")]
        alpha3: "FSM",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "691",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.425554,
            longitude: 150.550812,
            max_latitude: 10.2770863,
            max_longitude: 163.5177612,
            min_latitude: 0.1538084,
            min_longitude: 136.9226075,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 10.2770863,
                    lng: 163.5177612,
                },
                southwest: LatLng {
                    lat: 0.1538084,
                    lng: 136.9226075,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "FSM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federated States of Micronesia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Micronesia (Federated States of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Micronesian",
        #[cfg(feature = "number")]
        number: "583",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(9694[1-4])(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "FM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Micronesia",
            "Mikronesien",
            "Micronésie",
            "ミクロネシア連邦",
            "Micronesië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "FO",
        #[cfg(feature = "alpha3")]
        alpha3: "FRO",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "298",
        #[cfg(feature = "currency_code")]
        currency_code: "DKK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 61.89263500000001,
            longitude: -6.9118061,
            max_latitude: 62.4310742,
            max_longitude: -6.190796,
            min_latitude: 61.3677776,
            min_longitude: -7.7178956,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 62.4310742,
                    lng: -6.190796,
                },
                southwest: LatLng {
                    lat: 61.3677776,
                    lng: -7.7178956,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "FRO",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Faroe Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Faroe Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["fo"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fo"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Faroese",
        #[cfg(feature = "number")]
        number: "234",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "FO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Faroe Islands",
            "Färöer-Inseln",
            "Îles Féroé",
            "Islas Faroe",
            "フェロー諸島",
            "Faeröer",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "FR",
        #[cfg(feature = "alpha3")]
        alpha3: "FRA",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "33",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 46.227638,
            longitude: 2.213749,
            max_latitude: 51.1241999,
            max_longitude: 9.6624999,
            min_latitude: 41.31433,
            min_longitude: -5.5591,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 51.1241999,
                    lng: 9.6624999,
                },
                southwest: LatLng {
                    lat: 41.31433,
                    lng: -5.5591,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "FRA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The French Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "France",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[1],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "250",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{2} ?\d{3}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "FR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "France",
            "Frankreich",
            "the French Republic",
            "フランス",
            "Frankrijk",
            "Francia",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GA",
        #[cfg(feature = "alpha3")]
        alpha3: "GAB",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "241",
        #[cfg(feature = "currency_code")]
        currency_code: "XAF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GB",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -0.803689,
            longitude: 11.609444,
            max_latitude: 2.318109,
            max_longitude: 14.5269234,
            min_latitude: -4.1656,
            min_longitude: 8.421,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 2.318109,
                    lng: 14.5269234,
                },
                southwest: LatLng {
                    lat: -4.1656,
                    lng: 8.421,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GAB",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Gabonese Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Gabon",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Gabonese",
        #[cfg(feature = "number")]
        number: "266",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "GA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Gabon", "Gabun", "Gabón", "ガボン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GB",
        #[cfg(feature = "alpha3")]
        alpha3: "GBR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "44",
        #[cfg(feature = "currency_code")]
        currency_code: "GBP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "UK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 55.378051,
            longitude: -3.435973,
            max_latitude: 60.91569999999999,
            max_longitude: 1.68153079591,
            min_latitude: 49.959999905,
            min_longitude: -7.57216793459,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 60.91569999999999,
                    lng: 1.68153079591,
                },
                southwest: LatLng {
                    lat: 49.959999905,
                    lng: -7.57216793459,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GBR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The United Kingdom of Great Britain and Northern Ireland",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "United Kingdom of Great Britain and Northern Ireland",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "British",
        #[cfg(feature = "number")]
        number: "826",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"GIR ?0AA|(?:(?:AB|AL|B|BA|BB|BD|BF|BH|BL|BN|BR|BS|BT|BX|CA|CB|CF|CH|CM|CO|CR|CT|CV|CW|DA|DD|DE|DG|DH|DL|DN|DT|DY|E|EC|EH|EN|EX|FK|FY|G|GL|GY|GU|HA|HD|HG|HP|HR|HS|HU|HX|IG|IM|IP|IV|JE|KA|KT|KW|KY|L|LA|LD|LE|LL|LN|LS|LU|M|ME|MK|ML|N|NE|NG|NN|NP|NR|NW|OL|OX|PA|PE|PH|PL|PO|PR|RG|RH|RM|S|SA|SE|SG|SK|SL|SM|SN|SO|SP|SR|SS|ST|SW|SY|TA|TD|TF|TN|TQ|TR|TS|TW|UB|W|WA|WC|WD|WF|WN|WR|WS|WV|YO|ZE)(?:\d[\dA-Z]? ?\d[ABD-HJLN-UW-Z]{2}))|BFPO ?\d{1,4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "GB",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "United Kingdom",
            "The United Kingdom",
            "England",
            "Großbritannien",
            "Vereinigtes Königreich",
            "Royaume-Uni",
            "Reino Unido",
            "イギリス",
            "Verenigd Koninkrijk",
            "Great Britain (UK)",
            "UK",
            "Великобритания",
            "Velká Británie",
            "İngiltere",
            "Великобританія",
            "Great Britain",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GD",
        #[cfg(feature = "alpha3")]
        alpha3: "GRD",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "GJ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.1165,
            longitude: -61.67899999999999,
            max_latitude: 12.5367,
            max_longitude: -61.3746999,
            min_latitude: 11.9829051,
            min_longitude: -61.80589999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.5367,
                    lng: -61.3746999,
                },
                southwest: LatLng {
                    lat: 11.9829051,
                    lng: -61.80589999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "GRN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Grenada",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Grenada",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Grenadian",
        #[cfg(feature = "number")]
        number: "308",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "GD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Grenada", "グレナダ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GE",
        #[cfg(feature = "alpha3")]
        alpha3: "GEO",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "995",
        #[cfg(feature = "currency_code")]
        currency_code: "GEL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 42.315407,
            longitude: 43.35689199999999,
            max_latitude: 43.5866269,
            max_longitude: 46.7361189,
            min_latitude: 41.054942,
            min_longitude: 39.9792001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.5866269,
                    lng: 46.7361189,
                },
                southwest: LatLng {
                    lat: 41.054942,
                    lng: 39.9792001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "GEO",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Georgia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Georgia",
        #[cfg(feature = "languages_official")]
        languages_official: &["ka"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ka"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8*"),
        #[cfg(feature = "nationality")]
        nationality: "Georgian",
        #[cfg(feature = "number")]
        number: "268",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "GE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Georgia", "Georgien", "Géorgie", "グルジア", "Georgië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GF",
        #[cfg(feature = "alpha3")]
        alpha3: "GUF",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "594",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 3.933889,
            longitude: -53.125782,
            max_latitude: 5.9548,
            max_longitude: -51.6164491,
            min_latitude: 2.109287,
            min_longitude: -54.5544379,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 5.9548,
                    lng: -51.6164491,
                },
                southwest: LatLng {
                    lat: 2.109287,
                    lng: -54.5544379,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Guyane",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "French Guiana",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "French Guianan",
        #[cfg(feature = "number")]
        number: "254",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78]3\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "GF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "French Guiana",
            "Französisch Guyana",
            "Guayana Francesa",
            "フランス領ギアナ",
            "Frans-Guyana",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GG",
        #[cfg(feature = "alpha3")]
        alpha3: "GGY",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "44",
        #[cfg(feature = "currency_code")]
        currency_code: "GBP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 49.4481982,
            longitude: -2.58949,
            max_latitude: 49.5094108,
            max_longitude: -2.5016885,
            min_latitude: 49.4167199,
            min_longitude: -2.6745361,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 49.5094108,
                    lng: -2.5016885,
                },
                southwest: LatLng {
                    lat: 49.4167199,
                    lng: -2.6745361,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Bailiwick of Guernsey",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guernsey",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Channel Islander",
        #[cfg(feature = "number")]
        number: "831",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"GY\d[\dA-Z]? ?\d[ABD-HJLN-UW-Z]{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "GG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Guernsey and Alderney",
            "Guernsey und Alderney",
            "Guernsey et Alderney",
            "Guernsey y Alderney",
            "ガーンジー",
            "Guernsey",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GH",
        #[cfg(feature = "alpha3")]
        alpha3: "GHA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "233",
        #[cfg(feature = "currency_code")]
        currency_code: "GHS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.946527,
            longitude: -1.023194,
            max_latitude: 11.1750308,
            max_longitude: 1.199972,
            min_latitude: 4.6339001,
            min_longitude: -3.2607859,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 11.1750308,
                    lng: 1.199972,
                },
                southwest: LatLng {
                    lat: 4.6339001,
                    lng: -3.2607859,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GHA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Ghana",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Ghana",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5, 6, 7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Ghanaian",
        #[cfg(feature = "number")]
        number: "288",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "GH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Ghana", "ガーナ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GI",
        #[cfg(feature = "alpha3")]
        alpha3: "GIB",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "350",
        #[cfg(feature = "currency_code")]
        currency_code: "GIP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 36.140751,
            longitude: -5.353585,
            max_latitude: 36.1551186,
            max_longitude: -5.334499999999999,
            min_latitude: 36.1038999,
            min_longitude: -5.3721,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 36.1551186,
                    lng: -5.334499999999999,
                },
                southwest: LatLng {
                    lat: 36.1038999,
                    lng: -5.3721,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Gibraltar",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Gibraltar",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Gibraltar",
        #[cfg(feature = "number")]
        number: "292",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"GX11 1AA",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "GI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Gibraltar", "ジブラルタル"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GL",
        #[cfg(feature = "alpha3")]
        alpha3: "GRL",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "299",
        #[cfg(feature = "currency_code")]
        currency_code: "DKK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 71.706936,
            longitude: -42.604303,
            max_latitude: 83.9702561,
            max_longitude: -8.2617199,
            min_latitude: 58.26329,
            min_longitude: -73.8281196,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 83.9702561,
                    lng: -8.2617199,
                },
                southwest: LatLng {
                    lat: 58.26329,
                    lng: -73.8281196,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "009",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Kalaallit Nunaat",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Greenland",
        #[cfg(feature = "languages_official")]
        languages_official: &["kl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["kl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Greenlandic",
        #[cfg(feature = "number")]
        number: "304",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"39\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern America",
        #[cfg(feature = "un_locode")]
        un_locode: "GL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Greenland",
            "Grönland",
            "Groenland",
            "Groenlandia",
            "グリーンランド",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GM",
        #[cfg(feature = "alpha3")]
        alpha3: "GMB",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "220",
        #[cfg(feature = "currency_code")]
        currency_code: "GMD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 13.443182,
            longitude: -15.310139,
            max_latitude: 13.825058,
            max_longitude: -13.7913862,
            min_latitude: 13.0098999,
            min_longitude: -16.9464001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.825058,
                    lng: -13.7913862,
                },
                southwest: LatLng {
                    lat: 13.0098999,
                    lng: -16.9464001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GAM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of The Gambia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Gambia",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Gambian",
        #[cfg(feature = "number")]
        number: "270",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "GM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["The Gambia", "ガンビア"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GN",
        #[cfg(feature = "alpha3")]
        alpha3: "GIN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "224",
        #[cfg(feature = "currency_code")]
        currency_code: "GNF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 9.945587,
            longitude: -9.696645,
            max_latitude: 12.6748616,
            max_longitude: -7.637853,
            min_latitude: 7.190909099999999,
            min_longitude: -15.282,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.6748616,
                    lng: -7.637853,
                },
                southwest: LatLng {
                    lat: 7.190909099999999,
                    lng: -15.282,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GUI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Guinea",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guinea",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "ff"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "ff"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Guinean",
        #[cfg(feature = "number")]
        number: "324",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "GN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Guinea", "Guinée", "ギニア", "Guinee"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GP",
        #[cfg(feature = "alpha3")]
        alpha3: "GLP",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "590",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 16.265,
            longitude: -61.55099999999999,
            max_latitude: 16.5572273,
            max_longitude: -60.9473,
            min_latitude: 15.742032,
            min_longitude: -61.8468,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 16.5572273,
                    lng: -60.9473,
                },
                southwest: LatLng {
                    lat: 15.742032,
                    lng: -61.8468,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Guadeloupe",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guadeloupe",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "312",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78][01]\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "GP",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Guadeloupe", "Guadalupe", "グアドループ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GQ",
        #[cfg(feature = "alpha3")]
        alpha3: "GNQ",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "240",
        #[cfg(feature = "currency_code")]
        currency_code: "XAF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "EK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 1.650801,
            longitude: 10.267895,
            max_latitude: 3.8355,
            max_longitude: 11.3333,
            min_latitude: -1.5475,
            min_longitude: 5.541900099999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 3.8355,
                    lng: 11.3333,
                },
                southwest: LatLng {
                    lat: -1.5475,
                    lng: 5.541900099999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GEQ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Equatorial Guinea",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Equatorial Guinea",
        #[cfg(feature = "languages_official")]
        languages_official: &["es", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Equatorial Guinean",
        #[cfg(feature = "number")]
        number: "226",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "GQ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Equatorial Guinea",
            "Äquatorial-Guinea",
            "Guinée Équatoriale",
            "Guinea Ecuatorial",
            "赤道ギニア",
            "Equatoriaal-Guinea",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GR",
        #[cfg(feature = "alpha3")]
        alpha3: "GRC",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "30",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 39.074208,
            longitude: 21.824312,
            max_latitude: 41.7488784,
            max_longitude: 29.6527999,
            min_latitude: 34.5428,
            min_longitude: 19.3098,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 41.7488784,
                    lng: 29.6527999,
                },
                southwest: LatLng {
                    lat: 34.5428,
                    lng: 19.3098,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GRE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Hellenic Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Greece",
        #[cfg(feature = "languages_official")]
        languages_official: &["el"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["el"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Greek",
        #[cfg(feature = "number")]
        number: "300",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3} ?\d{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "GR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Greece",
            "Griechenland",
            "Grèce",
            "Grecia",
            "ギリシャ",
            "Griekenland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GS",
        #[cfg(feature = "alpha3")]
        alpha3: "SGS",
        #[cfg(feature = "continent")]
        continent: "Antarctica",
        #[cfg(feature = "country_code")]
        country_code: "500",
        #[cfg(feature = "currency_code")]
        currency_code: "GBP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SX",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -54.429579,
            longitude: -36.587909,
            max_latitude: -53.8525267,
            max_longitude: -25.4663085,
            min_latitude: -59.91097600000001,
            min_longitude: -38.4301758,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -53.8525267,
                    lng: -25.4663085,
                },
                southwest: LatLng {
                    lat: -59.91097600000001,
                    lng: -38.4301758,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "South Georgia and the South Sandwich Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "South Georgia and the South Sandwich Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "South Georgia and the South Sandwich Islander",
        #[cfg(feature = "number")]
        number: "239",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"SIQQ 1ZZ",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "GS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "South Georgia",
            "South Georgia and the South Sandwich Islands",
            "Südgeorgien und die Südlichen Sandwichinseln",
            "サウスジョージア・サウスサンドウィッチ諸島",
            "Zuid-Georgia en Zuidelijke Sandwicheilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GT",
        #[cfg(feature = "alpha3")]
        alpha3: "GTM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "502",
        #[cfg(feature = "currency_code")]
        currency_code: "GTQ",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.783471,
            longitude: -90.23075899999999,
            max_latitude: 17.815697,
            max_longitude: -88.1982001,
            min_latitude: 13.63,
            min_longitude: -92.2714,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.815697,
                    lng: -88.1982001,
                },
                southwest: LatLng {
                    lat: 13.63,
                    lng: -92.2714,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GUA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Guatemala",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guatemala",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Guatemalan",
        #[cfg(feature = "number")]
        number: "320",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "GT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Guatemala", "グアテマラ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GU",
        #[cfg(feature = "alpha3")]
        alpha3: "GUM",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "GQ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 13.444304,
            longitude: 144.793731,
            max_latitude: 13.7994072,
            max_longitude: 145.112915,
            min_latitude: 13.1022175,
            min_longitude: 144.4647218,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.7994072,
                    lng: 145.112915,
                },
                southwest: LatLng {
                    lat: 13.1022175,
                    lng: 144.4647218,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "GUM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of Guam",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guam",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ch", "es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ch", "es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Guamanian",
        #[cfg(feature = "number")]
        number: "316",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(969(?:[12]\d|3[12]))(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "GU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Guam", "グアム"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GW",
        #[cfg(feature = "alpha3")]
        alpha3: "GNB",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "245",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 11.803749,
            longitude: -15.180413,
            max_latitude: 12.6869468,
            max_longitude: -13.6265235,
            min_latitude: 10.7146,
            min_longitude: -16.9518999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.6869468,
                    lng: -13.6265235,
                },
                southwest: LatLng {
                    lat: 10.7146,
                    lng: -16.9518999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GBS",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Guinea-Bissau",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guinea-Bissau",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Guinea-Bissauan",
        #[cfg(feature = "number")]
        number: "624",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "GW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Guinea-Bissau",
            "Guinée-Bissau",
            "ギニアビサウ",
            "Guinee-Bissau",
            "Guinea Bissau",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "GY",
        #[cfg(feature = "alpha3")]
        alpha3: "GUY",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "592",
        #[cfg(feature = "currency_code")]
        currency_code: "GYD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "GY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 4.860416,
            longitude: -58.93018,
            max_latitude: 8.722199999999999,
            max_longitude: -56.49112,
            min_latitude: 1.164724,
            min_longitude: -61.414905,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 8.722199999999999,
                    lng: -56.49112,
                },
                southwest: LatLng {
                    lat: 1.164724,
                    lng: -61.414905,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "GUY",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Co-operative Republic of Guyana",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Guyana",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Guyanese",
        #[cfg(feature = "number")]
        number: "328",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "GY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Guyana", "ガイアナ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "HK",
        #[cfg(feature = "alpha3")]
        alpha3: "HKG",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "852",
        #[cfg(feature = "currency_code")]
        currency_code: "HKD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "HK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 22.3193039,
            longitude: 114.1693611,
            max_latitude: 22.5619469,
            max_longitude: 114.4294999,
            min_latitude: 22.1435,
            min_longitude: 113.8259001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 22.5619469,
                    lng: 114.4294999,
                },
                southwest: LatLng {
                    lat: 22.1435,
                    lng: 113.8259001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "001",
        #[cfg(feature = "ioc")]
        ioc: "HKG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Hong Kong Special Administrative Region of China",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Hong Kong",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "zh"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "zh"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Hong Kongese",
        #[cfg(feature = "number")]
        number: "344",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "HK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Hong Kong", "香港", "Hongkong"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "HM",
        #[cfg(feature = "alpha3")]
        alpha3: "HMD",
        #[cfg(feature = "continent")]
        continent: "Antarctica",
        #[cfg(feature = "country_code")]
        country_code: "672",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "HM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -53.08181,
            longitude: 73.50415799999999,
            max_latitude: -52.9609444,
            max_longitude: 73.7792016,
            min_latitude: -53.19168759999999,
            min_longitude: 73.25065599999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -52.9609444,
                    lng: 73.7792016,
                },
                southwest: LatLng {
                    lat: -53.19168759999999,
                    lng: 73.25065599999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of Heard Island and McDonald Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Heard Island and McDonald Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Heard and McDonald Islander",
        #[cfg(feature = "number")]
        number: "334",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "",
        #[cfg(feature = "un_locode")]
        un_locode: "HM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Heard and McDonald Islands",
            "Heard und die McDonaldinseln",
            "ハード島とマクドナルド諸島",
            "Heard- en McDonaldeilanden",
            "Heard Island and McDonald Islands",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "HN",
        #[cfg(feature = "alpha3")]
        alpha3: "HND",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "504",
        #[cfg(feature = "currency_code")]
        currency_code: "HNL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "HO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.199999,
            longitude: -86.241905,
            max_latitude: 17.4677999,
            max_longitude: -83.0621001,
            min_latitude: 12.9808201,
            min_longitude: -89.3564822,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.4677999,
                    lng: -83.0621001,
                },
                southwest: LatLng {
                    lat: 12.9808201,
                    lng: -89.3564822,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "HON",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Honduras",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Honduras",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Honduran",
        #[cfg(feature = "number")]
        number: "340",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "HN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Honduras", "ホンジュラス"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "HR",
        #[cfg(feature = "alpha3")]
        alpha3: "HRV",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "385",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "HR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 45.1,
            longitude: 15.2000001,
            max_latitude: 46.5549857,
            max_longitude: 19.4480523,
            min_latitude: 42.3385087,
            min_longitude: 13.3649,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 46.5549857,
                    lng: 19.4480523,
                },
                southwest: LatLng {
                    lat: 42.3385087,
                    lng: 13.3649,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CRO",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Croatia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Croatia",
        #[cfg(feature = "languages_official")]
        languages_official: &["hr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["hr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Croatian",
        #[cfg(feature = "number")]
        number: "191",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "HR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Croatia",
            "Kroatien",
            "Croatie",
            "Croacia",
            "クロアチア",
            "Kroatië",
            "Croatia (Hrvatska)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "HT",
        #[cfg(feature = "alpha3")]
        alpha3: "HTI",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "509",
        #[cfg(feature = "currency_code")]
        currency_code: "HTG",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "HA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.971187,
            longitude: -72.285215,
            max_latitude: 20.1282,
            max_longitude: -71.621754,
            min_latitude: 17.9422,
            min_longitude: -74.6082,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 20.1282,
                    lng: -71.621754,
                },
                southwest: LatLng {
                    lat: 17.9422,
                    lng: -74.6082,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "HAI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Haiti",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Haiti",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "ht"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "ht"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Haitian",
        #[cfg(feature = "number")]
        number: "332",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "HT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Haiti", "ハイチ", "Haïti"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "HU",
        #[cfg(feature = "alpha3")]
        alpha3: "HUN",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "36",
        #[cfg(feature = "currency_code")]
        currency_code: "HUF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "HU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 47.162494,
            longitude: 19.5033041,
            max_latitude: 48.585234,
            max_longitude: 22.8965438,
            min_latitude: 45.7370889,
            min_longitude: 16.1133078,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 48.585234,
                    lng: 22.8965438,
                },
                southwest: LatLng {
                    lat: 45.7370889,
                    lng: 16.1133078,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "HUN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Hungary",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Hungary",
        #[cfg(feature = "languages_official")]
        languages_official: &["hu"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["hu"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("06"),
        #[cfg(feature = "nationality")]
        nationality: "Hungarian",
        #[cfg(feature = "number")]
        number: "348",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "HU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Hungary",
            "Ungarn",
            "Hongrie",
            "Hungría",
            "ハンガリー",
            "Hongarije",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ID",
        #[cfg(feature = "alpha3")]
        alpha3: "IDN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "62",
        #[cfg(feature = "currency_code")]
        currency_code: "IDR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ID",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -0.789275,
            longitude: 113.921327,
            max_latitude: 6.216999899999999,
            max_longitude: 141.0425,
            min_latitude: -11.1082999,
            min_longitude: 94.7351,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 6.216999899999999,
                    lng: 141.0425,
                },
                southwest: LatLng {
                    lat: -11.1082999,
                    lng: 94.7351,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "001",
        #[cfg(feature = "ioc")]
        ioc: "INA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Indonesia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Indonesia",
        #[cfg(feature = "languages_official")]
        languages_official: &["id"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["id"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9, 10, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Indonesian",
        #[cfg(feature = "number")]
        number: "360",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "ID",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Indonesia",
            "Indonesien",
            "Indonésie",
            "インドネシア",
            "Indonesië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IE",
        #[cfg(feature = "alpha3")]
        alpha3: "IRL",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "353",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "EI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 53.1423672,
            longitude: -7.692053599999999,
            max_latitude: 55.38294149999999,
            max_longitude: -5.431909999999999,
            min_latitude: 51.4475448,
            min_longitude: -10.4800237,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 55.38294149999999,
                    lng: -5.431909999999999,
                },
                southwest: LatLng {
                    lat: 51.4475448,
                    lng: -10.4800237,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "IRL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Ireland",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Ireland",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ga"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ga"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Irish",
        #[cfg(feature = "number")]
        number: "372",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[\dA-Z]{3} ?[\dA-Z]{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "IE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Ireland",
            "Irland",
            "Irlande",
            "Irlanda",
            "アイルランド",
            "Ierland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IL",
        #[cfg(feature = "alpha3")]
        alpha3: "ISR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "972",
        #[cfg(feature = "currency_code")]
        currency_code: "ILS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 31.046051,
            longitude: 34.851612,
            max_latitude: 33.33280500000001,
            max_longitude: 35.896244,
            min_latitude: 29.47969999999999,
            min_longitude: 34.2673871,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 33.33280500000001,
                    lng: 35.896244,
                },
                southwest: LatLng {
                    lat: 29.47969999999999,
                    lng: 34.2673871,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ISR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The State of Israel",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Israel",
        #[cfg(feature = "languages_official")]
        languages_official: &["he", "ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["he", "ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Israeli",
        #[cfg(feature = "number")]
        number: "376",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}(?:\d{2})?",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "IL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Israel", "Israël", "イスラエル"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IM",
        #[cfg(feature = "alpha3")]
        alpha3: "IMN",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "44",
        #[cfg(feature = "currency_code")]
        currency_code: "GBP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 54.236107,
            longitude: -4.548056,
            max_latitude: 54.4369363,
            max_longitude: -4.270618199999999,
            min_latitude: 54.0186764,
            min_longitude: -4.8736609,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 54.4369363,
                    lng: -4.270618199999999,
                },
                southwest: LatLng {
                    lat: 54.0186764,
                    lng: -4.8736609,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Isle of Man",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Isle of Man",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "gv"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "gv"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Manx",
        #[cfg(feature = "number")]
        number: "833",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"IM\d[\dA-Z]? ?\d[ABD-HJLN-UW-Z]{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "IM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Isle of Man",
            "Insel Man",
            "Île de Man",
            "Isla de Man",
            "マン島",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IN",
        #[cfg(feature = "alpha3")]
        alpha3: "IND",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "91",
        #[cfg(feature = "currency_code")]
        currency_code: "INR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 20.593684,
            longitude: 78.96288,
            max_latitude: 35.513327,
            max_longitude: 97.39535869999999,
            min_latitude: 6.4626999,
            min_longitude: 68.1097,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 35.513327,
                    lng: 97.39535869999999,
                },
                southwest: LatLng {
                    lat: 6.4626999,
                    lng: 68.1097,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "IND",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of India",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "India",
        #[cfg(feature = "languages_official")]
        languages_official: &["hi", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["hi", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Indian",
        #[cfg(feature = "number")]
        number: "356",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "IN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["India", "Indien", "Inde", "インド"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IO",
        #[cfg(feature = "alpha3")]
        alpha3: "IOT",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "246",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -6.343194,
            longitude: 71.876519,
            max_latitude: -5.1401857,
            max_longitude: 72.5880433,
            min_latitude: -7.4891118,
            min_longitude: 71.1859131,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -5.1401857,
                    lng: 72.5880433,
                },
                southwest: LatLng {
                    lat: -7.4891118,
                    lng: 71.1859131,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The British Indian Ocean Territory",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "British Indian Ocean Territory",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Indian",
        #[cfg(feature = "number")]
        number: "086",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"BBND 1ZZ",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "IO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "British Indian Ocean Territory",
            "Britisches Territorium im Indischen Ozean",
            "イギリス領インド洋地域",
            "Britse Gebieden in de Indische Oceaan",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IQ",
        #[cfg(feature = "alpha3")]
        alpha3: "IRQ",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "964",
        #[cfg(feature = "currency_code")]
        currency_code: "IQD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 33.223191,
            longitude: 43.679291,
            max_latitude: 37.380645,
            max_longitude: 48.6350999,
            min_latitude: 29.0612079,
            min_longitude: 38.7936741,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 37.380645,
                    lng: 48.6350999,
                },
                southwest: LatLng {
                    lat: 29.0612079,
                    lng: 38.7936741,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "IRQ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Iraq",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Iraq",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Iraqi",
        #[cfg(feature = "number")]
        number: "368",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "IQ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Iraq", "العراق", "Irak", "イラク"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IR",
        #[cfg(feature = "alpha3")]
        alpha3: "IRN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "98",
        #[cfg(feature = "currency_code")]
        currency_code: "IRR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 32.427908,
            longitude: 53.688046,
            max_latitude: 39.782056,
            max_longitude: 63.3333366,
            min_latitude: 24.8066999,
            min_longitude: 44.0326949,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 39.782056,
                    lng: 63.3333366,
                },
                southwest: LatLng {
                    lat: 24.8066999,
                    lng: 44.0326949,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "IRI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Islamic Republic of Iran",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Iran (Islamic Republic of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["fa"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fa"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Iranian",
        #[cfg(feature = "number")]
        number: "364",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}-?\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "saturday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "IR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Iran",
            "Irán",
            "Iran (Islamic Republic Of)",
            "イラン・イスラム共和国",
            "Islamic Republic of Iran",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IS",
        #[cfg(feature = "alpha3")]
        alpha3: "ISL",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "354",
        #[cfg(feature = "currency_code")]
        currency_code: "ISK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 64.963051,
            longitude: -19.020835,
            max_latitude: 67.2466,
            max_longitude: -12.2388001,
            min_latitude: 62.4819,
            min_longitude: -26.2572999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 67.2466,
                    lng: -12.2388001,
                },
                southwest: LatLng {
                    lat: 62.4819,
                    lng: -26.2572999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ISL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Iceland",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Iceland",
        #[cfg(feature = "languages_official")]
        languages_official: &["is"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["is"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Icelander",
        #[cfg(feature = "number")]
        number: "352",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "IS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Iceland",
            "Island",
            "Islande",
            "Islandia",
            "アイスランド",
            "IJsland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "IT",
        #[cfg(feature = "alpha3")]
        alpha3: "ITA",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "39",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "IT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 41.87194,
            longitude: 12.56738,
            max_latitude: 47.092,
            max_longitude: 18.7975999,
            min_latitude: 35.4897,
            min_longitude: 6.6267201,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 47.092,
                    lng: 18.7975999,
                },
                southwest: LatLng {
                    lat: 35.4897,
                    lng: 6.6267201,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ITA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Italian Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Italy",
        #[cfg(feature = "languages_official")]
        languages_official: &["it"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["it"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Italian",
        #[cfg(feature = "number")]
        number: "380",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "IT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Italy", "Italien", "Italie", "Italia", "イタリア", "Italië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "JE",
        #[cfg(feature = "alpha3")]
        alpha3: "JEY",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "44",
        #[cfg(feature = "currency_code")]
        currency_code: "GBP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "JE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 49.214439,
            longitude: -2.13125,
            max_latitude: 49.26650009999999,
            max_longitude: -2.0013001,
            min_latitude: 49.1582,
            min_longitude: -2.2602001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 49.26650009999999,
                    lng: -2.0013001,
                },
                southwest: LatLng {
                    lat: 49.1582,
                    lng: -2.2602001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Bailiwick of Jersey",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Jersey",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "Channel Islander",
        #[cfg(feature = "number")]
        number: "832",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"JE\d[\dA-Z]? ?\d[ABD-HJLN-UW-Z]{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "JE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Jersey", "ジャージー"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "JM",
        #[cfg(feature = "alpha3")]
        alpha3: "JAM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "JMD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "JM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.109581,
            longitude: -77.297508,
            max_latitude: 18.5697821,
            max_longitude: -76.1448669,
            min_latitude: 17.6688854,
            min_longitude: -78.4073639,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.5697821,
                    lng: -76.1448669,
                },
                southwest: LatLng {
                    lat: 17.6688854,
                    lng: -78.4073639,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "JAM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Jamaica",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Jamaica",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Jamaican",
        #[cfg(feature = "number")]
        number: "388",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "JM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Jamaica", "Jamaika", "Jamaïque", "ジャマイカ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "JO",
        #[cfg(feature = "alpha3")]
        alpha3: "JOR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "962",
        #[cfg(feature = "currency_code")]
        currency_code: "JOD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "JO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 30.585164,
            longitude: 36.238414,
            max_latitude: 33.374735,
            max_longitude: 39.301154,
            min_latitude: 29.1850361,
            min_longitude: 34.9441001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 33.374735,
                    lng: 39.301154,
                },
                southwest: LatLng {
                    lat: 29.1850361,
                    lng: 34.9441001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "JOR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Hashemite Kingdom of Jordan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Jordan",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Jordanian",
        #[cfg(feature = "number")]
        number: "400",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "JO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Jordan",
            "الأردن",
            "Jordanien",
            "Jordanie",
            "Jordania",
            "ヨルダン",
            "Jordanië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "JP",
        #[cfg(feature = "alpha3")]
        alpha3: "JPN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "81",
        #[cfg(feature = "currency_code")]
        currency_code: "JPY",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "JA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 36.204824,
            longitude: 138.252924,
            max_latitude: 45.6412626,
            max_longitude: 154.0031455,
            min_latitude: 20.3585295,
            min_longitude: 122.8554688,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 45.6412626,
                    lng: 154.0031455,
                },
                southwest: LatLng {
                    lat: 20.3585295,
                    lng: 122.8554688,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "010",
        #[cfg(feature = "ioc")]
        ioc: "JPN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Japan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Japan",
        #[cfg(feature = "languages_official")]
        languages_official: &["ja"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ja"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Japanese",
        #[cfg(feature = "number")]
        number: "392",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}-?\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "JP",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Japan", "Japon", "Japón", "日本"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KE",
        #[cfg(feature = "alpha3")]
        alpha3: "KEN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "254",
        #[cfg(feature = "currency_code")]
        currency_code: "KES",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -0.023559,
            longitude: 37.906193,
            max_latitude: 5.033420899999999,
            max_longitude: 41.9069449,
            min_latitude: -4.724299999999999,
            min_longitude: 33.90982109999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 5.033420899999999,
                    lng: 41.9069449,
                },
                southwest: LatLng {
                    lat: -4.724299999999999,
                    lng: 33.90982109999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "000",
        #[cfg(feature = "ioc")]
        ioc: "KEN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Kenya",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Kenya",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "sw"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "sw"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Kenyan",
        #[cfg(feature = "number")]
        number: "404",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "KE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Kenya", "Kenia", "ケニア"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KG",
        #[cfg(feature = "alpha3")]
        alpha3: "KGZ",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "996",
        #[cfg(feature = "currency_code")]
        currency_code: "KGS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 41.20438,
            longitude: 74.766098,
            max_latitude: 43.2653569,
            max_longitude: 80.2281514,
            min_latitude: 39.180254,
            min_longitude: 69.250998,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.2653569,
                    lng: 80.2281514,
                },
                southwest: LatLng {
                    lat: 39.180254,
                    lng: 69.250998,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "KGZ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kyrgyz Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Kyrgyzstan",
        #[cfg(feature = "languages_official")]
        languages_official: &["ky", "ru"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ky", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Kirghiz",
        #[cfg(feature = "number")]
        number: "417",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "KG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Kyrgyzstan",
            "Kirgisistan",
            "Kirghizistan",
            "Kirguizistán",
            "キルギス",
            "Kirgizië",
            "Kyrgzstan",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KH",
        #[cfg(feature = "alpha3")]
        alpha3: "KHM",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "855",
        #[cfg(feature = "currency_code")]
        currency_code: "KHR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CB",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.565679,
            longitude: 104.990963,
            max_latitude: 14.6901791,
            max_longitude: 107.627687,
            min_latitude: 9.6007,
            min_longitude: 102.333542,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 14.6901791,
                    lng: 107.627687,
                },
                southwest: LatLng {
                    lat: 9.6007,
                    lng: 102.333542,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "CAM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Cambodia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cambodia",
        #[cfg(feature = "languages_official")]
        languages_official: &["km"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["km"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Cambodian",
        #[cfg(feature = "number")]
        number: "116",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5,6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "KH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Cambodia",
            "Kambodscha",
            "Cambodge",
            "Camboya",
            "カンボジア",
            "Cambodja",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KI",
        #[cfg(feature = "alpha3")]
        alpha3: "KIR",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "686",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -3.370417,
            longitude: -168.734039,
            max_latitude: 5.4082108,
            max_longitude: -145.1513674,
            min_latitude: -13.0502263,
            min_longitude: 168.8818359,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 5.4082108,
                    lng: -145.1513674,
                },
                southwest: LatLng {
                    lat: -13.0502263,
                    lng: 168.8818359,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "KIR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Kiribati",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Kiribati",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "I-Kiribati",
        #[cfg(feature = "number")]
        number: "296",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "KI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Kiribati", "キリバス"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KM",
        #[cfg(feature = "alpha3")]
        alpha3: "COM",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "269",
        #[cfg(feature = "currency_code")]
        currency_code: "KMF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -11.6455,
            longitude: 43.3333,
            max_latitude: -11.3373321,
            max_longitude: 44.5646666,
            min_latitude: -12.4687602,
            min_longitude: 43.1968689,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -11.3373321,
                    lng: 44.5646666,
                },
                southwest: LatLng {
                    lat: -12.4687602,
                    lng: 43.1968689,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "COM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Union of the Comoros",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Comoros",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Comoran",
        #[cfg(feature = "number")]
        number: "174",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "KM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Comoros",
            "Union der Komoren",
            "Comores",
            "コモロ",
            "Comoren",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KN",
        #[cfg(feature = "alpha3")]
        alpha3: "KNA",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "SC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 17.357822,
            longitude: -62.782998,
            max_latitude: 17.4205891,
            max_longitude: -62.52369989999999,
            min_latitude: 17.07861,
            min_longitude: -62.86949999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 17.4205891,
                    lng: -62.52369989999999,
                },
                southwest: LatLng {
                    lat: 17.07861,
                    lng: -62.86949999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "SKN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Saint Kitts and Nevis",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Kitts and Nevis",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Kittian and Nevisian",
        #[cfg(feature = "number")]
        number: "659",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "KN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saint Kitts and Nevis",
            "Föderation St. Kitts und Nevis",
            "Saint Kitts et Nevis",
            "Saint Kitts y Nevis",
            "セントクリストファー・ネイビス",
            "Saint Kitts en Nevis",
            "St. Kitts and Nevis",
            "St Kitts and Nevis",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KP",
        #[cfg(feature = "alpha3")]
        alpha3: "PRK",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "850",
        #[cfg(feature = "currency_code")]
        currency_code: "KPW",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 40.339852,
            longitude: 127.510093,
            max_latitude: 43.01159,
            max_longitude: 130.6990167,
            min_latitude: 37.5892001,
            min_longitude: 124.1491605,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.01159,
                    lng: 130.6990167,
                },
                southwest: LatLng {
                    lat: 37.5892001,
                    lng: 124.1491605,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PRK",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Democratic People's Republic of Korea",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Korea (Democratic People's Republic of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["ko"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ko"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "North Korean",
        #[cfg(feature = "number")]
        number: "408",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "KP",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Korea (North)",
            "North Korea",
            "Nordkorea",
            "Corée du Nord",
            "Corea del Norte",
            "朝鮮民主主義人民共和国",
            "Noord-Korea",
            "Korea Democratic People's Republic",
            "Korea (Democratic People s Republic of)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KR",
        #[cfg(feature = "alpha3")]
        alpha3: "KOR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "82",
        #[cfg(feature = "currency_code")]
        currency_code: "KRW",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 35.907757,
            longitude: 127.766922,
            max_latitude: 38.63400000000001,
            max_longitude: 131.1603,
            min_latitude: 33.0041,
            min_longitude: 124.5863,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 38.63400000000001,
                    lng: 131.1603,
                },
                southwest: LatLng {
                    lat: 33.0041,
                    lng: 124.5863,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "001",
        #[cfg(feature = "ioc")]
        ioc: "KOR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Korea",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Korea (Republic of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["ko"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ko"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "South Korean",
        #[cfg(feature = "number")]
        number: "410",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "KR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "South Korea",
            "Korea (South)",
            "Südkorea",
            "Corée du Sud",
            "Corea del Sur",
            "大韓民国",
            "Zuid-Korea",
            "Korea (Republic of)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KW",
        #[cfg(feature = "alpha3")]
        alpha3: "KWT",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "965",
        #[cfg(feature = "currency_code")]
        currency_code: "KWD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 29.31166,
            longitude: 47.481766,
            max_latitude: 30.1036993,
            max_longitude: 48.5184,
            min_latitude: 28.5244463,
            min_longitude: 46.55303989999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 30.1036993,
                    lng: 48.5184,
                },
                southwest: LatLng {
                    lat: 28.5244463,
                    lng: 46.55303989999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "KUW",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The State of Kuwait",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Kuwait",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Kuwaiti",
        #[cfg(feature = "number")]
        number: "414",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "KW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Kuwait", "الكويت", "Koweït", "クウェート", "Koeweit"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KY",
        #[cfg(feature = "alpha3")]
        alpha3: "CYM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "KYD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "CJ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 19.3133,
            longitude: -81.2546,
            max_latitude: 19.7616,
            max_longitude: -79.7191,
            min_latitude: 19.2538999,
            min_longitude: -81.42940010000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 19.7616,
                    lng: -79.7191,
                },
                southwest: LatLng {
                    lat: 19.2538999,
                    lng: -81.42940010000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "CAY",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Cayman Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Cayman Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Caymanian",
        #[cfg(feature = "number")]
        number: "136",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"KY\d-\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "KY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Cayman Islands",
            "Kaimaninseln",
            "Îles Caïmans",
            "Islas Caimán",
            "ケイマン諸島",
            "Caymaneilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "KZ",
        #[cfg(feature = "alpha3")]
        alpha3: "KAZ",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "7",
        #[cfg(feature = "currency_code")]
        currency_code: "KZT",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "KZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 48.019573,
            longitude: 66.923684,
            max_latitude: 55.4419839,
            max_longitude: 87.315415,
            min_latitude: 40.5685841,
            min_longitude: 46.493672,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 55.4419839,
                    lng: 87.315415,
                },
                southwest: LatLng {
                    lat: 40.5685841,
                    lng: 46.493672,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "KAZ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Kazakhstan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Kazakhstan",
        #[cfg(feature = "languages_official")]
        languages_official: &["kk", "ru"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["kk", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Kazakhstani",
        #[cfg(feature = "number")]
        number: "398",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "KZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Kazakhstan",
            "Kasachstan",
            "Kazajistán",
            "カザフスタン",
            "Kazachstan",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LA",
        #[cfg(feature = "alpha3")]
        alpha3: "LAO",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "856",
        #[cfg(feature = "currency_code")]
        currency_code: "LAK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 19.85627,
            longitude: 102.495496,
            max_latitude: 22.5090449,
            max_longitude: 107.635094,
            min_latitude: 13.9097198,
            min_longitude: 100.0832139,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 22.5090449,
                    lng: 107.635094,
                },
                southwest: LatLng {
                    lat: 13.9097198,
                    lng: 100.0832139,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LAO",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Lao People's Democratic Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Lao People's Democratic Republic",
        #[cfg(feature = "languages_official")]
        languages_official: &["lo"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["lo"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Laotian",
        #[cfg(feature = "number")]
        number: "418",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "LA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Laos",
            "ラオス人民民主共和国",
            "Lao People s Democratic Republic",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LB",
        #[cfg(feature = "alpha3")]
        alpha3: "LBN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "961",
        #[cfg(feature = "currency_code")]
        currency_code: "LBP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 33.854721,
            longitude: 35.862285,
            max_latitude: 34.69209,
            max_longitude: 36.62372,
            min_latitude: 33.0550256,
            min_longitude: 35.0711001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 34.69209,
                    lng: 36.62372,
                },
                southwest: LatLng {
                    lat: 33.0550256,
                    lng: 35.0711001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LBN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Lebanese Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Lebanon",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Lebanese",
        #[cfg(feature = "number")]
        number: "422",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:\d{4})(?: ?(?:\d{4}))?",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "LB",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Lebanon", "لبنان", "Libanon", "Liban", "Líbano", "レバノン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LC",
        #[cfg(feature = "alpha3")]
        alpha3: "LCA",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "ST",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 13.909444,
            longitude: -60.978893,
            max_latitude: 14.1209277,
            max_longitude: -60.85979460000001,
            min_latitude: 13.7047779,
            min_longitude: -61.0812378,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 14.1209277,
                    lng: -60.85979460000001,
                },
                southwest: LatLng {
                    lat: 13.7047779,
                    lng: -61.0812378,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "LCA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Saint Lucia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Lucia",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Saint Lucian",
        #[cfg(feature = "number")]
        number: "662",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "LC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saint Lucia",
            "Saint-Lucie",
            "Santa Lucía",
            "セントルシア",
            "St. Lucia",
            "St Lucia",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LI",
        #[cfg(feature = "alpha3")]
        alpha3: "LIE",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "423",
        #[cfg(feature = "currency_code")]
        currency_code: "CHF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 47.166,
            longitude: 9.555373,
            max_latitude: 47.2705467,
            max_longitude: 9.6356501,
            min_latitude: 47.04828999999999,
            min_longitude: 9.47162,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 47.2705467,
                    lng: 9.6356501,
                },
                southwest: LatLng {
                    lat: 47.04828999999999,
                    lng: 9.47162,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LIE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Principality of Liechtenstein",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Liechtenstein",
        #[cfg(feature = "languages_official")]
        languages_official: &["de"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["de"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Liechtensteiner",
        #[cfg(feature = "number")]
        number: "438",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"948[5-9]|949[0-8]",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "LI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Liechtenstein", "リヒテンシュタイン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LK",
        #[cfg(feature = "alpha3")]
        alpha3: "LKA",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "94",
        #[cfg(feature = "currency_code")]
        currency_code: "LKR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.873053999999999,
            longitude: 80.77179699999999,
            max_latitude: 10.03377,
            max_longitude: 82.14479999999999,
            min_latitude: 5.6816,
            min_longitude: 79.26769999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 10.03377,
                    lng: 82.14479999999999,
                },
                southwest: LatLng {
                    lat: 5.6816,
                    lng: 79.26769999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SRI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Democratic Socialist Republic of Sri Lanka",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Sri Lanka",
        #[cfg(feature = "languages_official")]
        languages_official: &["si", "ta"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["si", "ta"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Sri Lankan",
        #[cfg(feature = "number")]
        number: "144",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "LK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Sri Lanka", "スリランカ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LR",
        #[cfg(feature = "alpha3")]
        alpha3: "LBR",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "231",
        #[cfg(feature = "currency_code")]
        currency_code: "LRD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "LI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 6.428055,
            longitude: -9.429499000000002,
            max_latitude: 8.551986,
            max_longitude: -7.3692549,
            min_latitude: 4.269699999999999,
            min_longitude: -11.5355999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 8.551986,
                    lng: -7.3692549,
                },
                southwest: LatLng {
                    lat: 4.269699999999999,
                    lng: -11.5355999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LBR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Liberia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Liberia",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("22"),
        #[cfg(feature = "nationality")]
        nationality: "Liberian",
        #[cfg(feature = "number")]
        number: "430",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "LR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Liberia", "リベリア"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LS",
        #[cfg(feature = "alpha3")]
        alpha3: "LSO",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "266",
        #[cfg(feature = "currency_code")]
        currency_code: "LSL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -29.609988,
            longitude: 28.233608,
            max_latitude: -28.5708011,
            max_longitude: 29.4557087,
            min_latitude: -30.6755788,
            min_longitude: 27.011231,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -28.5708011,
                    lng: 29.4557087,
                },
                southwest: LatLng {
                    lat: -30.6755788,
                    lng: 27.011231,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LES",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Lesotho",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Lesotho",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "st"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "st"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Mosotho",
        #[cfg(feature = "number")]
        number: "426",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "LS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Lesotho", "レソト"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LT",
        #[cfg(feature = "alpha3")]
        alpha3: "LTU",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "370",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 55.169438,
            longitude: 23.881275,
            max_latitude: 56.45032089999999,
            max_longitude: 26.835523,
            min_latitude: 53.8967949,
            min_longitude: 20.931,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 56.45032089999999,
                    lng: 26.835523,
                },
                southwest: LatLng {
                    lat: 53.8967949,
                    lng: 20.931,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LTU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Lithuania",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Lithuania",
        #[cfg(feature = "languages_official")]
        languages_official: &["lt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["lt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Lithuanian",
        #[cfg(feature = "number")]
        number: "440",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "LT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Lithuania",
            "Litauen",
            "Lituanie",
            "Lituania",
            "リトアニア",
            "Litouwen",
            "Літва",
            "Lietuva",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LU",
        #[cfg(feature = "alpha3")]
        alpha3: "LUX",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "352",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 49.815273,
            longitude: 6.129582999999999,
            max_latitude: 50.18282,
            max_longitude: 6.530970099999999,
            min_latitude: 49.447779,
            min_longitude: 5.7356699,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 50.18282,
                    lng: 6.530970099999999,
                },
                southwest: LatLng {
                    lat: 49.447779,
                    lng: 5.7356699,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LUX",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Grand Duchy of Luxembourg",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Luxembourg",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "de", "lb"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "de", "lb"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Luxembourger",
        #[cfg(feature = "number")]
        number: "442",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "LU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Luxembourg", "Luxemburg", "Luxemburgo", "ルクセンブルク"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LV",
        #[cfg(feature = "alpha3")]
        alpha3: "LVA",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "371",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 56.879635,
            longitude: 24.603189,
            max_latitude: 58.0855688,
            max_longitude: 28.2414029,
            min_latitude: 55.6747769,
            min_longitude: 20.8465999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 58.0855688,
                    lng: 28.2414029,
                },
                southwest: LatLng {
                    lat: 55.6747769,
                    lng: 20.8465999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LAT",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Latvia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Latvia",
        #[cfg(feature = "languages_official")]
        languages_official: &["lv"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["lv"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Latvian",
        #[cfg(feature = "number")]
        number: "428",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"LV-\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "LV",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Latvia",
            "Lettland",
            "Lettonie",
            "Letonia",
            "ラトビア",
            "Letland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "LY",
        #[cfg(feature = "alpha3")]
        alpha3: "LBY",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "218",
        #[cfg(feature = "currency_code")]
        currency_code: "LYD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 26.3351,
            longitude: 17.228331,
            max_latitude: 33.2203,
            max_longitude: 25.2686,
            min_latitude: 19.5,
            min_longitude: 9.391466,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 33.2203,
                    lng: 25.2686,
                },
                southwest: LatLng {
                    lat: 19.5,
                    lng: 9.391466,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "LBA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The State of Libya",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Libya",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Libyan",
        #[cfg(feature = "number")]
        number: "434",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "LY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Libya",
            "ليبيا",
            "Libyen",
            "Libye",
            "Libia",
            "リビア",
            "Libië",
            "Libyan Arab Jamahiriya",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MA",
        #[cfg(feature = "alpha3")]
        alpha3: "MAR",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "212",
        #[cfg(feature = "currency_code")]
        currency_code: "MAD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 31.791702,
            longitude: -7.092619999999999,
            max_latitude: 35.9344,
            max_longitude: -0.9969759,
            min_latitude: 27.6672693,
            min_longitude: -13.3044001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 35.9344,
                    lng: -0.9969759,
                },
                southwest: LatLng {
                    lat: 27.6672693,
                    lng: -13.3044001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MAR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Morocco",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Morocco",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Moroccan",
        #[cfg(feature = "number")]
        number: "504",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "MA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Morocco",
            "المغرب",
            "Marokko",
            "Maroc",
            "Marruecos",
            "モロッコ",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MC",
        #[cfg(feature = "alpha3")]
        alpha3: "MCO",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "377",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 43.73841760000001,
            longitude: 7.424615799999999,
            max_latitude: 43.7519029,
            max_longitude: 7.4426,
            min_latitude: 43.7237999,
            min_longitude: 7.4091049,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.7519029,
                    lng: 7.4426,
                },
                southwest: LatLng {
                    lat: 43.7237999,
                    lng: 7.4091049,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MON",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Principality of Monaco",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Monaco",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Monegasque",
        #[cfg(feature = "number")]
        number: "492",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"980\d{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "MC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Monaco", "Mónaco", "モナコ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MD",
        #[cfg(feature = "alpha3")]
        alpha3: "MDA",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "373",
        #[cfg(feature = "currency_code")]
        currency_code: "MDL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MD",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 47.411631,
            longitude: 28.369885,
            max_latitude: 48.492029,
            max_longitude: 30.1635898,
            min_latitude: 45.4674379,
            min_longitude: 26.6164248,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 48.492029,
                    lng: 30.1635898,
                },
                southwest: LatLng {
                    lat: 45.4674379,
                    lng: 26.6164248,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MDA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Moldova",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Moldova (Republic of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["ro"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ro"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Moldovan",
        #[cfg(feature = "number")]
        number: "498",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "MD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Moldova",
            "Moldawien",
            "Moldavie",
            "Moldavia",
            "the Republic of Moldova",
            "モルドバ共和国",
            "Moldavië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ME",
        #[cfg(feature = "alpha3")]
        alpha3: "MNE",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "382",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MJ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 42.708678,
            longitude: 19.37439,
            max_latitude: 43.558743,
            max_longitude: 20.352926,
            min_latitude: 41.8297,
            min_longitude: 18.4337921,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.558743,
                    lng: 20.352926,
                },
                southwest: LatLng {
                    lat: 41.8297,
                    lng: 18.4337921,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "99",
        #[cfg(feature = "ioc")]
        ioc: "MNE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Montenegro",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Montenegro",
        #[cfg(feature = "languages_official")]
        languages_official: &["sr", "bs", "sq", "hr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sr", "bs", "sq", "hr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Montenegrin",
        #[cfg(feature = "number")]
        number: "499",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"8\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "ME",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Crna Gora", "Montenegro", "モンテネグロ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MF",
        #[cfg(feature = "alpha3")]
        alpha3: "MAF",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "590",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.08255,
            longitude: -63.05225100000001,
            max_latitude: 18.1356001,
            max_longitude: -62.9613001,
            min_latitude: 18.0462883,
            min_longitude: -63.1630001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.1356001,
                    lng: -62.9613001,
                },
                southwest: LatLng {
                    lat: 18.0462883,
                    lng: -63.1630001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Collectivity of Saint-Martin",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Martin (French part)",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "fr", "nl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "fr", "nl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "663",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78][01]\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "MF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saint Martin",
            "サン・マルタン（フランス領）",
            "Saint-Martin",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MG",
        #[cfg(feature = "alpha3")]
        alpha3: "MDG",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "261",
        #[cfg(feature = "currency_code")]
        currency_code: "MGA",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -18.766947,
            longitude: 46.869107,
            max_latitude: -11.4369999,
            max_longitude: 50.9985001,
            min_latitude: -26.2146,
            min_longitude: 42.7368,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -11.4369999,
                    lng: 50.9985001,
                },
                southwest: LatLng {
                    lat: -26.2146,
                    lng: 42.7368,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MAD",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Madagascar",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Madagascar",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "mg"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "mg"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Malagasy",
        #[cfg(feature = "number")]
        number: "450",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "MG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Madagascar",
            "Madagaskar",
            "the Republic of Madagascar",
            "マダガスカル",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MH",
        #[cfg(feature = "alpha3")]
        alpha3: "MHL",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "692",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.131474,
            longitude: 171.184478,
            max_latitude: 15.0190749,
            max_longitude: 172.5732421,
            min_latitude: 4.1601583,
            min_longitude: 159.8840332,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 15.0190749,
                    lng: 172.5732421,
                },
                southwest: LatLng {
                    lat: 4.1601583,
                    lng: 159.8840332,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MHL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of the Marshall Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Marshall Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "mh"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "mh"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Marshallese",
        #[cfg(feature = "number")]
        number: "584",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(969[67]\d)(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "MH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Marshall Islands",
            "Marshallinseln",
            "Îles Marshall",
            "Islas Marshall",
            "マーシャル諸島",
            "Marshalleilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MK",
        #[cfg(feature = "alpha3")]
        alpha3: "MKD",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "389",
        #[cfg(feature = "currency_code")]
        currency_code: "MKD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 41.608635,
            longitude: 21.745275,
            max_latitude: 42.373646,
            max_longitude: 23.0340441,
            min_latitude: 40.8537826,
            min_longitude: 20.452423,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 42.373646,
                    lng: 23.0340441,
                },
                southwest: LatLng {
                    lat: 40.8537826,
                    lng: 20.452423,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MKD",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of North Macedonia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "North Macedonia",
        #[cfg(feature = "languages_official")]
        languages_official: &["mk"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["mk"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Macedonian",
        #[cfg(feature = "number")]
        number: "807",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "MK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Macedonia",
            "Mazedonien",
            "Macédoine",
            "F.Y.R.O.M (Macedonia)",
            "マケドニア旧ユーゴスラビア共和国",
            "Macedonië [FYROM]",
            "Macedonia (The Former Yugoslav Republic of)",
            "North Macedonia",
            "Macedonia (FYROM)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ML",
        #[cfg(feature = "alpha3")]
        alpha3: "MLI",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "223",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ML",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 17.570692,
            longitude: -3.996166,
            max_latitude: 25.001084,
            max_longitude: 4.267382599999999,
            min_latitude: 10.147811,
            min_longitude: -12.2403447,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 25.001084,
                    lng: 4.267382599999999,
                },
                southwest: LatLng {
                    lat: 10.147811,
                    lng: -12.2403447,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MLI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Mali",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mali",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Malian",
        #[cfg(feature = "number")]
        number: "466",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ML",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Mali", "マリ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MM",
        #[cfg(feature = "alpha3")]
        alpha3: "MMR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "95",
        #[cfg(feature = "currency_code")]
        currency_code: "MMK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "BM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 21.916221,
            longitude: 95.955974,
            max_latitude: 28.5478351,
            max_longitude: 101.1702717,
            min_latitude: 9.4518,
            min_longitude: 92.171808,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 28.5478351,
                    lng: 101.1702717,
                },
                southwest: LatLng {
                    lat: 9.4518,
                    lng: 92.171808,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MYA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of the Union of Myanmar",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Myanmar",
        #[cfg(feature = "languages_official")]
        languages_official: &["my"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["my"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Myanmarian",
        #[cfg(feature = "number")]
        number: "104",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "MM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Myanmar (Burma)", "ミャンマー", "Burma"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MN",
        #[cfg(feature = "alpha3")]
        alpha3: "MNG",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "976",
        #[cfg(feature = "currency_code")]
        currency_code: "MNT",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 46.862496,
            longitude: 103.846656,
            max_latitude: 52.148355,
            max_longitude: 119.9315098,
            min_latitude: 41.581833,
            min_longitude: 87.7344789,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 52.148355,
                    lng: 119.9315098,
                },
                southwest: LatLng {
                    lat: 41.581833,
                    lng: 87.7344789,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "001",
        #[cfg(feature = "ioc")]
        ioc: "MGL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Mongolia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mongolia",
        #[cfg(feature = "languages_official")]
        languages_official: &["mn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["mn"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Mongolian",
        #[cfg(feature = "number")]
        number: "496",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "MN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Mongolia", "Mongolei", "Mongolie", "モンゴル", "Mongolië"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MO",
        #[cfg(feature = "alpha3")]
        alpha3: "MAC",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "853",
        #[cfg(feature = "currency_code")]
        currency_code: "MOP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 22.198745,
            longitude: 113.543873,
            max_latitude: 22.2170639,
            max_longitude: 113.6127001,
            min_latitude: 22.1066001,
            min_longitude: 113.5276053,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 22.2170639,
                    lng: 113.6127001,
                },
                southwest: LatLng {
                    lat: 22.1066001,
                    lng: 113.5276053,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Macao Special Administrative Region of China",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Macao",
        #[cfg(feature = "languages_official")]
        languages_official: &["zh", "pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["zh", "pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Chinese",
        #[cfg(feature = "number")]
        number: "446",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "MO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Macao", "Macau", "マカオ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MP",
        #[cfg(feature = "alpha3")]
        alpha3: "MNP",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "CQ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.0979,
            longitude: 145.6739,
            max_latitude: 20.6584862,
            max_longitude: 146.2060546,
            min_latitude: 13.9713848,
            min_longitude: 144.7668457,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 20.6584862,
                    lng: 146.2060546,
                },
                southwest: LatLng {
                    lat: 13.9713848,
                    lng: 144.7668457,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Commonwealth of the Northern Mariana Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Northern Mariana Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ch"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ch"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "American",
        #[cfg(feature = "number")]
        number: "580",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(9695[012])(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "MP",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Northern Mariana Islands",
            "Nördliche Marianen",
            "Mariannes du Nord",
            "Islas Marianas del Norte",
            "北マリアナ諸島",
            "Noordelijke Marianeneilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MQ",
        #[cfg(feature = "alpha3")]
        alpha3: "MTQ",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "596",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MB",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 14.641528,
            longitude: -61.024174,
            max_latitude: 14.8973451,
            max_longitude: -60.7856368,
            min_latitude: 14.370834,
            min_longitude: -61.24191279999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 14.8973451,
                    lng: -60.7856368,
                },
                southwest: LatLng {
                    lat: 14.370834,
                    lng: -61.24191279999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Martinique",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Martinique",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "474",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78]2\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "MQ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Martinique", "Martinica", "マルティニーク"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MR",
        #[cfg(feature = "alpha3")]
        alpha3: "MRT",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "222",
        #[cfg(feature = "currency_code")]
        currency_code: "MRU",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 21.00789,
            longitude: -10.940835,
            max_latitude: 27.3158916,
            max_longitude: -4.833334799999999,
            min_latitude: 14.721273,
            min_longitude: -17.0687276,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 27.3158916,
                    lng: -4.833334799999999,
                },
                southwest: LatLng {
                    lat: 14.721273,
                    lng: -17.0687276,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MTN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Islamic Republic of Mauritania",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mauritania",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Mauritanian",
        #[cfg(feature = "number")]
        number: "478",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "MR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Mauritania",
            "موريتانيا",
            "Mauretanien",
            "Mauritanie",
            "モーリタニア",
            "Mauritanië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MS",
        #[cfg(feature = "alpha3")]
        alpha3: "MSR",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 16.742498,
            longitude: -62.187366,
            max_latitude: 16.8260672,
            max_longitude: -62.14262009999999,
            min_latitude: 16.671007,
            min_longitude: -62.242584,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 16.8260672,
                    lng: -62.14262009999999,
                },
                southwest: LatLng {
                    lat: 16.671007,
                    lng: -62.242584,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Montserrat",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Montserrat",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Montserratian",
        #[cfg(feature = "number")]
        number: "500",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "MS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Montserrat", "モントセラト"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MT",
        #[cfg(feature = "alpha3")]
        alpha3: "MLT",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "356",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 35.937496,
            longitude: 14.375416,
            max_latitude: 36.0853,
            max_longitude: 14.5765999,
            min_latitude: 35.79960000000001,
            min_longitude: 14.1801001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 36.0853,
                    lng: 14.5765999,
                },
                southwest: LatLng {
                    lat: 35.79960000000001,
                    lng: 14.1801001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MLT",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Malta",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Malta",
        #[cfg(feature = "languages_official")]
        languages_official: &["mt", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["mt", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("21"),
        #[cfg(feature = "nationality")]
        nationality: "Maltese",
        #[cfg(feature = "number")]
        number: "470",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[A-Z]{3} ?\d{2,4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "MT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Malta", "Malte", "マルタ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MU",
        #[cfg(feature = "alpha3")]
        alpha3: "MUS",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "230",
        #[cfg(feature = "currency_code")]
        currency_code: "MUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -20.348404,
            longitude: 57.55215200000001,
            max_latitude: -10.0878538,
            max_longitude: 63.80859390000001,
            min_latitude: -20.7458403,
            min_longitude: 56.3159179,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -10.0878538,
                    lng: 63.80859390000001,
                },
                southwest: LatLng {
                    lat: -20.7458403,
                    lng: 56.3159179,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "020",
        #[cfg(feature = "ioc")]
        ioc: "MRI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Mauritius",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mauritius",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Mauritian",
        #[cfg(feature = "number")]
        number: "480",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}(?:\d{2}|[A-Z]{2}\d{3})",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "MU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Mauritius", "Île Maurice", "Mauricio", "モーリシャス"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MV",
        #[cfg(feature = "alpha3")]
        alpha3: "MDV",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "960",
        #[cfg(feature = "currency_code")]
        currency_code: "MVR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 3.202778,
            longitude: 73.22068,
            max_latitude: 7.5149809,
            max_longitude: 74.7290038,
            min_latitude: -1.2907844,
            min_longitude: 71.75170899999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 7.5149809,
                    lng: 74.7290038,
                },
                southwest: LatLng {
                    lat: -1.2907844,
                    lng: 71.75170899999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MDV",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Maldives",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Maldives",
        #[cfg(feature = "languages_official")]
        languages_official: &["dv"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["dv"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Maldivan",
        #[cfg(feature = "number")]
        number: "462",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "MV",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Maldives",
            "Malediven",
            "Maldivas",
            "モルディブ",
            "Maldiven",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MW",
        #[cfg(feature = "alpha3")]
        alpha3: "MWI",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "265",
        #[cfg(feature = "currency_code")]
        currency_code: "MWK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -13.254308,
            longitude: 34.301525,
            max_latitude: -9.3672272,
            max_longitude: 35.91857299999999,
            min_latitude: -17.1295216,
            min_longitude: 32.6725205,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -9.3672272,
                    lng: 35.91857299999999,
                },
                southwest: LatLng {
                    lat: -17.1295216,
                    lng: 32.6725205,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MAW",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Malawi",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Malawi",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ny"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ny"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Malawian",
        #[cfg(feature = "number")]
        number: "454",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "MW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Malawi", "マラウイ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MX",
        #[cfg(feature = "alpha3")]
        alpha3: "MEX",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "52",
        #[cfg(feature = "currency_code")]
        currency_code: "MXN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MX",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 23.634501,
            longitude: -102.552784,
            max_latitude: 32.7186534,
            max_longitude: -86.5887,
            min_latitude: 14.3895,
            min_longitude: -118.6523001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 32.7186534,
                    lng: -86.5887,
                },
                southwest: LatLng {
                    lat: 14.3895,
                    lng: -118.6523001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MEX",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The United Mexican States",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mexico",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("01"),
        #[cfg(feature = "nationality")]
        nationality: "Mexican",
        #[cfg(feature = "number")]
        number: "484",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "MX",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Mexico", "Mexiko", "Mexique", "México", "メキシコ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MY",
        #[cfg(feature = "alpha3")]
        alpha3: "MYS",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "60",
        #[cfg(feature = "currency_code")]
        currency_code: "MYR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 4.210484,
            longitude: 101.975766,
            max_latitude: 7.5191,
            max_longitude: 119.4000001,
            min_latitude: 0.8539281000000001,
            min_longitude: 98.9353999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 7.5191,
                    lng: 119.4000001,
                },
                southwest: LatLng {
                    lat: 0.8539281000000001,
                    lng: 98.9353999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MAS",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Malaysia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Malaysia",
        #[cfg(feature = "languages_official")]
        languages_official: &["ms", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ms", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Malaysian",
        #[cfg(feature = "number")]
        number: "458",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "MY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Malaysia", "Malaisie", "Malasia", "マレーシア", "Maleisië"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "MZ",
        #[cfg(feature = "alpha3")]
        alpha3: "MOZ",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "258",
        #[cfg(feature = "currency_code")]
        currency_code: "MZN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -18.665695,
            longitude: 35.529562,
            max_latitude: -10.3128929,
            max_longitude: 41.3965,
            min_latitude: -26.9612,
            min_longitude: 30.2155501,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -10.3128929,
                    lng: 41.3965,
                },
                southwest: LatLng {
                    lat: -26.9612,
                    lng: 30.2155501,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "MOZ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Mozambique",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mozambique",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Mozambican",
        #[cfg(feature = "number")]
        number: "508",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "MZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Mozambique", "Mosambik", "モザンビーク"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NA",
        #[cfg(feature = "alpha3")]
        alpha3: "NAM",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "264",
        #[cfg(feature = "currency_code")]
        currency_code: "NAD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "WA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -22.95764,
            longitude: 18.49041,
            max_latitude: -16.9634849,
            max_longitude: 25.261752,
            min_latitude: -28.97063889999999,
            min_longitude: 11.4696999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -16.9634849,
                    lng: 25.261752,
                },
                southwest: LatLng {
                    lat: -28.97063889999999,
                    lng: 11.4696999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NAM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Namibia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Namibia",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "af"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "af"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Namibian",
        #[cfg(feature = "number")]
        number: "516",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "NA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Namibia", "Namibie", "ナミビア", "Namibië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NC",
        #[cfg(feature = "alpha3")]
        alpha3: "NCL",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "687",
        #[cfg(feature = "currency_code")]
        currency_code: "XPF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -20.904305,
            longitude: 165.618042,
            max_latitude: -19.1607355,
            max_longitude: 168.3325194,
            min_latitude: -23.2514406,
            min_longitude: 163.3557129,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -19.1607355,
                    lng: 168.3325194,
                },
                southwest: LatLng {
                    lat: -23.2514406,
                    lng: 163.3557129,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "New Caledonia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "New Caledonia",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "New Caledonian",
        #[cfg(feature = "number")]
        number: "540",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"988\d{2}",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Melanesia",
        #[cfg(feature = "un_locode")]
        un_locode: "NC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "New Caledonia",
            "Neukaledonien",
            "Nouvelle-Calédonie",
            "Nueva Caledonia",
            "ニューカレドニア",
            "Nieuw-Caledonië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NE",
        #[cfg(feature = "alpha3")]
        alpha3: "NER",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "227",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 17.607789,
            longitude: 8.081666,
            max_latitude: 23.4999997,
            max_longitude: 15.9990339,
            min_latitude: 11.693756,
            min_longitude: 0.1617177,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 23.4999997,
                    lng: 15.9990339,
                },
                southwest: LatLng {
                    lat: 11.693756,
                    lng: 0.1617177,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NIG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of the Niger",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Niger",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Nigerian",
        #[cfg(feature = "number")]
        number: "562",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "NE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Niger", "Níger", "ニジェール"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NF",
        #[cfg(feature = "alpha3")]
        alpha3: "NFK",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "672",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -29.040835,
            longitude: 167.954712,
            max_latitude: -28.9929014,
            max_longitude: 167.9985523,
            min_latitude: -29.137506,
            min_longitude: 167.9134083,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -28.9929014,
                    lng: 167.9985523,
                },
                southwest: LatLng {
                    lat: -29.137506,
                    lng: 167.9134083,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of Norfolk Island",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Norfolk Island",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Norfolk Islander",
        #[cfg(feature = "number")]
        number: "574",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"2899",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Australia and New Zealand",
        #[cfg(feature = "un_locode")]
        un_locode: "NF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Norfolk Island",
            "Norfolkinsel",
            "Île de Norfolk",
            "Isla de Norfolk",
            "ノーフォーク島",
            "Norfolkeiland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NG",
        #[cfg(feature = "alpha3")]
        alpha3: "NGA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "234",
        #[cfg(feature = "currency_code")]
        currency_code: "NGN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 9.081999,
            longitude: 8.675277,
            max_latitude: 13.8856449,
            max_longitude: 14.677982,
            min_latitude: 4.1821001,
            min_longitude: 2.676932,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.8856449,
                    lng: 14.677982,
                },
                southwest: LatLng {
                    lat: 4.1821001,
                    lng: 2.676932,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "009",
        #[cfg(feature = "ioc")]
        ioc: "NGR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federal Republic of Nigeria",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Nigeria",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Nigerian",
        #[cfg(feature = "number")]
        number: "566",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "NG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Nigeria",
            "Nigéria",
            "the Federal Republic of Nigeria",
            "ナイジェリア",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NI",
        #[cfg(feature = "alpha3")]
        alpha3: "NIC",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "505",
        #[cfg(feature = "currency_code")]
        currency_code: "NIO",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.865416,
            longitude: -85.207229,
            max_latitude: 15.0297369,
            max_longitude: -82.2766,
            min_latitude: 10.7080549,
            min_longitude: -87.7588,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 15.0297369,
                    lng: -82.2766,
                },
                southwest: LatLng {
                    lat: 10.7080549,
                    lng: -87.7588,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NCA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Nicaragua",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Nicaragua",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Nicaraguan",
        #[cfg(feature = "number")]
        number: "558",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "NI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Nicaragua", "ニカラグア"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NL",
        #[cfg(feature = "alpha3")]
        alpha3: "NLD",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "31",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 52.132633,
            longitude: 5.291265999999999,
            max_latitude: 53.6316,
            max_longitude: 7.227510199999999,
            min_latitude: 50.75038379999999,
            min_longitude: 3.3316001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 53.6316,
                    lng: 7.227510199999999,
                },
                southwest: LatLng {
                    lat: 50.75038379999999,
                    lng: 3.3316001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NED",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of the Netherlands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Netherlands",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl", "fy"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Dutch",
        #[cfg(feature = "number")]
        number: "528",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4} ?[A-Z]{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "NL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Netherlands",
            "The Netherlands",
            "Niederlande",
            "Pays-Bas",
            "Países Bajos",
            "オランダ",
            "Nederland",
            "Нидерландия",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NO",
        #[cfg(feature = "alpha3")]
        alpha3: "NOR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "47",
        #[cfg(feature = "currency_code")]
        currency_code: "NOK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 60.47202399999999,
            longitude: 8.468945999999999,
            max_latitude: 71.30780000000001,
            max_longitude: 31.3549999,
            min_latitude: 57.8097,
            min_longitude: 4.0649,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 71.30780000000001,
                    lng: 31.3549999,
                },
                southwest: LatLng {
                    lat: 57.8097,
                    lng: 4.0649,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NOR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Norway",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Norway",
        #[cfg(feature = "languages_official")]
        languages_official: &["nb", "nn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nb", "nn"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Norwegian",
        #[cfg(feature = "number")]
        number: "578",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "NO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Norway",
            "Norwegen",
            "Norvège",
            "Noruega",
            "ノルウェー",
            "Noorwegen",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NP",
        #[cfg(feature = "alpha3")]
        alpha3: "NPL",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "977",
        #[cfg(feature = "currency_code")]
        currency_code: "NPR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 28.394857,
            longitude: 84.12400799999999,
            max_latitude: 30.4473898,
            max_longitude: 88.20182969999999,
            min_latitude: 26.3473741,
            min_longitude: 80.05846980000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 30.4473898,
                    lng: 88.20182969999999,
                },
                southwest: LatLng {
                    lat: 26.3473741,
                    lng: 80.05846980000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NEP",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federal Democratic Republic of Nepal",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Nepal",
        #[cfg(feature = "languages_official")]
        languages_official: &["ne"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ne", "mai", "bho", "new", "urd"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Nepalese",
        #[cfg(feature = "number")]
        number: "524",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "NP",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Nepal",
            "Népal",
            "the Federal Democratic Republic of Nepal",
            "ネパール",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NR",
        #[cfg(feature = "alpha3")]
        alpha3: "NRU",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "674",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NR",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -0.522778,
            longitude: 166.931503,
            max_latitude: -0.4978976000000001,
            max_longitude: 166.9631767,
            min_latitude: -0.5580623,
            min_longitude: 166.9071293,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -0.4978976000000001,
                    lng: 166.9631767,
                },
                southwest: LatLng {
                    lat: -0.5580623,
                    lng: 166.9071293,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NRU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Nauru",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Nauru",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "na"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "na"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Nauruan",
        #[cfg(feature = "number")]
        number: "520",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "NR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Nauru", "ナウル"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NU",
        #[cfg(feature = "alpha3")]
        alpha3: "NIU",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "683",
        #[cfg(feature = "currency_code")]
        currency_code: "NZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -19.054445,
            longitude: -169.867233,
            max_latitude: -18.952625,
            max_longitude: -169.7743248,
            min_latitude: -19.1555668,
            min_longitude: -169.9500846,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -18.952625,
                    lng: -169.7743248,
                },
                southwest: LatLng {
                    lat: -19.1555668,
                    lng: -169.9500846,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Niue",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Niue",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[4],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Niuean",
        #[cfg(feature = "number")]
        number: "570",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "NU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Niue", "ニウエ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "NZ",
        #[cfg(feature = "alpha3")]
        alpha3: "NZL",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "64",
        #[cfg(feature = "currency_code")]
        currency_code: "NZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -40.900557,
            longitude: 174.885971,
            max_latitude: -28.8773225,
            max_longitude: -175.1235077,
            min_latitude: -52.7224663,
            min_longitude: 165.7437641,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -28.8773225,
                    lng: -175.1235077,
                },
                southwest: LatLng {
                    lat: -52.7224663,
                    lng: 165.7437641,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "NZL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "New Zealand",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "New Zealand",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[1],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "New Zealander",
        #[cfg(feature = "number")]
        number: "554",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Australia and New Zealand",
        #[cfg(feature = "un_locode")]
        un_locode: "NZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "New Zealand",
            "Neuseeland",
            "Nouvelle Zélande",
            "Nueva Zelanda",
            "ニュージーランド",
            "Nieuw-Zeeland",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "OM",
        #[cfg(feature = "alpha3")]
        alpha3: "OMN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "968",
        #[cfg(feature = "currency_code")]
        currency_code: "OMR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 21.4735329,
            longitude: 55.975413,
            max_latitude: 26.4361001,
            max_longitude: 60.30399999999999,
            min_latitude: 16.4571999,
            min_longitude: 52.0000019,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 26.4361001,
                    lng: 60.30399999999999,
                },
                southwest: LatLng {
                    lat: 16.4571999,
                    lng: 52.0000019,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "OMA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Sultanate of Oman",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Oman",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Omani",
        #[cfg(feature = "number")]
        number: "512",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:PC )?\d{3}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "OM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Oman", "عمان", "Omán", "オマーン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PA",
        #[cfg(feature = "alpha3")]
        alpha3: "PAN",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "507",
        #[cfg(feature = "currency_code")]
        currency_code: "PAB",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 8.537981,
            longitude: -80.782127,
            max_latitude: 9.7145001,
            max_longitude: -77.1584879,
            min_latitude: 7.0409,
            min_longitude: -83.05224109999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 9.7145001,
                    lng: -77.1584879,
                },
                southwest: LatLng {
                    lat: 7.0409,
                    lng: -83.05224109999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PAN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Panamá",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Panama",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Panamanian",
        #[cfg(feature = "number")]
        number: "591",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "PA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Panama", "Panamá", "パナマ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PE",
        #[cfg(feature = "alpha3")]
        alpha3: "PER",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "51",
        #[cfg(feature = "currency_code")]
        currency_code: "PEN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -9.189967,
            longitude: -75.015152,
            max_latitude: -0.0387769,
            max_longitude: -68.65232879999999,
            min_latitude: -18.4483,
            min_longitude: -81.3867001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -0.0387769,
                    lng: -68.65232879999999,
                },
                southwest: LatLng {
                    lat: -18.4483,
                    lng: -81.3867001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PER",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Perú",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Peru",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Peruvian",
        #[cfg(feature = "number")]
        number: "604",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:LIMA \d{1,2}|CALLAO 0?\d)|[0-2]\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "PE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Peru", "Pérou", "Perú", "ペルー"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PF",
        #[cfg(feature = "alpha3")]
        alpha3: "PYF",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "689",
        #[cfg(feature = "currency_code")]
        currency_code: "XPF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -17.679742,
            longitude: -149.406843,
            max_latitude: -6.4682,
            max_longitude: -134.0551932,
            min_latitude: -28.61346,
            min_longitude: -155.125483,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -6.4682,
                    lng: -134.0551932,
                },
                southwest: LatLng {
                    lat: -28.61346,
                    lng: -155.125483,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "French Polynesia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "French Polynesia",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "French Polynesian",
        #[cfg(feature = "number")]
        number: "258",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"987\d{2}",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "PF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "French Polynesia",
            "Französisch-Polynesien",
            "Polynésie Française",
            "Polinesia Francesa",
            "フランス領ポリネシア",
            "Frans-Polynesië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PG",
        #[cfg(feature = "alpha3")]
        alpha3: "PNG",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "675",
        #[cfg(feature = "currency_code")]
        currency_code: "PGK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -6.314992999999999,
            longitude: 143.95555,
            max_latitude: -0.6702,
            max_longitude: 159.9609001,
            min_latitude: -12.0823,
            min_longitude: 140.8419695,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -0.6702,
                    lng: 159.9609001,
                },
                southwest: LatLng {
                    lat: -12.0823,
                    lng: 140.8419695,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "05",
        #[cfg(feature = "ioc")]
        ioc: "PNG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Independent State of Papua New Guinea",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Papua New Guinea",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Papua New Guinean",
        #[cfg(feature = "number")]
        number: "598",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Melanesia",
        #[cfg(feature = "un_locode")]
        un_locode: "PG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Papua New Guinea",
            "Papua-Neuguinea",
            "Papouasie Nouvelle-Guinée",
            "Papúa Nueva Guinea",
            "パプアニューギニア",
            "Papoea-Nieuw-Guinea",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PH",
        #[cfg(feature = "alpha3")]
        alpha3: "PHL",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "63",
        #[cfg(feature = "currency_code")]
        currency_code: "PHP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.879721,
            longitude: 121.774017,
            max_latitude: 21.2412572,
            max_longitude: 127.6444784,
            min_latitude: 4.2259,
            min_longitude: 116.1474999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 21.2412572,
                    lng: 127.6444784,
                },
                southwest: LatLng {
                    lat: 4.2259,
                    lng: 116.1474999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PHI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of the Philippines",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Philippines",
        #[cfg(feature = "languages_official")]
        languages_official: &["tl", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["tl", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Filipino",
        #[cfg(feature = "number")]
        number: "608",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "PH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Philippines",
            "Philippinen",
            "Filipinas",
            "フィリピン",
            "Filipijnen",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PK",
        #[cfg(feature = "alpha3")]
        alpha3: "PAK",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "92",
        #[cfg(feature = "currency_code")]
        currency_code: "PKR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 30.375321,
            longitude: 69.34511599999999,
            max_latitude: 37.0841069,
            max_longitude: 77.8231711,
            min_latitude: 23.6344999,
            min_longitude: 60.8729721,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 37.0841069,
                    lng: 77.8231711,
                },
                southwest: LatLng {
                    lat: 23.6344999,
                    lng: 60.8729721,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PAK",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Islamic Republic of Pakistan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Pakistan",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ur"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ur"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Pakistani",
        #[cfg(feature = "number")]
        number: "586",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "PK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Pakistan", "Paquistán", "パキスタン"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PL",
        #[cfg(feature = "alpha3")]
        alpha3: "POL",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "48",
        #[cfg(feature = "currency_code")]
        currency_code: "PLN",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 51.919438,
            longitude: 19.145136,
            max_latitude: 54.9054761,
            max_longitude: 24.1458931,
            min_latitude: 49.002025,
            min_longitude: 14.1228641,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 54.9054761,
                    lng: 24.1458931,
                },
                southwest: LatLng {
                    lat: 49.002025,
                    lng: 14.1228641,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "POL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Poland",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Poland",
        #[cfg(feature = "languages_official")]
        languages_official: &["pl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Polish",
        #[cfg(feature = "number")]
        number: "616",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{2}-\d{3}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "PL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Poland", "Polen", "Pologne", "Polonia", "ポーランド"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PM",
        #[cfg(feature = "alpha3")]
        alpha3: "SPM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "508",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SB",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 46.8852,
            longitude: -56.3159,
            max_latitude: 47.21579999999999,
            max_longitude: -55.98249999999999,
            min_latitude: 46.7003,
            min_longitude: -56.5233,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 47.21579999999999,
                    lng: -55.98249999999999,
                },
                southwest: LatLng {
                    lat: 46.7003,
                    lng: -56.5233,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Overseas Collectivity of Saint-Pierre and Miquelon",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Pierre and Miquelon",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "666",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78]5\d{2}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern America",
        #[cfg(feature = "un_locode")]
        un_locode: "PM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saint Pierre and Miquelon",
            "Saint-Pierre und Miquelon",
            "Saint-Pierre-et-Miquelon",
            "San Pedro y Miquelón",
            "サンピエール島・ミクロン島",
            "Saint Pierre en Miquelon",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PN",
        #[cfg(feature = "alpha3")]
        alpha3: "PCN",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "64",
        #[cfg(feature = "currency_code")]
        currency_code: "NZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -24.3767537,
            longitude: -128.3242376,
            max_latitude: -23.7928845,
            max_longitude: -124.5410156,
            min_latitude: -25.1776023,
            min_longitude: -130.9268188,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -23.7928845,
                    lng: -124.5410156,
                },
                southwest: LatLng {
                    lat: -25.1776023,
                    lng: -130.9268188,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Pitcairn, Henderson, Ducie and Oeno Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Pitcairn",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Pitcairn Islander",
        #[cfg(feature = "number")]
        number: "612",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"PCRN 1ZZ",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "PN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Pitcairn",
            "ピトケアン",
            "Pitcairneilanden",
            "Pitcairn Islands",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PR",
        #[cfg(feature = "alpha3")]
        alpha3: "PRI",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RQ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.220833,
            longitude: -66.590149,
            max_latitude: 18.5720479,
            max_longitude: -65.2105715,
            min_latitude: 17.8449191,
            min_longitude: -67.9611844,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.5720479,
                    lng: -65.2105715,
                },
                southwest: LatLng {
                    lat: 17.8449191,
                    lng: -67.9611844,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "PUR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Commonwealth of Puerto Rico",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Puerto Rico",
        #[cfg(feature = "languages_official")]
        languages_official: &["es", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Puerto Rican",
        #[cfg(feature = "number")]
        number: "630",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(00[679]\d{2})(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "PR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Puerto Rico", "プエルトリコ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PS",
        #[cfg(feature = "alpha3")]
        alpha3: "PSE",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "970",
        #[cfg(feature = "currency_code")]
        currency_code: "ILS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "WE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 31.952162,
            longitude: 35.233154,
            max_latitude: 32.5520999,
            max_longitude: 35.5740521,
            min_latitude: 31.219691,
            min_longitude: 34.21010010000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 32.5520999,
                    lng: 35.5740521,
                },
                southwest: LatLng {
                    lat: 31.219691,
                    lng: 34.21010010000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PLE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The State of Palestine",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Palestine, State of",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "he", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "he", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Palestinian",
        #[cfg(feature = "number")]
        number: "275",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "PS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Palestine",
            "فلسطين",
            "Palästina",
            "Palestina",
            "the Occupied Palestinian Territory",
            "パレスチナ",
            "Palestijnse gebieden",
            "Palestinian Territory Occupied",
            "Palestinian Authority",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PT",
        #[cfg(feature = "alpha3")]
        alpha3: "PRT",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "351",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 39.39987199999999,
            longitude: -8.224454,
            max_latitude: 42.1543111,
            max_longitude: -6.189159200000001,
            min_latitude: 32.2895,
            min_longitude: -31.4647999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 42.1543111,
                    lng: -6.189159200000001,
                },
                southwest: LatLng {
                    lat: 32.2895,
                    lng: -31.4647999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "POR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Portuguese Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Portugal",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Portuguese",
        #[cfg(feature = "number")]
        number: "620",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}-\d{3}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "PT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Portugal", "ポルトガル"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PW",
        #[cfg(feature = "alpha3")]
        alpha3: "PLW",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "680",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 7.514979999999999,
            longitude: 134.58252,
            max_latitude: 8.238674,
            max_longitude: 135.0769,
            min_latitude: 2.6394,
            min_longitude: 131.0115,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 8.238674,
                    lng: 135.0769,
                },
                southwest: LatLng {
                    lat: 2.6394,
                    lng: 131.0115,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "PLW",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Palau",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Palau",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Palauan",
        #[cfg(feature = "number")]
        number: "585",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(969(?:39|40))(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Micronesia",
        #[cfg(feature = "un_locode")]
        un_locode: "PW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Palau", "パラオ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "PY",
        #[cfg(feature = "alpha3")]
        alpha3: "PRY",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "595",
        #[cfg(feature = "currency_code")]
        currency_code: "PYG",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "PA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -23.442503,
            longitude: -58.443832,
            max_latitude: -19.2876589,
            max_longitude: -54.258562,
            min_latitude: -27.5817594,
            min_longitude: -62.63895230000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -19.2876589,
                    lng: -54.258562,
                },
                southwest: LatLng {
                    lat: -27.5817594,
                    lng: -62.63895230000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "002",
        #[cfg(feature = "ioc")]
        ioc: "PAR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Paraguay",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Paraguay",
        #[cfg(feature = "languages_official")]
        languages_official: &["es", "gn"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es", "gn"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Paraguayan",
        #[cfg(feature = "number")]
        number: "600",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "PY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Paraguay", "パラグアイ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "QA",
        #[cfg(feature = "alpha3")]
        alpha3: "QAT",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "974",
        #[cfg(feature = "currency_code")]
        currency_code: "QAR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "QA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 25.354826,
            longitude: 51.183884,
            max_latitude: 26.2171,
            max_longitude: 51.7144001,
            min_latitude: 24.471118,
            min_longitude: 50.7211001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 26.2171,
                    lng: 51.7144001,
                },
                southwest: LatLng {
                    lat: 24.471118,
                    lng: 50.7211001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "QAT",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The State of Qatar",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Qatar",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Qatari",
        #[cfg(feature = "number")]
        number: "634",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "QA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Qatar", "قطر", "Katar", "カタール"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "RE",
        #[cfg(feature = "alpha3")]
        alpha3: "REU",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "262",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -21.115141,
            longitude: 55.536384,
            max_latitude: -20.8671529,
            max_longitude: 55.84487919999999,
            min_latitude: -21.4035321,
            min_longitude: 55.209732,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -20.8671529,
                    lng: 55.84487919999999,
                },
                southwest: LatLng {
                    lat: -21.4035321,
                    lng: 55.209732,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Réunion",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Réunion",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "638",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"9[78]4\d{2}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "RE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Réunion", "Reunión", "Reunion", "レユニオン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "RO",
        #[cfg(feature = "alpha3")]
        alpha3: "ROU",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "40",
        #[cfg(feature = "currency_code")]
        currency_code: "RON",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 45.943161,
            longitude: 24.96676,
            max_latitude: 48.26518,
            max_longitude: 29.77839999999999,
            min_latitude: 43.6186193,
            min_longitude: 20.2617591,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 48.26518,
                    lng: 29.77839999999999,
                },
                southwest: LatLng {
                    lat: 43.6186193,
                    lng: 20.2617591,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ROU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Romania",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Romania",
        #[cfg(feature = "languages_official")]
        languages_official: &["ro"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ro"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Romanian",
        #[cfg(feature = "number")]
        number: "642",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "RO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Romania",
            "Rumänien",
            "Roumanie",
            "Rumania",
            "ルーマニア",
            "Roemenië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "RS",
        #[cfg(feature = "alpha3")]
        alpha3: "SRB",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "381",
        #[cfg(feature = "currency_code")]
        currency_code: "RSD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 44.016521,
            longitude: 21.005859,
            max_latitude: 46.190032,
            max_longitude: 23.0063095,
            min_latitude: 42.2315029,
            min_longitude: 18.8385221,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 46.190032,
                    lng: 23.0063095,
                },
                southwest: LatLng {
                    lat: 42.2315029,
                    lng: 18.8385221,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "99",
        #[cfg(feature = "ioc")]
        ioc: "SRB",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Serbia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Serbia",
        #[cfg(feature = "languages_official")]
        languages_official: &["sr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Serbian",
        #[cfg(feature = "number")]
        number: "688",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5,6}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "RS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Serbia", "Serbien", "Serbie", "セルビア", "Servië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "RU",
        #[cfg(feature = "alpha3")]
        alpha3: "RUS",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "7",
        #[cfg(feature = "currency_code")]
        currency_code: "RUB",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 61.52401,
            longitude: 105.318756,
            max_latitude: 82.1673907,
            max_longitude: -168.97788,
            min_latitude: 41.185353,
            min_longitude: 19.6160999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 82.1673907,
                    lng: -168.97788,
                },
                southwest: LatLng {
                    lat: 41.185353,
                    lng: 19.6160999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "RUS",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Russian Federation",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Russian Federation",
        #[cfg(feature = "languages_official")]
        languages_official: &["ru"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Russian",
        #[cfg(feature = "number")]
        number: "643",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "RU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Russia",
            "Russland",
            "Russie",
            "Rusia",
            "ロシア連邦",
            "Rusland",
            "Россия",
            "Расія",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "RW",
        #[cfg(feature = "alpha3")]
        alpha3: "RWA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "250",
        #[cfg(feature = "currency_code")]
        currency_code: "RWF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "RW",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -1.940278,
            longitude: 29.873888,
            max_latitude: -1.0473752,
            max_longitude: 30.8991179,
            min_latitude: -2.8399383,
            min_longitude: 28.861754,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -1.0473752,
                    lng: 30.8991179,
                },
                southwest: LatLng {
                    lat: -2.8399383,
                    lng: 28.861754,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "RWA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Rwanda",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Rwanda",
        #[cfg(feature = "languages_official")]
        languages_official: &["rw", "en", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["rw", "en", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Rwandan",
        #[cfg(feature = "number")]
        number: "646",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "RW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Rwanda", "Ruanda", "ルワンダ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SA",
        #[cfg(feature = "alpha3")]
        alpha3: "SAU",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "966",
        #[cfg(feature = "currency_code")]
        currency_code: "SAR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 23.885942,
            longitude: 45.079162,
            max_latitude: 32.154284,
            max_longitude: 55.6666999,
            min_latitude: 16.0036,
            min_longitude: 34.4815001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 32.154284,
                    lng: 55.6666999,
                },
                southwest: LatLng {
                    lat: 16.0036,
                    lng: 34.4815001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "KSA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Saudi Arabia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saudi Arabia",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Saudi Arabian",
        #[cfg(feature = "number")]
        number: "682",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "SA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saudi Arabia",
            "Kingdom of Saudi Arabia",
            "السعودية",
            "Saudi-Arabien",
            "Arabie Saoudite",
            "Arabia Saudí",
            "サウジアラビア",
            "Saoedi-Arabië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SB",
        #[cfg(feature = "alpha3")]
        alpha3: "SLB",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "677",
        #[cfg(feature = "currency_code")]
        currency_code: "SBD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "BP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -9.64571,
            longitude: 160.156194,
            max_latitude: -6.075011,
            max_longitude: 168.0249023,
            min_latitude: -12.6832149,
            min_longitude: 155.1187134,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -6.075011,
                    lng: 168.0249023,
                },
                southwest: LatLng {
                    lat: -12.6832149,
                    lng: 155.1187134,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SOL",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Solomon Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Solomon Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Solomon Islander",
        #[cfg(feature = "number")]
        number: "090",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Melanesia",
        #[cfg(feature = "un_locode")]
        un_locode: "SB",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Solomon Islands",
            "Salomonen",
            "Îles Salomon",
            "Islas Salomón",
            "ソロモン諸島",
            "Salomonseilanden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SC",
        #[cfg(feature = "alpha3")]
        alpha3: "SYC",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "248",
        #[cfg(feature = "currency_code")]
        currency_code: "SCR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -4.679574,
            longitude: 55.491977,
            max_latitude: -3.7091721,
            max_longitude: 56.3928224,
            min_latitude: -10.4716073,
            min_longitude: 45.9832764,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -3.7091721,
                    lng: 56.3928224,
                },
                southwest: LatLng {
                    lat: -10.4716073,
                    lng: 45.9832764,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SEY",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Seychelles",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Seychelles",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Seychellois",
        #[cfg(feature = "number")]
        number: "690",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Seychelles", "Seychellen", "セーシェル"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SD",
        #[cfg(feature = "alpha3")]
        alpha3: "SDN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "249",
        #[cfg(feature = "currency_code")]
        currency_code: "SDG",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.862807,
            longitude: 30.217636,
            max_latitude: 22.224918,
            max_longitude: 38.69379989999999,
            min_latitude: 9.3472209,
            min_longitude: 21.8146345,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 22.224918,
                    lng: 38.69379989999999,
                },
                southwest: LatLng {
                    lat: 9.3472209,
                    lng: 21.8146345,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SUD",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of the Sudan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Sudan",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Sudanese",
        #[cfg(feature = "number")]
        number: "729",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Sudan", "السودان", "Soudan", "Sudán", "スーダン", "Soedan"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SE",
        #[cfg(feature = "alpha3")]
        alpha3: "SWE",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "46",
        #[cfg(feature = "currency_code")]
        currency_code: "SEK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SW",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 60.12816100000001,
            longitude: 18.643501,
            max_latitude: 69.0599709,
            max_longitude: 24.1773101,
            min_latitude: 55.0059799,
            min_longitude: 10.5798,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 69.0599709,
                    lng: 24.1773101,
                },
                southwest: LatLng {
                    lat: 55.0059799,
                    lng: 10.5798,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SWE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Sweden",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Sweden",
        #[cfg(feature = "languages_official")]
        languages_official: &["sv"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sv"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Swedish",
        #[cfg(feature = "number")]
        number: "752",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3} ?\d{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "SE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Sweden",
            "Schweden",
            "Suède",
            "Suecia",
            "スウェーデン",
            "Zweden",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SG",
        #[cfg(feature = "alpha3")]
        alpha3: "SGP",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "65",
        #[cfg(feature = "currency_code")]
        currency_code: "SGD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 1.352083,
            longitude: 103.819836,
            max_latitude: 1.4784001,
            max_longitude: 104.0945001,
            min_latitude: 1.1496,
            min_longitude: 103.594,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 1.4784001,
                    lng: 104.0945001,
                },
                southwest: LatLng {
                    lat: 1.1496,
                    lng: 103.594,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "001",
        #[cfg(feature = "ioc")]
        ioc: "SGP",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Singapore",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Singapore",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ms", "ta"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ms", "ta"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Singaporean",
        #[cfg(feature = "number")]
        number: "702",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "SG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Singapore", "Singapur", "Singapour", "シンガポール"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SH",
        #[cfg(feature = "alpha3")]
        alpha3: "SHN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "290",
        #[cfg(feature = "currency_code")]
        currency_code: "SHP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "SH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -24.1434812,
            longitude: -10.0306945,
            max_latitude: -7.1008926,
            max_longitude: -5.0976561,
            min_latitude: -41.0371886,
            min_longitude: -15.4248047,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -7.1008926,
                    lng: -5.0976561,
                },
                southwest: LatLng {
                    lat: -41.0371886,
                    lng: -15.4248047,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Saint Helena, Ascension and Tristan da Cunha",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Helena, Ascension and Tristan da Cunha",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[4],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Saint Helenian",
        #[cfg(feature = "number")]
        number: "654",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(?:ASCN|STHL) 1ZZ",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saint Helena",
            "Sankt Helena",
            "Sainte Hélène",
            "Santa Helena",
            "セントヘレナ・アセンションおよびトリスタンダクーニャ",
            "Sint-Helena",
            "Saint Helena, Ascension and Tristan da Cunha",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SI",
        #[cfg(feature = "alpha3")]
        alpha3: "SVN",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "386",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 46.151241,
            longitude: 14.995463,
            max_latitude: 46.876659,
            max_longitude: 16.6107038,
            min_latitude: 45.4218356,
            min_longitude: 13.3753355,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 46.876659,
                    lng: 16.6107038,
                },
                southwest: LatLng {
                    lat: 45.4218356,
                    lng: 13.3753355,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SLO",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Slovenia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Slovenia",
        #[cfg(feature = "languages_official")]
        languages_official: &["sl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Slovene",
        #[cfg(feature = "number")]
        number: "705",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "SI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Slovenia",
            "Slowenien",
            "Slovénie",
            "Eslovenia",
            "スロベニア",
            "Slovenië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SJ",
        #[cfg(feature = "alpha3")]
        alpha3: "SJM",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "47",
        #[cfg(feature = "currency_code")]
        currency_code: "NOK",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 77.55360399999999,
            longitude: 23.6702719,
            max_latitude: 80.92842569999999,
            max_longitude: 34.8046879,
            min_latitude: 70.4662074,
            min_longitude: -10.5468752,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 80.92842569999999,
                    lng: 34.8046879,
                },
                southwest: LatLng {
                    lat: 70.4662074,
                    lng: -10.5468752,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Svalbard and Jan Mayen",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Svalbard and Jan Mayen",
        #[cfg(feature = "languages_official")]
        languages_official: &["no"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["no"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Norwegian",
        #[cfg(feature = "number")]
        number: "744",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "SJ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Svalbard and Jan Mayen",
            "Svalbard und Jan Mayen",
            "Îles Svalbard et Jan Mayen",
            "Islas Svalbard y Jan Mayen",
            "スヴァールバル諸島およびヤンマイエン島",
            "Svalbard en Jan Mayen",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SK",
        #[cfg(feature = "alpha3")]
        alpha3: "SVK",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "421",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "LO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 48.669026,
            longitude: 19.699024,
            max_latitude: 49.613805,
            max_longitude: 22.5658602,
            min_latitude: 47.731159,
            min_longitude: 16.8331821,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 49.613805,
                    lng: 22.5658602,
                },
                southwest: LatLng {
                    lat: 47.731159,
                    lng: 16.8331821,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SVK",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Slovak Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Slovakia",
        #[cfg(feature = "languages_official")]
        languages_official: &["sk"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sk"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Slovak",
        #[cfg(feature = "number")]
        number: "703",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3} ?\d{2}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "SK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Slovakia",
            "Slowakei",
            "Slovaquie",
            "República Eslovaca",
            "スロバキア",
            "Slowakije",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SL",
        #[cfg(feature = "alpha3")]
        alpha3: "SLE",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "232",
        #[cfg(feature = "currency_code")]
        currency_code: "SLL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 8.460555,
            longitude: -11.779889,
            max_latitude: 9.9999737,
            max_longitude: -10.2716829,
            min_latitude: 6.8446,
            min_longitude: -13.4032999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 9.9999737,
                    lng: -10.2716829,
                },
                southwest: LatLng {
                    lat: 6.8446,
                    lng: -13.4032999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SLE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Sierra Leone",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Sierra Leone",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Sierra Leonean",
        #[cfg(feature = "number")]
        number: "694",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Sierra Leone", "シエラレオネ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SM",
        #[cfg(feature = "alpha3")]
        alpha3: "SMR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "378",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 43.94236,
            longitude: 12.457777,
            max_latitude: 43.992075,
            max_longitude: 12.5167041,
            min_latitude: 43.8936809,
            min_longitude: 12.4034824,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 43.992075,
                    lng: 12.5167041,
                },
                southwest: LatLng {
                    lat: 43.8936809,
                    lng: 12.4034824,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SMR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of San Marino",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "San Marino",
        #[cfg(feature = "languages_official")]
        languages_official: &["it"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["it"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10, 11, 12],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Sammarinese",
        #[cfg(feature = "number")]
        number: "674",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"4789\d",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "SM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["San Marino", "Saint-Marin", "サンマリノ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SN",
        #[cfg(feature = "alpha3")]
        alpha3: "SEN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "221",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 14.497401,
            longitude: -14.452362,
            max_latitude: 16.6929572,
            max_longitude: -11.3457683,
            min_latitude: 12.2649001,
            min_longitude: -17.6879999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 16.6929572,
                    lng: -11.3457683,
                },
                southwest: LatLng {
                    lat: 12.2649001,
                    lng: -17.6879999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SEN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Senegal",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Senegal",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Senegalese",
        #[cfg(feature = "number")]
        number: "686",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Senegal", "Sénégal", "セネガル"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SO",
        #[cfg(feature = "alpha3")]
        alpha3: "SOM",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "252",
        #[cfg(feature = "currency_code")]
        currency_code: "SOS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 5.152149,
            longitude: 46.199616,
            max_latitude: 12.3615,
            max_longitude: 51.6138,
            min_latitude: -1.8673,
            min_longitude: 40.994373,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.3615,
                    lng: 51.6138,
                },
                southwest: LatLng {
                    lat: -1.8673,
                    lng: 40.994373,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SOM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Federal Republic of Somalia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Somalia",
        #[cfg(feature = "languages_official")]
        languages_official: &["so", "ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["so", "ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Somali",
        #[cfg(feature = "number")]
        number: "706",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[A-Z]{2} ?\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Somalia", "الصومال", "ソマリア", "Somalië"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SR",
        #[cfg(feature = "alpha3")]
        alpha3: "SUR",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "597",
        #[cfg(feature = "currency_code")]
        currency_code: "SRD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 3.919305,
            longitude: -56.027783,
            max_latitude: 6.1295999,
            max_longitude: -53.94289999999999,
            min_latitude: 1.837306,
            min_longitude: -58.07050590000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 6.1295999,
                    lng: -53.94289999999999,
                },
                southwest: LatLng {
                    lat: 1.837306,
                    lng: -58.07050590000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SUR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Suriname",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Suriname",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Surinamer",
        #[cfg(feature = "number")]
        number: "740",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "SR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Suriname", "Surinam", "スリナム"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SS",
        #[cfg(feature = "alpha3")]
        alpha3: "SSD",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "211",
        #[cfg(feature = "currency_code")]
        currency_code: "SSP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "OD",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 6.876991899999999,
            longitude: 31.3069788,
            max_latitude: 12.236389,
            max_longitude: 35.9489971,
            min_latitude: 3.48898,
            min_longitude: 23.4408491,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.236389,
                    lng: 35.9489971,
                },
                southwest: LatLng {
                    lat: 3.48898,
                    lng: 23.4408491,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "0",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of South Sudan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "South Sudan",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "South Sudanese",
        #[cfg(feature = "number")]
        number: "728",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["South Sudan", "Südsudan", "南スーダン", "Zuid-Soedan"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ST",
        #[cfg(feature = "alpha3")]
        alpha3: "STP",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "239",
        #[cfg(feature = "currency_code")]
        currency_code: "STD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 0.18636,
            longitude: 6.613080999999999,
            max_latitude: 1.8961687,
            max_longitude: 7.658843900000001,
            min_latitude: -0.09887689999999999,
            min_longitude: 6.328125,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 1.8961687,
                    lng: 7.658843900000001,
                },
                southwest: LatLng {
                    lat: -0.09887689999999999,
                    lng: 6.328125,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "STP",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Democratic Republic of São Tomé and Príncipe",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Sao Tome and Principe",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Sao Tomean",
        #[cfg(feature = "number")]
        number: "678",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ST",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "São Tomé and Príncipe",
            "São Tomé und Príncipe",
            "São Tomé et Príncipe",
            "Santo Tomé y Príncipe",
            "サントメ・プリンシペ",
            "Sao Tomé en Principe",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SV",
        #[cfg(feature = "alpha3")]
        alpha3: "SLV",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "503",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ES",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 13.794185,
            longitude: -88.89653,
            max_latitude: 14.4505567,
            max_longitude: -87.6682,
            min_latitude: 13.0473999,
            min_longitude: -90.19229999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 14.4505567,
                    lng: -87.6682,
                },
                southwest: LatLng {
                    lat: 13.0473999,
                    lng: -90.19229999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ESA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of El Salvador",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "El Salvador",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Salvadoran",
        #[cfg(feature = "number")]
        number: "222",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"CP [1-3][1-7][0-2]\d",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central America",
        #[cfg(feature = "un_locode")]
        un_locode: "SV",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["El Salvador", "Salvador", "エルサルバドル"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SX",
        #[cfg(feature = "alpha3")]
        alpha3: "SXM",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "ANG",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.04248,
            longitude: -63.05483,
            max_latitude: 18.0641707,
            max_longitude: -62.9784,
            min_latitude: 17.9941,
            min_longitude: -63.13979990000001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.0641707,
                    lng: -62.9784,
                },
                southwest: LatLng {
                    lat: 17.9941,
                    lng: -63.13979990000001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Sint Maarten",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Sint Maarten (Dutch part)",
        #[cfg(feature = "languages_official")]
        languages_official: &["nl", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["nl", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Dutch",
        #[cfg(feature = "number")]
        number: "534",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "SX",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Sint Maarten", "セント・マーチン島"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SY",
        #[cfg(feature = "alpha3")]
        alpha3: "SYR",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "963",
        #[cfg(feature = "currency_code")]
        currency_code: "SYP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 34.80207499999999,
            longitude: 38.996815,
            max_latitude: 37.318693,
            max_longitude: 42.376309,
            min_latitude: 32.311136,
            min_longitude: 35.62869999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 37.318693,
                    lng: 42.376309,
                },
                southwest: LatLng {
                    lat: 32.311136,
                    lng: 35.62869999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SYR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Syrian Arab Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Syrian Arab Republic",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Syrian",
        #[cfg(feature = "number")]
        number: "760",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "SY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Syria",
            "سوريا",
            "سورية",
            "Syrien",
            "Syrie",
            "Siria",
            "シリア・アラブ共和国",
            "Syrië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "SZ",
        #[cfg(feature = "alpha3")]
        alpha3: "SWZ",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "268",
        #[cfg(feature = "currency_code")]
        currency_code: "SZL",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "WZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -26.522503,
            longitude: 31.465866,
            max_latitude: -25.71792,
            max_longitude: 32.1349067,
            min_latitude: -27.317402,
            min_longitude: 30.79064,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -25.71792,
                    lng: 32.1349067,
                },
                southwest: LatLng {
                    lat: -27.317402,
                    lng: 30.79064,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SWZ",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Eswatini",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Eswatini",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "ss"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "ss"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Swazi",
        #[cfg(feature = "number")]
        number: "748",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"[HLMS]\d{3}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "SZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Swaziland", "Swasiland", "Suazilandia", "スワジランド"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TC",
        #[cfg(feature = "alpha3")]
        alpha3: "TCA",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "TK",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 21.694025,
            longitude: -71.797928,
            max_latitude: 22.0016285,
            max_longitude: -71.05949989999999,
            min_latitude: 21.1459922,
            min_longitude: -72.52069999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 22.0016285,
                    lng: -71.05949989999999,
                },
                southwest: LatLng {
                    lat: 21.1459922,
                    lng: -72.52069999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Turks and Caicos Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Turks and Caicos Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Turks and Caicos Islander",
        #[cfg(feature = "number")]
        number: "796",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"TKCA 1ZZ",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "TC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Turks and Caicos Islands",
            "Turks- und Caicosinseln",
            "Îles Turks et Caïcos",
            "Islas Turks y Caicos",
            "タークス・カイコス諸島",
            "Turks- en Caicoseilanden",
            "Turks and Caicos",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TD",
        #[cfg(feature = "alpha3")]
        alpha3: "TCD",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "235",
        #[cfg(feature = "currency_code")]
        currency_code: "XAF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "CD",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.454166,
            longitude: 18.732207,
            max_latitude: 23.449228,
            max_longitude: 24.0000011,
            min_latitude: 7.442975,
            min_longitude: 13.4699999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 23.449228,
                    lng: 24.0000011,
                },
                southwest: LatLng {
                    lat: 7.442975,
                    lng: 13.4699999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "15",
        #[cfg(feature = "ioc")]
        ioc: "CHA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Chad",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Chad",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Chadian",
        #[cfg(feature = "number")]
        number: "148",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Middle Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "TD",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Chad", "تشاد", "Tschad", "Tchad", "チャド", "Tsjaad"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TF",
        #[cfg(feature = "alpha3")]
        alpha3: "ATF",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "262",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "FS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -49.280366,
            longitude: 69.3485571,
            max_latitude: -45.7567331,
            max_longitude: 70.6558228,
            min_latitude: -50.0641918,
            min_longitude: 49.8519168,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -45.7567331,
                    lng: 70.6558228,
                },
                southwest: LatLng {
                    lat: -50.0641918,
                    lng: 49.8519168,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The French Southern and Antarctic Lands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "French Southern Territories",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "260",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "TF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "French Southern Territories",
            "Französische Süd- und Antarktisgebiete",
            "Terres Australes Françaises",
            "Territorios Franceses del Sur",
            "フランス領南方・南極地域",
            "Franse Gebieden in de zuidelijke Indische Oceaan",
            "French Southern and Antarctic Lands",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TG",
        #[cfg(feature = "alpha3")]
        alpha3: "TGO",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "228",
        #[cfg(feature = "currency_code")]
        currency_code: "XOF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TO",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 8.619543,
            longitude: 0.824782,
            max_latitude: 11.139617,
            max_longitude: 1.8089071,
            min_latitude: 6.0812,
            min_longitude: -0.1440418,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 11.139617,
                    lng: 1.8089071,
                },
                southwest: LatLng {
                    lat: 6.0812,
                    lng: -0.1440418,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "TOG",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Togolese Republic",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Togo",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Togolese",
        #[cfg(feature = "number")]
        number: "768",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "TG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Togo", "トーゴ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TH",
        #[cfg(feature = "alpha3")]
        alpha3: "THA",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "66",
        #[cfg(feature = "currency_code")]
        currency_code: "THB",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.870032,
            longitude: 100.992541,
            max_latitude: 20.465143,
            max_longitude: 105.636812,
            min_latitude: 5.613038,
            min_longitude: 97.343396,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 20.465143,
                    lng: 105.636812,
                },
                southwest: LatLng {
                    lat: 5.613038,
                    lng: 97.343396,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "001",
        #[cfg(feature = "ioc")]
        ioc: "THA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Thailand",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Thailand",
        #[cfg(feature = "languages_official")]
        languages_official: &["th"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["th"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Thai",
        #[cfg(feature = "number")]
        number: "764",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "TH",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Thailand", "Thaïlande", "Tailandia", "タイ", "ประเทศไทย"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TJ",
        #[cfg(feature = "alpha3")]
        alpha3: "TJK",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "992",
        #[cfg(feature = "currency_code")]
        currency_code: "TJS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 38.861034,
            longitude: 71.276093,
            max_latitude: 41.044367,
            max_longitude: 75.1539564,
            min_latitude: 36.6719898,
            min_longitude: 67.34201209999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 41.044367,
                    lng: 75.1539564,
                },
                southwest: LatLng {
                    lat: 36.6719898,
                    lng: 67.34201209999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "TJK",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Tajikistan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Tajikistan",
        #[cfg(feature = "languages_official")]
        languages_official: &["tg", "ru"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["tg", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Tadzhik",
        #[cfg(feature = "number")]
        number: "762",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "TJ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Tajikistan",
            "Tadschikistan",
            "Tayikistán",
            "タジキスタン",
            "Tadzjikistan",
            "Tajikstan",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TK",
        #[cfg(feature = "alpha3")]
        alpha3: "TKL",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "690",
        #[cfg(feature = "currency_code")]
        currency_code: "NZD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TL",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -9.200199999999999,
            longitude: -171.8484,
            max_latitude: -8.4221116,
            max_longitude: -171.0928346,
            min_latitude: -9.5059527,
            min_longitude: -172.6625061,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -8.4221116,
                    lng: -171.0928346,
                },
                southwest: LatLng {
                    lat: -9.5059527,
                    lng: -172.6625061,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Tokelau",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Tokelau",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[4],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Tokelauan",
        #[cfg(feature = "number")]
        number: "772",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "TK",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Tokelau", "Îles Tokelau", "Islas Tokelau", "トケラウ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TL",
        #[cfg(feature = "alpha3")]
        alpha3: "TLS",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "670",
        #[cfg(feature = "currency_code")]
        currency_code: "IDR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -8.874217,
            longitude: 125.727539,
            max_latitude: -8.048399999999999,
            max_longitude: 127.4249,
            min_latitude: -9.5303001,
            min_longitude: 124.0332,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -8.048399999999999,
                    lng: 127.4249,
                },
                southwest: LatLng {
                    lat: -9.5303001,
                    lng: 124.0332,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "None",
        #[cfg(feature = "ioc")]
        ioc: "TLS",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Democratic Republic of Timor-Leste",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Timor-Leste",
        #[cfg(feature = "languages_official")]
        languages_official: &["pt"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["pt"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "East Timorese",
        #[cfg(feature = "number")]
        number: "626",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "TL",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "East Timor",
            "Timor-Leste",
            "Timor oriental",
            "Timor Oriental",
            "東ティモール",
            "Oost-Timor",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TM",
        #[cfg(feature = "alpha3")]
        alpha3: "TKM",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "993",
        #[cfg(feature = "currency_code")]
        currency_code: "TMT",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TX",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 38.969719,
            longitude: 59.556278,
            max_latitude: 42.798844,
            max_longitude: 66.70735309999999,
            min_latitude: 35.12876,
            min_longitude: 52.3169,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 42.798844,
                    lng: 66.70735309999999,
                },
                southwest: LatLng {
                    lat: 35.12876,
                    lng: 52.3169,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "TKM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Turkmenistan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Turkmenistan",
        #[cfg(feature = "languages_official")]
        languages_official: &["tk"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["tk", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Turkmen",
        #[cfg(feature = "number")]
        number: "795",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "TM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Turkmenistan",
            "Turkménistan",
            "Turkmenistán",
            "トルクメニスタン",
            "Turkmenia",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TN",
        #[cfg(feature = "alpha3")]
        alpha3: "TUN",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "216",
        #[cfg(feature = "currency_code")]
        currency_code: "TND",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 33.886917,
            longitude: 9.537499,
            max_latitude: 37.5359,
            max_longitude: 11.599217,
            min_latitude: 30.2280339,
            min_longitude: 7.522311,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 37.5359,
                    lng: 11.599217,
                },
                southwest: LatLng {
                    lat: 30.2280339,
                    lng: 7.522311,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "TUN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Tunisia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Tunisia",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Tunisian",
        #[cfg(feature = "number")]
        number: "788",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "TN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Tunisia",
            "تونس",
            "Tunesien",
            "Tunisie",
            "Túnez",
            "チュニジア",
            "Tunesië",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TO",
        #[cfg(feature = "alpha3")]
        alpha3: "TON",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "676",
        #[cfg(feature = "currency_code")]
        currency_code: "TOP",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TN",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -21.178986,
            longitude: -175.198242,
            max_latitude: -15.4060236,
            max_longitude: -173.2543946,
            min_latitude: -21.8360059,
            min_longitude: -175.9570313,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -15.4060236,
                    lng: -173.2543946,
                },
                southwest: LatLng {
                    lat: -21.8360059,
                    lng: -175.9570313,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "TGA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Kingdom of Tonga",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Tonga",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "to"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "to"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5, 6, 7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Tongan",
        #[cfg(feature = "number")]
        number: "776",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "TO",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Tonga", "トンガ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TR",
        #[cfg(feature = "alpha3")]
        alpha3: "TUR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "90",
        #[cfg(feature = "currency_code")]
        currency_code: "TRY",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TU",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 38.963745,
            longitude: 35.243322,
            max_latitude: 42.3666999,
            max_longitude: 44.8178449,
            min_latitude: 35.808592,
            min_longitude: 25.5377,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 42.3666999,
                    lng: 44.8178449,
                },
                southwest: LatLng {
                    lat: 35.808592,
                    lng: 25.5377,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "TUR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Türkiye",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Türkiye",
        #[cfg(feature = "languages_official")]
        languages_official: &["tr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["tr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Turkish",
        #[cfg(feature = "number")]
        number: "792",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "TR",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Turkey",
            "Türkei",
            "Turquie",
            "Turquía",
            "トルコ",
            "Turkije",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TT",
        #[cfg(feature = "alpha3")]
        alpha3: "TTO",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "TTD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TD",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 10.691803,
            longitude: -61.222503,
            max_latitude: 11.4004,
            max_longitude: -60.45089989999999,
            min_latitude: 9.9930001,
            min_longitude: -61.9725001,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 11.4004,
                    lng: -60.45089989999999,
                },
                southwest: LatLng {
                    lat: 9.9930001,
                    lng: -61.9725001,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "TRI",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Trinidad and Tobago",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Trinidad and Tobago",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Trinidadian",
        #[cfg(feature = "number")]
        number: "780",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "TT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Trinidad and Tobago",
            "Trinidad und Tobago",
            "Trinité et Tobago",
            "Trinidad y Tobago",
            "トリニダード・トバゴ",
            "Trinidad en Tobago",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TV",
        #[cfg(feature = "alpha3")]
        alpha3: "TUV",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "688",
        #[cfg(feature = "currency_code")]
        currency_code: "AUD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TV",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -7.109534999999999,
            longitude: 177.64933,
            max_latitude: -5.4300853,
            max_longitude: 179.9999999,
            min_latitude: -11.1891797,
            min_longitude: 175.5615234,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -5.4300853,
                    lng: 179.9999999,
                },
                southwest: LatLng {
                    lat: -11.1891797,
                    lng: 175.5615234,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "TUV",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Tuvalu",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Tuvalu",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Tuvaluan",
        #[cfg(feature = "number")]
        number: "798",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "TV",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Tuvalu", "ツバル"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TW",
        #[cfg(feature = "alpha3")]
        alpha3: "TWN",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "886",
        #[cfg(feature = "currency_code")]
        currency_code: "TWD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TW",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 23.69781,
            longitude: 120.960515,
            max_latitude: 26.4545,
            max_longitude: 123.5021012,
            min_latitude: 20.5170001,
            min_longitude: 116.6665,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 26.4545,
                    lng: 123.5021012,
                },
                southwest: LatLng {
                    lat: 20.5170001,
                    lng: 116.6665,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "002",
        #[cfg(feature = "ioc")]
        ioc: "TPE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Taiwan, Province of China",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Taiwan, Province of China",
        #[cfg(feature = "languages_official")]
        languages_official: &["zh"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["zh"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Taiwanese",
        #[cfg(feature = "number")]
        number: "158",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{3}(?:\d{2,3})?",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "TW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Taiwan", "Taiwán", "台灣", "臺灣", "台湾"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "TZ",
        #[cfg(feature = "alpha3")]
        alpha3: "TZA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "255",
        #[cfg(feature = "currency_code")]
        currency_code: "TZS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "TZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -6.369028,
            longitude: 34.888822,
            max_latitude: -0.9843968000000001,
            max_longitude: 40.6398,
            min_latitude: -11.7612539,
            min_longitude: 29.34,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -0.9843968000000001,
                    lng: 40.6398,
                },
                southwest: LatLng {
                    lat: -11.7612539,
                    lng: 29.34,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "000",
        #[cfg(feature = "ioc")]
        ioc: "TAN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The United Republic of Tanzania",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Tanzania, United Republic of",
        #[cfg(feature = "languages_official")]
        languages_official: &["sw", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sw", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Tanzanian",
        #[cfg(feature = "number")]
        number: "834",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4,5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "TZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Tanzania",
            "Tansania",
            "Tanzanie",
            "タンザニア",
            "Tanzania United Republic",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "UA",
        #[cfg(feature = "alpha3")]
        alpha3: "UKR",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "380",
        #[cfg(feature = "currency_code")]
        currency_code: "UAH",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "UP",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 48.379433,
            longitude: 31.1655799,
            max_latitude: 52.3793713,
            max_longitude: 40.2204802,
            min_latitude: 44.2924,
            min_longitude: 22.137159,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 52.3793713,
                    lng: 40.2204802,
                },
                southwest: LatLng {
                    lat: 44.2924,
                    lng: 22.137159,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "UKR",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Ukraine",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Ukraine",
        #[cfg(feature = "languages_official")]
        languages_official: &["uk"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["uk"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Ukrainian",
        #[cfg(feature = "number")]
        number: "804",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "UA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Ukraine",
            "Ucrania",
            "ウクライナ",
            "Oekraïne",
            "Украина",
            "Україна",
            "Украіна",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "UG",
        #[cfg(feature = "alpha3")]
        alpha3: "UGA",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "256",
        #[cfg(feature = "currency_code")]
        currency_code: "UGX",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "UG",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 1.373333,
            longitude: 32.290275,
            max_latitude: 4.218628,
            max_longitude: 35.0330489,
            min_latitude: -1.4823178,
            min_longitude: 29.573433,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 4.218628,
                    lng: 35.0330489,
                },
                southwest: LatLng {
                    lat: -1.4823178,
                    lng: 29.573433,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "000",
        #[cfg(feature = "ioc")]
        ioc: "UGA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Uganda",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Uganda",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "sw"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "sw"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Ugandan",
        #[cfg(feature = "number")]
        number: "800",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "UG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Uganda", "ウガンダ", "Oeganda"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "UM",
        #[cfg(feature = "alpha3")]
        alpha3: "UMI",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 19.2823192,
            longitude: 166.647047,
            max_latitude: 28.3977184,
            max_longitude: -159.9849071,
            min_latitude: -0.3824678,
            min_longitude: 166.5989221,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 28.3977184,
                    lng: -159.9849071,
                },
                southwest: LatLng {
                    lat: -0.3824678,
                    lng: 166.5989221,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "United States Minor Outlying Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "United States Minor Outlying Islands",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some(""),
        #[cfg(feature = "nationality")]
        nationality: "American",
        #[cfg(feature = "number")]
        number: "581",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"96898",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Northern America",
        #[cfg(feature = "un_locode")]
        un_locode: "UM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "United States Minor Outlying Islands",
            "US-Amerikanische Hoheitsgebiete",
            "Dépendances américaines",
            "Islas menores de Estados Unidos",
            "合衆国領有小離島",
            "Kleine afgelegen eilanden van de Verenigde Staten",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "US",
        #[cfg(feature = "alpha3")]
        alpha3: "USA",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "US",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 37.09024,
            longitude: -95.712891,
            max_latitude: 71.3577635769,
            max_longitude: -66.96466,
            min_latitude: 18.91619,
            min_longitude: -171.791110603,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 71.3577635769,
                    lng: -66.96466,
                },
                southwest: LatLng {
                    lat: 18.91619,
                    lng: -171.791110603,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "USA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The United States of America",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "United States",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "American",
        #[cfg(feature = "number")]
        number: "840",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(\d{5})(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Northern America",
        #[cfg(feature = "un_locode")]
        un_locode: "US",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "United States",
            "USA",
            "Vereinigte Staaten von Amerika",
            "États-Unis",
            "Estados Unidos",
            "アメリカ合衆国",
            "Verenigde Staten",
            "Соединенные Штаты Америки",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "UY",
        #[cfg(feature = "alpha3")]
        alpha3: "URY",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "598",
        #[cfg(feature = "currency_code")]
        currency_code: "UYU",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "UY",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -32.522779,
            longitude: -55.765835,
            max_latitude: -30.0852149,
            max_longitude: -53.0779284,
            min_latitude: -35.1558001,
            min_longitude: -58.4913609,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -30.0852149,
                    lng: -53.0779284,
                },
                southwest: LatLng {
                    lat: -35.1558001,
                    lng: -58.4913609,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "URU",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Oriental Republic of Uruguay",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Uruguay",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Uruguayan",
        #[cfg(feature = "number")]
        number: "858",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "UY",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Uruguay", "ウルグアイ"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "UZ",
        #[cfg(feature = "alpha3")]
        alpha3: "UZB",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "998",
        #[cfg(feature = "currency_code")]
        currency_code: "UZS",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "UZ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 41.377491,
            longitude: 64.585262,
            max_latitude: 45.590075,
            max_longitude: 73.148946,
            min_latitude: 37.1722571,
            min_longitude: 55.9982179,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 45.590075,
                    lng: 73.148946,
                },
                southwest: LatLng {
                    lat: 37.1722571,
                    lng: 55.9982179,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "810",
        #[cfg(feature = "ioc")]
        ioc: "UZB",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Uzbekistan",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Uzbekistan",
        #[cfg(feature = "languages_official")]
        languages_official: &["uz"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["uz", "ru"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("8"),
        #[cfg(feature = "nationality")]
        nationality: "Uzbekistani",
        #[cfg(feature = "number")]
        number: "860",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{6}",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Central Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "UZ",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Uzbekistan",
            "Usbekistan",
            "Ouzbékistan",
            "Uzbekistán",
            "ウズベキスタン",
            "Oezbekistan",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VA",
        #[cfg(feature = "alpha3")]
        alpha3: "VAT",
        #[cfg(feature = "continent")]
        continent: "Europe",
        #[cfg(feature = "country_code")]
        country_code: "39",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "VT",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 41.902916,
            longitude: 12.453389,
            max_latitude: 41.90744309999999,
            max_longitude: 12.4583938,
            min_latitude: 41.9001896,
            min_longitude: 12.4457286,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 41.90744309999999,
                    lng: 12.4583938,
                },
                southwest: LatLng {
                    lat: 41.9001896,
                    lng: 12.4457286,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Holy See",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Holy See",
        #[cfg(feature = "languages_official")]
        languages_official: &["it", "la"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["it", "la"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Italian",
        #[cfg(feature = "number")]
        number: "336",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"00120",
        #[cfg(feature = "region")]
        region: "Europe",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Europe",
        #[cfg(feature = "un_locode")]
        un_locode: "VA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Vatican City",
            "Vatikan",
            "Cité du Vatican",
            "Ciudad del Vaticano",
            "バチカン市国",
            "Vaticaanstad",
            "Vatican City State (Holy See)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VC",
        #[cfg(feature = "alpha3")]
        alpha3: "VCT",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "XCD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "VC",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 12.984305,
            longitude: -61.287228,
            max_latitude: 13.4136657,
            max_longitude: -61.0846,
            min_latitude: 12.5294999,
            min_longitude: -61.4822,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 13.4136657,
                    lng: -61.0846,
                },
                southwest: LatLng {
                    lat: 12.5294999,
                    lng: -61.4822,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "VIN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "Saint Vincent and the Grenadines",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Saint Vincent and the Grenadines",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Saint Vincentian",
        #[cfg(feature = "number")]
        number: "670",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"VC\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "VC",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Saint Vincent and the Grenadines",
            "Saint Vincent und die Grenadinen",
            "Saint-Vincent et les Grenadines",
            "San Vicente y Granadinas",
            "セントビンセントおよびグレナディーン諸島",
            "Saint Vincent en de Grenadines",
            "St. Vincent Grenadines",
            "St Vincent Grenadines",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VE",
        #[cfg(feature = "alpha3")]
        alpha3: "VEN",
        #[cfg(feature = "continent")]
        continent: "South America",
        #[cfg(feature = "country_code")]
        country_code: "58",
        #[cfg(feature = "currency_code")]
        currency_code: "VES",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "VE",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 6.42375,
            longitude: -66.58973,
            max_latitude: 12.6886,
            max_longitude: -59.805666,
            min_latitude: 0.6475291,
            min_longitude: -73.36703899999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 12.6886,
                    lng: -59.805666,
                },
                southwest: LatLng {
                    lat: 0.6475291,
                    lng: -73.36703899999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "VEN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Bolivarian Republic of Venezuela",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Venezuela (Bolivarian Republic of)",
        #[cfg(feature = "languages_official")]
        languages_official: &["es"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["es"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Venezuelan",
        #[cfg(feature = "number")]
        number: "862",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South America",
        #[cfg(feature = "un_locode")]
        un_locode: "VE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Venezuela", "ベネズエラ・ボリバル共和国"],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VG",
        #[cfg(feature = "alpha3")]
        alpha3: "VGB",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "VI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.420695,
            longitude: -64.639968,
            max_latitude: 18.7539999,
            max_longitude: -64.2651999,
            min_latitude: 18.2899998,
            min_longitude: -64.8775,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.7539999,
                    lng: -64.2651999,
                },
                southwest: LatLng {
                    lat: 18.2899998,
                    lng: -64.8775,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "IVB",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Virgin Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Virgin Islands (British)",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Virgin Islander",
        #[cfg(feature = "number")]
        number: "092",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"VG\d{4}",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "VG",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "British Virgin Islands",
            "Britische Jungferninseln",
            "Îles Vierges britanniques",
            "Islas Vírgenes del Reino Unido",
            "イギリス領ヴァージン諸島",
            "Britse Maagdeneilanden",
            "Virgin Islands (British)",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VI",
        #[cfg(feature = "alpha3")]
        alpha3: "VIR",
        #[cfg(feature = "continent")]
        continent: "North America",
        #[cfg(feature = "country_code")]
        country_code: "1",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "VQ",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 18.335765,
            longitude: -64.896335,
            max_latitude: 18.4239,
            max_longitude: -64.4391,
            min_latitude: 17.5482999,
            min_longitude: -65.1101,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.4239,
                    lng: -64.4391,
                },
                southwest: LatLng {
                    lat: 17.5482999,
                    lng: -65.1101,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "011",
        #[cfg(feature = "ioc")]
        ioc: "ISV",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Virgin Islands of the United States",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Virgin Islands (U.S.)",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[3],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("1"),
        #[cfg(feature = "nationality")]
        nationality: "Virgin Islander",
        #[cfg(feature = "number")]
        number: "850",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"(008(?:(?:[0-4]\d)|(?:5[01])))(?:[ \-](\d{4}))?",
        #[cfg(feature = "region")]
        region: "Americas",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Caribbean",
        #[cfg(feature = "un_locode")]
        un_locode: "VI",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Virgin Islands of the United States",
            "Amerikanische Jungferninseln",
            "Îles Vierges américaines",
            "Islas Vírgenes de los Estados Unidos",
            "アメリカ領ヴァージン諸島",
            "Amerikaanse Maagdeneilanden",
            "Virgin Islands (U.S.)",
            "United States Virgin Islands",
            "U.S. Virgin Islands",
        ],
        #[cfg(feature = "world_region")]
        world_region: "AMER",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VN",
        #[cfg(feature = "alpha3")]
        alpha3: "VNM",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "84",
        #[cfg(feature = "currency_code")]
        currency_code: "VND",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "VM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 14.058324,
            longitude: 108.277199,
            max_latitude: 23.3926504,
            max_longitude: 109.6765,
            min_latitude: 8.1952001,
            min_longitude: 102.1439156,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 23.3926504,
                    lng: 109.6765,
                },
                southwest: LatLng {
                    lat: 8.1952001,
                    lng: 102.1439156,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "VIE",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Socialist Republic of Viet Nam",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Viet Nam",
        #[cfg(feature = "languages_official")]
        languages_official: &["vi"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["vi"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9, 10],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Vietnamese",
        #[cfg(feature = "number")]
        number: "704",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}\d?",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "South-Eastern Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "VN",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Vietnam", "ベトナム", "Viet Nam"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "VU",
        #[cfg(feature = "alpha3")]
        alpha3: "VUT",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "678",
        #[cfg(feature = "currency_code")]
        currency_code: "VUV",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "NH",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -15.376706,
            longitude: 166.959158,
            max_latitude: -12.8064449,
            max_longitude: 170.5023193,
            min_latitude: -20.5350773,
            min_longitude: 166.0583495,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -12.8064449,
                    lng: 170.5023193,
                },
                southwest: LatLng {
                    lat: -20.5350773,
                    lng: 166.0583495,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "VAN",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Vanuatu",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Vanuatu",
        #[cfg(feature = "languages_official")]
        languages_official: &["bi", "en", "fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["bi", "en", "fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[5, 6, 7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Ni-Vanuatu",
        #[cfg(feature = "number")]
        number: "548",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Melanesia",
        #[cfg(feature = "un_locode")]
        un_locode: "VU",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Vanuatu", "バヌアツ"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "WF",
        #[cfg(feature = "alpha3")]
        alpha3: "WLF",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "681",
        #[cfg(feature = "currency_code")]
        currency_code: "XPF",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "WF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -14.2938,
            longitude: -178.1165,
            max_latitude: -13.1303042,
            max_longitude: -176.0971068,
            min_latitude: -14.4187203,
            min_longitude: -178.2284546,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -13.1303042,
                    lng: -176.0971068,
                },
                southwest: LatLng {
                    lat: -14.4187203,
                    lng: -178.2284546,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "19",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Territory of the Wallis and Futuna Islands",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Wallis and Futuna",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Wallis and Futuna Islander",
        #[cfg(feature = "number")]
        number: "876",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"986\d{2}",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "WF",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "Wallis and Futuna",
            "Wallis und Futuna",
            "Wallis et Futuna",
            "Wallis y Futuna",
            "ウォリス・フツナ",
            "Wallis en Futuna",
        ],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "WS",
        #[cfg(feature = "alpha3")]
        alpha3: "WSM",
        #[cfg(feature = "continent")]
        continent: "Australia",
        #[cfg(feature = "country_code")]
        country_code: "685",
        #[cfg(feature = "currency_code")]
        currency_code: "WST",
        #[cfg(feature = "distance_unit")]
        distance_unit: "MI",
        #[cfg(feature = "gec")]
        gec: "WS",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -13.759029,
            longitude: -172.104629,
            max_latitude: -13.4203449,
            max_longitude: -171.3950515,
            min_latitude: -14.0833012,
            min_longitude: -172.8108215,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -13.4203449,
                    lng: -171.3950515,
                },
                southwest: LatLng {
                    lat: -14.0833012,
                    lng: -172.8108215,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "SAM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Independent State of Samoa",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Samoa",
        #[cfg(feature = "languages_official")]
        languages_official: &["sm", "en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["sm", "en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[6, 7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "Samoan",
        #[cfg(feature = "number")]
        number: "882",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Oceania",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Polynesia",
        #[cfg(feature = "un_locode")]
        un_locode: "WS",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Samoa", "サモア"],
        #[cfg(feature = "world_region")]
        world_region: "APAC",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "YE",
        #[cfg(feature = "alpha3")]
        alpha3: "YEM",
        #[cfg(feature = "continent")]
        continent: "Asia",
        #[cfg(feature = "country_code")]
        country_code: "967",
        #[cfg(feature = "currency_code")]
        currency_code: "YER",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "YM",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: 15.552727,
            longitude: 48.516388,
            max_latitude: 18.9996331,
            max_longitude: 54.67899999999999,
            min_latitude: 11.7975,
            min_longitude: 41.70959999999999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: 18.9996331,
                    lng: 54.67899999999999,
                },
                southwest: LatLng {
                    lat: 11.7975,
                    lng: 41.70959999999999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "YEM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Yemen",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Yemen",
        #[cfg(feature = "languages_official")]
        languages_official: &["ar"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["ar"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7, 8, 9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Yemeni",
        #[cfg(feature = "number")]
        number: "887",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Asia",
        #[cfg(feature = "start_of_week")]
        start_of_week: "sunday",
        #[cfg(feature = "subregion")]
        subregion: "Western Asia",
        #[cfg(feature = "un_locode")]
        un_locode: "YE",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Yemen", "اليمن", "Jemen", "Yémen", "イエメン"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "YT",
        #[cfg(feature = "alpha3")]
        alpha3: "MYT",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "262",
        #[cfg(feature = "currency_code")]
        currency_code: "EUR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "MF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -12.8275,
            longitude: 45.166244,
            max_latitude: -12.5772665,
            max_longitude: 45.32014849999999,
            min_latitude: -13.0358332,
            min_longitude: 44.9914169,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -12.5772665,
                    lng: 45.32014849999999,
                },
                southwest: LatLng {
                    lat: -13.0358332,
                    lng: 44.9914169,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Department of Mayotte",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Mayotte",
        #[cfg(feature = "languages_official")]
        languages_official: &["fr"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["fr"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[7],
        #[cfg(feature = "national_prefix")]
        national_prefix: None,
        #[cfg(feature = "nationality")]
        nationality: "French",
        #[cfg(feature = "number")]
        number: "175",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"976\d{2}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "YT",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Mayotte", "マヨット"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ZA",
        #[cfg(feature = "alpha3")]
        alpha3: "ZAF",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "27",
        #[cfg(feature = "currency_code")]
        currency_code: "ZAR",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "SF",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -30.559482,
            longitude: 22.937506,
            max_latitude: -22.1254239,
            max_longitude: 38.2216904,
            min_latitude: -47.1313489,
            min_longitude: 16.2816999,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -22.1254239,
                    lng: 38.2216904,
                },
                southwest: LatLng {
                    lat: -47.1313489,
                    lng: 16.2816999,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "09",
        #[cfg(feature = "ioc")]
        ioc: "RSA",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of South Africa",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "South Africa",
        #[cfg(feature = "languages_official")]
        languages_official: &["af", "en", "nr", "st", "ss", "tn", "ts", "ve", "xh", "zu"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["af", "en", "nr", "st", "ss", "tn", "ts", "ve", "xh", "zu"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "South African",
        #[cfg(feature = "number")]
        number: "710",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{4}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Southern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ZA",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &[
            "South Africa",
            "Republik Südafrika",
            "Afrique du Sud",
            "República de Sudáfrica",
            "南アフリカ",
            "Zuid-Afrika",
        ],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ZM",
        #[cfg(feature = "alpha3")]
        alpha3: "ZMB",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "260",
        #[cfg(feature = "currency_code")]
        currency_code: "ZMW",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ZA",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -13.133897,
            longitude: 27.849332,
            max_latitude: -8.2032838,
            max_longitude: 33.7090305,
            min_latitude: -18.0774179,
            min_longitude: 21.999351,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -8.2032838,
                    lng: 33.7090305,
                },
                southwest: LatLng {
                    lat: -18.0774179,
                    lng: 21.999351,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ZAM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Zambia",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Zambia",
        #[cfg(feature = "languages_official")]
        languages_official: &["en"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[9],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Zambian",
        #[cfg(feature = "number")]
        number: "894",
        #[cfg(feature = "postal_code")]
        postal_code: true,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"\d{5}",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ZM",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Zambia", "Sambia", "Zambie", "ザンビア"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
    Country {
        #[cfg(feature = "iso_code")]
        iso_code: "ZW",
        #[cfg(feature = "alpha3")]
        alpha3: "ZWE",
        #[cfg(feature = "continent")]
        continent: "Africa",
        #[cfg(feature = "country_code")]
        country_code: "263",
        #[cfg(feature = "currency_code")]
        currency_code: "USD",
        #[cfg(feature = "distance_unit")]
        distance_unit: "KM",
        #[cfg(feature = "gec")]
        gec: "ZI",
        #[cfg(feature = "geo")]
        geo: Geo {
            latitude: -19.015438,
            longitude: 29.154857,
            max_latitude: -15.6093188,
            max_longitude: 33.068236,
            min_latitude: -22.4219117,
            min_longitude: 25.237368,
            bounds: Bounds {
                northeast: LatLng {
                    lat: -15.6093188,
                    lng: 33.068236,
                },
                southwest: LatLng {
                    lat: -22.4219117,
                    lng: 25.237368,
                },
            },
        },
        #[cfg(feature = "international_prefix")]
        international_prefix: "00",
        #[cfg(feature = "ioc")]
        ioc: "ZIM",
        #[cfg(feature = "iso_long_name")]
        iso_long_name: "The Republic of Zimbabwe",
        #[cfg(feature = "iso_short_name")]
        iso_short_name: "Zimbabwe",
        #[cfg(feature = "languages_official")]
        languages_official: &["en", "sn", "nd"],
        #[cfg(feature = "languages_spoken")]
        languages_spoken: &["en", "sn", "nd"],
        #[cfg(feature = "national_destination_code_lengths")]
        national_destination_code_lengths: &[2],
        #[cfg(feature = "national_number_lengths")]
        national_number_lengths: &[8, 9, 10, 11],
        #[cfg(feature = "national_prefix")]
        national_prefix: Some("0"),
        #[cfg(feature = "nationality")]
        nationality: "Zimbabwean",
        #[cfg(feature = "number")]
        number: "716",
        #[cfg(feature = "postal_code")]
        postal_code: false,
        #[cfg(feature = "postal_code_format")]
        postal_code_format: r"",
        #[cfg(feature = "region")]
        region: "Africa",
        #[cfg(feature = "start_of_week")]
        start_of_week: "monday",
        #[cfg(feature = "subregion")]
        subregion: "Eastern Africa",
        #[cfg(feature = "un_locode")]
        un_locode: "ZW",
        #[cfg(feature = "unofficial_names")]
        unofficial_names: &["Zimbabwe", "Simbabwe", "Zimbabue", "ジンバブエ"],
        #[cfg(feature = "world_region")]
        world_region: "EMEA",
    },
];

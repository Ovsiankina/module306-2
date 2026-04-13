use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Category {
    All,
    Luxury,
    Fashion,
    Sport,
    Home,
    Kids,
}

#[derive(Clone)]
struct Store {
    name: String,
    category: String,
    level: String,
    unit: String,
    image: String,
}

pub fn Home() -> Element {
    let mut category = use_signal(|| Category::All);
    let mut search_query = use_signal(String::new);
    let mut email = use_signal(String::new);

    let stores = vec![
        Store {
            name: "GUCCI".to_string(),
            category: "LUXURY FASHION".to_string(),
            level: "LEVEL 2".to_string(),
            unit: "UNIT 245".to_string(),
            image: "/public/images/gucci-367.png".to_string(),
        },
        Store {
            name: "PRADA".to_string(),
            category: "LUXURY FASHION".to_string(),
            level: "LEVEL 1".to_string(),
            unit: "UNIT 112".to_string(),
            image: "/public/images/prada-378.png".to_string(),
        },
        Store {
            name: "NIKE FACTORY".to_string(),
            category: "SPORT & PERFORMANCE".to_string(),
            level: "LEVEL 0".to_string(),
            unit: "UNIT 056".to_string(),
            image: "/public/images/nike-389.png".to_string(),
        },
        Store {
            name: "ARMANI".to_string(),
            category: "PREMIUM FASHION".to_string(),
            level: "LEVEL 2".to_string(),
            unit: "UNIT 210".to_string(),
            image: "/public/images/armani-400.png".to_string(),
        },
        Store {
            name: "BURBERRY".to_string(),
            category: "LUXURY HERITAGE".to_string(),
            level: "LEVEL 1".to_string(),
            unit: "UNIT 150".to_string(),
            image: "/public/images/burberry-411.png".to_string(),
        },
        Store {
            name: "ADIDAS".to_string(),
            category: "SPORT & LIFESTYLE".to_string(),
            level: "LEVEL 0".to_string(),
            unit: "UNIT 020".to_string(),
            image: "/public/images/adidas-422.png".to_string(),
        },
        Store {
            name: "DOLCE & GABBANA".to_string(),
            category: "LUXURY FASHION".to_string(),
            level: "LEVEL 2".to_string(),
            unit: "UNIT 280".to_string(),
            image: "/public/images/dolce-gabbana-433.png".to_string(),
        },
        Store {
            name: "VALENTINO".to_string(),
            category: "LUXURY FASHION".to_string(),
            level: "LEVEL 1".to_string(),
            unit: "UNIT 195".to_string(),
            image: "/public/images/valentino-444.png".to_string(),
        },
    ];

    rsx! {
        div { class: "node-332",
            div { class: "main-333",
                // Header section
                div { class: "node-334",
                    div { class: "container-335",
                        div { class: "container-336",
                            p { class: "text-337 text-rgb-237-134-6",
                                "STORE DIRECTORY"
                            }
                        }
                    }
                    div { class: "heading-1-338",
                        p { class: "text-339 text-rgb-26-28-28",
                            "THE ARCHIVE OF EXCELLENCE."
                        }
                    }
                    div { class: "container-340",
                        p { class: "text-341 text-rgb-95-94-94",
                            "Discover over 160 stores from the most prestigious international brands with discounts from 30% to 70% all year round."
                        }
                    }
                }

                // Search bar
                div { class: "search-integration-342",
                    div { class: "container-343",
                        div { class: "input-344",
                            div { class: "container-345",
                                input {
                                    class: "w-full bg-transparent outline-none text-rgb-163-163-163 placeholder-gray-400",
                                    r#type: "text",
                                    placeholder: "FIND A BRAND...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value()),
                                }
                            }
                        }
                        div { class: "container-347",
                            img {
                                src: asset!("/public/images/icon-348.svg"),
                                class: "icon-348",
                                alt: "search"
                            }
                        }
                    }
                }
            }

            // Filter bar
            div { class: "section-filter-bar-349",
                div { class: "container-350",
                    button {
                        class: if category() == Category::All {
                            "button-351"
                        } else {
                            "button-353"
                        },
                        onclick: move |_| category.set(Category::All),
                        p { class: "text-352 text-rgb-26-28-28",
                            "ALL BRANDS"
                        }
                    }
                    button {
                        class: if category() == Category::Luxury {
                            "button-351"
                        } else {
                            "button-353"
                        },
                        onclick: move |_| category.set(Category::Luxury),
                        p { class: "text-354 text-rgb-95-94-94",
                            "LUXURY"
                        }
                    }
                    button {
                        class: if category() == Category::Fashion {
                            "button-351"
                        } else {
                            "button-353"
                        },
                        onclick: move |_| category.set(Category::Fashion),
                        p { class: "text-356 text-rgb-95-94-94",
                            "FASHION & ACCESSORIES"
                        }
                    }
                    button {
                        class: if category() == Category::Sport {
                            "button-351"
                        } else {
                            "button-353"
                        },
                        onclick: move |_| category.set(Category::Sport),
                        p { class: "text-358 text-rgb-95-94-94",
                            "SPORT & OUTDOOR"
                        }
                    }
                    button {
                        class: if category() == Category::Home {
                            "button-351"
                        } else {
                            "button-353"
                        },
                        onclick: move |_| category.set(Category::Home),
                        p { class: "text-360 text-rgb-95-94-94",
                            "HOME & LIFESTYLE"
                        }
                    }
                    button {
                        class: if category() == Category::Kids {
                            "button-351"
                        } else {
                            "button-353"
                        },
                        onclick: move |_| category.set(Category::Kids),
                        p { class: "text-362 text-rgb-95-94-94",
                            "KIDS"
                        }
                    }
                }
            }

            // Store grid
            div { class: "section-store-grid-363",
                for store in stores.iter() {
                    div { class: "store-card",
                        div { class: "margin-365",
                            div { class: "container-366",
                                img {
                                    src: "{store.image}",
                                    alt: "{store.name}",
                                    class: "store-image"
                                }
                            }
                        }
                        div { class: "container-368",
                            div { class: "heading-3-369",
                                p { class: "text-370 text-rgb-26-28-28",
                                    "{store.name}"
                                }
                            }
                            div { class: "container-371",
                                p { class: "text-372 text-rgb-237-134-6",
                                    "{store.category}"
                                }
                            }
                            div { class: "container-373",
                                p { class: "text-374 text-rgb-163-163-163",
                                    "{store.level} • {store.unit}"
                                }
                            }
                        }
                    }
                }
            }

            // Newsletter section
            div { class: "newsletter-section-452",
                div { class: "container-453",
                    div { class: "heading-2-margin-454",
                        div { class: "heading-2-455",
                            p { class: "text-456 text-white",
                                "NEVER MISS AN EXCLUSIVE DROP."
                            }
                        }
                    }
                    div { class: "margin-457",
                        div { class: "container-458",
                            p { class: "text-459 text-rgb-163-163-163",
                                "Join our private list to receive early access to seasonal sales, limited events, and new store openings at FoxTown."
                            }
                        }
                    }
                    div { class: "container-460",
                        div { class: "input-461",
                            div { class: "container-462",
                                input {
                                    class: "w-full bg-transparent outline-none text-rgb-82-82-82 placeholder-gray-400",
                                    r#type: "email",
                                    placeholder: "YOUR EMAIL ADDRESS",
                                    value: "{email}",
                                    oninput: move |e| email.set(e.value()),
                                }
                            }
                        }
                        div { class: "button-464",
                            p { class: "text-465 text-white",
                                "SUBSCRIBE"
                            }
                        }
                    }
                }
                div { class: "container-466",
                    img {
                        src: asset!("/public/images/editorial-fashion-467.png"),
                        class: "editorial-fashion-467",
                        alt: "editorial-fashion"
                    }
                }
            }

            // Footer
            div { class: "footer-468",
                div { class: "container-469",
                    div { class: "container-470",
                        p { class: "text-471 text-rgb-23-23-23",
                            "FOXTOWN"
                        }
                    }
                    div { class: "margin-472",
                        div { class: "container-473",
                            div { class: "link-474",
                                p { class: "text-475 text-rgb-163-163-163",
                                    "CONTACT"
                                }
                            }
                            div { class: "link-476",
                                p { class: "text-477 text-rgb-163-163-163",
                                    "DIRECTIONS"
                                }
                            }
                            div { class: "link-478",
                                p { class: "text-479 text-rgb-163-163-163",
                                    "PRIVACY"
                                }
                            }
                            div { class: "link-480",
                                p { class: "text-481 text-rgb-163-163-163",
                                    "TERMS"
                                }
                            }
                        }
                    }
                }
                div { class: "margin-482",
                    div { class: "horizontalborder-483",
                        div { class: "container-484",
                            p { class: "text-485 text-rgb-212-212-212",
                                "© 2024 FOXTOWN FACTORY STORES. ALL RIGHTS RESERVED."
                            }
                        }
                    }
                }
            }
        }
    }
}

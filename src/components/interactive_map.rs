use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum FloorLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

pub fn InteractiveMap() -> Element {
    let mut current_floor = use_signal(|| FloorLevel::Level0);
    let mut search_query = use_signal(String::new);

    let stores = vec![
        ("GUCCI", "L0 • 120"),
        ("PRADA", "L1 • 245"),
        ("ADIDAS", "L2 • 312"),
    ];

    let floor_map = match current_floor() {
        FloorLevel::Level0 => asset!("/public/images/node-80.png"),
        FloorLevel::Level1 => asset!("/public/images/node-80.png"),
        FloorLevel::Level2 => asset!("/public/images/node-80.png"),
        FloorLevel::Level3 => asset!("/public/images/node-80.png"),
    };

    rsx! {
        div { class: "node-1",
            div { class: "main-2",
                // Header section
                div { class: "node-3",
                    div { class: "heading-1-4",
                        p { class: "text-5 text-rgb-26-28-28 text-4xl font-bold",
                            "Navigate Luxury."
                        }
                    }
                    div { class: "container-6",
                        p { class: "text-7 text-rgb-95-94-94 text-lg",
                            "Explore three floors of premium outlet shopping. Locate your favorite boutiques, find exclusive dining, and plan your journey through FoxTown."
                        }
                    }
                }

                // Main content container
                div { class: "container-8",
                    // Left sidebar - Search and legend
                    div { class: "node-9 w-full lg:w-1/3",
                        // Store search
                        div { class: "store-search-10 p-6",
                            div { class: "heading-3-11 mb-4",
                                p { class: "text-12 text-rgb-237-134-6 text-lg font-bold",
                                    "Find a Store"
                                }
                            }

                            // Search box
                            div { class: "horizontalborder-13 border border-gray-200 rounded-lg overflow-hidden mb-4",
                                div { class: "input-14 p-4 bg-white",
                                    input {
                                        class: "w-full bg-transparent text-rgb-163-163-163 placeholder-gray-400 outline-none",
                                        r#type: "text",
                                        placeholder: "Search brands...",
                                        value: "{search_query}",
                                        oninput: move |e| search_query.set(e.value()),
                                    }
                                }
                                div { class: "container-17 p-4 bg-gray-50 flex items-center justify-center",
                                    img {
                                        src: asset!("/public/images/icon-18.svg"),
                                        class: "icon-18 w-5 h-5",
                                        alt: "search"
                                    }
                                }
                            }

                            // Search results
                            div { class: "container-19 space-y-2",
                                for (store_name, store_location) in stores.iter() {
                                    div { class: "background-20 border border-gray-200 rounded-lg p-4 hover:bg-gray-50 cursor-pointer transition",
                                        div { class: "container-21 mb-2",
                                            p { class: "text-22 text-rgb-26-28-28 font-bold",
                                                "{store_name}"
                                            }
                                        }
                                        div { class: "container-23",
                                            p { class: "text-24 text-rgb-163-163-163 text-sm",
                                                "{store_location}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Facilities legend
                        div { class: "legend-section-35 p-6 bg-gray-50 rounded-lg mt-6",
                            div { class: "heading-3-36 mb-4",
                                p { class: "text-37 text-rgb-26-28-28 font-bold text-lg",
                                    "Facilities"
                                }
                            }
                            div { class: "container-38 space-y-3",
                                div { class: "container-39 flex items-center gap-3",
                                    div { class: "container-40 flex-shrink-0",
                                        img {
                                            src: asset!("/public/images/icon-41.svg"),
                                            class: "icon-41 w-6 h-6",
                                            alt: "icon"
                                        }
                                    }
                                    div { class: "margin-42",
                                        p { class: "text-43 text-rgb-38-38-38",
                                            "Restrooms"
                                        }
                                    }
                                }
                                div { class: "container-44 flex items-center gap-3",
                                    div { class: "container-45 flex-shrink-0",
                                        img {
                                            src: asset!("/public/images/icon-46.svg"),
                                            class: "icon-46 w-6 h-6",
                                            alt: "icon"
                                        }
                                    }
                                    div { class: "margin-47",
                                        p { class: "text-48 text-rgb-38-38-38",
                                            "Elevators"
                                        }
                                    }
                                }
                                div { class: "container-49 flex items-start gap-3",
                                    div { class: "container-50 flex-shrink-0 mt-1",
                                        img {
                                            src: asset!("/public/images/icon-51.svg"),
                                            class: "icon-51 w-6 h-6",
                                            alt: "icon"
                                        }
                                    }
                                    div { class: "margin-52",
                                        div { class: "container-53",
                                            div { class: "container-54",
                                                p { class: "text-55 text-rgb-38-38-38 font-semibold",
                                                    "Food Court"
                                                }
                                            }
                                            div { class: "container-56",
                                                p { class: "text-57 text-rgb-237-134-6 text-sm",
                                                    "Level 2"
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "container-58 flex items-center gap-3",
                                    div { class: "container-59 flex-shrink-0",
                                        img {
                                            src: asset!("/public/images/icon-60.svg"),
                                            class: "icon-60 w-6 h-6",
                                            alt: "icon"
                                        }
                                    }
                                    div { class: "margin-61",
                                        p { class: "text-62 text-rgb-38-38-38",
                                            "Parking Access"
                                        }
                                    }
                                }
                                div { class: "container-63 flex items-center gap-3",
                                    div { class: "container-64 flex-shrink-0",
                                        img {
                                            src: asset!("/public/images/icon-65.svg"),
                                            class: "icon-65 w-6 h-6",
                                            alt: "icon"
                                        }
                                    }
                                    div { class: "margin-66",
                                        p { class: "text-67 text-rgb-38-38-38",
                                            "First Aid"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Right side - Map area
                    div { class: "main-map-area-68 w-full lg:w-2/3",
                        // Floor level selector
                        div { class: "floor-level-selector-69 flex gap-2 p-6",
                            div { class: "container-70 flex gap-2",
                                button {
                                    class: if current_floor() == FloorLevel::Level0 {
                                        "button-71 px-4 py-2 bg-gray-900 text-white rounded-lg font-bold"
                                    } else {
                                        "button-73 px-4 py-2 bg-white text-rgb-163-163-163 border border-gray-200 rounded-lg hover:bg-gray-50 transition"
                                    },
                                    onclick: move |_| current_floor.set(FloorLevel::Level0),
                                    "Level 0"
                                }
                                button {
                                    class: if current_floor() == FloorLevel::Level1 {
                                        "button-71 px-4 py-2 bg-gray-900 text-white rounded-lg font-bold"
                                    } else {
                                        "button-73 px-4 py-2 bg-white text-rgb-163-163-163 border border-gray-200 rounded-lg hover:bg-gray-50 transition"
                                    },
                                    onclick: move |_| current_floor.set(FloorLevel::Level1),
                                    "Level 1"
                                }
                                button {
                                    class: if current_floor() == FloorLevel::Level2 {
                                        "button-71 px-4 py-2 bg-gray-900 text-white rounded-lg font-bold"
                                    } else {
                                        "button-73 px-4 py-2 bg-white text-rgb-163-163-163 border border-gray-200 rounded-lg hover:bg-gray-50 transition"
                                    },
                                    onclick: move |_| current_floor.set(FloorLevel::Level2),
                                    "Level 2"
                                }
                                button {
                                    class: if current_floor() == FloorLevel::Level3 {
                                        "button-71 px-4 py-2 bg-gray-900 text-white rounded-lg font-bold"
                                    } else {
                                        "button-73 px-4 py-2 bg-white text-rgb-163-163-163 border border-gray-200 rounded-lg hover:bg-gray-50 transition"
                                    },
                                    onclick: move |_| current_floor.set(FloorLevel::Level3),
                                    "Level 3"
                                }
                            }
                        }

                        // Map canvas
                        div { class: "map-canvas-79 relative bg-gray-100 rounded-lg overflow-hidden p-4",
                            img {
                                src: "{floor_map}",
                                class: "node-80 w-full h-auto rounded",
                                alt: "shopping-center-floor-plan"
                            }

                            // Interactive elements overlay
                            div { class: "node-81 absolute top-4 left-4",
                                div { class: "background-border-82 bg-white border border-gray-200 rounded-lg p-4 shadow-sm",
                                    div { class: "overlay-shadow-83" }
                                    div { class: "container-84" }
                                    div { class: "margin-86",
                                        p { class: "text-87 text-rgb-26-28-28 text-sm font-semibold",
                                            "Pinch to Zoom & Explore"
                                        }
                                    }
                                }
                            }

                            // Zoom controls
                            div { class: "node-88 absolute bottom-4 right-4 flex flex-col gap-2",
                                div { class: "button-89 bg-white border border-gray-200 rounded-lg p-2 hover:bg-gray-50 cursor-pointer shadow-sm",
                                    img {
                                        src: asset!("/public/images/icon-92.svg"),
                                        class: "icon-92 w-5 h-5",
                                        alt: "zoom-in"
                                    }
                                }
                                div { class: "button-margin-93",
                                    div { class: "button-94 bg-white border border-gray-200 rounded-lg p-2 hover:bg-gray-50 cursor-pointer shadow-sm",
                                        img {
                                            src: asset!("/public/images/icon-97.svg"),
                                            class: "icon-97 w-5 h-5",
                                            alt: "zoom-out"
                                        }
                                    }
                                }
                                div { class: "button-margin-98",
                                    div { class: "button-99 bg-white border border-gray-200 rounded-lg p-2 hover:bg-gray-50 cursor-pointer shadow-sm",
                                        img {
                                            src: asset!("/public/images/icon-102.svg"),
                                            class: "icon-102 w-5 h-5",
                                            alt: "reset"
                                        }
                                    }
                                }
                            }

                            // North wing label
                            div { class: "node-103 absolute top-1/3 left-1/3",
                                div { class: "container-104",
                                    div { class: "background-border-105 border border-gray-400 rounded-lg p-2",
                                        div { class: "overlay-shadow-106" }
                                    }
                                    div { class: "background-107 bg-gray-700 text-white px-3 py-1 rounded text-sm font-bold",
                                        p { class: "text-108 text-white",
                                            "NORTH WING"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

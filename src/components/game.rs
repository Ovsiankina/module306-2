use dioxus::prelude::*;

#[derive(Clone)]
struct Winner {
    name: String,
    prize: String,
    time_ago: String,
}

#[derive(Clone)]
struct Brand {
    name: String,
    subtitle: String,
    image: String,
}

pub fn Game() -> Element {
    let mut spins_remaining = use_signal(|| 1);

    let winners = vec![
        Winner {
            name: "MARCO R.".to_string(),
            prize: "WON 20% OFF ARMANI".to_string(),
            time_ago: "2M AGO".to_string(),
        },
        Winner {
            name: "ELENA S.".to_string(),
            prize: "WON SHOPPING VOUCHER".to_string(),
            time_ago: "14M AGO".to_string(),
        },
        Winner {
            name: "JULIAN B.".to_string(),
            prize: "WON 15% OFF PRADA".to_string(),
            time_ago: "1H AGO".to_string(),
        },
    ];

    let brands = vec![
        Brand {
            name: "ARMANI".to_string(),
            subtitle: "Outlet Store".to_string(),
            image: "/public/images/armani-263.png".to_string(),
        },
        Brand {
            name: "BURBERRY".to_string(),
            subtitle: "Factory Store".to_string(),
            image: "/public/images/burberry-271.png".to_string(),
        },
        Brand {
            name: "PRADA".to_string(),
            subtitle: "Outlet".to_string(),
            image: "/public/images/prada-279.png".to_string(),
        },
        Brand {
            name: "GUCCI".to_string(),
            subtitle: "Factory Shop".to_string(),
            image: "/public/images/gucci-287.png".to_string(),
        },
    ];

    rsx! {
        div { class: "node-157 w-full",
            div { class: "main-158",
                // How to play section
                div { class: "how-to-play-section-159 py-12 px-4 md:px-8 lg:px-16",
                    div { class: "container-160 mb-12",
                        div { class: "container-161 mb-8",
                            div { class: "heading-2-162 mb-4",
                                p { class: "text-163 text-rgb-26-28-28 text-3xl font-bold",
                                    "HOW TO PLAY"
                                }
                            }
                            div { class: "background-164 w-20 h-1 bg-rgb-237-134-6" }
                        }

                        // Steps grid
                        div { class: "container-165 grid md:grid-cols-3 gap-8",
                            div { class: "step-1-166",
                                div { class: "overlay-167 mb-4",
                                    p { class: "text-168 text-rgb-237-134-6 text-4xl font-bold",
                                        "01"
                                    }
                                }
                                div { class: "heading-4-169 mb-3",
                                    p { class: "text-170 text-rgb-26-28-28 text-xl font-bold",
                                        "LOG IN"
                                    }
                                }
                                div { class: "container-171",
                                    p { class: "text-172 text-rgb-95-94-94 text-sm leading-relaxed",
                                        "Sign in to your FoxTown Rewards account to access your daily lucky spin and save your winnings instantly."
                                    }
                                }
                            }

                            div { class: "step-2-173",
                                div { class: "overlay-174 mb-4",
                                    p { class: "text-175 text-rgb-237-134-6 text-4xl font-bold",
                                        "02"
                                    }
                                }
                                div { class: "heading-4-176 mb-3",
                                    p { class: "text-177 text-rgb-26-28-28 text-xl font-bold",
                                        "SPIN DAILY"
                                    }
                                }
                                div { class: "container-178",
                                    p { class: "text-179 text-rgb-95-94-94 text-sm leading-relaxed",
                                        "Every 24 hours you get a fresh chance. The wheel is reset with new designer perks and exclusive boutique vouchers."
                                    }
                                }
                            }

                            div { class: "step-3-180",
                                div { class: "overlay-181 mb-4",
                                    p { class: "text-182 text-rgb-237-134-6 text-4xl font-bold",
                                        "03"
                                    }
                                }
                                div { class: "heading-4-183 mb-3",
                                    p { class: "text-184 text-rgb-26-28-28 text-xl font-bold",
                                        "REDEEM"
                                    }
                                }
                                div { class: "container-185",
                                    p { class: "text-186 text-rgb-95-94-94 text-sm leading-relaxed",
                                        "Present your winning digital voucher at the participating boutique in FoxTown Mendrisio to claim your prize."
                                    }
                                }
                            }
                        }
                    }
                }

                // Hero game section
                div { class: "hero-game-section-187 py-12 px-4 md:px-8 lg:px-16 bg-gradient-to-b from-gray-900 to-gray-800 text-white",
                    div { class: "container-192 grid md:grid-cols-2 gap-12 items-center",
                        // Left content
                        div { class: "left-content-193",
                            div { class: "container-194 mb-8",
                                div { class: "overlay-195 mb-4",
                                    p { class: "text-196 text-rgb-237-134-6 text-sm font-bold tracking-widest",
                                        "EXCLUSIVE REWARDS"
                                    }
                                }
                                div { class: "heading-1-197 mb-4",
                                    p { class: "text-198 text-5xl font-bold",
                                        "SPIN"
                                    }
                                    p { class: "text-rgb-237-134-6 text-5xl font-bold",
                                        "TO WIN"
                                    }
                                }
                                div { class: "container-199",
                                    p { class: "text-200 text-gray-300 text-lg leading-relaxed",
                                        "Experience the thrill of luxury rewards. Spin the wheel daily for a chance to unlock exclusive designer discounts and premium boutique prizes."
                                    }
                                }
                            }
                        }

                        // Right content - Recent winners
                        div { class: "node-201",
                            div { class: "container-202 mb-6",
                                div { class: "heading-3-203 mb-3",
                                    p { class: "text-204 text-white text-2xl font-bold",
                                        "Recent Winners"
                                    }
                                }
                                div { class: "container-205 flex items-center gap-2",
                                    div { class: "background-206 w-3 h-3 bg-rgb-237-134-6 rounded-full" }
                                    p { class: "text-207 text-rgb-237-134-6 text-sm font-bold",
                                        "LIVE"
                                    }
                                }
                            }

                            div { class: "container-208 space-y-3",
                                for winner in winners.iter() {
                                    div { class: "winner-1-209 flex gap-4 p-4 bg-gray-700 bg-opacity-50 rounded-lg hover:bg-opacity-75 transition",
                                        div { class: "background-210 flex-shrink-0",
                                            div { class: "container-211 w-12 h-12 bg-gray-600 rounded-full flex items-center justify-center",
                                                img {
                                                    src: asset!("/public/images/icon-212.svg"),
                                                    class: "icon-212 w-6 h-6",
                                                    alt: "avatar"
                                                }
                                            }
                                        }
                                        div { class: "margin-213 flex-grow",
                                            div { class: "container-214 mb-1",
                                                div { class: "container-215 mb-1",
                                                    p { class: "text-216 text-white font-semibold text-sm",
                                                        "{winner.name}"
                                                    }
                                                }
                                                div { class: "container-217",
                                                    p { class: "text-218 text-rgb-237-134-6 text-xs font-bold",
                                                        "{winner.prize}"
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "margin-219",
                                            p { class: "text-220 text-rgb-163-163-163 text-xs",
                                                "{winner.time_ago}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Spin button section
                    div { class: "node-245 mt-12 flex flex-col items-center justify-center",
                        div { class: "node-246",
                            div { class: "spin-button-counter-247",
                                button {
                                    class: "button-248 px-8 py-4 bg-rgb-237-134-6 text-gray-900 text-lg font-bold rounded-lg hover:bg-yellow-500 transition disabled:opacity-50 disabled:cursor-not-allowed",
                                    disabled: spins_remaining() == 0,
                                    onclick: move |_| {
                                        if spins_remaining() > 0 {
                                            spins_remaining.set(spins_remaining() - 1);
                                        }
                                    },
                                    "SPIN THE WHEEL"
                                }
                                div { class: "container-250 mt-3",
                                    p { class: "text-251 text-rgb-163-163-163 text-sm",
                                        "{spins_remaining()} SPIN"
                                        if spins_remaining() != 1 { "S" } else { "" }
                                        " REMAINING TODAY"
                                    }
                                }
                            }
                        }
                    }
                }

                // Participating brands section
                div { class: "node-252 py-12 px-4 md:px-8 lg:px-16 bg-gray-50",
                    div { class: "container-253 mb-8",
                        div { class: "container-254",
                            div { class: "heading-2-255 mb-4",
                                p { class: "text-256 text-rgb-26-28-28 text-3xl font-bold",
                                    "PARTICIPATING BRANDS"
                                }
                            }
                            div { class: "container-257",
                                p { class: "text-258 text-rgb-95-94-94 text-lg",
                                    "Discover exclusive prizes redeemable at over 160 luxury stores across the outlet."
                                }
                            }
                        }
                        div { class: "link-259 mt-4",
                            p { class: "text-260 text-rgb-237-134-6 font-bold cursor-pointer hover:underline",
                                "VIEW ALL STORES"
                            }
                        }
                    }

                    // Brand cards grid
                    div { class: "container-261 grid md:grid-cols-2 lg:grid-cols-4 gap-6",
                        for brand in brands.iter() {
                            div { class: "brand-cards-262 relative group overflow-hidden rounded-lg h-64 cursor-pointer",
                                img {
                                    src: "{brand.image}",
                                    class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                                    alt: "{brand.name}"
                                }
                                div { class: "gradient-264 absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent" }
                                div { class: "container-265 absolute bottom-0 left-0 right-0 p-6 text-white",
                                    div { class: "container-266 mb-2",
                                        p { class: "text-267 text-white font-bold text-xl",
                                            "{brand.name}"
                                        }
                                    }
                                    div { class: "container-268",
                                        p { class: "text-269 text-white text-sm opacity-90",
                                            "{brand.subtitle}"
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

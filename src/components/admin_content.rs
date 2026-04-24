use crate::admin::{
    create_banner, create_event, create_news, delete_banner, delete_event, delete_news,
    list_all_banners, list_events, list_news, set_banner_active, update_banner, update_event,
    update_news, Banner, Event, NewsItem,
};
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::{read_token, AuthState};
use crate::i18n::{translate, Locale};
use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    News,
    Events,
    Banners,
}

pub fn AdminContentPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    let mut tab = use_signal(|| Tab::News);
    let mut news_list = use_signal(Vec::<NewsItem>::new);
    let mut event_list = use_signal(Vec::<Event>::new);
    let mut banner_list = use_signal(Vec::<Banner>::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(String::new);

    // Role check
    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            let _ = nav.replace(Route::Login {});
        }
    });

    // Load content once on mount or when auth changes
    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            return;
        }

        loading.set(true);
        error.set(String::new());

        spawn(async move {
            let Some(token) = read_token() else {
                loading.set(false);
                error.set("Missing auth token".to_string());
                return;
            };

            // Fetch all content
            if let Ok(news) = list_news().await {
                news_list.set(news);
            }
            if let Ok(events) = list_events().await {
                event_list.set(events);
            }
            if let Ok(banners) = list_all_banners(token).await {
                banner_list.set(banners);
            }
            loading.set(false);
        });
    });

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Rewards }

            section { class: "max-w-7xl mx-auto w-full px-6 py-16 flex-1",
                div { class: "mb-8",
                    p { class: "text-xs font-bold tracking-widest text-accent mb-2", "ADMIN AREA" }
                    h1 { class: "text-3xl md:text-4xl font-extrabold text-dark mb-4", "Content Management" }
                    p { class: "text-sm text-muted", "Manage banners, events, and news displayed on the home page." }
                }

                // Tab navigation
                div { class: "flex gap-2 border-b border-gray-200 mb-8",
                    for &t in &[Tab::News, Tab::Events, Tab::Banners] {
                        button {
                            class: if tab() == t {
                                "px-4 py-3 font-semibold text-accent border-b-2 border-accent -mb-px"
                            } else {
                                "px-4 py-3 font-semibold text-muted hover:text-dark transition-colors"
                            },
                            onclick: move |_| tab.set(t),
                            {match t {
                                Tab::News => "News",
                                Tab::Events => "Events",
                                Tab::Banners => "Banners",
                            }}
                        }
                    }
                }

                // Error display
                if !error().is_empty() {
                    div { class: "mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm",
                        "{error}"
                    }
                }

                // Tab content
                if tab() == Tab::News {
                    NewsTab { news_list, error }
                } else if tab() == Tab::Events {
                    EventsTab { event_list, error }
                } else {
                    BannersTab { banner_list, error }
                }
            }

            Footer { dark: false, stick_to_bottom: false }
        }
    }
}

#[component]
fn NewsTab(mut news_list: Signal<Vec<NewsItem>>, mut error: Signal<String>) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut show_form = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<u32>);
    let mut title = use_signal(String::new);
    let mut body = use_signal(String::new);

    let handle_submit = move |_| {
        let t = title();
        let b = body();
        if t.is_empty() || b.is_empty() {
            error.set("Title and body are required".to_string());
            return;
        }

        spawn(async move {
            let Some(token) = crate::context::auth::read_token() else {
                error.set("Missing auth token".to_string());
                return;
            };

            if let Some(id) = edit_id() {
                // Update existing
                match update_news(token, id, t, b).await {
                    Ok(item) => {
                        news_list.with_mut(|list| {
                            if let Some(idx) = list.iter().position(|n| n.id == id) {
                                list[idx] = item;
                            }
                        });
                        title.set(String::new());
                        body.set(String::new());
                        show_form.set(false);
                        edit_id.set(None);
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Failed to update news: {e}")),
                }
            } else {
                // Create new
                match create_news(token, t, b).await {
                    Ok(item) => {
                        news_list.with_mut(|list| list.push(item));
                        title.set(String::new());
                        body.set(String::new());
                        show_form.set(false);
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Failed to create news: {e}")),
                }
            }
        });
    };

    rsx! {
        div {
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "text-xl font-bold text-dark", "News Items" }
                button {
                    class: "px-4 py-2 bg-accent text-white rounded-lg hover:bg-amber-600 transition-colors text-sm font-semibold",
                    onclick: move |_| {
                        if show_form() {
                            title.set(String::new());
                            body.set(String::new());
                            edit_id.set(None);
                        }
                        show_form.set(!show_form());
                    },
                    if show_form() { "Cancel" } else { "New News" }
                }
            }

            // Form
            if show_form() {
                div { class: "mb-6 p-6 bg-gray-50 rounded-lg border border-gray-200",
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Title" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "News title",
                                value: "{title}",
                                oninput: move |e| title.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Body (HTML)" }
                            textarea {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent min-h-32",
                                placeholder: "HTML content",
                                value: "{body}",
                                oninput: move |e| body.set(e.value()),
                            }
                        }
                        button {
                            class: "w-full py-2 bg-accent text-white rounded-lg hover:bg-amber-600 transition-colors font-semibold",
                            onclick: handle_submit,
                            if edit_id().is_some() { "Update News Item" } else { "Create News Item" }
                        }
                    }
                }
            }

            // List
            div { class: "space-y-4",
                if news_list().is_empty() {
                    p { class: "text-muted text-sm", "No news items yet." }
                } else {
                    for item in news_list() {
                        div {
                            key: "{item.id}",
                            class: "p-4 border border-gray-200 rounded-lg hover:border-accent transition-colors",
                            div { class: "flex justify-between items-start gap-4 mb-2",
                                div {
                                    h3 { class: "font-semibold text-dark", "{item.title}" }
                                    p { class: "text-xs text-muted mt-1", "By {item.author}" }
                                }
                                div { class: "flex gap-2",
                                    button {
                                        class: "px-3 py-1 text-xs bg-blue-50 text-blue-600 hover:bg-blue-100 rounded transition-colors",
                                        onclick: move |_| {
                                            title.set(item.title.clone());
                                            body.set(item.body.clone());
                                            edit_id.set(Some(item.id));
                                            show_form.set(true);
                                        },
                                        "Edit"
                                    }
                                    button {
                                        class: "px-3 py-1 text-xs bg-red-50 text-red-600 hover:bg-red-100 rounded transition-colors",
                                        onclick: move |_| {
                                            let item_id = item.id;
                                            spawn(async move {
                                                let Some(token) = crate::context::auth::read_token() else { return; };
                                                if delete_news(token, item_id).await.is_ok() {
                                                    news_list.with_mut(|list| list.retain(|n| n.id != item_id));
                                                }
                                            });
                                        },
                                        "Delete"
                                    }
                                }
                            }
                            p { class: "text-sm text-body line-clamp-2", "{item.body}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EventsTab(mut event_list: Signal<Vec<Event>>, mut error: Signal<String>) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut show_form = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<u32>);
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut date = use_signal(String::new);
    let mut location = use_signal(String::new);

    let handle_submit = move |_| {
        let t = title();
        let d = description();
        let dt = date();
        let l = location();
        if t.is_empty() || d.is_empty() || dt.is_empty() || l.is_empty() {
            error.set("All fields are required".to_string());
            return;
        }

        spawn(async move {
            let Some(token) = crate::context::auth::read_token() else {
                error.set("Missing auth token".to_string());
                return;
            };

            if let Some(id) = edit_id() {
                // Update existing
                match update_event(token, id, t, d, dt, None, l).await {
                    Ok(item) => {
                        event_list.with_mut(|list| {
                            if let Some(idx) = list.iter().position(|e| e.id == id) {
                                list[idx] = item;
                            }
                        });
                        title.set(String::new());
                        description.set(String::new());
                        date.set(String::new());
                        location.set(String::new());
                        show_form.set(false);
                        edit_id.set(None);
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Failed to update event: {e}")),
                }
            } else {
                // Create new
                match create_event(token, t, d, dt, None, l).await {
                    Ok(item) => {
                        event_list.with_mut(|list| list.push(item));
                        title.set(String::new());
                        description.set(String::new());
                        date.set(String::new());
                        location.set(String::new());
                        show_form.set(false);
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Failed to create event: {e}")),
                }
            }
        });
    };

    rsx! {
        div {
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "text-xl font-bold text-dark", "Events" }
                button {
                    class: "px-4 py-2 bg-accent text-white rounded-lg hover:bg-amber-600 transition-colors text-sm font-semibold",
                    onclick: move |_| {
                        if show_form() {
                            title.set(String::new());
                            description.set(String::new());
                            date.set(String::new());
                            location.set(String::new());
                            edit_id.set(None);
                        }
                        show_form.set(!show_form());
                    },
                    if show_form() { "Cancel" } else { "New Event" }
                }
            }

            // Form
            if show_form() {
                div { class: "mb-6 p-6 bg-gray-50 rounded-lg border border-gray-200",
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Title" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "Event title",
                                value: "{title}",
                                oninput: move |e| title.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Date (YYYY-MM-DD)" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "2026-05-15",
                                value: "{date}",
                                oninput: move |e| date.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Location" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "Event location",
                                value: "{location}",
                                oninput: move |e| location.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Description (HTML)" }
                            textarea {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent min-h-32",
                                placeholder: "HTML content",
                                value: "{description}",
                                oninput: move |e| description.set(e.value()),
                            }
                        }
                        button {
                            class: "w-full py-2 bg-accent text-white rounded-lg hover:bg-amber-600 transition-colors font-semibold",
                            onclick: handle_submit,
                            if edit_id().is_some() { "Update Event" } else { "Create Event" }
                        }
                    }
                }
            }

            // List
            div { class: "space-y-4",
                if event_list().is_empty() {
                    p { class: "text-muted text-sm", "No events yet." }
                } else {
                    for event in event_list() {
                        div {
                            key: "{event.id}",
                            class: "p-4 border border-gray-200 rounded-lg hover:border-accent transition-colors",
                            div { class: "flex justify-between items-start gap-4 mb-2",
                                div {
                                    h3 { class: "font-semibold text-dark", "{event.title}" }
                                    p { class: "text-xs text-muted mt-1", "{event.date} • {event.location}" }
                                }
                                div { class: "flex gap-2",
                                    button {
                                        class: "px-3 py-1 text-xs bg-blue-50 text-blue-600 hover:bg-blue-100 rounded transition-colors",
                                        onclick: move |_| {
                                            title.set(event.title.clone());
                                            description.set(event.description.clone());
                                            date.set(event.date.clone());
                                            location.set(event.location.clone());
                                            edit_id.set(Some(event.id));
                                            show_form.set(true);
                                        },
                                        "Edit"
                                    }
                                    button {
                                        class: "px-3 py-1 text-xs bg-red-50 text-red-600 hover:bg-red-100 rounded transition-colors",
                                        onclick: move |_| {
                                            let event_id = event.id;
                                            spawn(async move {
                                                let Some(token) = crate::context::auth::read_token() else { return; };
                                                if delete_event(token, event_id).await.is_ok() {
                                                    event_list.with_mut(|list| list.retain(|e| e.id != event_id));
                                                }
                                            });
                                        },
                                        "Delete"
                                    }
                                }
                            }
                            p { class: "text-sm text-body line-clamp-2", "{event.description}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BannersTab(mut banner_list: Signal<Vec<Banner>>, mut error: Signal<String>) -> Element {
    let mut show_form = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<u32>);
    let mut title = use_signal(String::new);
    let mut image_url = use_signal(String::new);
    let mut link_url = use_signal(String::new);

    let handle_submit = move |_| {
        let t = title();
        let img = image_url();
        if t.is_empty() || img.is_empty() {
            error.set("Title and image URL are required".to_string());
            return;
        }

        spawn(async move {
            let Some(token) = crate::context::auth::read_token() else {
                error.set("Missing auth token".to_string());
                return;
            };

            let link = if link_url().is_empty() { None } else { Some(link_url()) };

            if let Some(id) = edit_id() {
                // Update existing
                match update_banner(token, id, t, img, link).await {
                    Ok(item) => {
                        banner_list.with_mut(|list| {
                            if let Some(idx) = list.iter().position(|b| b.id == id) {
                                list[idx] = item;
                            }
                        });
                        title.set(String::new());
                        image_url.set(String::new());
                        link_url.set(String::new());
                        show_form.set(false);
                        edit_id.set(None);
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Failed to update banner: {e}")),
                }
            } else {
                // Create new
                match create_banner(token, t, img, link, banner_list().len() as u32 + 1).await {
                    Ok(item) => {
                        banner_list.with_mut(|list| list.push(item));
                        title.set(String::new());
                        image_url.set(String::new());
                        link_url.set(String::new());
                        show_form.set(false);
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Failed to create banner: {e}")),
                }
            }
        });
    };

    rsx! {
        div {
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "text-xl font-bold text-dark", "Banners" }
                button {
                    class: "px-4 py-2 bg-accent text-white rounded-lg hover:bg-amber-600 transition-colors text-sm font-semibold",
                    onclick: move |_| {
                        if show_form() {
                            title.set(String::new());
                            image_url.set(String::new());
                            link_url.set(String::new());
                            edit_id.set(None);
                        }
                        show_form.set(!show_form());
                    },
                    if show_form() { "Cancel" } else { "New Banner" }
                }
            }

            // Form
            if show_form() {
                div { class: "mb-6 p-6 bg-gray-50 rounded-lg border border-gray-200",
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Title" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "Banner title",
                                value: "{title}",
                                oninput: move |e| title.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Image URL" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "/path/to/image.jpg",
                                value: "{image_url}",
                                oninput: move |e| image_url.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-semibold text-dark mb-1", "Link URL (optional)" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent",
                                r#type: "text",
                                placeholder: "/stores",
                                value: "{link_url}",
                                oninput: move |e| link_url.set(e.value()),
                            }
                        }
                        button {
                            class: "w-full py-2 bg-accent text-white rounded-lg hover:bg-amber-600 transition-colors font-semibold",
                            onclick: handle_submit,
                            if edit_id().is_some() { "Update Banner" } else { "Create Banner" }
                        }
                    }
                }
            }

            // List
            div { class: "space-y-4",
                if banner_list().is_empty() {
                    p { class: "text-muted text-sm", "No banners yet." }
                } else {
                    for banner in banner_list() {
                        div {
                            key: "{banner.id}",
                            class: "p-4 border border-gray-200 rounded-lg hover:border-accent transition-colors",
                            div { class: "flex justify-between items-start gap-4 mb-2",
                                div {
                                    h3 { class: "font-semibold text-dark", "{banner.title}" }
                                    p { class: "text-xs text-muted mt-1", "{banner.image_url}" }
                                }
                                div { class: "flex gap-2",
                                    button {
                                        class: "px-3 py-1 text-xs bg-blue-50 text-blue-600 hover:bg-blue-100 rounded transition-colors",
                                        onclick: move |_| {
                                            title.set(banner.title.clone());
                                            image_url.set(banner.image_url.clone());
                                            link_url.set(banner.link_url.clone().unwrap_or_default());
                                            edit_id.set(Some(banner.id));
                                            show_form.set(true);
                                        },
                                        "Edit"
                                    }
                                    button {
                                        class: if banner.active {
                                            "px-3 py-1 text-xs bg-green-50 text-green-600 hover:bg-green-100 rounded transition-colors"
                                        } else {
                                            "px-3 py-1 text-xs bg-gray-100 text-gray-600 hover:bg-gray-200 rounded transition-colors"
                                        },
                                        onclick: move |_| {
                                            let banner_id = banner.id;
                                            let new_active = !banner.active;
                                            spawn(async move {
                                                let Some(token) = crate::context::auth::read_token() else { return; };
                                                if set_banner_active(token, banner_id, new_active).await.is_ok() {
                                                    banner_list.with_mut(|list| {
                                                        if let Some(b) = list.iter_mut().find(|b| b.id == banner_id) {
                                                            b.active = new_active;
                                                        }
                                                    });
                                                }
                                            });
                                        },
                                        if banner.active { "Active" } else { "Inactive" }
                                    }
                                    button {
                                        class: "px-3 py-1 text-xs bg-red-50 text-red-600 hover:bg-red-100 rounded transition-colors",
                                        onclick: move |_| {
                                            let banner_id = banner.id;
                                            spawn(async move {
                                                let Some(token) = crate::context::auth::read_token() else { return; };
                                                if delete_banner(token, banner_id).await.is_ok() {
                                                    banner_list.with_mut(|list| list.retain(|b| b.id != banner_id));
                                                }
                                            });
                                        },
                                        "Delete"
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

use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::AuthState;
use crate::i18n::{translate, Locale};
use crate::stores::{
    create_store, delete_store, list_store_rows, update_store, Category, StoreAdminRow,
};
use crate::Route;
use dioxus::prelude::*;

const SERVER_FN_BODY_SAFE_LIMIT_BYTES: usize = 5 * 1024 * 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
enum StoresAdminTab {
    Add,
    Manage,
}

fn icon_src_from_db_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    // Static files are served from /brands in this app.
    if trimmed.starts_with("/public/brands/") {
        return trimmed.replacen("/public/brands/", "/brands/", 1);
    }
    trimmed.to_string()
}

fn store_admin_row_matches_query(row: &StoreAdminRow, q_lower: &str) -> bool {
    if q_lower.is_empty() {
        return true;
    }
    row.name.to_lowercase().contains(q_lower)
        || row.id.to_string().contains(q_lower)
        || row
            .store_number
            .as_deref()
            .unwrap_or_default()
            .to_lowercase()
            .contains(q_lower)
        || row
            .level
            .map(|v| v.to_string())
            .unwrap_or_default()
            .contains(q_lower)
        || row
            .phone
            .as_deref()
            .unwrap_or_default()
            .to_lowercase()
            .contains(q_lower)
        || row
            .website
            .as_deref()
            .unwrap_or_default()
            .to_lowercase()
            .contains(q_lower)
        || row
            .icon_path
            .as_deref()
            .unwrap_or_default()
            .to_lowercase()
            .contains(q_lower)
}

pub fn StoresAdminPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    let mut rows = use_signal(Vec::<StoreAdminRow>::new);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(String::new);

    let mut edit_id = use_signal(|| None::<i64>);
    let mut name = use_signal(String::new);
    let mut category = use_signal(|| Category::LadiesMenswear);
    let mut store_number = use_signal(String::new);
    let mut level = use_signal(String::new);
    let mut phone = use_signal(String::new);
    let mut website = use_signal(String::new);
    let mut icon_path = use_signal(String::new);
    let mut upload_status = use_signal(String::new);
    let mut uploading_icon = use_signal(|| false);
    let mut saving_store = use_signal(|| false);
    let mut selected_file_name = use_signal(|| None::<String>);
    let mut selected_file_bytes = use_signal(|| None::<Vec<u8>>);
    let mut desktop_tab = use_signal(|| StoresAdminTab::Add);
    let mut preview_image_src = use_signal(|| None::<String>);
    let mut manage_search = use_signal(String::new);

    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            let _ = nav.replace(Route::Login {});
            return;
        }
        if matches!(auth(), AuthState::LoggedIn(user) if user.role != Role::Admin) {
            let _ = nav.replace(Route::Rewards {});
            return;
        }
        let is_admin = matches!(auth(), AuthState::LoggedIn(user) if user.role == Role::Admin);
        if !is_admin {
            return;
        }

        loading.set(true);
        spawn(async move {
            match list_store_rows().await {
                Ok(list) => {
                    rows.set(list);
                    error.set(String::new());
                }
                Err(e) => error.set(format!("Failed to load stores: {e}")),
            }
            loading.set(false);
        });
    });

    let save_store = move |_| {
        let n = name().trim().to_string();
        if n.is_empty() {
            error.set(translate(locale(), "stores_admin.error.name_required"));
            return;
        }
        if n.len() < 2 {
            error.set("Le nom du magasin doit contenir au moins 2 caracteres.".to_string());
            return;
        }
        if n.len() > 120 {
            error.set("Le nom du magasin ne peut pas depasser 120 caracteres.".to_string());
            return;
        }

        let is_store_number_valid = |value: &str| {
            value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/' || c == ' ')
        };
        let is_phone_valid = |value: &str| {
            value
                .chars()
                .all(|c| c.is_ascii_digit() || c == '+' || c == ' ' || c == '-' || c == '(' || c == ')')
        };

        let lvl = if level().trim().is_empty() {
            None
        } else {
            match level().trim().parse::<u8>() {
                Ok(v) if v <= 20 => Some(v),
                Ok(_) => {
                    error.set("Le niveau doit etre compris entre 0 et 20.".to_string());
                    return;
                }
                Err(_) => {
                    error.set(translate(locale(), "stores_admin.error.level_number"));
                    return;
                }
            }
        };

        let sn = if store_number().trim().is_empty() {
            None
        } else {
            let value = store_number().trim().to_string();
            if value.len() > 32 {
                error.set("Le numero de magasin ne peut pas depasser 32 caracteres.".to_string());
                return;
            }
            if !is_store_number_valid(&value) {
                error.set("Le numero de magasin contient des caracteres invalides.".to_string());
                return;
            }
            Some(value)
        };
        let ph = if phone().trim().is_empty() {
            None
        } else {
            let value = phone().trim().to_string();
            if value.len() > 32 {
                error.set("Le telephone ne peut pas depasser 32 caracteres.".to_string());
                return;
            }
            if !is_phone_valid(&value) {
                error.set("Le telephone contient des caracteres invalides.".to_string());
                return;
            }
            Some(value)
        };
        let web = if website().trim().is_empty() {
            None
        } else {
            let value = website().trim().to_string();
            if value.len() > 255 {
                error.set("Le site web ne peut pas depasser 255 caracteres.".to_string());
                return;
            }
            if !value.starts_with("http://") && !value.starts_with("https://") {
                error.set("Le site web doit commencer par http:// ou https://".to_string());
                return;
            }
            Some(value)
        };
        let icon = if icon_path().trim().is_empty() {
            None
        } else {
            Some(icon_path().trim().to_string())
        };
        let cat = category();

        saving_store.set(true);
        upload_status.set(String::new());
        error.set(String::new());

        spawn(async move {
            let upload_filename = selected_file_name();
            let upload_bytes = selected_file_bytes();
            if let Some(bytes) = upload_bytes.as_ref() {
                if bytes.len() > SERVER_FN_BODY_SAFE_LIMIT_BYTES {
                    error.set(format!(
                        "Image too large for current submit transport ({} bytes). Please use an image under {} MB.",
                        bytes.len(),
                        SERVER_FN_BODY_SAFE_LIMIT_BYTES / (1024 * 1024)
                    ));
                    saving_store.set(false);
                    return;
                }
            }

            if let Some(id) = edit_id() {
                let result = update_store(id, n, cat, sn, lvl, ph, web, icon, upload_filename, upload_bytes).await;
                match result {
                    Ok(_) => {
                        match list_store_rows().await {
                            Ok(list) => {
                                if let Some(updated) = list.iter().find(|row| row.id == id) {
                                    icon_path.set(updated.icon_path.clone().unwrap_or_default());
                                }
                                rows.set(list);
                            }
                            Err(e) => error.set(format!("Reload failed: {e}")),
                        }
                        selected_file_name.set(None);
                        selected_file_bytes.set(None);
                        uploading_icon.set(false);
                        upload_status.set("Store updated successfully.".to_string());
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Save failed: {e}")),
                }
            } else {
                let result = create_store(n, cat, sn, lvl, ph, web, icon, upload_filename, upload_bytes).await;
                match result {
                    Ok(created_id) => {
                        edit_id.set(Some(created_id));
                        selected_file_name.set(None);
                        selected_file_bytes.set(None);
                        uploading_icon.set(false);
                        upload_status.set(format!("Store created with id_stores={created_id}."));
                        match list_store_rows().await {
                            Ok(list) => {
                                if let Some(created) = list.iter().find(|row| row.id == created_id) {
                                    icon_path.set(created.icon_path.clone().unwrap_or_default());
                                }
                                rows.set(list);
                            }
                            Err(e) => error.set(format!("Reload failed: {e}")),
                        }
                        error.set(String::new());
                    }
                    Err(e) => error.set(format!("Save failed: {e}")),
                }
            }
            saving_store.set(false);
        });
    };

    let upload_icon = move |evt: FormEvent| {
        let files = evt.files();
        let Some(file) = files.first().cloned() else {
            error.set("No file detected. Please choose an image file again.".to_string());
            return;
        };
        let file_name = file.name();

        uploading_icon.set(true);
        upload_status.set(format!("Uploading {file_name}..."));
        error.set(String::new());

        spawn(async move {
            let bytes = match file.read_bytes().await {
                Ok(content) => content.to_vec(),
                Err(_) => {
                    error.set("Unable to read selected file".to_string());
                    uploading_icon.set(false);
                    return;
                }
            };
            if bytes.len() > SERVER_FN_BODY_SAFE_LIMIT_BYTES {
                selected_file_name.set(None);
                selected_file_bytes.set(None);
                error.set(format!(
                    "Selected image is too large for current upload transport ({} bytes). Please use an image under {} MB.",
                    bytes.len(),
                    SERVER_FN_BODY_SAFE_LIMIT_BYTES / (1024 * 1024)
                ));
                uploading_icon.set(false);
                return;
            }
            selected_file_name.set(Some(file_name.clone()));
            selected_file_bytes.set(Some(bytes));
            upload_status.set(format!("Image selected: {file_name}. It will be uploaded on save."));
            uploading_icon.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::None }

            section { class: "max-w-7xl mx-auto w-full px-6 py-12 flex-1",
                h1 { class: "text-3xl font-extrabold text-dark mb-2", {translate(locale(), "stores_admin.title")} }
                p { class: "text-sm text-muted mb-6", {translate(locale(), "stores_admin.subtitle")} }

                if !error().is_empty() {
                    div { class: "mb-4 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700",
                        "{error}"
                    }
                }

                div { class: "mb-4 flex flex-col gap-3 md:flex-row md:items-center md:justify-between md:gap-4",
                    div { class: "flex w-full flex-col gap-3 md:w-auto md:shrink-0",
                        div { class: "hidden md:flex rounded-xl border border-gray-200 p-1 w-fit gap-1",
                            button {
                                class: if desktop_tab() == StoresAdminTab::Add {
                                    "rounded-lg bg-dark px-4 py-2 text-xs font-bold tracking-wider text-white"
                                } else {
                                    "rounded-lg px-4 py-2 text-xs font-bold tracking-wider text-dark hover:bg-gray-100"
                                },
                                onclick: move |_| desktop_tab.set(StoresAdminTab::Add),
                                {translate(locale(), "stores_admin.tab.add")}
                            }
                            button {
                                class: if desktop_tab() == StoresAdminTab::Manage {
                                    "rounded-lg bg-dark px-4 py-2 text-xs font-bold tracking-wider text-white"
                                } else {
                                    "rounded-lg px-4 py-2 text-xs font-bold tracking-wider text-dark hover:bg-gray-100"
                                },
                                onclick: move |_| desktop_tab.set(StoresAdminTab::Manage),
                                {translate(locale(), "stores_admin.tab.manage")}
                            }
                        }
                        div { class: "flex rounded-xl border border-gray-200 p-1 gap-1 md:hidden",
                            button {
                                class: if desktop_tab() == StoresAdminTab::Add {
                                    "flex-1 rounded-lg bg-dark px-4 py-2 text-xs font-bold tracking-wider text-white"
                                } else {
                                    "flex-1 rounded-lg px-4 py-2 text-xs font-bold tracking-wider text-dark hover:bg-gray-100"
                                },
                                onclick: move |_| desktop_tab.set(StoresAdminTab::Add),
                                {translate(locale(), "stores_admin.tab.add")}
                            }
                            button {
                                class: if desktop_tab() == StoresAdminTab::Manage {
                                    "flex-1 rounded-lg bg-dark px-4 py-2 text-xs font-bold tracking-wider text-white"
                                } else {
                                    "flex-1 rounded-lg px-4 py-2 text-xs font-bold tracking-wider text-dark hover:bg-gray-100"
                                },
                                onclick: move |_| desktop_tab.set(StoresAdminTab::Manage),
                                {translate(locale(), "stores_admin.tab.manage")}
                            }
                        }
                    }
                    div { class: "flex w-full max-w-lg md:ml-auto md:w-auto md:min-w-0 md:flex-1 md:justify-end",
                        div { class: "flex w-full max-w-lg",
                            div { class: "flex-1 relative",
                                input {
                                    class: "w-full py-3.5 pl-4 pr-12 text-sm border border-gray-200 rounded-l-lg placeholder-muted focus:ring-accent focus:border-accent focus:outline-none",
                                    r#type: "text",
                                    placeholder: {translate(locale(), "stores_admin.search.placeholder")},
                                    value: "{manage_search}",
                                    oninput: move |e| {
                                        let value = e.value();
                                        if !value.trim().is_empty() {
                                            desktop_tab.set(StoresAdminTab::Manage);
                                        }
                                        manage_search.set(value);
                                    },
                                }
                            }
                            button {
                                class: "px-5 bg-dark text-white rounded-r-lg hover:bg-gray-700 transition-colors",
                                r#type: "button",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "18",
                                    height: "18",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "11", cy: "11", r: "8" }
                                    line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                                }
                            }
                        }
                    }
                }

                if desktop_tab() == StoresAdminTab::Add {
                    div { class: "mb-8 rounded-xl border border-gray-200 py-4 px-6 space-y-3",
                        h2 { class: "text-sm font-bold tracking-wider text-dark",
                            if edit_id().is_some() {
                                {translate(locale(), "stores_admin.form.edit_store")}
                            } else {
                                {translate(locale(), "stores_admin.form.create_store")}
                            }
                        }
                        input {
                            class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                            r#type: "text",
                            placeholder: {translate(locale(), "stores_admin.form.name")},
                            value: "{name}",
                            maxlength: 120,
                            oninput: move |e| name.set(e.value()),
                        }
                        select {
                            class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                            value: "{category().key()}",
                            onchange: move |e| {
                                if let Some(c) = Category::from_key(&e.value()) {
                                    category.set(c);
                                }
                            },
                            for c in Category::all() {
                                option {
                                    value: "{c.key()}",
                                    selected: c == category(),
                                    "{c.label()}"
                                }
                            }
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                            input {
                                class: "rounded border border-gray-300 px-3 py-2 text-sm",
                                r#type: "text",
                                placeholder: {translate(locale(), "stores_admin.form.store_number")},
                                value: "{store_number}",
                                maxlength: 32,
                                oninput: move |e| store_number.set(e.value()),
                            }
                            input {
                                class: "rounded border border-gray-300 px-3 py-2 text-sm",
                                r#type: "text",
                                placeholder: {translate(locale(), "stores_admin.form.level")},
                                value: "{level}",
                                maxlength: 2,
                                oninput: move |e| level.set(e.value()),
                            }
                        }
                        input {
                            class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                            r#type: "text",
                            placeholder: {translate(locale(), "stores_admin.form.phone")},
                            value: "{phone}",
                            maxlength: 32,
                            oninput: move |e| phone.set(e.value()),
                        }
                        input {
                            class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                            r#type: "text",
                            placeholder: {translate(locale(), "stores_admin.form.website")},
                            value: "{website}",
                            maxlength: 255,
                            oninput: move |e| website.set(e.value()),
                        }
                        input {
                            class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                            r#type: "file",
                            accept: ".jpg,.jpeg,.png,.webp,image/jpeg,image/png,image/webp",
                            onchange: upload_icon,
                            oninput: upload_icon,
                        }
                        if !icon_path().is_empty() {
                            div { class: "flex items-center gap-3",
                                img {
                                    class: "h-14 w-14 rounded border border-gray-200 object-cover bg-gray-100",
                                    src: "{icon_src_from_db_path(&icon_path())}",
                                    alt: "Store icon preview",
                                }
                                p { class: "text-xs text-muted", "{icon_path}" }
                            }
                        }
                        p { class: "text-xs text-muted",
                            if uploading_icon() {
                                "Uploading icon..."
                            } else if !upload_status().is_empty() {
                                "{upload_status}"
                            } else {
                                "Max 5 MB. Allowed: JPG/PNG/WEBP. Spaces in file names are replaced by _."
                            }
                        }
                        if saving_store() {
                            p { class: "text-xs text-muted", "Saving store..." }
                        }
                        div { class: "flex gap-2",
                            button {
                                class: "rounded bg-accent px-4 py-2 text-xs font-bold tracking-wider text-white",
                                onclick: save_store,
                                disabled: saving_store(),
                                if edit_id().is_some() {
                                    if saving_store() {
                                        "Enregistrement..."
                                    } else {
                                        {translate(locale(), "stores_admin.action.update")}
                                    }
                                } else {
                                    if saving_store() {
                                        "Enregistrement..."
                                    } else {
                                        {translate(locale(), "stores_admin.action.create")}
                                    }
                                }
                            }
                            button {
                                class: "rounded border border-gray-300 px-4 py-2 text-xs font-bold tracking-wider text-dark",
                                onclick: move |_| {
                                    edit_id.set(None);
                                    name.set(String::new());
                                    category.set(Category::LadiesMenswear);
                                    store_number.set(String::new());
                                    level.set(String::new());
                                    phone.set(String::new());
                                    website.set(String::new());
                                    icon_path.set(String::new());
                                },
                                {translate(locale(), "stores_admin.action.reset")}
                            }
                        }
                    }
                }

                // Mobile duplicate kept disabled to avoid double rendering.
                div { class: "hidden",
                    h2 { class: "text-sm font-bold tracking-wider text-dark",
                        if edit_id().is_some() {
                            {translate(locale(), "stores_admin.form.edit_store")}
                        } else {
                            {translate(locale(), "stores_admin.form.create_store")}
                        }
                    }
                    input {
                        class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                        r#type: "text",
                        placeholder: {translate(locale(), "stores_admin.form.name")},
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                    select {
                        class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                        value: "{category().key()}",
                        onchange: move |e| {
                            if let Some(c) = Category::from_key(&e.value()) {
                                category.set(c);
                            }
                        },
                        for c in Category::all() {
                            option {
                                value: "{c.key()}",
                                selected: c == category(),
                                "{c.label()}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                        input {
                            class: "rounded border border-gray-300 px-3 py-2 text-sm",
                            r#type: "text",
                            placeholder: {translate(locale(), "stores_admin.form.store_number")},
                            value: "{store_number}",
                            oninput: move |e| store_number.set(e.value()),
                        }
                        input {
                            class: "rounded border border-gray-300 px-3 py-2 text-sm",
                            r#type: "text",
                            placeholder: {translate(locale(), "stores_admin.form.level")},
                            value: "{level}",
                            oninput: move |e| level.set(e.value()),
                        }
                    }
                    input {
                        class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                        r#type: "text",
                        placeholder: {translate(locale(), "stores_admin.form.phone")},
                        value: "{phone}",
                        oninput: move |e| phone.set(e.value()),
                    }
                    input {
                        class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                        r#type: "text",
                        placeholder: {translate(locale(), "stores_admin.form.website")},
                        value: "{website}",
                        oninput: move |e| website.set(e.value()),
                    }
                    input {
                        class: "w-full rounded border border-gray-300 px-3 py-2 text-sm",
                        r#type: "file",
                        accept: ".jpg,.jpeg,.png,.webp,image/jpeg,image/png,image/webp",
                        onchange: upload_icon,
                    }
                    p { class: "text-xs text-muted",
                        if uploading_icon() {
                            "Uploading icon..."
                        } else if !upload_status().is_empty() {
                            "{upload_status}"
                        } else {
                            "Max 5 MB. Allowed: JPG/PNG/WEBP. Spaces in file names are replaced by _."
                        }
                    }
                    div { class: "flex gap-2",
                        button {
                            class: "rounded bg-accent px-4 py-2 text-xs font-bold tracking-wider text-white",
                            onclick: save_store,
                            if edit_id().is_some() {
                                {translate(locale(), "stores_admin.action.update")}
                            } else {
                                {translate(locale(), "stores_admin.action.create")}
                            }
                        }
                        button {
                            class: "rounded border border-gray-300 px-4 py-2 text-xs font-bold tracking-wider text-dark",
                            onclick: move |_| {
                                edit_id.set(None);
                                name.set(String::new());
                                category.set(Category::LadiesMenswear);
                                store_number.set(String::new());
                                level.set(String::new());
                                phone.set(String::new());
                                website.set(String::new());
                                icon_path.set(String::new());
                            },
                            {translate(locale(), "stores_admin.action.reset")}
                        }
                    }
                }

                if loading() {
                    p { class: "text-sm text-muted", {translate(locale(), "stores_admin.loading")} }
                } else {
                    if desktop_tab() == StoresAdminTab::Manage {
                        div { class: "mb-2 text-xs font-semibold tracking-wider text-muted",
                            {translate(locale(), "stores_admin.tab.manage")} " - {rows().len()} stores"
                        }
                        if rows().is_empty() {
                            div { class: "rounded-lg border border-gray-200 bg-gray-50 px-4 py-6 text-sm text-muted",
                                "Aucune boutique a afficher."
                            }
                        } else if !manage_search().trim().is_empty()
                            && rows()
                                .iter()
                                .filter(|row| {
                                    store_admin_row_matches_query(
                                        row,
                                        &manage_search().trim().to_lowercase(),
                                    )
                                })
                                .count()
                                == 0
                        {
                            div { class: "rounded-lg border border-gray-200 bg-amber-50 px-4 py-6 text-sm text-amber-900",
                                {translate(locale(), "stores_admin.search.no_match")}
                            }
                        } else {
                            div { class: "grid grid-cols-1 gap-3",
                                for row in rows().into_iter().filter(|row| {
                                    store_admin_row_matches_query(
                                        row,
                                        &manage_search().trim().to_lowercase(),
                                    )
                                }) {
                                    div {
                                        key: "{row.id}",
                                        class: "rounded-lg border border-gray-200 p-3",
                                        div { class: "flex flex-col gap-3 md:flex-row md:items-center md:justify-between",
                                            div { class: "px-3 py-2 flex items-start justify-between gap-3 min-w-0 flex-1",
                                                div { class: "flex items-center gap-2 md:gap-3 min-w-0",
                                                    p { class: "text-dark shrink-0", "{row.id}" }
                                                    p { class: "text-dark font-semibold truncate", "{row.name}" }
                                                }
                                                p { class: "text-xs text-muted text-right shrink min-w-0 truncate",
                                                    "{row.icon_path.clone().unwrap_or_else(|| \"(no icon_path)\".to_string())}"
                                                }
                                            }
                                            div {
                                                class: "px-6 py-2 grid flex-1 grid-cols-3 grid-rows-1 items-center gap-6 text-center md:w-auto",
                                                style: "grid-template-columns: repeat(3, 1fr); grid-template-rows: repeat(1, 1fr);",
                                                if let Some(path) = row.icon_path.clone() {
                                                    if !path.trim().is_empty() {
                                                        button {
                                                            class: "w-auto justify-self-start rounded bg-gray-100 px-3 py-1 text-xs font-semibold text-dark hover:bg-gray-200",
                                                            r#type: "button",
                                                            onclick: move |_| {
                                                                preview_image_src.set(Some(icon_src_from_db_path(&path)));
                                                            },
                                                            {translate(locale(), "stores_admin.action.view")}
                                                        }
                                                    } else {
                                                        span {}
                                                    }
                                                } else {
                                                    span {}
                                                }
                                                button {
                                                    class: "w-auto justify-self-center rounded bg-blue-50 px-3 py-1 text-xs font-semibold text-blue-700",
                                                    onclick: move |_| {
                                                        desktop_tab.set(StoresAdminTab::Add);
                                                        edit_id.set(Some(row.id));
                                                        name.set(row.name.clone());
                                                        category.set(row.category.clone());
                                                        store_number.set(row.store_number.clone().unwrap_or_default());
                                                        level.set(row.level.map(|v| v.to_string()).unwrap_or_default());
                                                        phone.set(row.phone.clone().unwrap_or_default());
                                                        website.set(row.website.clone().unwrap_or_default());
                                                        icon_path.set(row.icon_path.clone().unwrap_or_default());
                                                    },
                                                    {translate(locale(), "stores_admin.action.edit")}
                                                }
                                                button {
                                                    class: "w-auto justify-self-end rounded bg-red-50 px-3 py-1 text-xs font-semibold text-red-700",
                                                    onclick: move |_| {
                                                        let id = row.id;
                                                        spawn(async move {
                                                            match delete_store(id).await {
                                                                Ok(_) => match list_store_rows().await {
                                                                    Ok(list) => rows.set(list),
                                                                    Err(e) => error.set(format!("Reload failed: {e}")),
                                                                },
                                                                Err(e) => error.set(format!("Delete failed: {e}")),
                                                            }
                                                        });
                                                    },
                                                    {translate(locale(), "stores_admin.action.delete")}
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

            if let Some(src) = preview_image_src() {
                div {
                    style: "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0, 0, 0, 0.6); padding: 1rem;",
                    onclick: move |_| preview_image_src.set(None),
                    div {
                        style: "position: relative; background: white; border-radius: 0.5rem; padding: 1rem; box-shadow: 0 10px 25px rgba(0,0,0,0.2);",
                        onclick: move |evt| evt.stop_propagation(),
                        button {
                            r#type: "button",
                            onclick: move |_| preview_image_src.set(None),
                            style: "position: absolute; top: 8px; right: 8px; width: 28px; height: 28px; border: 1px solid #d1d5db; border-radius: 9999px; background: #fff; color: #374151; font-size: 18px; line-height: 1; cursor: pointer;",
                            aria_label: "Fermer l'aperçu",
                            "×"
                        }
                        img {
                            class: "block rounded border border-gray-200 object-cover bg-gray-100",
                            style: "width: 230px; height: 230px;",
                            src: "{src}",
                            alt: "Store image preview",
                        }
                    }
                }
            }

            Footer { dark: false, stick_to_bottom: false }
        }
    }
}

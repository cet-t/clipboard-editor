mod types;

pub use types::Locales;

#[macro_export(local_inner_macros)]
macro_rules! create_bundle {
    ($locales: expr, $($path: expr),+) => {{
        use fluent::{FluentResource, bundle::FluentBundle};

        let mut bundle = FluentBundle::new_concurrent($locales);

        $({
            let ftl = std::fs::read_to_string($path).expect("FTL File not found");
            let ftl = FluentResource::try_new(ftl).expect("FTL Parse Error");
            bundle.add_resource(ftl).expect("unable to add resource");
        })+

        bundle
    }};
    ($locales: expr, $($path: expr,)+) => {
        create_bundle!($locales, $($path),+)
    };
}

#[macro_export]
macro_rules! load_locales {
    ($locales:expr, [$($id:expr),+ $(,)?]) => { $({
        const ID: unic_langid::LanguageIdentifier = unic_langid::langid!($id);
        let bun = $crate::create_bundle!(vec![ID], format!("locales/{}/main.ftl", $id));
        $locales.insert(ID.language, bun);
    })+ };
}

#[macro_export]
macro_rules! value_locale {
    ($locales:expr, $id:expr, $key:expr) => {
        if let Some(bun) = $locales.get(&$id.language)
            && let Some(value) = bun.get_message($key)
            && let Some(value) = value.value()
        {
            Some(bun.format_pattern(value, None, &mut vec![]))
        } else {
            None
        }
    };
}

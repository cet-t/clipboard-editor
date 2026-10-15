pub type Locales = std::collections::HashMap<
    unic_langid::subtags::Language,
    fluent::bundle::FluentBundle<
        fluent::FluentResource,
        intl_memoizer::concurrent::IntlLangMemoizer,
    >,
>;

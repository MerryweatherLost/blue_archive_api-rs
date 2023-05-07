use blue_archive::Language;

macro_rules! lang_test {
    ($test_name:ident, $language_type:expr) => {
        #[tokio::test]
        async fn $test_name() {
            assert!(blue_archive::fetch_all_summons($language_type)
                .await
                .is_ok())
        }
    };
}

lang_test!(fetch_summons_by_english, Language::English);
lang_test!(fetch_summons_by_japanese, Language::Japanese);
lang_test!(fetch_summons_by_chinese, Language::Chinese);
lang_test!(fetch_summons_by_korean, Language::Korean);
lang_test!(fetch_summons_by_thai, Language::Thai);
lang_test!(fetch_summons_by_taiwanese, Language::Taiwanese);
lang_test!(fetch_summons_by_vietnamese, Language::Vietnamese);

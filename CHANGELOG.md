# blue_archive-rs

## 0.4.0 - 2024-03-23

### Additions ✨

- This changelog. It will be assumed that the latest update will be on the top. Read from ascending order.
- Currency API has been added. You can use the existing functions:
  - `blue_archive::fetch_all_currencies`
  - `blue_archive::fetch_currency_by_name`
- Equipment API has been added. You can use the existing functions:
  - `blue_archive::fetch_all_equipment`
  - `blue_archive::fetch_equipment_by_name`
  - `blue_archive::fetch_equipment_by_category`
- Enemies API has been added. You can use the existing functions:
  - `blue_archive::fetch_all_enemies`
  - `blue_archive::fetch_enemy_by_name`

### Changes 📝

- `impl Into<String>` has been changed to a more suitable `AsRef<str>`. This is an internal change related to name-based searching.
- `ID(u32)`'s inner value has been set to private, access the value by using `ID::to_u32`.
- Raid skills have been modified into enumerations, which allows for better differentiation of the data.
- Descriptions are now directly decoded through `serde` deserialization.
  - Instead of `description()`, please use `description`.
- Student's first and last names have been made directly public through member field access.
  - Instead of `Student::first_name()` & `Student::last_name()`, please use `Student::first_name` & `Student::last_name`.
- `Released` type has been changed to a struct with an inner tuple value, though to get the regions, you use the functions:
  - `Released::japan()`, `Released::global()`, and `Released::china()` for example.

## Fixes ⚒️

- Applied a change to the `Student::position` function, was passing in the `Student::armor_type` for some reason... oops!

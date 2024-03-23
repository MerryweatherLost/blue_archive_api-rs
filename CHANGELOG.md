# blue_archive-rs

## 0.4.0 - 2024-03-23

### Additions ✨

- Currency API has been added. You can use the existing functions:
  - `blue_archive::fetch_all_currencies`
  - `blue_archive::fetch_currency_by_name`

### Changes 📝

- `ID(u32)`'s inner value has been set to private, access the value by using `ID::to_u32`.
- Raid skills have been modified into enumerations, which allows for better differentiation of the data.
- Descriptions are now directly decoded through `serde` deserialization.
  - Instead of `description()`, please use `description`.
- Student's first and last names have been made directly public through member field access.
  - Instead of `Student::first_name()` & `Student::last_name()`, please use `Student::first_name` & `Student::last_name`.

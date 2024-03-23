# blue_archive-rs

## 0.4.0 - 2024-03-23

- `ID(u32)`'s inner value has been set to private, access the value by using `ID::to_u32`.
- Currency API has been added. You can use the existing functions:
  - `blue_archive::fetch_all_currencies`
  - `blue_archive::fetch_currency_by_name`
- Raid skills have been modified into enumerations, which allows for better differentiation of the data.
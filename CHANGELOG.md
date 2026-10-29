# Changelog

## 0.2.0 — 2026-10-29

First published release. 0.1.0 was never published, so this entry covers everything in the library.

There is no earlier public API, so nothing needs migrating.

- `CALENDAR_NOT_PUBLISHED` for the 404 returned when a holiday calendar year is not published.
- Stocks: `nw_equities_price`, `nw_equities_prices`, `nw_equities_bars`, `nw_equities_profile`,
  `nw_equities_dividends`, `nw_equities_splits`, `nw_equities_earnings`,
  `nw_equities_valuation`, `nw_equities_valuation_history`, `nw_equities_movers`,
  `nw_equities_search` and `nw_equities_cost_of_capital`. US listings on NYSE, Nasdaq and NYSE
  American, and Korea. Ids look like `XNAS:AAPL`.
- Funds: `nw_funds_price`, `nw_funds_prices`, `nw_funds_bars`, `nw_funds_profile`,
  `nw_funds_distributions` and `nw_funds_search`. `nw_funds_price` answers an ETF or a mutual
  fund NAV, told apart by its type.
- Crypto: `nw_crypto_price`, `nw_crypto_prices`, `nw_crypto_bars` and `nw_crypto_list_pairs`.
- Company calendars: `nw_company_calendars_dividends` and `nw_company_calendars_splits`.
- Central banks: `nw_economics_policy_rate`, `nw_economics_policy_rate_history`,
  `nw_economics_central_bank_balance_sheet` and `nw_economics_central_bank_reserves`. A policy
  rate is a point rate (`nw_point_policy_rate`) or a target range (`nw_range_policy_rate`) in
  `nw_policy_rate`.
- Equity risk premium: `nw_economics_list_equity_risk_premium` for every country,
  `nw_economics_get_equity_risk_premium` for one country (optional `date`) and
  `nw_economics_equity_risk_premium_history` (Free plan). `nw_equity_risk_premium` splits
  `total_erp` into `mature_erp` and `country_risk_premium`, all in percent as decimal strings,
  with a `basis` for each part. Release the answers with `nw_equity_risk_premium_list_free`,
  `nw_equity_risk_premium_free` and `nw_equity_risk_premium_history_response_free`.
- Companies: `nw_filings_company` returns `nw_company_with_listings`, a company with the stock
  listing ids (`listings`) you can pass to `nw_equities_*`.
- Filings API: `nw_filings_search_companies`, `nw_filings_company`, `nw_filings_financials`,
  `nw_filings_list_company_filings`, `nw_filings_filing`, `nw_filings_list_latest_filings`,
  `nw_filings_list_holders` and `nw_filings_as_reported`. Every money, share, percent and ratio
  value is a decimal string with the exact wire text. `nw_financial_row` holds the period
  fields (`fy`, `fq`, `fh`, `end`, `currency`, `derived`); its financial item values are not
  exposed in the C ABI.
- Custom series: `nw_economics_search_catalogue`, `nw_economics_read_catalogue_series`,
  `nw_economics_read_catalogue_observations` and `nw_economics_read_catalogue_latest_batch`.
  Series ids look like `fred:unrate`.
- Registries: filings cover 31 registries (`usa`, `jpn`, `kor`, `twn`, `gbr` and 26 EU and EEA
  countries). The UK is covered for UK issuers on regulated markets (annual reports), and `gbr`
  company ids are LEIs, for example `gbr:549300MKFYEKVRWML317`. Taiwan includes TPEx issuers
  (listing MIC `ROCO`) besides `XTAI`.
- Taiwan: filings accept `period` `"semiannual"`. Taiwan reports semiannual year-to-date income
  statements and balance sheets.
- As-reported statements are available for US filings only; other registries answer
  `as_reported_unavailable` (404).
- Webhooks: the new `filings` family and the `companies` watch list on destinations;
  deliveries for it carry a `filing` record (`nw_filing_webhook_event`).
- History reads: the economics `*_history` params (`nw_economics_cpi_history_params`, gdp,
  gdp growth, inflation, labour force participation, unemployment and trade) take optional
  `as_of` and `version`; `nw_economics_yield_curve_history_params` takes `as_of` only.
  Observation rows have `first_reported_at`, and `nw_resource_history` has `as_of`.
- `nw_webhook_verify` returns `nw_webhook_event` parsed from the documented delivery bodies.
  Free it with `nw_webhook_event_free`.
- `nw_economics_read_series_observations`; prediction history takes `cursor` and returns
  `next_cursor`; webhook destinations carry `disabled_reason`.

## 0.1.0 — not published (folded into 0.2.0)

- `libnarwhalapi`, a C ABI over the Rust client, built from source as static and
  shared libraries, with the generated header `narwhalapi.h` and a pkg-config file.
- Exact decimal strings (`const char *`), typed error accessors, response metadata,
  size-prefixed params structs and `nw_version_check`.
- Stable economics, FX, calendars, physical palm oil, spot price
  (`nw_commodities_spot_price`), events, webhook destinations, calendar subscriptions and
  `nw_webhook_verify` are declared without `-DNW_UNSTABLE`. Stable field order is locked
  from this release on: later releases only append.
- `-DNW_UNSTABLE` declares operations not yet in the live API; none in this release.
- The release also covers prediction market reads, FX and commodity bar history, and physical commodity reference price series.
- Spot `bid` and `ask` are optional because the API may omit them.

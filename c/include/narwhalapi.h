/* Generated from spec/next/openapi.json sha256 5071bd97e9507eb56b344d00a24ba056ce1340fe1184faa4d755fe7e407e1cf6. Do not edit. */
/*
 * narwhalapi.h: C client for the Narwhal API.
 *
 * Decimals are exact wire strings (`const char *`); never convert them to floating point.
 * Operations marked "Stability: unstable" and their types are declared only when
 * NW_UNSTABLE is defined before this header is included.
 *
 * Ownership rules
 *
 * 1. The caller owns every `**out` it receives and frees it with that type's `_free`. Passing
 *    `NULL` to `_free` is a no-op.
 * 2. Everything reachable from a response is owned by that response and stays valid until its
 *    `_free`. Never free inner pointers.
 * 3. Input pointers are borrowed for the duration of the call only; nothing is retained.
 * 4. On failure `*out` stays `NULL` and `*err` is set (when `err` is non-NULL). On success
 *    `*err` stays `NULL`.
 * 5. `nw_client` may be shared across threads, and calls on it run concurrently. Free it only
 *    after all calls on it have returned.
 * 6. No global init. Rust panics never cross the boundary: every export is wrapped in
 *    `catch_unwind`. Status-returning exports return `NW_ERR_INTERNAL`; other exports use
 *    the fallbacks below.
 * 7. Strings are NUL-terminated UTF-8, and arrays are `const T *items` plus `size_t items_len`.
 *    Integer fields from the API are `int64_t`, enums are `int32_t`, and status accessors return
 *    `int`.
 *
 * Return values without a status
 *
 * Every export catches panics. Required NULL arguments to status-returning exports return
 * `NW_ERR_INVALID_ARGUMENT`; a caught panic returns `NW_ERR_INTERNAL`. Optional NULL inputs
 * remain valid as documented. Non-status exports use these fallbacks on NULL or panic:
 *
 * - Void functions (`*_free`, including `nw_client_free`) do nothing; `_free(NULL)` is a no-op.
 * - Pointer-returning accessors return NULL.
 * - `nw_error_status` returns 0.
 * - `nw_error_retry_after` returns -1.
 * - `nw_version` returns a static string and is never NULL.
 * - `nw_version_check` returns false.
 *
 * ABI stability
 *
 * Patch releases keep the ABI for dynamically linked programs. Response structs only gain
 * fields at the end, and existing enum constants keep their values (new values take the
 * next number). Union structs start with `kind`, `unknown_raw` and `meta`; variant pointers
 * follow and new variants are appended. Structs the caller allocates (`nw_config` and every
 * `*_params`) start with `size_t size`, which the caller sets to `sizeof` the struct; the
 * library reads only fields inside that size, so a program built against an older header
 * keeps working. `size` 0 is `NW_ERR_INVALID_ARGUMENT`. `nw_header` is frozen. Call
 * `nw_version_check(NW_VERSION_MAJOR, NW_VERSION_MINOR)` at startup to confirm the loaded
 * library serves this header.
 *
 * Representation
 *
 * - Optional strings, models and arrays are NULL when absent; a present empty array has a
 *   non-NULL pointer and length 0. That pointer must not be dereferenced, and passing it to
 *   memcpy or memcmp is undefined even with length 0: check the length first. Optional
 *   integers and booleans carry `has_<field>`.
 * - An enum field `x` is an `int32_t` constant (`NW_<ENUM>_UNKNOWN` for values this version
 *   does not know) and `x_raw` is its exact wire text.
 * - A response string containing U+0000 cannot be a C string; the call returns
 *   `NW_ERR_DECODE`.
 * - Arrays of models and unions hold pointers (`const nw_x *const *items`), and a model's
 *   `meta` is a pointer, so response structs are only ever reached through pointers.
 * - Paginated operations return one page. Pass the page's `next_cursor` as `params->cursor`
 *   to fetch the next; `next_cursor` is NULL on the last page.
 */

#ifndef NARWHALAPI_H
#define NARWHALAPI_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// `CatalogueLatestFrequency`: `annual`.
#define NW_CATALOGUE_LATEST_FREQUENCY_ANNUAL 1

// `CatalogueLatestFrequency`: `daily`.
#define NW_CATALOGUE_LATEST_FREQUENCY_DAILY 2

// `CatalogueLatestFrequency`: `monthly`.
#define NW_CATALOGUE_LATEST_FREQUENCY_MONTHLY 3

// `CatalogueLatestFrequency`: `quarterly`.
#define NW_CATALOGUE_LATEST_FREQUENCY_QUARTERLY 4

// `CatalogueLatestFrequency`: a value this version does not know.
#define NW_CATALOGUE_LATEST_FREQUENCY_UNKNOWN 0

// `CatalogueLatestSeasonalAdjustment`: `not_applicable`.
#define NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_APPLICABLE 1

// `CatalogueLatestSeasonalAdjustment`: `not_seasonally_adjusted`.
#define NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_SEASONALLY_ADJUSTED 2

// `CatalogueLatestSeasonalAdjustment`: `not_stated`.
#define NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_STATED 3

// `CatalogueLatestSeasonalAdjustment`: `seasonally_adjusted`.
#define NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_SEASONALLY_ADJUSTED 4

// `CatalogueLatestSeasonalAdjustment`: a value this version does not know.
#define NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_UNKNOWN 0

// `CentralBankBalanceSheetScale`: `millions`.
#define NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_MILLIONS 1

// `CentralBankBalanceSheetScale`: `units`.
#define NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_UNITS 2

// `CentralBankBalanceSheetScale`: a value this version does not know.
#define NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_UNKNOWN 0

// `CommodityUnit`: `barrel`.
#define NW_COMMODITY_UNIT_BARREL 1

// `CommodityUnit`: `pound`.
#define NW_COMMODITY_UNIT_POUND 2

// `CommodityUnit`: `troy_ounce`.
#define NW_COMMODITY_UNIT_TROY_OUNCE 3

// `CommodityUnit`: a value this version does not know.
#define NW_COMMODITY_UNIT_UNKNOWN 0

// `CompanyRelationKind`: `cross_listed`.
#define NW_COMPANY_RELATION_KIND_CROSS_LISTED 1

// `CompanyRelationKind`: `predecessor`.
#define NW_COMPANY_RELATION_KIND_PREDECESSOR 2

// `CompanyRelationKind`: `successor`.
#define NW_COMPANY_RELATION_KIND_SUCCESSOR 3

// `CompanyRelationKind`: a value this version does not know.
#define NW_COMPANY_RELATION_KIND_UNKNOWN 0

// `CompanySearchMatch`: `code`.
#define NW_COMPANY_SEARCH_MATCH_CODE 1

// `CompanySearchMatch`: `former_code`.
#define NW_COMPANY_SEARCH_MATCH_FORMER_CODE 2

// `CompanySearchMatch`: `former_name`.
#define NW_COMPANY_SEARCH_MATCH_FORMER_NAME 3

// `CompanySearchMatch`: `id`.
#define NW_COMPANY_SEARCH_MATCH_ID 4

// `CompanySearchMatch`: `name`.
#define NW_COMPANY_SEARCH_MATCH_NAME 5

// `CompanySearchMatch`: a value this version does not know.
#define NW_COMPANY_SEARCH_MATCH_UNKNOWN 0

// `CompanyStandard`: `ifrs`.
#define NW_COMPANY_STANDARD_IFRS 1

// `CompanyStandard`: `jmis`.
#define NW_COMPANY_STANDARD_JMIS 3

// `CompanyStandard`: `j_gaap`.
#define NW_COMPANY_STANDARD_J_GAAP 2

// `CompanyStandard`: `k_gaap`.
#define NW_COMPANY_STANDARD_K_GAAP 4

// `CompanyStandard`: a value this version does not know.
#define NW_COMPANY_STANDARD_UNKNOWN 0

// `CompanyStandard`: `us_gaap`.
#define NW_COMPANY_STANDARD_US_GAAP 5

// `CostEquityRiskPremiumBasis`: `modelled`.
#define NW_COST_EQUITY_RISK_PREMIUM_BASIS_MODELLED 1

// `CostEquityRiskPremiumBasis`: `observed`.
#define NW_COST_EQUITY_RISK_PREMIUM_BASIS_OBSERVED 2

// `CostEquityRiskPremiumBasis`: `table`.
#define NW_COST_EQUITY_RISK_PREMIUM_BASIS_TABLE 3

// `CostEquityRiskPremiumBasis`: a value this version does not know.
#define NW_COST_EQUITY_RISK_PREMIUM_BASIS_UNKNOWN 0

// `CostEquityRiskPremiumSource`: `caller`.
#define NW_COST_EQUITY_RISK_PREMIUM_SOURCE_CALLER 1

// `CostEquityRiskPremiumSource`: `narwhal`.
#define NW_COST_EQUITY_RISK_PREMIUM_SOURCE_NARWHAL 2

// `CostEquityRiskPremiumSource`: a value this version does not know.
#define NW_COST_EQUITY_RISK_PREMIUM_SOURCE_UNKNOWN 0

// `CostOfCapitalBeta`: `adjusted`.
#define NW_COST_OF_CAPITAL_BETA_ADJUSTED 1

// `CostOfCapitalBeta`: `raw`.
#define NW_COST_OF_CAPITAL_BETA_RAW 2

// `CostOfCapitalBeta`: a value this version does not know.
#define NW_COST_OF_CAPITAL_BETA_UNKNOWN 0

// `CostOfCapitalFrequency`: `monthly`.
#define NW_COST_OF_CAPITAL_FREQUENCY_MONTHLY 1

// `CostOfCapitalFrequency`: a value this version does not know.
#define NW_COST_OF_CAPITAL_FREQUENCY_UNKNOWN 0

// `CostOfCapitalFrequency`: `weekly`.
#define NW_COST_OF_CAPITAL_FREQUENCY_WEEKLY 2

// `CostOfCapitalPeriod`: `annual`.
#define NW_COST_OF_CAPITAL_PERIOD_ANNUAL 1

// `CostOfCapitalPeriod`: `ttm`.
#define NW_COST_OF_CAPITAL_PERIOD_TTM 2

// `CostOfCapitalPeriod`: a value this version does not know.
#define NW_COST_OF_CAPITAL_PERIOD_UNKNOWN 0

// `CostOfCapitalWindow`: `2y`.
#define NW_COST_OF_CAPITAL_WINDOW_2Y 1

// `CostOfCapitalWindow`: `5y`.
#define NW_COST_OF_CAPITAL_WINDOW_5Y 2

// `CostOfCapitalWindow`: a value this version does not know.
#define NW_COST_OF_CAPITAL_WINDOW_UNKNOWN 0

// `CostOfDebtBasis`: `ours`.
#define NW_COST_OF_DEBT_BASIS_OURS 1

// `CostOfDebtBasis`: `table`.
#define NW_COST_OF_DEBT_BASIS_TABLE 2

// `CostOfDebtBasis`: a value this version does not know.
#define NW_COST_OF_DEBT_BASIS_UNKNOWN 0

// `CostRiskFreeBasis`: `caller`.
#define NW_COST_RISK_FREE_BASIS_CALLER 1

// `CostRiskFreeBasis`: `ours`.
#define NW_COST_RISK_FREE_BASIS_OURS 2

// `CostRiskFreeBasis`: a value this version does not know.
#define NW_COST_RISK_FREE_BASIS_UNKNOWN 0

// `CostRiskFreeSource`: `caller`.
#define NW_COST_RISK_FREE_SOURCE_CALLER 1

// `CostRiskFreeSource`: a value this version does not know.
#define NW_COST_RISK_FREE_SOURCE_UNKNOWN 0

// `CostRiskFreeSource`: `yield_curve`.
#define NW_COST_RISK_FREE_SOURCE_YIELD_CURVE 2

// `CostTaxRateBasis`: `caller`.
#define NW_COST_TAX_RATE_BASIS_CALLER 1

// `CostTaxRateBasis`: `table`.
#define NW_COST_TAX_RATE_BASIS_TABLE 2

// `CostTaxRateBasis`: a value this version does not know.
#define NW_COST_TAX_RATE_BASIS_UNKNOWN 0

// `CostTaxRateSource`: `caller`.
#define NW_COST_TAX_RATE_SOURCE_CALLER 1

// `CostTaxRateSource`: `country_table`.
#define NW_COST_TAX_RATE_SOURCE_COUNTRY_TABLE 3

// `CostTaxRateSource`: a value this version does not know.
#define NW_COST_TAX_RATE_SOURCE_UNKNOWN 0

// `CryptoPriceErrorCode`: `not_covered`.
#define NW_CRYPTO_PRICE_ERROR_CODE_NOT_COVERED 1

// `CryptoPriceErrorCode`: `not_found`.
#define NW_CRYPTO_PRICE_ERROR_CODE_NOT_FOUND 2

// `CryptoPriceErrorCode`: a value this version does not know.
#define NW_CRYPTO_PRICE_ERROR_CODE_UNKNOWN 0

// `CryptoPriceErrorCode`: `upstream_busy`.
#define NW_CRYPTO_PRICE_ERROR_CODE_UPSTREAM_BUSY 3

// `DeliveryFamily`: `activity`.
#define NW_DELIVERY_FAMILY_ACTIVITY 7

// `DeliveryFamily`: `gdp`.
#define NW_DELIVERY_FAMILY_GDP 1

// `DeliveryFamily`: `holidays`.
#define NW_DELIVERY_FAMILY_HOLIDAYS 2

// `DeliveryFamily`: `housing`.
#define NW_DELIVERY_FAMILY_HOUSING 8

// `DeliveryFamily`: `labour`.
#define NW_DELIVERY_FAMILY_LABOUR 3

// `DeliveryFamily`: `money`.
#define NW_DELIVERY_FAMILY_MONEY 9

// `DeliveryFamily`: `policy_rate`.
#define NW_DELIVERY_FAMILY_POLICY_RATE 10

// `DeliveryFamily`: `prices`.
#define NW_DELIVERY_FAMILY_PRICES 4

// `DeliveryFamily`: `public_finance`.
#define NW_DELIVERY_FAMILY_PUBLIC_FINANCE 11

// `DeliveryFamily`: `survey`.
#define NW_DELIVERY_FAMILY_SURVEY 12

// `DeliveryFamily`: `trade`.
#define NW_DELIVERY_FAMILY_TRADE 5

// `DeliveryFamily`: a value this version does not know.
#define NW_DELIVERY_FAMILY_UNKNOWN 0

// `DeliveryFamily`: `yield_curve`.
#define NW_DELIVERY_FAMILY_YIELD_CURVE 6

// `EconomicFamily`: `activity`.
#define NW_ECONOMIC_FAMILY_ACTIVITY 6

// `EconomicFamily`: `gdp`.
#define NW_ECONOMIC_FAMILY_GDP 1

// `EconomicFamily`: `housing`.
#define NW_ECONOMIC_FAMILY_HOUSING 7

// `EconomicFamily`: `labour`.
#define NW_ECONOMIC_FAMILY_LABOUR 2

// `EconomicFamily`: `money`.
#define NW_ECONOMIC_FAMILY_MONEY 8

// `EconomicFamily`: `policy_rate`.
#define NW_ECONOMIC_FAMILY_POLICY_RATE 9

// `EconomicFamily`: `prices`.
#define NW_ECONOMIC_FAMILY_PRICES 3

// `EconomicFamily`: `public_finance`.
#define NW_ECONOMIC_FAMILY_PUBLIC_FINANCE 10

// `EconomicFamily`: `survey`.
#define NW_ECONOMIC_FAMILY_SURVEY 11

// `EconomicFamily`: `trade`.
#define NW_ECONOMIC_FAMILY_TRADE 4

// `EconomicFamily`: a value this version does not know.
#define NW_ECONOMIC_FAMILY_UNKNOWN 0

// `EconomicFamily`: `yield_curve`.
#define NW_ECONOMIC_FAMILY_YIELD_CURVE 5

// `EconomicReleaseValueValueStatus`: `pending_capture`.
#define NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_PENDING_CAPTURE 1

// `EconomicReleaseValueValueStatus`: `unavailable`.
#define NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_UNAVAILABLE 2

// `EconomicReleaseValueValueStatus`: a value this version does not know.
#define NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_UNKNOWN 0

// `EquityEarningTime`: `after_close`.
#define NW_EQUITY_EARNING_TIME_AFTER_CLOSE 1

// `EquityEarningTime`: `before_open`.
#define NW_EQUITY_EARNING_TIME_BEFORE_OPEN 2

// `EquityEarningTime`: `during`.
#define NW_EQUITY_EARNING_TIME_DURING 3

// `EquityEarningTime`: a value this version does not know.
#define NW_EQUITY_EARNING_TIME_UNKNOWN 0

// `EquityMarketSession`: `closed`.
#define NW_EQUITY_MARKET_SESSION_CLOSED 1

// `EquityMarketSession`: `open`.
#define NW_EQUITY_MARKET_SESSION_OPEN 2

// `EquityMarketSession`: `post`.
#define NW_EQUITY_MARKET_SESSION_POST 3

// `EquityMarketSession`: `pre`.
#define NW_EQUITY_MARKET_SESSION_PRE 4

// `EquityMarketSession`: a value this version does not know.
#define NW_EQUITY_MARKET_SESSION_UNKNOWN 0

// `EquityType`: `closed_end_fund`.
#define NW_EQUITY_TYPE_CLOSED_END_FUND 1

// `EquityType`: `common_stock`.
#define NW_EQUITY_TYPE_COMMON_STOCK 2

// `EquityType`: `depositary_receipt`.
#define NW_EQUITY_TYPE_DEPOSITARY_RECEIPT 3

// `EquityType`: `other`.
#define NW_EQUITY_TYPE_OTHER 4

// `EquityType`: `preferred_stock`.
#define NW_EQUITY_TYPE_PREFERRED_STOCK 5

// `EquityType`: `reit`.
#define NW_EQUITY_TYPE_REIT 6

// `EquityType`: `right`.
#define NW_EQUITY_TYPE_RIGHT 7

// `EquityType`: `unit`.
#define NW_EQUITY_TYPE_UNIT 8

// `EquityType`: a value this version does not know.
#define NW_EQUITY_TYPE_UNKNOWN 0

// `EquityType`: `warrant`.
#define NW_EQUITY_TYPE_WARRANT 9

// `ExtremeObservation`: `any_release`.
#define NW_EXTREME_OBSERVATION_ANY_RELEASE 3

// `ExtremeObservation`: `any_trade`.
#define NW_EXTREME_OBSERVATION_ANY_TRADE 1

// `ExtremeObservation`: `close`.
#define NW_EXTREME_OBSERVATION_CLOSE 2

// `ExtremeObservation`: a value this version does not know.
#define NW_EXTREME_OBSERVATION_UNKNOWN 0

// `FilingEventType`: `annual`.
#define NW_FILING_EVENT_TYPE_ANNUAL 1

// `FilingEventType`: `event`.
#define NW_FILING_EVENT_TYPE_EVENT 2

// `FilingEventType`: `ownership`.
#define NW_FILING_EVENT_TYPE_OWNERSHIP 3

// `FilingEventType`: `quarterly`.
#define NW_FILING_EVENT_TYPE_QUARTERLY 4

// `FilingEventType`: `semiannual`.
#define NW_FILING_EVENT_TYPE_SEMIANNUAL 5

// `FilingEventType`: a value this version does not know.
#define NW_FILING_EVENT_TYPE_UNKNOWN 0

// `FilingType`: `annual`.
#define NW_FILING_TYPE_ANNUAL 1

// `FilingType`: `event`.
#define NW_FILING_TYPE_EVENT 2

// `FilingType`: `insider`.
#define NW_FILING_TYPE_INSIDER 3

// `FilingType`: `other`.
#define NW_FILING_TYPE_OTHER 4

// `FilingType`: `ownership`.
#define NW_FILING_TYPE_OWNERSHIP 5

// `FilingType`: `quarterly`.
#define NW_FILING_TYPE_QUARTERLY 6

// `FilingType`: `semiannual`.
#define NW_FILING_TYPE_SEMIANNUAL 7

// `FilingType`: a value this version does not know.
#define NW_FILING_TYPE_UNKNOWN 0

// `FinancialsBasis`: `consolidated`.
#define NW_FINANCIALS_BASIS_CONSOLIDATED 1

// `FinancialsBasis`: `standalone`.
#define NW_FINANCIALS_BASIS_STANDALONE 2

// `FinancialsBasis`: a value this version does not know.
#define NW_FINANCIALS_BASIS_UNKNOWN 0

// `FiscalPeriod`: `FY`.
#define NW_FISCAL_PERIOD_FY 1

// `FiscalPeriod`: `H1`.
#define NW_FISCAL_PERIOD_H1 2

// `FiscalPeriod`: `H2`.
#define NW_FISCAL_PERIOD_H2 3

// `FiscalPeriod`: `Q1`.
#define NW_FISCAL_PERIOD_Q1 4

// `FiscalPeriod`: `Q2`.
#define NW_FISCAL_PERIOD_Q2 5

// `FiscalPeriod`: `Q3`.
#define NW_FISCAL_PERIOD_Q3 6

// `FiscalPeriod`: `Q4`.
#define NW_FISCAL_PERIOD_Q4 7

// `FiscalPeriod`: a value this version does not know.
#define NW_FISCAL_PERIOD_UNKNOWN 0

// `FundDistributionKind`: `capital_gain`.
#define NW_FUND_DISTRIBUTION_KIND_CAPITAL_GAIN 1

// `FundDistributionKind`: `income`.
#define NW_FUND_DISTRIBUTION_KIND_INCOME 2

// `FundDistributionKind`: a value this version does not know.
#define NW_FUND_DISTRIBUTION_KIND_UNKNOWN 0

// `FundType`: `etf`.
#define NW_FUND_TYPE_ETF 1

// `FundType`: `mutual_fund`.
#define NW_FUND_TYPE_MUTUAL_FUND 2

// `FundType`: a value this version does not know.
#define NW_FUND_TYPE_UNKNOWN 0

// `HolidayScope`: `bank`.
#define NW_HOLIDAY_SCOPE_BANK 1

// `HolidayScope`: `central_government`.
#define NW_HOLIDAY_SCOPE_CENTRAL_GOVERNMENT 2

// `HolidayScope`: `collective_leave`.
#define NW_HOLIDAY_SCOPE_COLLECTIVE_LEAVE 3

// `HolidayScope`: `federal_government`.
#define NW_HOLIDAY_SCOPE_FEDERAL_GOVERNMENT 4

// `HolidayScope`: `national_public`.
#define NW_HOLIDAY_SCOPE_NATIONAL_PUBLIC 5

// `HolidayScope`: `private_sector`.
#define NW_HOLIDAY_SCOPE_PRIVATE_SECTOR 6

// `HolidayScope`: `public_sector`.
#define NW_HOLIDAY_SCOPE_PUBLIC_SECTOR 7

// `HolidayScope`: `regional_public`.
#define NW_HOLIDAY_SCOPE_REGIONAL_PUBLIC 8

// `HolidayScope`: a value this version does not know.
#define NW_HOLIDAY_SCOPE_UNKNOWN 0

// `IndicatorUnit`: `balance`.
#define NW_INDICATOR_UNIT_BALANCE 1

// `IndicatorUnit`: `currency`.
#define NW_INDICATOR_UNIT_CURRENCY 2

// `IndicatorUnit`: `index`.
#define NW_INDICATOR_UNIT_INDEX 3

// `IndicatorUnit`: `percent`.
#define NW_INDICATOR_UNIT_PERCENT 4

// `IndicatorUnit`: `ratio`.
#define NW_INDICATOR_UNIT_RATIO 5

// `IndicatorUnit`: `thousands`.
#define NW_INDICATOR_UNIT_THOUSANDS 6

// `IndicatorUnit`: a value this version does not know.
#define NW_INDICATOR_UNIT_UNKNOWN 0

// `MarketSession`: `interbank_closed`.
#define NW_MARKET_SESSION_INTERBANK_CLOSED 1

// `MarketSession`: `open`.
#define NW_MARKET_SESSION_OPEN 2

// `MarketSession`: a value this version does not know.
#define NW_MARKET_SESSION_UNKNOWN 0

// `MarketSession`: `weekend`.
#define NW_MARKET_SESSION_WEEKEND 3

// `MoversDirection`: `active`.
#define NW_MOVERS_DIRECTION_ACTIVE 1

// `MoversDirection`: `gainers`.
#define NW_MOVERS_DIRECTION_GAINERS 2

// `MoversDirection`: `losers`.
#define NW_MOVERS_DIRECTION_LOSERS 3

// `MoversDirection`: a value this version does not know.
#define NW_MOVERS_DIRECTION_UNKNOWN 0

// `PalmoilFrequency`: `daily`.
#define NW_PALMOIL_FREQUENCY_DAILY 1

// `PalmoilFrequency`: `fortnightly`.
#define NW_PALMOIL_FREQUENCY_FORTNIGHTLY 2

// `PalmoilFrequency`: `irregular`.
#define NW_PALMOIL_FREQUENCY_IRREGULAR 3

// `PalmoilFrequency`: `monthly`.
#define NW_PALMOIL_FREQUENCY_MONTHLY 4

// `PalmoilFrequency`: a value this version does not know.
#define NW_PALMOIL_FREQUENCY_UNKNOWN 0

// `PalmoilFrequency`: `weekly`.
#define NW_PALMOIL_FREQUENCY_WEEKLY 5

// `PalmoilPeriodType`: `effective`.
#define NW_PALMOIL_PERIOD_TYPE_EFFECTIVE 1

// `PalmoilPeriodType`: `observation`.
#define NW_PALMOIL_PERIOD_TYPE_OBSERVATION 2

// `PalmoilPeriodType`: a value this version does not know.
#define NW_PALMOIL_PERIOD_TYPE_UNKNOWN 0

// `PalmoilProduct`: `cpo`.
#define NW_PALMOIL_PRODUCT_CPO 1

// `PalmoilProduct`: `ffb`.
#define NW_PALMOIL_PRODUCT_FFB 2

// `PalmoilProduct`: a value this version does not know.
#define NW_PALMOIL_PRODUCT_UNKNOWN 0

// `PalmoilScalarPriceType`: `government_reference`.
#define NW_PALMOIL_SCALAR_PRICE_TYPE_GOVERNMENT_REFERENCE 1

// `PalmoilScalarPriceType`: `market_average`.
#define NW_PALMOIL_SCALAR_PRICE_TYPE_MARKET_AVERAGE 2

// `PalmoilScalarPriceType`: a value this version does not know.
#define NW_PALMOIL_SCALAR_PRICE_TYPE_UNKNOWN 0

// `PalmoilScalarUnit`: `metric_ton`.
#define NW_PALMOIL_SCALAR_UNIT_METRIC_TON 1

// `PalmoilScalarUnit`: `one_percent_oer`.
#define NW_PALMOIL_SCALAR_UNIT_ONE_PERCENT_OER 2

// `PalmoilScalarUnit`: a value this version does not know.
#define NW_PALMOIL_SCALAR_UNIT_UNKNOWN 0

// `PhysicalCategory`: `emissions`.
#define NW_PHYSICAL_CATEGORY_EMISSIONS 1

// `PhysicalCategory`: `energy`.
#define NW_PHYSICAL_CATEGORY_ENERGY 2

// `PhysicalCategory`: `fertilizers`.
#define NW_PHYSICAL_CATEGORY_FERTILIZERS 3

// `PhysicalCategory`: `forestry`.
#define NW_PHYSICAL_CATEGORY_FORESTRY 4

// `PhysicalCategory`: `grains`.
#define NW_PHYSICAL_CATEGORY_GRAINS 5

// `PhysicalCategory`: `livestock-dairy`.
#define NW_PHYSICAL_CATEGORY_LIVESTOCK_DAIRY 6

// `PhysicalCategory`: `metals`.
#define NW_PHYSICAL_CATEGORY_METALS 7

// `PhysicalCategory`: `oils-oilseeds`.
#define NW_PHYSICAL_CATEGORY_OILS_OILSEEDS 8

// `PhysicalCategory`: `softs`.
#define NW_PHYSICAL_CATEGORY_SOFTS 9

// `PhysicalCategory`: a value this version does not know.
#define NW_PHYSICAL_CATEGORY_UNKNOWN 0

// `PhysicalFrequency`: `annual`.
#define NW_PHYSICAL_FREQUENCY_ANNUAL 1

// `PhysicalFrequency`: `daily`.
#define NW_PHYSICAL_FREQUENCY_DAILY 2

// `PhysicalFrequency`: `intraday`.
#define NW_PHYSICAL_FREQUENCY_INTRADAY 3

// `PhysicalFrequency`: `irregular`.
#define NW_PHYSICAL_FREQUENCY_IRREGULAR 4

// `PhysicalFrequency`: `monthly`.
#define NW_PHYSICAL_FREQUENCY_MONTHLY 5

// `PhysicalFrequency`: `quarterly`.
#define NW_PHYSICAL_FREQUENCY_QUARTERLY 6

// `PhysicalFrequency`: `semiannual`.
#define NW_PHYSICAL_FREQUENCY_SEMIANNUAL 7

// `PhysicalFrequency`: `three_times_monthly`.
#define NW_PHYSICAL_FREQUENCY_THREE_TIMES_MONTHLY 8

// `PhysicalFrequency`: `twice_monthly`.
#define NW_PHYSICAL_FREQUENCY_TWICE_MONTHLY 9

// `PhysicalFrequency`: a value this version does not know.
#define NW_PHYSICAL_FREQUENCY_UNKNOWN 0

// `PhysicalFrequency`: `weekly`.
#define NW_PHYSICAL_FREQUENCY_WEEKLY 10

// `PhysicalPriceType`: `auction_clearing`.
#define NW_PHYSICAL_PRICE_TYPE_AUCTION_CLEARING 1

// `PhysicalPriceType`: `export_unit_value`.
#define NW_PHYSICAL_PRICE_TYPE_EXPORT_UNIT_VALUE 2

// `PhysicalPriceType`: `farmgate_average`.
#define NW_PHYSICAL_PRICE_TYPE_FARMGATE_AVERAGE 3

// `PhysicalPriceType`: `government_reference`.
#define NW_PHYSICAL_PRICE_TYPE_GOVERNMENT_REFERENCE 4

// `PhysicalPriceType`: `import_unit_value`.
#define NW_PHYSICAL_PRICE_TYPE_IMPORT_UNIT_VALUE 5

// `PhysicalPriceType`: `international_benchmark`.
#define NW_PHYSICAL_PRICE_TYPE_INTERNATIONAL_BENCHMARK 6

// `PhysicalPriceType`: `market_average`.
#define NW_PHYSICAL_PRICE_TYPE_MARKET_AVERAGE 7

// `PhysicalPriceType`: `official_buy`.
#define NW_PHYSICAL_PRICE_TYPE_OFFICIAL_BUY 8

// `PhysicalPriceType`: `official_reference_rate`.
#define NW_PHYSICAL_PRICE_TYPE_OFFICIAL_REFERENCE_RATE 9

// `PhysicalPriceType`: `official_sell`.
#define NW_PHYSICAL_PRICE_TYPE_OFFICIAL_SELL 10

// `PhysicalPriceType`: `regulated_average`.
#define NW_PHYSICAL_PRICE_TYPE_REGULATED_AVERAGE 11

// `PhysicalPriceType`: `retail_average`.
#define NW_PHYSICAL_PRICE_TYPE_RETAIL_AVERAGE 12

// `PhysicalPriceType`: `retail_buy`.
#define NW_PHYSICAL_PRICE_TYPE_RETAIL_BUY 13

// `PhysicalPriceType`: `retail_sell`.
#define NW_PHYSICAL_PRICE_TYPE_RETAIL_SELL 14

// `PhysicalPriceType`: a value this version does not know.
#define NW_PHYSICAL_PRICE_TYPE_UNKNOWN 0

// `PhysicalPriceType`: `wholesale_average`.
#define NW_PHYSICAL_PRICE_TYPE_WHOLESALE_AVERAGE 15

// `PointPolicyRateQualifier`: `around`.
#define NW_POINT_POLICY_RATE_QUALIFIER_AROUND 1

// `PointPolicyRateQualifier`: `exact`.
#define NW_POINT_POLICY_RATE_QUALIFIER_EXACT 2

// `PointPolicyRateQualifier`: a value this version does not know.
#define NW_POINT_POLICY_RATE_QUALIFIER_UNKNOWN 0

// `PredictionEventStatus`: `closed`.
#define NW_PREDICTION_EVENT_STATUS_CLOSED 1

// `PredictionEventStatus`: `settled`.
#define NW_PREDICTION_EVENT_STATUS_SETTLED 2

// `PredictionEventStatus`: `trading`.
#define NW_PREDICTION_EVENT_STATUS_TRADING 3

// `PredictionEventStatus`: a value this version does not know.
#define NW_PREDICTION_EVENT_STATUS_UNKNOWN 0

// `PredictionEventStatus`: `upcoming`.
#define NW_PREDICTION_EVENT_STATUS_UPCOMING 4

// `PredictionEventStatus`: `void`.
#define NW_PREDICTION_EVENT_STATUS_VOID 5

// `PredictionShape`: `binary`.
#define NW_PREDICTION_SHAPE_BINARY 1

// `PredictionShape`: `categorical`.
#define NW_PREDICTION_SHAPE_CATEGORICAL 2

// `PredictionShape`: `extreme`.
#define NW_PREDICTION_SHAPE_EXTREME 3

// `PredictionShape`: `scalar`.
#define NW_PREDICTION_SHAPE_SCALAR 4

// `PredictionShape`: `set`.
#define NW_PREDICTION_SHAPE_SET 5

// `PredictionShape`: `timing`.
#define NW_PREDICTION_SHAPE_TIMING 6

// `PredictionShape`: a value this version does not know.
#define NW_PREDICTION_SHAPE_UNKNOWN 0

// `PredictionVenueSettlement`: `settled`.
#define NW_PREDICTION_VENUE_SETTLEMENT_SETTLED 1

// `PredictionVenueSettlement`: a value this version does not know.
#define NW_PREDICTION_VENUE_SETTLEMENT_UNKNOWN 0

// `PredictionVenueSettlement`: `void`.
#define NW_PREDICTION_VENUE_SETTLEMENT_VOID 2

// `PredictionWebhookStatus`: `closed`.
#define NW_PREDICTION_WEBHOOK_STATUS_CLOSED 1

// `PredictionWebhookStatus`: `settled`.
#define NW_PREDICTION_WEBHOOK_STATUS_SETTLED 2

// `PredictionWebhookStatus`: a value this version does not know.
#define NW_PREDICTION_WEBHOOK_STATUS_UNKNOWN 0

// `PredictionWebhookStatus`: `void`.
#define NW_PREDICTION_WEBHOOK_STATUS_VOID 3

// `ProducerArrangement`: `purchase_agreement`.
#define NW_PRODUCER_ARRANGEMENT_PURCHASE_AGREEMENT 1

// `ProducerArrangement`: `supported_scheme`.
#define NW_PRODUCER_ARRANGEMENT_SUPPORTED_SCHEME 2

// `ProducerArrangement`: a value this version does not know.
#define NW_PRODUCER_ARRANGEMENT_UNKNOWN 0

// `Registry`: `aut`.
#define NW_REGISTRY_AUT 4

// `Registry`: `bel`.
#define NW_REGISTRY_BEL 5

// `Registry`: `cyp`.
#define NW_REGISTRY_CYP 6

// `Registry`: `cze`.
#define NW_REGISTRY_CZE 7

// `Registry`: `dnk`.
#define NW_REGISTRY_DNK 8

// `Registry`: `esp`.
#define NW_REGISTRY_ESP 9

// `Registry`: `est`.
#define NW_REGISTRY_EST 10

// `Registry`: `fin`.
#define NW_REGISTRY_FIN 11

// `Registry`: `fra`.
#define NW_REGISTRY_FRA 12

// `Registry`: `gbr`.
#define NW_REGISTRY_GBR 13

// `Registry`: `grc`.
#define NW_REGISTRY_GRC 14

// `Registry`: `hrv`.
#define NW_REGISTRY_HRV 15

// `Registry`: `hun`.
#define NW_REGISTRY_HUN 16

// `Registry`: `isl`.
#define NW_REGISTRY_ISL 17

// `Registry`: `ita`.
#define NW_REGISTRY_ITA 18

// `Registry`: `jpn`.
#define NW_REGISTRY_JPN 1

// `Registry`: `kor`.
#define NW_REGISTRY_KOR 2

// `Registry`: `ltu`.
#define NW_REGISTRY_LTU 19

// `Registry`: `lux`.
#define NW_REGISTRY_LUX 20

// `Registry`: `lva`.
#define NW_REGISTRY_LVA 21

// `Registry`: `mlt`.
#define NW_REGISTRY_MLT 22

// `Registry`: `nld`.
#define NW_REGISTRY_NLD 23

// `Registry`: `nor`.
#define NW_REGISTRY_NOR 24

// `Registry`: `pol`.
#define NW_REGISTRY_POL 25

// `Registry`: `prt`.
#define NW_REGISTRY_PRT 26

// `Registry`: `rou`.
#define NW_REGISTRY_ROU 27

// `Registry`: `svk`.
#define NW_REGISTRY_SVK 28

// `Registry`: `svn`.
#define NW_REGISTRY_SVN 29

// `Registry`: `swe`.
#define NW_REGISTRY_SWE 30

// `Registry`: `twn`.
#define NW_REGISTRY_TWN 31

// `Registry`: a value this version does not know.
#define NW_REGISTRY_UNKNOWN 0

// `Registry`: `usa`.
#define NW_REGISTRY_USA 3

// `ReleaseStage`: `advance`.
#define NW_RELEASE_STAGE_ADVANCE 1

// `ReleaseStage`: `final`.
#define NW_RELEASE_STAGE_FINAL 2

// `ReleaseStage`: `first`.
#define NW_RELEASE_STAGE_FIRST 3

// `ReleaseStage`: `flash`.
#define NW_RELEASE_STAGE_FLASH 4

// `ReleaseStage`: `preliminary`.
#define NW_RELEASE_STAGE_PRELIMINARY 5

// `ReleaseStage`: `second`.
#define NW_RELEASE_STAGE_SECOND 6

// `ReleaseStage`: `third`.
#define NW_RELEASE_STAGE_THIRD 7

// `ReleaseStage`: a value this version does not know.
#define NW_RELEASE_STAGE_UNKNOWN 0

// `ReleaseStatus`: `cancelled`.
#define NW_RELEASE_STATUS_CANCELLED 1

// `ReleaseStatus`: `published`.
#define NW_RELEASE_STATUS_PUBLISHED 2

// `ReleaseStatus`: `scheduled`.
#define NW_RELEASE_STATUS_SCHEDULED 3

// `ReleaseStatus`: a value this version does not know.
#define NW_RELEASE_STATUS_UNKNOWN 0

// `ReleaseTopic`: `balance_of_payments`.
#define NW_RELEASE_TOPIC_BALANCE_OF_PAYMENTS 1

// `ReleaseTopic`: `business_confidence`.
#define NW_RELEASE_TOPIC_BUSINESS_CONFIDENCE 2

// `ReleaseTopic`: `construction`.
#define NW_RELEASE_TOPIC_CONSTRUCTION 3

// `ReleaseTopic`: `consumer_confidence`.
#define NW_RELEASE_TOPIC_CONSUMER_CONFIDENCE 4

// `ReleaseTopic`: `consumer_prices`.
#define NW_RELEASE_TOPIC_CONSUMER_PRICES 5

// `ReleaseTopic`: `employment`.
#define NW_RELEASE_TOPIC_EMPLOYMENT 6

// `ReleaseTopic`: `energy`.
#define NW_RELEASE_TOPIC_ENERGY 7

// `ReleaseTopic`: `gdp`.
#define NW_RELEASE_TOPIC_GDP 8

// `ReleaseTopic`: `home_sales`.
#define NW_RELEASE_TOPIC_HOME_SALES 9

// `ReleaseTopic`: `household_spending`.
#define NW_RELEASE_TOPIC_HOUSEHOLD_SPENDING 11

// `ReleaseTopic`: `house_prices`.
#define NW_RELEASE_TOPIC_HOUSE_PRICES 10

// `ReleaseTopic`: `housing_construction`.
#define NW_RELEASE_TOPIC_HOUSING_CONSTRUCTION 12

// `ReleaseTopic`: `import_export_prices`.
#define NW_RELEASE_TOPIC_IMPORT_EXPORT_PRICES 13

// `ReleaseTopic`: `industrial_production`.
#define NW_RELEASE_TOPIC_INDUSTRIAL_PRODUCTION 14

// `ReleaseTopic`: `investment`.
#define NW_RELEASE_TOPIC_INVESTMENT 15

// `ReleaseTopic`: `jobless_claims`.
#define NW_RELEASE_TOPIC_JOBLESS_CLAIMS 16

// `ReleaseTopic`: `labour_force`.
#define NW_RELEASE_TOPIC_LABOUR_FORCE 17

// `ReleaseTopic`: `manufacturing`.
#define NW_RELEASE_TOPIC_MANUFACTURING 18

// `ReleaseTopic`: `merchandise_trade`.
#define NW_RELEASE_TOPIC_MERCHANDISE_TRADE 19

// `ReleaseTopic`: `money_credit`.
#define NW_RELEASE_TOPIC_MONEY_CREDIT 20

// `ReleaseTopic`: `other`.
#define NW_RELEASE_TOPIC_OTHER 21

// `ReleaseTopic`: `policy_communication`.
#define NW_RELEASE_TOPIC_POLICY_COMMUNICATION 22

// `ReleaseTopic`: `policy_minutes`.
#define NW_RELEASE_TOPIC_POLICY_MINUTES 23

// `ReleaseTopic`: `policy_rate`.
#define NW_RELEASE_TOPIC_POLICY_RATE 24

// `ReleaseTopic`: `producer_prices`.
#define NW_RELEASE_TOPIC_PRODUCER_PRICES 25

// `ReleaseTopic`: `public_finance`.
#define NW_RELEASE_TOPIC_PUBLIC_FINANCE 26

// `ReleaseTopic`: `retail_sales`.
#define NW_RELEASE_TOPIC_RETAIL_SALES 27

// `ReleaseTopic`: `services`.
#define NW_RELEASE_TOPIC_SERVICES 28

// `ReleaseTopic`: `tourism`.
#define NW_RELEASE_TOPIC_TOURISM 29

// `ReleaseTopic`: a value this version does not know.
#define NW_RELEASE_TOPIC_UNKNOWN 0

// `ReleaseTopic`: `wages`.
#define NW_RELEASE_TOPIC_WAGES 30

// `ReleaseTopic`: `wholesale_trade`.
#define NW_RELEASE_TOPIC_WHOLESALE_TRADE 31

// `ResolutionKind`: `occurrence`.
#define NW_RESOLUTION_KIND_OCCURRENCE 1

// `ResolutionKind`: `price_fix`.
#define NW_RESOLUTION_KIND_PRICE_FIX 2

// `ResolutionKind`: `price_window`.
#define NW_RESOLUTION_KIND_PRICE_WINDOW 3

// `ResolutionKind`: `release`.
#define NW_RESOLUTION_KIND_RELEASE 4

// `ResolutionKind`: a value this version does not know.
#define NW_RESOLUTION_KIND_UNKNOWN 0

// `ResourceObservationStatus`: `break_in_series`.
#define NW_RESOURCE_OBSERVATION_STATUS_BREAK_IN_SERIES 1

// `ResourceObservationStatus`: `break_in_series_and_definition_differs`.
#define NW_RESOURCE_OBSERVATION_STATUS_BREAK_IN_SERIES_AND_DEFINITION_DIFFERS 2

// `ResourceObservationStatus`: `definition_differs`.
#define NW_RESOURCE_OBSERVATION_STATUS_DEFINITION_DIFFERS 3

// `ResourceObservationStatus`: `estimated`.
#define NW_RESOURCE_OBSERVATION_STATUS_ESTIMATED 4

// `ResourceObservationStatus`: `provisional`.
#define NW_RESOURCE_OBSERVATION_STATUS_PROVISIONAL 5

// `ResourceObservationStatus`: a value this version does not know.
#define NW_RESOURCE_OBSERVATION_STATUS_UNKNOWN 0

// `ResourceUnit`: `currency`.
#define NW_RESOURCE_UNIT_CURRENCY 1

// `ResourceUnit`: `currency_per_hour`.
#define NW_RESOURCE_UNIT_CURRENCY_PER_HOUR 2

// `ResourceUnit`: `currency_per_month`.
#define NW_RESOURCE_UNIT_CURRENCY_PER_MONTH 3

// `ResourceUnit`: `currency_per_week`.
#define NW_RESOURCE_UNIT_CURRENCY_PER_WEEK 4

// `ResourceUnit`: `currency_per_year`.
#define NW_RESOURCE_UNIT_CURRENCY_PER_YEAR 5

// `ResourceUnit`: `index`.
#define NW_RESOURCE_UNIT_INDEX 6

// `ResourceUnit`: `job`.
#define NW_RESOURCE_UNIT_JOB 7

// `ResourceUnit`: `percent`.
#define NW_RESOURCE_UNIT_PERCENT 8

// `ResourceUnit`: `person`.
#define NW_RESOURCE_UNIT_PERSON 9

// `ResourceUnit`: a value this version does not know.
#define NW_RESOURCE_UNIT_UNKNOWN 0

// `ScalarEventAbove`: `median`.
#define NW_SCALAR_EVENT_ABOVE_MEDIAN 1

// `ScalarEventAbove`: `p10`.
#define NW_SCALAR_EVENT_ABOVE_P10 2

// `ScalarEventAbove`: `p90`.
#define NW_SCALAR_EVENT_ABOVE_P90 3

// `ScalarEventAbove`: a value this version does not know.
#define NW_SCALAR_EVENT_ABOVE_UNKNOWN 0

// `SecurityType`: `adr`.
#define NW_SECURITY_TYPE_ADR 1

// `SecurityType`: `common`.
#define NW_SECURITY_TYPE_COMMON 2

// `SecurityType`: `other`.
#define NW_SECURITY_TYPE_OTHER 3

// `SecurityType`: `preferred`.
#define NW_SECURITY_TYPE_PREFERRED 4

// `SecurityType`: a value this version does not know.
#define NW_SECURITY_TYPE_UNKNOWN 0

// `SettlementStatus`: `disputed`.
#define NW_SETTLEMENT_STATUS_DISPUTED 1

// `SettlementStatus`: `pending`.
#define NW_SETTLEMENT_STATUS_PENDING 2

// `SettlementStatus`: `settled`.
#define NW_SETTLEMENT_STATUS_SETTLED 3

// `SettlementStatus`: a value this version does not know.
#define NW_SETTLEMENT_STATUS_UNKNOWN 0

// `SettlementStatus`: `void`.
#define NW_SETTLEMENT_STATUS_VOID 4

// `TimingStepKind`: `date`.
#define NW_TIMING_STEP_KIND_DATE 1

// `TimingStepKind`: `meeting`.
#define NW_TIMING_STEP_KIND_MEETING 2

// `TimingStepKind`: a value this version does not know.
#define NW_TIMING_STEP_KIND_UNKNOWN 0

// `TopicGroup`: `central_banks`.
#define NW_TOPIC_GROUP_CENTRAL_BANKS 1

// `TopicGroup`: `energy`.
#define NW_TOPIC_GROUP_ENERGY 2

// `TopicGroup`: `global_activity`.
#define NW_TOPIC_GROUP_GLOBAL_ACTIVITY 7

// `TopicGroup`: `global_prices`.
#define NW_TOPIC_GROUP_GLOBAL_PRICES 8

// `TopicGroup`: `metals`.
#define NW_TOPIC_GROUP_METALS 3

// `TopicGroup`: a value this version does not know.
#define NW_TOPIC_GROUP_UNKNOWN 0

// `TopicGroup`: `us_activity`.
#define NW_TOPIC_GROUP_US_ACTIVITY 4

// `TopicGroup`: `us_jobs`.
#define NW_TOPIC_GROUP_US_JOBS 5

// `TopicGroup`: `us_prices`.
#define NW_TOPIC_GROUP_US_PRICES 6

// `VenueDifference`: `outcome_set`.
#define NW_VENUE_DIFFERENCE_OUTCOME_SET 1

// `VenueDifference`: `rules`.
#define NW_VENUE_DIFFERENCE_RULES 2

// `VenueDifference`: `source`.
#define NW_VENUE_DIFFERENCE_SOURCE 3

// `VenueDifference`: `timing`.
#define NW_VENUE_DIFFERENCE_TIMING 4

// `VenueDifference`: a value this version does not know.
#define NW_VENUE_DIFFERENCE_UNKNOWN 0

// `VenueExclusionReason`: `outcome_set`.
#define NW_VENUE_EXCLUSION_REASON_OUTCOME_SET 1

// `VenueExclusionReason`: `rules`.
#define NW_VENUE_EXCLUSION_REASON_RULES 2

// `VenueExclusionReason`: `source`.
#define NW_VENUE_EXCLUSION_REASON_SOURCE 3

// `VenueExclusionReason`: `stale`.
#define NW_VENUE_EXCLUSION_REASON_STALE 4

// `VenueExclusionReason`: `thin`.
#define NW_VENUE_EXCLUSION_REASON_THIN 5

// `VenueExclusionReason`: `timing`.
#define NW_VENUE_EXCLUSION_REASON_TIMING 6

// `VenueExclusionReason`: a value this version does not know.
#define NW_VENUE_EXCLUSION_REASON_UNKNOWN 0

// `Venue`: `kalshi`.
#define NW_VENUE_KALSHI 1

// `Venue`: `polymarket`.
#define NW_VENUE_POLYMARKET 2

// `Venue`: a value this version does not know.
#define NW_VENUE_UNKNOWN 0

// Major version of this header and library.
#define NW_VERSION_MAJOR 0

// Minor version of this header and library.
#define NW_VERSION_MINOR 2

// Patch version of this header and library.
#define NW_VERSION_PATCH 0

// `WebhookDeliveryState`: `cancelled`.
#define NW_WEBHOOK_DELIVERY_STATE_CANCELLED 1

// `WebhookDeliveryState`: `failed`.
#define NW_WEBHOOK_DELIVERY_STATE_FAILED 2

// `WebhookDeliveryState`: `leased`.
#define NW_WEBHOOK_DELIVERY_STATE_LEASED 3

// `WebhookDeliveryState`: `pending`.
#define NW_WEBHOOK_DELIVERY_STATE_PENDING 4

// `WebhookDeliveryState`: `succeeded`.
#define NW_WEBHOOK_DELIVERY_STATE_SUCCEEDED 5

// `WebhookDeliveryState`: a value this version does not know.
#define NW_WEBHOOK_DELIVERY_STATE_UNKNOWN 0

// `WebhookDisabledReason`: `customer`.
#define NW_WEBHOOK_DISABLED_REASON_CUSTOMER 1

// `WebhookDisabledReason`: `plan_limit`.
#define NW_WEBHOOK_DISABLED_REASON_PLAN_LIMIT 2

// `WebhookDisabledReason`: a value this version does not know.
#define NW_WEBHOOK_DISABLED_REASON_UNKNOWN 0

// `WebhookEventType`: `dataset.published`.
#define NW_WEBHOOK_EVENT_TYPE_DATASET_PUBLISHED 1

// `WebhookEventType`: `dataset.revised`.
#define NW_WEBHOOK_EVENT_TYPE_DATASET_REVISED 2

// `WebhookEventType`: `filing.published`.
#define NW_WEBHOOK_EVENT_TYPE_FILING_PUBLISHED 13

// `WebhookEventType`: `holiday.calendar.revised`.
#define NW_WEBHOOK_EVENT_TYPE_HOLIDAY_CALENDAR_REVISED 3

// `WebhookEventType`: `prediction.event.closed`.
#define NW_WEBHOOK_EVENT_TYPE_PREDICTION_EVENT_CLOSED 4

// `WebhookEventType`: `prediction.event.settled`.
#define NW_WEBHOOK_EVENT_TYPE_PREDICTION_EVENT_SETTLED 5

// `WebhookEventType`: `release.cancelled`.
#define NW_WEBHOOK_EVENT_TYPE_RELEASE_CANCELLED 6

// `WebhookEventType`: `release.rescheduled`.
#define NW_WEBHOOK_EVENT_TYPE_RELEASE_RESCHEDULED 7

// `WebhookEventType`: `release.scheduled`.
#define NW_WEBHOOK_EVENT_TYPE_RELEASE_SCHEDULED 8

// `WebhookEventType`: `release.updated`.
#define NW_WEBHOOK_EVENT_TYPE_RELEASE_UPDATED 9

// `WebhookEventType`: a value this version does not know.
#define NW_WEBHOOK_EVENT_TYPE_UNKNOWN 0

// `WebhookEventType`: `webhook.test`.
#define NW_WEBHOOK_EVENT_TYPE_WEBHOOK_TEST 10

// `WebhookEventType`: `yield_curve.published`.
#define NW_WEBHOOK_EVENT_TYPE_YIELD_CURVE_PUBLISHED 11

// `WebhookEventType`: `yield_curve.revised`.
#define NW_WEBHOOK_EVENT_TYPE_YIELD_CURVE_REVISED 12

// `WebhookFamily`: `activity`.
#define NW_WEBHOOK_FAMILY_ACTIVITY 1

// `WebhookFamily`: `filings`.
#define NW_WEBHOOK_FAMILY_FILINGS 14

// `WebhookFamily`: `gdp`.
#define NW_WEBHOOK_FAMILY_GDP 2

// `WebhookFamily`: `holidays`.
#define NW_WEBHOOK_FAMILY_HOLIDAYS 3

// `WebhookFamily`: `housing`.
#define NW_WEBHOOK_FAMILY_HOUSING 4

// `WebhookFamily`: `labour`.
#define NW_WEBHOOK_FAMILY_LABOUR 5

// `WebhookFamily`: `money`.
#define NW_WEBHOOK_FAMILY_MONEY 6

// `WebhookFamily`: `policy_rate`.
#define NW_WEBHOOK_FAMILY_POLICY_RATE 7

// `WebhookFamily`: `predictions`.
#define NW_WEBHOOK_FAMILY_PREDICTIONS 8

// `WebhookFamily`: `prices`.
#define NW_WEBHOOK_FAMILY_PRICES 9

// `WebhookFamily`: `public_finance`.
#define NW_WEBHOOK_FAMILY_PUBLIC_FINANCE 10

// `WebhookFamily`: `survey`.
#define NW_WEBHOOK_FAMILY_SURVEY 11

// `WebhookFamily`: `trade`.
#define NW_WEBHOOK_FAMILY_TRADE 12

// `WebhookFamily`: a value this version does not know.
#define NW_WEBHOOK_FAMILY_UNKNOWN 0

// `WebhookFamily`: `yield_curve`.
#define NW_WEBHOOK_FAMILY_YIELD_CURVE 13

// `WebhookTestStatus`: `disabled`.
#define NW_WEBHOOK_TEST_STATUS_DISABLED 1

// `WebhookTestStatus`: `queued`.
#define NW_WEBHOOK_TEST_STATUS_QUEUED 2

// `WebhookTestStatus`: `rate_limited`.
#define NW_WEBHOOK_TEST_STATUS_RATE_LIMITED 3

// `WebhookTestStatus`: a value this version does not know.
#define NW_WEBHOOK_TEST_STATUS_UNKNOWN 0

// `YieldCurveType`: `constant_maturity_ytm`.
#define NW_YIELD_CURVE_TYPE_CONSTANT_MATURITY_YTM 1

// `YieldCurveType`: `nominal_par`.
#define NW_YIELD_CURVE_TYPE_NOMINAL_PAR 2

// `YieldCurveType`: `nominal_zero_coupon`.
#define NW_YIELD_CURVE_TYPE_NOMINAL_ZERO_COUPON 3

// `YieldCurveType`: `spot_svensson`.
#define NW_YIELD_CURVE_TYPE_SPOT_SVENSSON 4

// `YieldCurveType`: a value this version does not know.
#define NW_YIELD_CURVE_TYPE_UNKNOWN 0

// `YieldCurveType`: `zero_coupon`.
#define NW_YIELD_CURVE_TYPE_ZERO_COUPON 5

// Result of every export that can fail.
typedef enum nw_status {
  NW_OK = 0,
  NW_ERR_API,
  NW_ERR_TRANSPORT,
  NW_ERR_DECODE,
  NW_ERR_INVALID_ARGUMENT,
  NW_ERR_WEBHOOK,
  NW_ERR_INTERNAL,
} nw_status;

// Opaque, thread-safe client. Create with `nw_client_new`, free with `nw_client_free`.
typedef struct nw_client nw_client;

// Opaque error. Read it with the `nw_error_*` accessors and free it with `nw_error_free`.
typedef struct nw_error nw_error;

// One rate-limit window from response headers. `reset` is ISO-8601 UTC text, or NULL.
typedef struct nw_rate_limit {
  bool has_limit;
  int64_t limit;
  bool has_remaining;
  int64_t remaining;
  const char *reset;
} nw_rate_limit;

// Header metadata of a top-level response. Absent parts are NULL. `short_limit` is the
// short window (`X-RateLimit-Short-*`); `quota` is the plan quota (`X-RateLimit-*`).
typedef struct nw_response_meta {
  const char *request_id;
  const struct nw_rate_limit *quota;
  const struct nw_rate_limit *short_limit;
} nw_response_meta;

// Mirror of `AsReportedValue`. Owned by the enclosing response.
typedef struct nw_as_reported_value {
  const char *end;
  const char *start;
  const char *value;
} nw_as_reported_value;

// Mirror of `AsReportedConcept`. Owned by the enclosing response.
typedef struct nw_as_reported_concept {
  const char *concept;
  const char *label;
  const char *taxonomy;
  const char *unit;
  const struct nw_as_reported_value *const *values;
  size_t values_len;
} nw_as_reported_concept;

// Mirror of `AsReportedPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_as_reported_page {
  const struct nw_response_meta *meta;
  const struct nw_as_reported_concept *const *concepts;
  size_t concepts_len;
} nw_as_reported_page;

// Mirror of `VenueMatch`. Owned by the enclosing response.
typedef struct nw_venue_match {
  const int32_t *differences;
  const char *const *differences_raw;
  size_t differences_len;
  const char *note;
} nw_venue_match;

// Mirror of `PredictionVenue`. Owned by the enclosing response.
typedef struct nw_prediction_venue {
  int32_t excluded_reason;
  const char *excluded_reason_raw;
  const struct nw_venue_match *match_;
  const char *open_interest_usd;
  const char *rules_text;
  const char *rules_url;
  const char *settlement_source;
  const char *url;
  int32_t venue;
  const char *venue_raw;
  const char *volume_24h_usd;
  const char *weight;
  int32_t settlement;
  const char *settlement_raw;
} nw_prediction_venue;

// Mirror of `BinaryLabels`. Owned by the enclosing response.
typedef struct nw_binary_labels {
  const char *no;
  const char *yes;
} nw_binary_labels;

// Mirror of `Quote`. Owned by the enclosing response.
typedef struct nw_quote {
  const char *const *market_ids;
  size_t market_ids_len;
  const char *probability;
  int32_t venue;
  const char *venue_raw;
} nw_quote;

// Mirror of `PredictionResultMarket`. Owned by the enclosing response.
typedef struct nw_prediction_result_market {
  const char *market_id;
  const char *outcome;
  const char *value;
} nw_prediction_result_market;

// Mirror of `PredictionResultVenue`. Owned by the enclosing response.
typedef struct nw_prediction_result_venue {
  const char *amended_at;
  bool has_final_;
  bool final_;
  const struct nw_prediction_result_market *const *markets;
  size_t markets_len;
  const char *settled_at;
  bool has_unavailable;
  bool unavailable;
  int32_t venue;
  const char *venue_raw;
} nw_prediction_result_venue;

// Mirror of `PredictionResult`. Owned by the enclosing response.
typedef struct nw_prediction_result {
  const char *amended_at;
  bool final_;
  const char *settled_at;
  const struct nw_prediction_result_venue *const *venues;
  size_t venues_len;
} nw_prediction_result;

// Mirror of `BinaryEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_binary_event {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *closes_at;
  const char *event_id;
  const char *question;
  const char *release_id;
  const char *resolves_at;
  int32_t status;
  const char *status_raw;
  const char *topic_id;
  const char *unit;
  const struct nw_prediction_venue *const *venues;
  size_t venues_len;
  const struct nw_binary_labels *labels;
  const char *probability;
  const struct nw_quote *const *quotes;
  size_t quotes_len;
  const char *resolution;
  const char *resolves_with;
  const struct nw_prediction_result *result;
  bool has_stale;
  bool stale;
} nw_binary_event;

// Mirror of `CalendarCoverage`. Owned by the enclosing response.
typedef struct nw_calendar_coverage {
  int32_t scope;
  const char *scope_raw;
  const char *const *subdivision_codes;
  size_t subdivision_codes_len;
} nw_calendar_coverage;

// Mirror of `CalendarHoliday`. Owned by the enclosing response.
typedef struct nw_calendar_holiday {
  const char *date;
  const char *local_name;
  const char *name;
  bool has_observed;
  bool observed;
  int32_t scope;
  const char *scope_raw;
  const char *const *subdivision_codes;
  size_t subdivision_codes_len;
} nw_calendar_holiday;

// Mirror of `CalendarHolidays`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_calendar_holidays {
  const struct nw_response_meta *meta;
  const char *country;
  const struct nw_calendar_coverage *const *coverage;
  size_t coverage_len;
  const struct nw_calendar_holiday *const *holidays;
  size_t holidays_len;
} nw_calendar_holidays;

// Mirror of `CalendarHolidaysOnDate`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_calendar_holidays_on_date {
  const struct nw_response_meta *meta;
  const char *country;
  const struct nw_calendar_coverage *const *coverage;
  size_t coverage_len;
  const struct nw_calendar_holiday *const *holidays;
  size_t holidays_len;
} nw_calendar_holidays_on_date;

// Mirror of `CalendarNextHoliday`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_calendar_next_holiday {
  const struct nw_response_meta *meta;
  const char *country;
  const struct nw_calendar_coverage *const *coverage;
  size_t coverage_len;
  const struct nw_calendar_holiday *holiday;
} nw_calendar_next_holiday;

// Mirror of `CalendarSubscriptionCreated`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_calendar_subscription_created {
  const struct nw_response_meta *meta;
  const char *expires_at;
  const char *subscription_id;
  const char *url;
} nw_calendar_subscription_created;

// Optional inputs for `nw_calendar_subscriptions_create`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_calendar_subscriptions_create_params {
  size_t size;
  const char *const *countries;
  size_t countries_len;
  const char *end_date;
  const char *const *families;
  size_t families_len;
  bool has_include_holidays;
  bool include_holidays;
  bool has_include_releases;
  bool include_releases;
  const char *scope;
  const char *start_date;
} nw_calendar_subscriptions_create_params;

// Mirror of `NoContent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_no_content {
  const struct nw_response_meta *meta;
} nw_no_content;

// Optional inputs for `nw_calendars_holidays_on_date`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_calendars_holidays_on_date_params {
  size_t size;
  const char *scope;
} nw_calendars_holidays_on_date_params;

// Optional inputs for `nw_calendars_list_holidays`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_calendars_list_holidays_params {
  size_t size;
  const char *scope;
  const char *version_id;
} nw_calendars_list_holidays_params;

// Optional inputs for `nw_calendars_next_holiday`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_calendars_next_holiday_params {
  size_t size;
  const char *scope;
} nw_calendars_next_holiday_params;

// Mirror of `CatalogueLatest`. Owned by the enclosing response.
typedef struct nw_catalogue_latest {
  const char *attribution;
  const char *base_period;
  const char *country;
  const char *currency;
  const char *fetched_at;
  const char *first_reported_at;
  int32_t frequency;
  const char *frequency_raw;
  const char *id;
  const char *licence_url;
  const char *period;
  const char *released_on;
  const char *scale;
  int32_t seasonal_adjustment;
  const char *seasonal_adjustment_raw;
  bool has_stale;
  bool stale;
  int32_t status;
  const char *status_raw;
  const char *title;
  const char *unit;
  const char *unit_label;
  const char *value;
} nw_catalogue_latest;

// Mirror of `CatalogueBulkLatest`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_catalogue_bulk_latest {
  const struct nw_response_meta *meta;
  const char *const *missing;
  size_t missing_len;
  const struct nw_catalogue_latest *const *observations;
  size_t observations_len;
} nw_catalogue_bulk_latest;

// Mirror of `ResourceObservation`. Owned by the enclosing response.
typedef struct nw_resource_observation {
  const char *period;
  const char *released_on;
  int32_t status;
  const char *status_raw;
  const char *value;
  const char *first_reported_at;
} nw_resource_observation;

// Mirror of `CatalogueHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_catalogue_history {
  const struct nw_response_meta *meta;
  const char *attribution;
  const char *base_period;
  const char *country;
  const char *currency;
  const char *fetched_at;
  int32_t frequency;
  const char *frequency_raw;
  const char *id;
  const char *licence_url;
  const char *next_cursor;
  const struct nw_resource_observation *const *observations;
  size_t observations_len;
  const char *scale;
  int32_t seasonal_adjustment;
  const char *seasonal_adjustment_raw;
  bool has_stale;
  bool stale;
  const char *title;
  const char *unit;
  const char *unit_label;
} nw_catalogue_history;

// Mirror of `CatalogueEntry`. Owned by the enclosing response.
typedef struct nw_catalogue_entry {
  const char *country;
  const char *first_period;
  int32_t frequency;
  const char *frequency_raw;
  const char *id;
  const char *id_template;
  const char *last_period;
  const char *source;
  const char *title;
  const char *unit_label;
} nw_catalogue_entry;

// Mirror of `CatalogueSearchPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_catalogue_search_page {
  const struct nw_response_meta *meta;
  const char *next_cursor;
  const struct nw_catalogue_entry *const *results;
  size_t results_len;
} nw_catalogue_search_page;

// Mirror of `CatalogueSeries`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_catalogue_series {
  const struct nw_response_meta *meta;
  const char *attribution;
  const char *base_period;
  const char *country;
  const char *currency;
  const char *fetched_at;
  const char *first_period;
  int32_t frequency;
  const char *frequency_raw;
  const char *id;
  const char *last_period;
  const char *licence_url;
  const char *scale;
  int32_t seasonal_adjustment;
  const char *seasonal_adjustment_raw;
  bool has_stale;
  bool stale;
  const char *title;
  const char *unit;
  const char *unit_label;
} nw_catalogue_series;

// Mirror of `CentralBankBalanceSheet`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_central_bank_balance_sheet {
  const struct nw_response_meta *meta;
  const char *country;
  const char *currency;
  const char *period;
  const char *released_on;
  int32_t scale;
  const char *scale_raw;
  const char *value;
} nw_central_bank_balance_sheet;

// Mirror of `CentralBankReserves`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_central_bank_reserves {
  const struct nw_response_meta *meta;
  const char *country;
  const char *currency;
  const char *period;
  const char *released_on;
  int32_t scale;
  const char *scale_raw;
  const char *value;
} nw_central_bank_reserves;

// One extra request header. Authorization cannot be overridden. Its layout is frozen
// because it is passed by value in arrays.
typedef struct nw_header {
  const char *name;
  const char *value;
} nw_header;

// Client configuration. Zero-initialize, set `size` to `sizeof(nw_config)`, then set fields;
// a NULL `config` uses every default.
// `api_key` NULL reads `NARWHAL_API_KEY`; `base_url` NULL reads `NARWHAL_BASE_URL`, else
// `https://api.narwhalapi.com`. Unset `has_` flags keep the defaults: a 30 s timeout per
// attempt, 2 retries and a 30 s maximum `Retry-After` wait.
typedef struct nw_config {
  size_t size;
  const char *api_key;
  const char *base_url;
  bool has_timeout_ms;
  uint64_t timeout_ms;
  bool has_max_retries;
  uint32_t max_retries;
  bool has_max_retry_wait_ms;
  uint64_t max_retry_wait_ms;
  const struct nw_header *extra_headers;
  size_t extra_headers_len;
} nw_config;

// Mirror of `PhysicalSeries`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_physical_series {
  const struct nw_response_meta *meta;
  const char *area;
  const char *attribution;
  const char *basis;
  int32_t category;
  const char *category_raw;
  const char *commodity;
  const char *coverage_end;
  const char *coverage_start;
  const char *currency;
  const char *description;
  int32_t frequency;
  const char *frequency_raw;
  int32_t price_type;
  const char *price_type_raw;
  const char *product;
  const char *series_id;
  const char *unit;
  const char *unit_quantity;
  const char *country;
} nw_physical_series;

// Optional inputs for `nw_commodities_list_palmoil_physical_series`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_commodities_list_palmoil_physical_series_params {
  size_t size;
  const char *area;
  const char *country;
  const char *currency;
  const char *cursor;
  bool has_limit;
  int64_t limit;
  const char *price_type;
  const char *producer_arrangement;
  const char *producer_type;
  const char *product;
} nw_commodities_list_palmoil_physical_series_params;

// Mirror of `PalmoilScheduleSeries`. Owned by the enclosing response.
typedef struct nw_palmoil_schedule_series {
  const char *area;
  const char *country;
  const char *coverage_end;
  const char *coverage_start;
  const char *currency;
  int32_t frequency;
  const char *frequency_raw;
  const char *observation_type;
  const char *period_type;
  const char *price_type;
  int32_t producer_arrangement;
  const char *producer_arrangement_raw;
  const char *product;
  const char *series_id;
  const char *unit;
} nw_palmoil_schedule_series;

// Mirror of `PalmoilScalarSeries`. Owned by the enclosing response.
typedef struct nw_palmoil_scalar_series {
  const char *area;
  const char *country;
  const char *coverage_end;
  const char *coverage_start;
  const char *currency;
  int32_t frequency;
  const char *frequency_raw;
  const char *observation_type;
  int32_t period_type;
  const char *period_type_raw;
  int32_t price_type;
  const char *price_type_raw;
  int32_t product;
  const char *product_raw;
  const char *series_id;
  int32_t unit;
  const char *unit_raw;
} nw_palmoil_scalar_series;

// Union `PalmoilSeries`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_palmoil_series {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_palmoil_schedule_series *palmoil_schedule_series;
  const struct nw_palmoil_scalar_series *palmoil_scalar_series;
} nw_palmoil_series;

// Mirror of `PalmoilSeriesDiscovery`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_series_discovery {
  const struct nw_response_meta *meta;
  const char *next_cursor;
  const struct nw_palmoil_series *const *series;
  size_t series_len;
} nw_palmoil_series_discovery;

// Optional inputs for `nw_commodities_list_physical_series`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_commodities_list_physical_series_params {
  size_t size;
  const char *category;
  const char *commodity;
  const char *country;
  const char *currency;
  const char *cursor;
  bool has_limit;
  int64_t limit;
  const char *price_type;
} nw_commodities_list_physical_series_params;

// Mirror of `PhysicalSeriesPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_physical_series_page {
  const struct nw_response_meta *meta;
  const char *next_cursor;
  const struct nw_physical_series *const *series;
  size_t series_len;
} nw_physical_series_page;

// Optional inputs for `nw_commodities_palmoil_physical_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_commodities_palmoil_physical_observations_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *price_month;
  const char *start;
} nw_commodities_palmoil_physical_observations_params;

// Mirror of `CommodityOilPalmFfbAgePrice`. Owned by the enclosing response.
typedef struct nw_commodity_oil_palm_ffb_age_price {
  const char *price;
  int64_t tree_age_max_years;
  int64_t tree_age_min_years;
} nw_commodity_oil_palm_ffb_age_price;

// Mirror of `PalmoilScheduleObservation`. Owned by the enclosing response.
typedef struct nw_palmoil_schedule_observation {
  const char *period_end;
  const char *period_start;
  const struct nw_commodity_oil_palm_ffb_age_price *const *prices;
  size_t prices_len;
  int64_t revision;
} nw_palmoil_schedule_observation;

// Mirror of `PalmoilScheduleHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_schedule_history {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *next_cursor;
  const struct nw_palmoil_schedule_observation *const *observations;
  size_t observations_len;
  const char *series_id;
  const char *unit;
} nw_palmoil_schedule_history;

// Mirror of `PalmoilMarketObservation`. Owned by the enclosing response.
typedef struct nw_palmoil_market_observation {
  const char *period_end;
  const char *period_start;
  const char *price;
  const char *price_month;
  int64_t revision;
} nw_palmoil_market_observation;

// Mirror of `PalmoilMarketHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_market_history {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *next_cursor;
  const struct nw_palmoil_market_observation *const *observations;
  size_t observations_len;
  const char *series_id;
  const char *unit;
} nw_palmoil_market_history;

// Mirror of `PalmoilScalarObservation`. Owned by the enclosing response.
typedef struct nw_palmoil_scalar_observation {
  const char *period_end;
  const char *period_start;
  const char *price;
  int64_t revision;
} nw_palmoil_scalar_observation;

// Mirror of `PalmoilScalarHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_scalar_history {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *next_cursor;
  const struct nw_palmoil_scalar_observation *const *observations;
  size_t observations_len;
  const char *series_id;
  const char *unit;
} nw_palmoil_scalar_history;

// Mirror of `PalmoilFfbObservation`. Owned by the enclosing response.
typedef struct nw_palmoil_ffb_observation {
  const char *period_end;
  const char *period_start;
  const char *price;
  int64_t revision;
} nw_palmoil_ffb_observation;

// Mirror of `PalmoilFfbMarketHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_ffb_market_history {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *next_cursor;
  const struct nw_palmoil_ffb_observation *const *observations;
  size_t observations_len;
  const char *series_id;
  const char *unit;
} nw_palmoil_ffb_market_history;

// Union `PalmoilHistory`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_palmoil_history {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_palmoil_schedule_history *palmoil_schedule_history;
  const struct nw_palmoil_market_history *palmoil_market_history;
  const struct nw_palmoil_scalar_history *palmoil_scalar_history;
  const struct nw_palmoil_ffb_market_history *palmoil_ffb_market_history;
} nw_palmoil_history;

// Optional inputs for `nw_commodities_palmoil_physical_price`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_commodities_palmoil_physical_price_params {
  size_t size;
  const char *price_month;
} nw_commodities_palmoil_physical_price_params;

// Mirror of `PalmoilSchedulePrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_schedule_price {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *period_end;
  const char *period_start;
  const struct nw_commodity_oil_palm_ffb_age_price *const *prices;
  size_t prices_len;
  int64_t revision;
  const char *series_id;
  const char *unit;
} nw_palmoil_schedule_price;

// Mirror of `PalmoilMarketPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_market_price {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *period_end;
  const char *period_start;
  const char *price;
  const char *price_month;
  int64_t revision;
  const char *series_id;
  const char *unit;
} nw_palmoil_market_price;

// Mirror of `PalmoilScalarPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_scalar_price {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *period_end;
  const char *period_start;
  const char *price;
  int64_t revision;
  const char *series_id;
  const char *unit;
} nw_palmoil_scalar_price;

// Mirror of `PalmoilFfbMarketPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_palmoil_ffb_market_price {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *period_end;
  const char *period_start;
  const char *price;
  int64_t revision;
  const char *series_id;
  const char *unit;
} nw_palmoil_ffb_market_price;

// Union `PalmoilPrice`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_palmoil_price {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_palmoil_schedule_price *palmoil_schedule_price;
  const struct nw_palmoil_market_price *palmoil_market_price;
  const struct nw_palmoil_scalar_price *palmoil_scalar_price;
  const struct nw_palmoil_ffb_market_price *palmoil_ffb_market_price;
} nw_palmoil_price;

// Mirror of `PhysicalLatest`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_physical_latest {
  const struct nw_response_meta *meta;
  const char *attribution;
  const char *currency;
  const char *period_end;
  const char *period_start;
  const char *price;
  const char *published_on;
  int64_t revision;
  const char *series_id;
  const char *unit;
  const char *unit_quantity;
} nw_physical_latest;

// Optional inputs for `nw_commodities_physical_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_commodities_physical_observations_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
} nw_commodities_physical_observations_params;

// Mirror of `PhysicalObservation`. Owned by the enclosing response.
typedef struct nw_physical_observation {
  const char *period_end;
  const char *period_start;
  const char *price;
  const char *published_on;
  int64_t revision;
} nw_physical_observation;

// Mirror of `PhysicalObservations`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_physical_observations {
  const struct nw_response_meta *meta;
  const char *attribution;
  const char *currency;
  const char *next_cursor;
  const struct nw_physical_observation *const *observations;
  size_t observations_len;
  const char *series_id;
  const char *unit;
  const char *unit_quantity;
} nw_physical_observations;

// Optional inputs for `nw_commodities_price_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_commodities_price_history_params {
  size_t size;
  const char *cursor;
  const char *interval;
  bool has_limit;
  int64_t limit;
} nw_commodities_price_history_params;

// Mirror of `CommodityBar`. Owned by the enclosing response.
typedef struct nw_commodity_bar {
  const char *close;
  const char *high;
  const char *low;
  const char *open;
  const char *timestamp;
} nw_commodity_bar;

// Mirror of `CommodityBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_commodity_bars {
  const struct nw_response_meta *meta;
  const struct nw_commodity_bar *const *bars;
  size_t bars_len;
  const char *currency;
  const char *end;
  const char *next_cursor;
  const char *start;
  const char *symbol;
  int32_t unit;
  const char *unit_raw;
} nw_commodity_bars;

// Mirror of `CommoditySpot`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_commodity_spot {
  const struct nw_response_meta *meta;
  const char *currency;
  const char *price;
  bool stale;
  const char *symbol;
  const char *timestamp;
  int32_t unit;
  const char *unit_raw;
} nw_commodity_spot;

// Optional inputs for `nw_company_calendars_dividends`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_company_calendars_dividends_params {
  size_t size;
  const char *country;
  const char *end;
  const char *start;
} nw_company_calendars_dividends_params;

// Mirror of `DividendsCalendarEntry`. Owned by the enclosing response.
typedef struct nw_dividends_calendar_entry {
  const char *amount;
  const char *currency;
  const char *ex_date;
  const char *id;
  const char *name;
  const char *symbol;
} nw_dividends_calendar_entry;

// Mirror of `DividendsCalendar`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_dividends_calendar {
  const struct nw_response_meta *meta;
  const struct nw_dividends_calendar_entry *const *entries;
  size_t entries_len;
  const char *next_cursor;
} nw_dividends_calendar;

// Optional inputs for `nw_company_calendars_splits`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_company_calendars_splits_params {
  size_t size;
  const char *country;
  const char *end;
  const char *start;
} nw_company_calendars_splits_params;

// Mirror of `SplitsCalendarEntry`. Owned by the enclosing response.
typedef struct nw_splits_calendar_entry {
  const char *date;
  const char *id;
  const char *name;
  const char *ratio;
  const char *symbol;
} nw_splits_calendar_entry;

// Mirror of `SplitsCalendar`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_splits_calendar {
  const struct nw_response_meta *meta;
  const struct nw_splits_calendar_entry *const *entries;
  size_t entries_len;
  const char *next_cursor;
} nw_splits_calendar;

// Mirror of `ListedFiling`. Owned by the enclosing response.
typedef struct nw_listed_filing {
  const char *filed;
  const char *form;
  const char *id;
  const char *period_end;
  const char *title;
  int32_t type_;
  const char *type_raw;
} nw_listed_filing;

// Mirror of `CompanyFilingsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_company_filings_page {
  const struct nw_response_meta *meta;
  const char *filed_from;
  const struct nw_listed_filing *const *filings;
  size_t filings_len;
  const char *id;
  const char *next_cursor;
} nw_company_filings_page;

// Mirror of `CompanySearchHit`. Owned by the enclosing response.
typedef struct nw_company_search_hit {
  const char *code;
  const char *country;
  bool has_delisted;
  bool delisted;
  bool has_former_code;
  bool former_code;
  const char *former_name;
  const char *id;
  int32_t matched;
  const char *matched_raw;
  const char *mic;
  const char *name;
} nw_company_search_hit;

// Mirror of `RegistryCoverage`. Owned by the enclosing response.
typedef struct nw_registry_coverage {
  int64_t companies;
  int32_t registry;
  const char *registry_raw;
} nw_registry_coverage;

// Mirror of `CompanySearchPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_company_search_page {
  const struct nw_response_meta *meta;
  const struct nw_company_search_hit *const *companies;
  size_t companies_len;
  const struct nw_registry_coverage *const *coverage;
  size_t coverage_len;
} nw_company_search_page;

// Mirror of `CompanyIndustry`. Owned by the enclosing response.
typedef struct nw_company_industry {
  const char *industry;
  const char *isic;
  const char *sector;
  const char *source_code;
  const char *source_scheme;
} nw_company_industry;

// Mirror of `CompanyNameSpan`. Owned by the enclosing response.
typedef struct nw_company_name_span {
  const char *from;
  const char *name;
  const char *to;
} nw_company_name_span;

// Mirror of `CompanyRelation`. Owned by the enclosing response.
typedef struct nw_company_relation {
  const char *id;
  int32_t kind;
  const char *kind_raw;
} nw_company_relation;

// Mirror of `SecurityListing`. Owned by the enclosing response.
typedef struct nw_security_listing {
  const char *code;
  const char *from;
  const char *mic;
  const char *to;
} nw_security_listing;

// Mirror of `CompanySecurity`. Owned by the enclosing response.
typedef struct nw_company_security {
  const char *adr_ratio;
  const char *class_;
  const char *figi;
  const char *isin;
  const struct nw_security_listing *const *listings;
  size_t listings_len;
  bool has_primary;
  bool primary;
  const char *title;
  int32_t type_;
  const char *type_raw;
} nw_company_security;

// Mirror of `CompanyWithListings`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_company_with_listings {
  const struct nw_response_meta *meta;
  const char *country;
  bool has_delisted;
  bool delisted;
  const char *fiscal_year_end;
  const char *id;
  const struct nw_company_industry *industry;
  const char *jp_corporate_number;
  const char *lei;
  const char *const *listings;
  size_t listings_len;
  const char *name;
  const struct nw_company_name_span *const *names;
  size_t names_len;
  const struct nw_company_relation *const *related;
  size_t related_len;
  const struct nw_company_security *const *securities;
  size_t securities_len;
  int32_t standard;
  const char *standard_raw;
  const char *standard_variant;
  const char *status_date;
} nw_company_with_listings;

// Mirror of `SeriesSource`. Owned by the enclosing response.
typedef struct nw_series_source {
  const char *distributor;
  const char *publisher;
} nw_series_source;

// Mirror of `Cpi`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_cpi {
  const struct nw_response_meta *meta;
  const char *country;
  const char *period;
  const char *released_on;
  const struct nw_series_source *source;
  const char *value;
  const char *index_base;
} nw_cpi;

// Mirror of `CpiHistoryObservation`. Owned by the enclosing response.
typedef struct nw_cpi_history_observation {
  const char *period;
  const char *value;
  const char *first_reported_at;
} nw_cpi_history_observation;

// Mirror of `CpiHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_cpi_history {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *country;
  const char *index_base;
  const char *next_cursor;
  const struct nw_cpi_history_observation *const *observations;
  size_t observations_len;
  const struct nw_series_source *source;
  const char *start;
} nw_cpi_history;

// Optional inputs for `nw_crypto_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_crypto_bars_params {
  size_t size;
  const char *cursor;
  const char *end;
  const char *interval;
  const char *start;
} nw_crypto_bars_params;

// Mirror of `CryptoBar`. Owned by the enclosing response.
typedef struct nw_crypto_bar {
  const char *close;
  const char *high;
  const char *low;
  const char *open;
  const char *timestamp;
} nw_crypto_bar;

// Mirror of `CryptoBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_crypto_bars_model {
  const struct nw_response_meta *meta;
  const struct nw_crypto_bar *const *bars;
  size_t bars_len;
  const char *end;
  const char *next_cursor;
  const char *pair;
  const char *start;
} nw_crypto_bars_model;

// Optional inputs for `nw_crypto_list_pairs`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_crypto_list_pairs_params {
  size_t size;
  const char *quote;
} nw_crypto_list_pairs_params;

// Mirror of `CryptoPair`. Owned by the enclosing response.
typedef struct nw_crypto_pair {
  const char *base;
  const char *name;
  const char *pair;
  const char *quote;
} nw_crypto_pair;

// Mirror of `CryptoPairs`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_crypto_pairs {
  const struct nw_response_meta *meta;
  const struct nw_crypto_pair *const *pairs;
  size_t pairs_len;
} nw_crypto_pairs;

// Mirror of `CryptoPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_crypto_price_model {
  const struct nw_response_meta *meta;
  const char *base;
  const char *change_24h;
  const char *change_24h_percent;
  bool has_delayed;
  bool delayed;
  const char *pair;
  const char *price;
  const char *quote;
  bool stale;
  const char *timestamp;
} nw_crypto_price_model;

// Mirror of `CryptoPriceError`. Owned by the enclosing response.
typedef struct nw_crypto_price_error {
  int32_t code;
  const char *code_raw;
  const char *id;
} nw_crypto_price_error;

// Mirror of `CryptoPrices`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_crypto_prices_model {
  const struct nw_response_meta *meta;
  const struct nw_crypto_price_error *const *errors;
  size_t errors_len;
  const struct nw_crypto_price_model *const *prices;
  size_t prices_len;
} nw_crypto_prices_model;

// Mirror of `EconomicReleaseValue`. Owned by the enclosing response.
typedef struct nw_economic_release_value {
  const char *actual;
  bool has_annualized;
  bool annualized;
  const char *currency;
  const char *index_base;
  const char *measure;
  bool has_not_seasonally_adjusted;
  bool not_seasonally_adjusted;
  const char *period;
  const char *previous;
  const char *previous_revised_from;
  const char *scale;
  const char *transformation;
  int32_t unit;
  const char *unit_raw;
  int32_t value_status;
  const char *value_status_raw;
} nw_economic_release_value;

// Mirror of `EconomicReleaseSummary`. Owned by the enclosing response.
typedef struct nw_economic_release_summary {
  const char *at;
  const char *const *between;
  size_t between_len;
  const char *by;
  const char *country;
  const char *date;
  int32_t family;
  const char *family_raw;
  const char *name;
  const char *notice;
  const char *period;
  const char *release_id;
  int64_t revision;
  const char *series_id;
  int32_t stage;
  const char *stage_raw;
  int32_t status;
  const char *status_raw;
  const char *timezone;
  int32_t topic;
  const char *topic_raw;
  const struct nw_economic_release_value *const *values;
  size_t values_len;
} nw_economic_release_summary;

// Mirror of `EconomicCalendar`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_economic_calendar {
  const struct nw_response_meta *meta;
  const char *next_cursor;
  size_t releases_len;
  const struct nw_economic_release_summary *const *releases;
} nw_economic_calendar;

// Mirror of `EconomicReleaseDetail`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_economic_release_detail {
  const struct nw_response_meta *meta;
  const char *at;
  const char *const *between;
  size_t between_len;
  const char *by;
  const char *country;
  const char *date;
  int32_t family;
  const char *family_raw;
  const char *name;
  const char *notice;
  const char *observed_at;
  const char *period;
  const char *release_id;
  int64_t revision;
  const char *series_id;
  int32_t stage;
  const char *stage_raw;
  int32_t status;
  const char *status_raw;
  const char *timezone;
  int32_t topic;
  const char *topic_raw;
  const struct nw_economic_release_value *const *values;
  size_t values_len;
  const char *version;
} nw_economic_release_detail;

// Optional inputs for `nw_economics_central_bank_balance_sheet`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_central_bank_balance_sheet_params {
  size_t size;
  const char *period;
} nw_economics_central_bank_balance_sheet_params;

// Optional inputs for `nw_economics_central_bank_reserves`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_central_bank_reserves_params {
  size_t size;
  const char *period;
} nw_economics_central_bank_reserves_params;

// Optional inputs for `nw_economics_cpi`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_cpi_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_cpi_params;

// Optional inputs for `nw_economics_cpi_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_cpi_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_cpi_history_params;

// Optional inputs for `nw_economics_equity_risk_premium_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_equity_risk_premium_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
} nw_economics_equity_risk_premium_history_params;

// Mirror of `EquityRiskPremiumBasis`. Owned by the enclosing response.
typedef struct nw_equity_risk_premium_basis {
  int32_t country_default_spread;
  const char *country_default_spread_raw;
  int32_t country_risk_premium;
  const char *country_risk_premium_raw;
  int32_t mature_erp;
  const char *mature_erp_raw;
  int32_t total_erp;
  const char *total_erp_raw;
  int32_t volatility_ratio;
  const char *volatility_ratio_raw;
} nw_equity_risk_premium_basis;

// Mirror of `EquityRiskPremium`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_risk_premium {
  const struct nw_response_meta *meta;
  const char *as_of;
  const struct nw_equity_risk_premium_basis *basis;
  const char *country;
  const char *country_default_spread;
  const char *country_risk_premium;
  const char *mature_erp;
  const char *methodology_url;
  const char *total_erp;
  const char *unit;
  const char *volatility_ratio;
} nw_equity_risk_premium;

// Mirror of `EquityRiskPremiumHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_risk_premium_history_response {
  const struct nw_response_meta *meta;
  const char *country;
  const char *next_cursor;
  const struct nw_equity_risk_premium *const *observations;
  size_t observations_len;
} nw_equity_risk_premium_history_response;

// Optional inputs for `nw_economics_gdp`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_gdp_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_gdp_params;

// Mirror of `Gdp`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_gdp {
  const struct nw_response_meta *meta;
  const char *country;
  const char *currency;
  const char *period;
  const char *released_on;
  const struct nw_series_source *source;
  const char *value;
  bool has_annualized;
  bool annualized;
  bool has_not_seasonally_adjusted;
  bool not_seasonally_adjusted;
} nw_gdp;

// Optional inputs for `nw_economics_gdp_growth`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_gdp_growth_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_gdp_growth_params;

// Mirror of `GdpGrowth`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_gdp_growth {
  const struct nw_response_meta *meta;
  const char *country;
  const char *period;
  const char *quarter_over_quarter;
  const char *released_on;
  const struct nw_series_source *source;
  const char *year_over_year;
  const char *year_to_date;
  bool has_annualized;
  bool annualized;
  bool has_not_seasonally_adjusted;
  bool not_seasonally_adjusted;
  bool has_quarter_over_quarter_not_seasonally_adjusted;
  bool quarter_over_quarter_not_seasonally_adjusted;
  bool has_year_over_year_not_seasonally_adjusted;
  bool year_over_year_not_seasonally_adjusted;
} nw_gdp_growth;

// Optional inputs for `nw_economics_gdp_growth_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_gdp_growth_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_gdp_growth_history_params;

// Mirror of `GdpGrowthHistoryObservation`. Owned by the enclosing response.
typedef struct nw_gdp_growth_history_observation {
  const char *period;
  const char *quarter_over_quarter;
  const char *year_over_year;
  const char *year_to_date;
  const char *first_reported_at;
} nw_gdp_growth_history_observation;

// Mirror of `GdpGrowthHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_gdp_growth_history {
  const struct nw_response_meta *meta;
  bool has_annualized;
  bool annualized;
  const char *as_of;
  const char *country;
  const char *next_cursor;
  bool has_not_seasonally_adjusted;
  bool not_seasonally_adjusted;
  const struct nw_gdp_growth_history_observation *const *observations;
  size_t observations_len;
  bool has_quarter_over_quarter_not_seasonally_adjusted;
  bool quarter_over_quarter_not_seasonally_adjusted;
  const struct nw_series_source *source;
  const char *start;
  bool has_year_over_year_not_seasonally_adjusted;
  bool year_over_year_not_seasonally_adjusted;
} nw_gdp_growth_history;

// Optional inputs for `nw_economics_gdp_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_gdp_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_gdp_history_params;

// Mirror of `GdpHistoryObservation`. Owned by the enclosing response.
typedef struct nw_gdp_history_observation {
  const char *period;
  const char *value;
  const char *first_reported_at;
} nw_gdp_history_observation;

// Mirror of `GdpHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_gdp_history {
  const struct nw_response_meta *meta;
  bool has_annualized;
  bool annualized;
  const char *as_of;
  const char *country;
  const char *currency;
  const char *next_cursor;
  bool has_not_seasonally_adjusted;
  bool not_seasonally_adjusted;
  const struct nw_gdp_history_observation *const *observations;
  size_t observations_len;
  const struct nw_series_source *source;
  const char *start;
} nw_gdp_history;

// Optional inputs for `nw_economics_get_equity_risk_premium`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_get_equity_risk_premium_params {
  size_t size;
  const char *date;
} nw_economics_get_equity_risk_premium_params;

// Optional inputs for `nw_economics_get_release`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_get_release_params {
  size_t size;
  const char *version_id;
} nw_economics_get_release_params;

// Optional inputs for `nw_economics_inflation`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_inflation_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_inflation_params;

// Mirror of `Inflation`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_inflation {
  const struct nw_response_meta *meta;
  const char *country;
  const char *month_over_month;
  const char *period;
  const char *released_on;
  const struct nw_series_source *source;
  const char *year_over_year;
  const char *year_to_date;
} nw_inflation;

// Optional inputs for `nw_economics_inflation_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_inflation_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_inflation_history_params;

// Mirror of `InflationHistoryObservation`. Owned by the enclosing response.
typedef struct nw_inflation_history_observation {
  const char *month_over_month;
  const char *period;
  const char *year_over_year;
  const char *year_to_date;
  const char *first_reported_at;
} nw_inflation_history_observation;

// Mirror of `InflationHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_inflation_history {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *country;
  const char *next_cursor;
  const struct nw_inflation_history_observation *const *observations;
  size_t observations_len;
  const struct nw_series_source *source;
  const char *start;
} nw_inflation_history;

// Optional inputs for `nw_economics_labour_force_participation`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_labour_force_participation_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_labour_force_participation_params;

// Mirror of `LabourForceParticipation`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_labour_force_participation {
  const struct nw_response_meta *meta;
  const char *country;
  const char *period;
  const char *rate;
  const char *released_on;
  const struct nw_series_source *source;
} nw_labour_force_participation;

// Optional inputs for `nw_economics_labour_force_participation_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_labour_force_participation_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_labour_force_participation_history_params;

// Mirror of `RateHistoryObservation`. Owned by the enclosing response.
typedef struct nw_rate_history_observation {
  const char *period;
  const char *rate;
  const char *first_reported_at;
} nw_rate_history_observation;

// Mirror of `RateHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_rate_history {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *country;
  const char *next_cursor;
  const struct nw_rate_history_observation *const *observations;
  size_t observations_len;
  const struct nw_series_source *source;
  const char *start;
} nw_rate_history;

// Mirror of `EquityRiskPremiumList`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_risk_premium_list {
  const struct nw_response_meta *meta;
  const struct nw_equity_risk_premium *const *countries;
  size_t countries_len;
} nw_equity_risk_premium_list;

// Optional inputs for `nw_economics_list_releases`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_list_releases_params {
  size_t size;
  const char *const *countries;
  size_t countries_len;
  const char *cursor;
  const char *end_date;
  bool has_limit;
  int64_t limit;
  const char *start_date;
} nw_economics_list_releases_params;

// Optional inputs for `nw_economics_policy_rate`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_policy_rate_params {
  size_t size;
  const char *date;
} nw_economics_policy_rate_params;

// Mirror of `PointPolicyRate`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_point_policy_rate {
  const struct nw_response_meta *meta;
  const char *announced_on;
  const char *authority;
  const char *country;
  const char *currency;
  const char *effective_on;
  const char *instrument;
  const char *observed_on;
  int32_t qualifier;
  const char *qualifier_raw;
  const char *rate;
  const char *rate_type;
  const char *unit;
} nw_point_policy_rate;

// Mirror of `RangePolicyRate`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_range_policy_rate {
  const struct nw_response_meta *meta;
  const char *announced_on;
  const char *authority;
  const char *country;
  const char *currency;
  const char *effective_on;
  const char *instrument;
  const char *lower_bound;
  const char *observed_on;
  const char *rate_type;
  const char *unit;
  const char *upper_bound;
} nw_range_policy_rate;

// Union `PolicyRate`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_policy_rate {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_point_policy_rate *point_policy_rate;
  const struct nw_range_policy_rate *range_policy_rate;
} nw_policy_rate;

// Optional inputs for `nw_economics_policy_rate_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_policy_rate_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
} nw_economics_policy_rate_history_params;

// Union `PolicyRateObservation`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_policy_rate_observation {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_point_policy_rate *point_policy_rate;
  const struct nw_range_policy_rate *range_policy_rate;
} nw_policy_rate_observation;

// Mirror of `PolicyRateHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_policy_rate_history_response {
  const struct nw_response_meta *meta;
  const char *country;
  const char *next_cursor;
  const struct nw_policy_rate_observation *const *observations;
  size_t observations_len;
} nw_policy_rate_history_response;

// Optional inputs for `nw_economics_read_catalogue_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_read_catalogue_observations_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
} nw_economics_read_catalogue_observations_params;

// Optional inputs for `nw_economics_read_series_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_read_series_observations_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_read_series_observations_params;

// Mirror of `ResourceHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_resource_history {
  const struct nw_response_meta *meta;
  const char *country;
  const char *currency;
  const char *next_cursor;
  const struct nw_resource_observation *const *observations;
  size_t observations_len;
  const char *series_id;
  const struct nw_series_source *source;
  int32_t unit;
  const char *unit_raw;
  const char *as_of;
} nw_resource_history;

// Optional inputs for `nw_economics_search_catalogue`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_search_catalogue_params {
  size_t size;
  const char *country;
  const char *cursor;
  const char *frequency;
  bool has_limit;
  int64_t limit;
  const char *q;
  const char *source;
} nw_economics_search_catalogue_params;

// Optional inputs for `nw_economics_trade`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_trade_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_trade_params;

// Mirror of `TradeSummary`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_trade_summary {
  const struct nw_response_meta *meta;
  const char *balance;
  const char *country;
  const char *currency;
  const char *exports;
  const char *imports;
  const char *period;
  const char *released_on;
  const struct nw_series_source *source;
} nw_trade_summary;

// Optional inputs for `nw_economics_trade_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_trade_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_trade_history_params;

// Mirror of `TradeHistoryObservation`. Owned by the enclosing response.
typedef struct nw_trade_history_observation {
  const char *exports;
  const char *imports;
  const char *period;
  const char *first_reported_at;
} nw_trade_history_observation;

// Mirror of `TradeHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_trade_history {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *country;
  const char *currency;
  const char *next_cursor;
  const struct nw_trade_history_observation *const *observations;
  size_t observations_len;
  const struct nw_series_source *source;
  const char *start;
} nw_trade_history;

// Optional inputs for `nw_economics_unemployment`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_unemployment_params {
  size_t size;
  const char *period;
  const char *version_id;
} nw_economics_unemployment_params;

// Mirror of `Unemployment`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_unemployment {
  const struct nw_response_meta *meta;
  const char *country;
  const char *period;
  const char *rate;
  const char *released_on;
  const struct nw_series_source *source;
} nw_unemployment;

// Optional inputs for `nw_economics_unemployment_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_unemployment_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *start;
  const char *as_of;
  const char *version;
} nw_economics_unemployment_history_params;

// Optional inputs for `nw_economics_yield_curve`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_economics_yield_curve_params {
  size_t size;
  const char *date;
  const char *const *maturity;
  size_t maturity_len;
  const char *version_id;
} nw_economics_yield_curve_params;

// Mirror of `YieldPoint`. Owned by the enclosing response.
typedef struct nw_yield_point {
  const char *maturity;
  const char *yield_percent;
} nw_yield_point;

// Mirror of `YieldCurve`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_yield_curve {
  const struct nw_response_meta *meta;
  const char *country;
  int32_t curve_type;
  const char *curve_type_raw;
  const char *date;
  const struct nw_yield_point *const *yields;
  size_t yields_len;
} nw_yield_curve;

// Optional inputs for `nw_economics_yield_curve_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_economics_yield_curve_history_params {
  size_t size;
  const char *cursor;
  const char *end;
  bool has_limit;
  int64_t limit;
  const char *const *maturity;
  size_t maturity_len;
  const char *start;
  const char *as_of;
} nw_economics_yield_curve_history_params;

// Mirror of `YieldCurveHistoryObservation`. Owned by the enclosing response.
typedef struct nw_yield_curve_history_observation {
  const char *date;
  const struct nw_yield_point *const *yields;
  size_t yields_len;
} nw_yield_curve_history_observation;

// Mirror of `YieldCurveHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_yield_curve_history_response {
  const struct nw_response_meta *meta;
  const char *country;
  int32_t curve_type;
  const char *curve_type_raw;
  const char *next_cursor;
  const struct nw_yield_curve_history_observation *const *observations;
  size_t observations_len;
} nw_yield_curve_history_response;

// Optional inputs for `nw_equities_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_equities_bars_params {
  size_t size;
  const char *cursor;
  const char *end;
  const char *interval;
  const char *start;
} nw_equities_bars_params;

// Mirror of `EquityBar`. Owned by the enclosing response.
typedef struct nw_equity_bar {
  const char *close;
  const char *date;
  const char *high;
  const char *low;
  const char *open;
  const char *timestamp;
  const char *volume;
} nw_equity_bar;

// Mirror of `EquityBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_bars {
  const struct nw_response_meta *meta;
  bool adjusted;
  const struct nw_equity_bar *const *bars;
  size_t bars_len;
  const char *currency;
  const char *end;
  const char *id;
  const char *next_cursor;
  const char *start;
} nw_equity_bars;

// Optional inputs for `nw_equities_cost_of_capital`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_equities_cost_of_capital_params {
  size_t size;
  const char *benchmark;
  const char *beta;
  const char *erp;
  const char *frequency;
  const char *period;
  const char *risk_free;
  const char *tax_rate;
  const char *window;
} nw_equities_cost_of_capital_params;

// Mirror of `CostBenchmark`. Owned by the enclosing response.
typedef struct nw_cost_benchmark {
  bool has_default_;
  bool default_;
  const char *id;
} nw_cost_benchmark;

// Mirror of `CostBeta`. Owned by the enclosing response.
typedef struct nw_cost_beta {
  const char *adjusted;
  const struct nw_cost_benchmark *benchmark;
  const char *end;
  int32_t frequency;
  const char *frequency_raw;
  int64_t observations;
  const char *r_squared;
  const char *raw;
  const char *start;
  const char *std_error;
  int32_t used;
  const char *used_raw;
  int32_t window;
  const char *window_raw;
} nw_cost_beta;

// Mirror of `CostOfDebt`. Owned by the enclosing response.
typedef struct nw_cost_of_debt {
  const char *after_tax;
  const char *average_debt;
  const char *closing_debt;
  const char *interest_expense;
  const char *opening_debt;
  int32_t period;
  const char *period_raw;
  const char *period_end;
  const char *pre_tax;
  int32_t basis;
  const char *basis_raw;
  const char *default_spread;
  const char *interest_coverage;
  const char *table;
  const char *table_update;
  const char *table_url;
} nw_cost_of_debt;

// Mirror of `CostEquityRiskPremium`. Owned by the enclosing response.
typedef struct nw_cost_equity_risk_premium {
  const char *rate;
  const char *as_of;
  int32_t basis;
  const char *basis_raw;
  const char *country;
  int32_t source;
  const char *source_raw;
} nw_cost_equity_risk_premium;

// Mirror of `CostRiskFree`. Owned by the enclosing response.
typedef struct nw_cost_risk_free {
  const char *as_of;
  const char *country;
  const char *curve_type;
  const char *maturity;
  const char *rate;
  int32_t source;
  const char *source_raw;
  int32_t basis;
  const char *basis_raw;
} nw_cost_risk_free;

// Mirror of `CostTaxRate`. Owned by the enclosing response.
typedef struct nw_cost_tax_rate {
  const char *rate;
  int32_t source;
  const char *source_raw;
  int32_t basis;
  const char *basis_raw;
  const char *country;
  const char *table;
  const char *table_update;
  const char *table_url;
} nw_cost_tax_rate;

// Mirror of `CostWarning`. Owned by the enclosing response.
typedef struct nw_cost_warning {
  const char *code;
  const char *message;
} nw_cost_warning;

// Mirror of `CostWeights`. Owned by the enclosing response.
typedef struct nw_cost_weights {
  const char *debt;
  const char *equity;
  const char *market_cap;
  const char *market_cap_as_of;
  const char *total_debt;
  const char *total_debt_as_of;
} nw_cost_weights;

// Mirror of `EquityCostOfCapital`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_cost_of_capital {
  const struct nw_response_meta *meta;
  const struct nw_cost_beta *beta;
  const char *company_id;
  const struct nw_cost_of_debt *cost_of_debt;
  const char *cost_of_equity;
  const char *currency;
  const struct nw_cost_equity_risk_premium *equity_risk_premium;
  const char *id;
  const char *methodology_url;
  const struct nw_cost_risk_free *risk_free;
  const struct nw_cost_tax_rate *tax_rate;
  const char *wacc;
  const struct nw_cost_warning *const *warnings;
  size_t warnings_len;
  const struct nw_cost_weights *weights;
} nw_equity_cost_of_capital;

// Optional inputs for `nw_equities_dividends`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_equities_dividends_params {
  size_t size;
  const char *cursor;
} nw_equities_dividends_params;

// Mirror of `EquityDividend`. Owned by the enclosing response.
typedef struct nw_equity_dividend {
  const char *amount;
  const char *currency;
  const char *ex_date;
  const char *pay_date;
  const char *record_date;
} nw_equity_dividend;

// Mirror of `EquityDividends`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_dividends {
  const struct nw_response_meta *meta;
  const struct nw_equity_dividend *const *dividends;
  size_t dividends_len;
  const char *id;
  const char *next_cursor;
  const char *start;
} nw_equity_dividends;

// Optional inputs for `nw_equities_earnings`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_equities_earnings_params {
  size_t size;
  const char *cursor;
} nw_equities_earnings_params;

// Mirror of `EquityEarning`. Owned by the enclosing response.
typedef struct nw_equity_earning {
  const char *date;
  const char *eps_actual;
  const char *eps_estimate;
  const char *filing_id;
  const char *surprise;
  const char *surprise_percent;
  int32_t time;
  const char *time_raw;
} nw_equity_earning;

// Mirror of `EquityEarnings`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_earnings {
  const struct nw_response_meta *meta;
  const struct nw_equity_earning *const *earnings;
  size_t earnings_len;
  const char *id;
  const char *next_cursor;
  const char *start;
} nw_equity_earnings;

// Optional inputs for `nw_equities_movers`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_equities_movers_params {
  size_t size;
  const char *direction;
  const char *market;
} nw_equities_movers_params;

// Mirror of `EquityMover`. Owned by the enclosing response.
typedef struct nw_equity_mover {
  const char *change_percent;
  const char *id;
  const char *name;
  const char *price;
  const char *symbol;
  const char *volume;
} nw_equity_mover;

// Mirror of `EquityMovers`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_movers {
  const struct nw_response_meta *meta;
  int32_t direction;
  const char *direction_raw;
  const char *market;
  const struct nw_equity_mover *const *movers;
  size_t movers_len;
  const char *timestamp;
} nw_equity_movers;

// Mirror of `EquityPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_price {
  const struct nw_response_meta *meta;
  const char *change;
  const char *change_percent;
  const char *currency;
  bool has_delayed;
  bool delayed;
  const char *id;
  int32_t market_session;
  const char *market_session_raw;
  const char *previous_close;
  const char *price;
  bool stale;
  const char *symbol;
  const char *timestamp;
  const char *volume;
} nw_equity_price;

// Mirror of `EquityPriceError`. Owned by the enclosing response.
typedef struct nw_equity_price_error {
  int32_t code;
  const char *code_raw;
  const char *id;
} nw_equity_price_error;

// Mirror of `EquityPrices`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_prices {
  const struct nw_response_meta *meta;
  const struct nw_equity_price_error *const *errors;
  size_t errors_len;
  const struct nw_equity_price *const *prices;
  size_t prices_len;
} nw_equity_prices;

// Mirror of `EquityProfile`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_profile {
  const struct nw_response_meta *meta;
  const char *company_id;
  const char *country;
  const char *currency;
  const char *id;
  const char *industry;
  const char *logo_url;
  const char *mic;
  const char *name;
  const char *sector;
  const char *symbol;
  int32_t type_;
  const char *type_raw;
  const char *website;
} nw_equity_profile;

// Optional inputs for `nw_equities_search`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_equities_search_params {
  size_t size;
  const char *country;
  const char *cursor;
  const char *type_;
} nw_equities_search_params;

// Mirror of `EquitySearchResult`. Owned by the enclosing response.
typedef struct nw_equity_search_result {
  const char *company_id;
  const char *country;
  const char *currency;
  const char *id;
  const char *mic;
  const char *name;
  const char *symbol;
  int32_t type_;
  const char *type_raw;
} nw_equity_search_result;

// Mirror of `EquitySearchResults`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_search_results {
  const struct nw_response_meta *meta;
  const char *next_cursor;
  const struct nw_equity_search_result *const *results;
  size_t results_len;
} nw_equity_search_results;

// Mirror of `EquitySplit`. Owned by the enclosing response.
typedef struct nw_equity_split {
  const char *date;
  const char *ratio;
} nw_equity_split;

// Mirror of `EquitySplits`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_splits {
  const struct nw_response_meta *meta;
  const char *id;
  const struct nw_equity_split *const *splits;
  size_t splits_len;
  const char *start;
} nw_equity_splits;

// Mirror of `EquityValuation`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_valuation {
  const struct nw_response_meta *meta;
  const char *company_id;
  const char *currency;
  const char *dividend_per_share_ttm;
  const char *dividend_yield;
  const char *dividends_period_end;
  const char *eps_period_end;
  const char *eps_ttm;
  const char *equity_as_of;
  const char *id;
  const char *market_cap;
  const char *pb_ratio;
  const char *pe_ratio;
  const char *ps_ratio;
  const char *revenue_period_end;
  const char *revenue_ttm;
  const char *shares_as_of;
  const char *shares_outstanding;
  const char *timestamp;
  const char *total_equity;
} nw_equity_valuation;

// Mirror of `EquityValuationPoint`. Owned by the enclosing response.
typedef struct nw_equity_valuation_point {
  const char *date;
  const char *dividend_yield;
  const char *market_cap;
  const char *pb_ratio;
  const char *pe_ratio;
  const char *price;
  const char *price_date;
  const char *ps_ratio;
} nw_equity_valuation_point;

// Mirror of `EquityValuationHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_equity_valuation_history {
  const struct nw_response_meta *meta;
  const char *company_id;
  const char *currency;
  const char *id;
  const struct nw_equity_valuation_point *const *points;
  size_t points_len;
  const char *start;
} nw_equity_valuation_history;

// Mirror of `EventDetail`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_event_detail {
  const struct nw_response_meta *meta;
  const char *country;
  const char *cursor;
  const char *event_id;
  int32_t family;
  const char *family_raw;
  bool has_holiday_generation;
  int64_t holiday_generation;
  bool has_holiday_year;
  int64_t holiday_year;
  const char *recorded_at;
  const char *release_id;
  const char *resource_id;
  const char *resource_version;
  const char *type_;
} nw_event_detail;

// Optional inputs for `nw_events_list`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_events_list_params {
  size_t size;
  const char *const *countries;
  size_t countries_len;
  const char *cursor;
  bool has_limit;
  int64_t limit;
} nw_events_list_params;

// Mirror of `DeliveryEvent`. Owned by the enclosing response.
typedef struct nw_delivery_event {
  const char *country;
  const char *cursor;
  const char *event_id;
  int32_t family;
  const char *family_raw;
  bool has_holiday_generation;
  int64_t holiday_generation;
  bool has_holiday_year;
  int64_t holiday_year;
  const char *recorded_at;
  const char *release_id;
  const char *resource_id;
  const char *resource_version;
  const char *type_;
} nw_delivery_event;

// Mirror of `EventsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_events_page {
  const struct nw_response_meta *meta;
  const char *cursor;
  const struct nw_delivery_event *const *events;
  size_t events_len;
  const char *next_cursor;
} nw_events_page;

// Mirror of `ExtremeLevel`. Owned by the enclosing response.
typedef struct nw_extreme_level {
  const char *key;
  const char *level;
  const char *probability;
  const struct nw_quote *const *quotes;
  size_t quotes_len;
} nw_extreme_level;

// Mirror of `ExtremeWindow`. Owned by the enclosing response.
typedef struct nw_extreme_window {
  const char *end;
  const char *start;
} nw_extreme_window;

// Mirror of `ExtremeEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_extreme_event {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *closes_at;
  const char *event_id;
  const char *question;
  const char *release_id;
  const char *resolves_at;
  int32_t status;
  const char *status_raw;
  const char *topic_id;
  const char *unit;
  const struct nw_prediction_venue *const *venues;
  size_t venues_len;
  const struct nw_extreme_level *const *down;
  size_t down_len;
  int32_t observation;
  const char *observation_raw;
  const char *resolution;
  const char *resolves_with;
  const struct nw_prediction_result *result;
  bool has_stale;
  bool stale;
  const struct nw_extreme_level *const *up;
  size_t up_len;
  const struct nw_extreme_window *window;
} nw_extreme_event;

// Mirror of `Filing`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_filing {
  const struct nw_response_meta *meta;
  const char *amends;
  const char *company_id;
  const char *filed_at;
  const char *filer_id;
  const char *form;
  int32_t fp;
  const char *fp_raw;
  bool has_fy;
  int64_t fy;
  const char *id;
  const char *period_end;
  const char *title;
  int32_t type_;
  const char *type_raw;
  const char *url;
  bool has_withdrawn;
  bool withdrawn;
} nw_filing;

// Optional inputs for `nw_filings_company`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_filings_company_params {
  size_t size;
  const char *as_of;
  const char *lang;
} nw_filings_company_params;

// Optional inputs for `nw_filings_filing`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_filings_filing_params {
  size_t size;
  const char *lang;
} nw_filings_filing_params;

// Optional inputs for `nw_filings_financials`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_filings_financials_params {
  size_t size;
  const char *as_of;
  const char *basis;
  const char *fields;
  bool has_fiscal_year;
  int64_t fiscal_year;
  const char *period;
  const char *statement;
  const char *version;
} nw_filings_financials_params;

// Mirror of `FinancialRow`. Owned by the enclosing response.
typedef struct nw_financial_row {
  const char *currency;
  const char *const *derived;
  size_t derived_len;
  const char *end;
  bool has_fh;
  int64_t fh;
  bool has_fq;
  int64_t fq;
  int64_t fy;
} nw_financial_row;

// Mirror of `Financials`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_financials {
  const struct nw_response_meta *meta;
  const struct nw_financial_row *const *annual;
  size_t annual_len;
  const char *as_of;
  int32_t basis;
  const char *basis_raw;
  const char *currency;
  bool has_from_fy;
  int64_t from_fy;
  const char *id;
  const struct nw_financial_row *const *quarterly;
  size_t quarterly_len;
  const struct nw_financial_row *const *semiannual;
  size_t semiannual_len;
} nw_financials;

// Optional inputs for `nw_filings_list_company_filings`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_filings_list_company_filings_params {
  size_t size;
  const char *cursor;
  const char *filed_from;
  const char *filed_to;
  const char *form;
  bool has_include_amendments;
  bool include_amendments;
  const char *lang;
  bool has_limit;
  int64_t limit;
  const char *type_;
} nw_filings_list_company_filings_params;

// Optional inputs for `nw_filings_list_holders`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_filings_list_holders_params {
  size_t size;
  const char *lang;
  bool has_limit;
  int64_t limit;
  const char *security;
} nw_filings_list_holders_params;

// Mirror of `HolderSecurity`. Owned by the enclosing response.
typedef struct nw_holder_security {
  const char *class_;
  const char *isin;
} nw_holder_security;

// Mirror of `Holder`. Owned by the enclosing response.
typedef struct nw_holder {
  const char *event_date;
  const char *filed;
  const char *filing_id;
  const char *holder;
  const char *percent;
  const struct nw_holder_security *security;
  const char *shares;
} nw_holder;

// Mirror of `HoldersPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_holders_page {
  const struct nw_response_meta *meta;
  const char *filed_from;
  const struct nw_holder *const *holders;
  size_t holders_len;
  const char *id;
} nw_holders_page;

// Optional inputs for `nw_filings_list_latest_filings`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_filings_list_latest_filings_params {
  size_t size;
  const char *country;
  const char *lang;
  bool has_limit;
  int64_t limit;
  const char *type_;
} nw_filings_list_latest_filings_params;

// Mirror of `LatestFiling`. Owned by the enclosing response.
typedef struct nw_latest_filing {
  const char *company_id;
  const char *filed;
  const char *form;
  const char *id;
  const char *title;
  int32_t type_;
  const char *type_raw;
} nw_latest_filing;

// Mirror of `LatestFilingsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_latest_filings_page {
  const struct nw_response_meta *meta;
  const struct nw_latest_filing *const *filings;
  size_t filings_len;
} nw_latest_filings_page;

// Optional inputs for `nw_filings_search_companies`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_filings_search_companies_params {
  size_t size;
  const char *as_of;
  const char *country;
  const char *lang;
  bool has_limit;
  int64_t limit;
  const char *q;
  const char *registry;
} nw_filings_search_companies_params;

// Mirror of `FundDistribution`. Owned by the enclosing response.
typedef struct nw_fund_distribution {
  const char *amount;
  const char *currency;
  const char *ex_date;
  int32_t kind;
  const char *kind_raw;
  const char *pay_date;
  const char *record_date;
} nw_fund_distribution;

// Mirror of `FundDistributions`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fund_distributions {
  const struct nw_response_meta *meta;
  const struct nw_fund_distribution *const *distributions;
  size_t distributions_len;
  const char *id;
  const char *next_cursor;
  const char *start;
} nw_fund_distributions;

// Mirror of `FundEtfPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fund_etf_price {
  const struct nw_response_meta *meta;
  const char *change;
  const char *change_percent;
  const char *currency;
  bool has_delayed;
  bool delayed;
  const char *id;
  int32_t market_session;
  const char *market_session_raw;
  const char *previous_close;
  const char *price;
  bool stale;
  const char *symbol;
  const char *timestamp;
  const char *type_;
  const char *volume;
} nw_fund_etf_price;

// Mirror of `FundNavPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fund_nav_price {
  const struct nw_response_meta *meta;
  const char *change;
  const char *change_percent;
  const char *currency;
  const char *date;
  const char *id;
  const char *nav;
  const char *previous_nav;
  bool stale;
  const char *symbol;
  const char *type_;
} nw_fund_nav_price;

// Union `FundPrice`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_fund_price {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_fund_etf_price *fund_etf_price;
  const struct nw_fund_nav_price *fund_nav_price;
} nw_fund_price;

// Mirror of `FundPriceError`. Owned by the enclosing response.
typedef struct nw_fund_price_error {
  int32_t code;
  const char *code_raw;
  const char *id;
} nw_fund_price_error;

// Union `FundPriceItem`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
typedef struct nw_fund_price_item {
  const char *kind;
  const char *unknown_raw;
  const struct nw_response_meta *meta;
  const struct nw_fund_etf_price *fund_etf_price;
  const struct nw_fund_nav_price *fund_nav_price;
} nw_fund_price_item;

// Mirror of `FundPrices`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fund_prices {
  const struct nw_response_meta *meta;
  const struct nw_fund_price_error *const *errors;
  size_t errors_len;
  size_t prices_len;
  const struct nw_fund_price_item *const *prices;
} nw_fund_prices;

// Mirror of `FundProfile`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fund_profile {
  const struct nw_response_meta *meta;
  const char *country;
  const char *currency;
  const char *id;
  const char *mic;
  const char *name;
  const char *symbol;
  int32_t type_;
  const char *type_raw;
} nw_fund_profile;

// Mirror of `FundSearchResult`. Owned by the enclosing response.
typedef struct nw_fund_search_result {
  const char *country;
  const char *currency;
  const char *id;
  const char *mic;
  const char *name;
  const char *symbol;
  int32_t type_;
  const char *type_raw;
} nw_fund_search_result;

// Mirror of `FundSearchResults`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fund_search_results {
  const struct nw_response_meta *meta;
  const char *next_cursor;
  const struct nw_fund_search_result *const *results;
  size_t results_len;
} nw_fund_search_results;

// Optional inputs for `nw_funds_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_funds_bars_params {
  size_t size;
  const char *cursor;
  const char *end;
  const char *interval;
  const char *start;
} nw_funds_bars_params;

// Optional inputs for `nw_funds_distributions`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_funds_distributions_params {
  size_t size;
  const char *cursor;
} nw_funds_distributions_params;

// Optional inputs for `nw_funds_search`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_funds_search_params {
  size_t size;
  const char *country;
  const char *cursor;
  const char *type_;
} nw_funds_search_params;

// Mirror of `FxBar`. Owned by the enclosing response.
typedef struct nw_fx_bar {
  const char *close;
  const char *high;
  const char *low;
  const char *open;
  const char *timestamp;
} nw_fx_bar;

// Mirror of `FxBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fx_bars {
  const struct nw_response_meta *meta;
  const struct nw_fx_bar *const *bars;
  size_t bars_len;
  const char *base;
  const char *currency;
  const char *end;
  const char *next_cursor;
  const char *start;
} nw_fx_bars;

// Mirror of `FxConversion`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fx_conversion {
  const struct nw_response_meta *meta;
  const char *converted;
  const char *from;
  int32_t market_session;
  const char *market_session_raw;
  const char *rate;
  bool stale;
  const char *timestamp;
  const char *to;
} nw_fx_conversion;

// Mirror of `FxHistoricalRate`. Owned by the enclosing response.
typedef struct nw_fx_historical_rate {
  const char *currency;
  const char *rate;
} nw_fx_historical_rate;

// Mirror of `FxHistoricalObservation`. Owned by the enclosing response.
typedef struct nw_fx_historical_observation {
  const char *date;
  const struct nw_fx_historical_rate *const *rates;
  size_t rates_len;
} nw_fx_historical_observation;

// Mirror of `FxHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fx_history {
  const struct nw_response_meta *meta;
  const char *base;
  const char *end_date;
  const char *next_cursor;
  const struct nw_fx_historical_observation *const *observations;
  size_t observations_len;
  const char *start_date;
} nw_fx_history;

// Optional inputs for `nw_fx_rate_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_fx_rate_bars_params {
  size_t size;
  const char *cursor;
  const char *interval;
  bool has_limit;
  int64_t limit;
} nw_fx_rate_bars_params;

// Optional inputs for `nw_fx_rate_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_fx_rate_history_params {
  size_t size;
  const char *cursor;
  bool has_limit;
  int64_t limit;
} nw_fx_rate_history_params;

// Optional inputs for `nw_fx_rates`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_fx_rates_params {
  size_t size;
  const char *const *currencies;
  size_t currencies_len;
} nw_fx_rates_params;

// Mirror of `FxRate`. Owned by the enclosing response.
typedef struct nw_fx_rate {
  const char *currency;
  const char *rate;
  bool stale;
  const char *timestamp;
} nw_fx_rate;

// Mirror of `FxRates`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_fx_rates_model {
  const struct nw_response_meta *meta;
  const char *base;
  int32_t market_session;
  const char *market_session_raw;
  const struct nw_fx_rate *const *rates;
  size_t rates_len;
} nw_fx_rates_model;

// Mirror of `PolicyPathMeeting`. Owned by the enclosing response.
typedef struct nw_policy_path_meeting {
  const char *cumulative_change_bps;
  const char *event_id;
  const char *expected_change_bps;
  const char *p_cut;
  const char *p_hike;
  const char *p_hold;
  const char *resolves_at;
} nw_policy_path_meeting;

// Mirror of `PolicyPath`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_policy_path {
  const struct nw_response_meta *meta;
  const char *as_of;
  const struct nw_policy_path_meeting *const *meetings;
  size_t meetings_len;
} nw_policy_path;

// Mirror of `PolicyPathOdds`. Owned by the enclosing response.
typedef struct nw_policy_path_odds {
  const char *cumulative_change_bps;
  const char *event_id;
  const char *expected_change_bps;
  const char *p_cut;
  const char *p_hike;
  const char *p_hold;
} nw_policy_path_odds;

// Mirror of `PolicyPathDay`. Owned by the enclosing response.
typedef struct nw_policy_path_day {
  const char *date;
  size_t meetings_len;
  const struct nw_policy_path_odds *const *meetings;
} nw_policy_path_day;

// Mirror of `PolicyPathMeetingDate`. Owned by the enclosing response.
typedef struct nw_policy_path_meeting_date {
  const char *event_id;
  const char *resolves_at;
} nw_policy_path_meeting_date;

// Mirror of `PolicyPathHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_policy_path_history {
  const struct nw_response_meta *meta;
  const struct nw_policy_path_day *const *days;
  size_t days_len;
  const struct nw_policy_path_meeting_date *const *meeting_dates;
  size_t meeting_dates_len;
} nw_policy_path_history;

// Mirror of `PredictionResultSummary`. Owned by the enclosing response.
typedef struct nw_prediction_result_summary {
  const char *amended_at;
  bool final_;
  const char *settled_at;
} nw_prediction_result_summary;

// Mirror of `PredictionEventSummary`. Owned by the enclosing response.
typedef struct nw_prediction_event_summary {
  const char *closes_at;
  const char *event_id;
  const char *question;
  const char *release_id;
  const char *resolves_at;
  int32_t shape;
  const char *shape_raw;
  int32_t status;
  const char *status_raw;
  const char *topic_id;
  const struct nw_prediction_result_summary *result;
} nw_prediction_event_summary;

// Mirror of `PredictionEventsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_prediction_events_page {
  const struct nw_response_meta *meta;
  const struct nw_prediction_event_summary *const *events;
  size_t events_len;
  const char *next_cursor;
} nw_prediction_events_page;

// Mirror of `PredictionBar`. Owned by the enclosing response.
typedef struct nw_prediction_bar {
  const char *close;
  const char *high;
  const char *low;
  const char *open;
  const char *timestamp;
} nw_prediction_bar;

// Mirror of `PredictionSeries`. Owned by the enclosing response.
typedef struct nw_prediction_series {
  const char *active_from;
  const char *active_to;
  const struct nw_prediction_bar *const *bars;
  size_t bars_len;
  const char *key;
} nw_prediction_series;

// Mirror of `PredictionHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_prediction_history {
  const struct nw_response_meta *meta;
  int32_t backfill_venue;
  const char *backfill_venue_raw;
  const char *end;
  const struct nw_prediction_series *const *series;
  size_t series_len;
  const char *next_cursor;
} nw_prediction_history;

// Mirror of `TopicCalendar`. Owned by the enclosing response.
typedef struct nw_topic_calendar {
  const char *country;
  const char *family;
} nw_topic_calendar;

// Mirror of `PredictionTopicEvent`. Owned by the enclosing response.
typedef struct nw_prediction_topic_event {
  const char *closes_at;
  const char *event_id;
  const char *question;
  const char *release_id;
  const char *resolves_at;
  const struct nw_prediction_result_summary *result;
  int32_t status;
  const char *status_raw;
} nw_prediction_topic_event;

// Mirror of `PredictionTopicDetail`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_prediction_topic_detail {
  const struct nw_response_meta *meta;
  const struct nw_topic_calendar *calendar;
  size_t events_len;
  int32_t group;
  const char *group_raw;
  int32_t resolution_kind;
  const char *resolution_kind_raw;
  int32_t shape;
  const char *shape_raw;
  const char *title;
  const char *topic_id;
  const char *unit;
  const int32_t *venues;
  const char *const *venues_raw;
  size_t venues_len;
  const struct nw_prediction_topic_event *const *events;
} nw_prediction_topic_detail;

// Mirror of `PredictionTopic`. Owned by the enclosing response.
typedef struct nw_prediction_topic {
  const struct nw_topic_calendar *calendar;
  int32_t group;
  const char *group_raw;
  const char *next_event_id;
  int32_t resolution_kind;
  const char *resolution_kind_raw;
  int32_t shape;
  const char *shape_raw;
  const char *title;
  const char *topic_id;
  const char *unit;
  const int32_t *venues;
  const char *const *venues_raw;
  size_t venues_len;
} nw_prediction_topic;

// Mirror of `PredictionTopics`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_prediction_topics {
  const struct nw_response_meta *meta;
  const struct nw_prediction_topic *const *topics;
  size_t topics_len;
} nw_prediction_topics;

// Optional inputs for `nw_predictions_binary`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_predictions_binary_params {
  size_t size;
  const char *at;
} nw_predictions_binary_params;

// Optional inputs for `nw_predictions_binary_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_predictions_binary_history_params {
  size_t size;
  const char *interval;
  const char *cursor;
} nw_predictions_binary_history_params;

// Optional inputs for `nw_predictions_extreme`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_predictions_extreme_params {
  size_t size;
  const char *at;
} nw_predictions_extreme_params;

// Optional inputs for `nw_predictions_extreme_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_predictions_extreme_history_params {
  size_t size;
  const char *interval;
  const char *cursor;
} nw_predictions_extreme_history_params;

// Optional inputs for `nw_predictions_list_events`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_predictions_list_events_params {
  size_t size;
  const char *cursor;
  const char *from;
  bool has_limit;
  int64_t limit;
  const char *release_id;
  const char *shape;
  const char *status;
  const char *to;
  const char *topic;
} nw_predictions_list_events_params;

// Optional inputs for `nw_predictions_list_topics`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_predictions_list_topics_params {
  size_t size;
  const char *group;
} nw_predictions_list_topics_params;

// Optional inputs for `nw_predictions_scalar`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_predictions_scalar_params {
  size_t size;
  const char *at;
} nw_predictions_scalar_params;

// Mirror of `ScalarOutcome`. Owned by the enclosing response.
typedef struct nw_scalar_outcome {
  const char *key;
  const char *label;
  const char *lower;
  const char *probability;
  const struct nw_quote *const *quotes;
  size_t quotes_len;
  const char *upper;
} nw_scalar_outcome;

// Mirror of `ScalarEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_scalar_event {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *closes_at;
  const char *event_id;
  const char *question;
  const char *release_id;
  const char *resolves_at;
  int32_t status;
  const char *status_raw;
  const char *topic_id;
  const char *unit;
  const struct nw_prediction_venue *const *venues;
  size_t venues_len;
  const int32_t *above;
  const char *const *above_raw;
  size_t above_len;
  const char *median;
  const struct nw_scalar_outcome *const *outcomes;
  size_t outcomes_len;
  const char *p10;
  const char *p90;
  const char *resolution;
  const char *resolves_with;
  const struct nw_prediction_result *result;
  bool has_stale;
  bool stale;
} nw_scalar_event;

// Optional inputs for `nw_predictions_scalar_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_predictions_scalar_history_params {
  size_t size;
  const char *interval;
  const char *cursor;
} nw_predictions_scalar_history_params;

// Optional inputs for `nw_predictions_timing`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_predictions_timing_params {
  size_t size;
  const char *at;
} nw_predictions_timing_params;

// Mirror of `TimingStep`. Owned by the enclosing response.
typedef struct nw_timing_step {
  const char *by;
  const char *key;
  const char *label;
  const char *probability;
  const struct nw_quote *const *quotes;
  size_t quotes_len;
  int32_t kind;
  const char *kind_raw;
} nw_timing_step;

// Mirror of `TimingEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_timing_event {
  const struct nw_response_meta *meta;
  const char *as_of;
  const char *closes_at;
  const char *event_id;
  const char *question;
  const char *release_id;
  const char *resolves_at;
  int32_t status;
  const char *status_raw;
  const char *topic_id;
  const char *unit;
  const struct nw_prediction_venue *const *venues;
  size_t venues_len;
  const char *not_within_window;
  const char *resolution;
  const char *resolves_with;
  const struct nw_prediction_result *result;
  bool has_stale;
  bool stale;
  const struct nw_timing_step *const *steps;
  size_t steps_len;
} nw_timing_event;

// Optional inputs for `nw_predictions_timing_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_predictions_timing_history_params {
  size_t size;
  const char *interval;
  const char *cursor;
} nw_predictions_timing_history_params;

// Mirror of `WebhookDelivery`. Owned by the enclosing response.
typedef struct nw_webhook_delivery {
  int64_t attempts;
  int64_t config_version;
  const char *created_at;
  const char *delivery_id;
  bool has_event_cursor;
  int64_t event_cursor;
  bool has_is_test;
  bool is_test;
  const char *last_error;
  bool has_last_status;
  int64_t last_status;
  const char *next_attempt_at;
  int64_t replays;
  const char *retry_until;
  int32_t state;
  const char *state_raw;
  const char *updated_at;
  bool has_replays;
} nw_webhook_delivery;

// Mirror of `WebhookDeliveriesResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_webhook_deliveries_response {
  const struct nw_response_meta *meta;
  const struct nw_webhook_delivery *const *deliveries;
  size_t deliveries_len;
  const char *next_cursor;
} nw_webhook_deliveries_response;

// Mirror of `WebhookDestinationCreated`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_webhook_destination_created {
  const struct nw_response_meta *meta;
  int64_t config_version;
  const char *const *countries;
  size_t countries_len;
  const char *created_at;
  const char *destination_id;
  bool enabled;
  const int32_t *families;
  const char *const *families_raw;
  size_t families_len;
  const char *secret;
  int64_t secret_version;
  const char *updated_at;
  const char *url;
  int32_t disabled_reason;
  const char *disabled_reason_raw;
  const char *const *companies;
  size_t companies_len;
} nw_webhook_destination_created;

// Mirror of `WebhookDestinationResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_webhook_destination_response {
  const struct nw_response_meta *meta;
  int64_t config_version;
  const char *const *countries;
  size_t countries_len;
  const char *created_at;
  const char *destination_id;
  bool enabled;
  const int32_t *families;
  const char *const *families_raw;
  size_t families_len;
  int64_t secret_version;
  const char *updated_at;
  const char *url;
  int32_t disabled_reason;
  const char *disabled_reason_raw;
  const char *const *companies;
  size_t companies_len;
} nw_webhook_destination_response;

// Mirror of `WebhookDestinationList`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_webhook_destination_list {
  const struct nw_response_meta *meta;
  const struct nw_webhook_destination_response *const *destinations;
  size_t destinations_len;
  int64_t destination_limit;
} nw_webhook_destination_list;

// Mirror of `PredictionWebhookSettlement`. Owned by the enclosing response.
typedef struct nw_prediction_webhook_settlement {
  int32_t status;
  const char *status_raw;
  int32_t venue;
  const char *venue_raw;
} nw_prediction_webhook_settlement;

// Mirror of `PredictionWebhookEvent`. Owned by the enclosing response.
typedef struct nw_prediction_webhook_event {
  const char *closes_at;
  const char *release_id;
  const char *resolves_at;
  const struct nw_prediction_webhook_settlement *const *settlements;
  size_t settlements_len;
  int32_t shape;
  const char *shape_raw;
  int32_t status;
  const char *status_raw;
  const char *topic_id;
  const char *prediction_event_id;
} nw_prediction_webhook_event;

// Mirror of `FilingWebhookEvent`. Owned by the enclosing response.
typedef struct nw_filing_webhook_event {
  const char *company_id;
  const char *filed_at;
  const char *filing_id;
  const char *form;
  int32_t fp;
  const char *fp_raw;
  bool has_fy;
  int64_t fy;
  const char *period_end;
  const char *subject_id;
  int32_t type_;
  const char *type_raw;
} nw_filing_webhook_event;

// Mirror of `WebhookEvent`. Owned by the enclosing response.
typedef struct nw_webhook_event {
  const char *country;
  const char *cursor;
  const char *event_id;
  int32_t family;
  const char *family_raw;
  bool has_holiday_generation;
  int64_t holiday_generation;
  bool has_holiday_year;
  int64_t holiday_year;
  const struct nw_prediction_webhook_event *prediction;
  const char *recorded_at;
  const char *release_id;
  const char *resource_id;
  const char *resource_version;
  const char *schema_version;
  int32_t type_;
  const char *type_raw;
  const struct nw_filing_webhook_event *filing;
} nw_webhook_event;

// Mirror of `WebhookConfigurationHistory`. Owned by the enclosing response.
typedef struct nw_webhook_configuration_history {
  const char *const *countries;
  size_t countries_len;
  bool enabled;
  const int32_t *families;
  const char *const *families_raw;
  size_t families_len;
  const char *recorded_at;
  const char *url;
  int64_t config_version;
} nw_webhook_configuration_history;

// Mirror of `WebhookHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_webhook_history_response {
  const struct nw_response_meta *meta;
  const struct nw_webhook_configuration_history *const *history;
  size_t history_len;
} nw_webhook_history_response;

// Mirror of `WebhookTestResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
typedef struct nw_webhook_test_response {
  const struct nw_response_meta *meta;
  const char *delivery_id;
  bool has_retry_after_seconds;
  int64_t retry_after_seconds;
  int32_t status;
  const char *status_raw;
} nw_webhook_test_response;

// Optional inputs for `nw_webhooks_create_destination`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_webhooks_create_destination_params {
  size_t size;
  const char *const *countries;
  size_t countries_len;
  bool has_enabled;
  bool enabled;
  const char *const *families;
  size_t families_len;
  const char *url;
  const char *const *companies;
  size_t companies_len;
} nw_webhooks_create_destination_params;

// Optional inputs for `nw_webhooks_delivery_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
typedef struct nw_webhooks_delivery_history_params {
  size_t size;
  const char *cursor;
  bool has_limit;
  int64_t limit;
} nw_webhooks_delivery_history_params;

// Optional inputs for `nw_webhooks_update_destination`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
typedef struct nw_webhooks_update_destination_params {
  size_t size;
  const char *const *countries;
  size_t countries_len;
  bool has_enabled;
  bool enabled;
  const char *const *families;
  size_t families_len;
  const char *url;
  const char *const *companies;
  size_t companies_len;
} nw_webhooks_update_destination_params;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Free a `nw_as_reported_page` returned through `**out`. NULL is a no-op.
void nw_as_reported_page_free(struct nw_as_reported_page *value);

// Free a `nw_binary_event` returned through `**out`. NULL is a no-op.
void nw_binary_event_free(struct nw_binary_event *value);

// Free a `nw_calendar_holidays` returned through `**out`. NULL is a no-op.
void nw_calendar_holidays_free(struct nw_calendar_holidays *value);

// Free a `nw_calendar_holidays_on_date` returned through `**out`. NULL is a no-op.
void nw_calendar_holidays_on_date_free(struct nw_calendar_holidays_on_date *value);

// Free a `nw_calendar_next_holiday` returned through `**out`. NULL is a no-op.
void nw_calendar_next_holiday_free(struct nw_calendar_next_holiday *value);

// Free a `nw_calendar_subscription_created` returned through `**out`. NULL is a no-op.
void nw_calendar_subscription_created_free(struct nw_calendar_subscription_created *value);

// Create Calendar Subscription
//
// Stability: stable. `POST /v1/calendars/subscriptions`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_calendar_subscription_created_free`.
enum nw_status nw_calendar_subscriptions_create(struct nw_client *client,
                                                const struct nw_calendar_subscriptions_create_params *params,
                                                struct nw_calendar_subscription_created **out,
                                                struct nw_error **err);

// Revoke Calendar Subscription
//
// Stability: stable. `DELETE /v1/calendars/subscriptions/{subscription_id}`.
// On success `*out` is owned by the caller; free it with `nw_no_content_free`.
enum nw_status nw_calendar_subscriptions_revoke(struct nw_client *client,
                                                const char *subscription_id,
                                                struct nw_no_content **out,
                                                struct nw_error **err);

// Get Calendar Holidays On Date
//
// Stability: stable. `GET /v1/calendars/holidays/{country}/on/{date}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_calendar_holidays_on_date_free`.
enum nw_status nw_calendars_holidays_on_date(struct nw_client *client,
                                             const char *country,
                                             const char *date,
                                             const struct nw_calendars_holidays_on_date_params *params,
                                             struct nw_calendar_holidays_on_date **out,
                                             struct nw_error **err);

// List Calendar Holidays
//
// Stability: stable. `GET /v1/calendars/holidays/{country}/{year}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_calendar_holidays_free`.
enum nw_status nw_calendars_list_holidays(struct nw_client *client,
                                          const char *country,
                                          int64_t year,
                                          const struct nw_calendars_list_holidays_params *params,
                                          struct nw_calendar_holidays **out,
                                          struct nw_error **err);

// Get Calendar Next Holiday
//
// Stability: stable. `GET /v1/calendars/holidays/{country}/next`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_calendar_next_holiday_free`.
enum nw_status nw_calendars_next_holiday(struct nw_client *client,
                                         const char *country,
                                         const char *from_date,
                                         const struct nw_calendars_next_holiday_params *params,
                                         struct nw_calendar_next_holiday **out,
                                         struct nw_error **err);

// Free a `nw_catalogue_bulk_latest` returned through `**out`. NULL is a no-op.
void nw_catalogue_bulk_latest_free(struct nw_catalogue_bulk_latest *value);

// Free a `nw_catalogue_history` returned through `**out`. NULL is a no-op.
void nw_catalogue_history_free(struct nw_catalogue_history *value);

// Free a `nw_catalogue_search_page` returned through `**out`. NULL is a no-op.
void nw_catalogue_search_page_free(struct nw_catalogue_search_page *value);

// Free a `nw_catalogue_series` returned through `**out`. NULL is a no-op.
void nw_catalogue_series_free(struct nw_catalogue_series *value);

// Free a `nw_central_bank_balance_sheet` returned through `**out`. NULL is a no-op.
void nw_central_bank_balance_sheet_free(struct nw_central_bank_balance_sheet *value);

// Free a `nw_central_bank_reserves` returned through `**out`. NULL is a no-op.
void nw_central_bank_reserves_free(struct nw_central_bank_reserves *value);

// Free a client after every call on it has returned. NULL is a no-op.
void nw_client_free(struct nw_client *client);

// Create a client. `config` may be NULL. Free the client with `nw_client_free`.
enum nw_status nw_client_new(const struct nw_config *config,
                             struct nw_client **out,
                             struct nw_error **err);

// Get Physical Series
//
// Stability: stable. `GET /v1/commodities/physical/series/{series_id}`.
// On success `*out` is owned by the caller; free it with `nw_physical_series_free`.
enum nw_status nw_commodities_get_physical_series(struct nw_client *client,
                                                  const char *series_id,
                                                  struct nw_physical_series **out,
                                                  struct nw_error **err);

// List Palmoil Physical
//
// Stability: stable. `GET /v1/commodities/palmoil/physical`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_palmoil_series_discovery_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_commodities_list_palmoil_physical_series(struct nw_client *client,
                                                           const struct nw_commodities_list_palmoil_physical_series_params *params,
                                                           struct nw_palmoil_series_discovery **out,
                                                           struct nw_error **err);

// List Physical Series
//
// Stability: stable. `GET /v1/commodities/physical/series`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_physical_series_page_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_commodities_list_physical_series(struct nw_client *client,
                                                   const struct nw_commodities_list_physical_series_params *params,
                                                   struct nw_physical_series_page **out,
                                                   struct nw_error **err);

// Get Palmoil Observations
//
// Stability: stable. `GET /v1/commodities/palmoil/physical/{series_id}/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_palmoil_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_commodities_palmoil_physical_observations(struct nw_client *client,
                                                            const char *series_id,
                                                            const struct nw_commodities_palmoil_physical_observations_params *params,
                                                            struct nw_palmoil_history **out,
                                                            struct nw_error **err);

// Get Palmoil Physical
//
// Stability: stable. `GET /v1/commodities/palmoil/physical/{series_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_palmoil_price_free`.
enum nw_status nw_commodities_palmoil_physical_price(struct nw_client *client,
                                                     const char *series_id,
                                                     const struct nw_commodities_palmoil_physical_price_params *params,
                                                     struct nw_palmoil_price **out,
                                                     struct nw_error **err);

// Get Physical Latest
//
// Stability: stable. `GET /v1/commodities/physical/series/{series_id}/latest`.
// On success `*out` is owned by the caller; free it with `nw_physical_latest_free`.
enum nw_status nw_commodities_physical_latest(struct nw_client *client,
                                              const char *series_id,
                                              struct nw_physical_latest **out,
                                              struct nw_error **err);

// Get Physical Observations
//
// Stability: stable. `GET /v1/commodities/physical/series/{series_id}/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_physical_observations_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_commodities_physical_observations(struct nw_client *client,
                                                    const char *series_id,
                                                    const struct nw_commodities_physical_observations_params *params,
                                                    struct nw_physical_observations **out,
                                                    struct nw_error **err);

// Get Commodity Price History
//
// Stability: stable. `GET /v1/commodities/spot/{commodity}/{currency}/bars`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_commodity_bars_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_commodities_price_history(struct nw_client *client,
                                            const char *commodity,
                                            const char *currency,
                                            const char *end,
                                            const char *start,
                                            const struct nw_commodities_price_history_params *params,
                                            struct nw_commodity_bars **out,
                                            struct nw_error **err);

// Get Commodity Spot
//
// Stability: stable. `GET /v1/commodities/spot/{commodity}/{currency}`.
// On success `*out` is owned by the caller; free it with `nw_commodity_spot_free`.
enum nw_status nw_commodities_spot_price(struct nw_client *client,
                                         const char *commodity,
                                         const char *currency,
                                         struct nw_commodity_spot **out,
                                         struct nw_error **err);

// Free a `nw_commodity_bars` returned through `**out`. NULL is a no-op.
void nw_commodity_bars_free(struct nw_commodity_bars *value);

// Free a `nw_commodity_spot` returned through `**out`. NULL is a no-op.
void nw_commodity_spot_free(struct nw_commodity_spot *value);

// Company dividends calendar
//
// Stability: stable. `GET /v1/calendars/dividends`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_dividends_calendar_free`.
enum nw_status nw_company_calendars_dividends(struct nw_client *client,
                                              const struct nw_company_calendars_dividends_params *params,
                                              struct nw_dividends_calendar **out,
                                              struct nw_error **err);

// Company splits calendar
//
// Stability: stable. `GET /v1/calendars/splits`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_splits_calendar_free`.
enum nw_status nw_company_calendars_splits(struct nw_client *client,
                                           const struct nw_company_calendars_splits_params *params,
                                           struct nw_splits_calendar **out,
                                           struct nw_error **err);

// Free a `nw_company_filings_page` returned through `**out`. NULL is a no-op.
void nw_company_filings_page_free(struct nw_company_filings_page *value);

// Free a `nw_company_search_page` returned through `**out`. NULL is a no-op.
void nw_company_search_page_free(struct nw_company_search_page *value);

// Free a `nw_company_with_listings` returned through `**out`. NULL is a no-op.
void nw_company_with_listings_free(struct nw_company_with_listings *value);

// Free a `nw_cpi` returned through `**out`. NULL is a no-op.
void nw_cpi_free(struct nw_cpi *value);

// Free a `nw_cpi_history` returned through `**out`. NULL is a no-op.
void nw_cpi_history_free(struct nw_cpi_history *value);

// Price bars of one crypto pair
//
// Stability: stable. `GET /v1/crypto/{pair}/bars`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_crypto_bars_model_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_crypto_bars(struct nw_client *client,
                              const char *pair,
                              const struct nw_crypto_bars_params *params,
                              struct nw_crypto_bars_model **out,
                              struct nw_error **err);

// Free a `nw_crypto_bars_model` returned through `**out`. NULL is a no-op.
void nw_crypto_bars_model_free(struct nw_crypto_bars_model *value);

// Crypto pairs we price
//
// Stability: stable. `GET /v1/crypto/pairs`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_crypto_pairs_free`.
enum nw_status nw_crypto_list_pairs(struct nw_client *client,
                                    const struct nw_crypto_list_pairs_params *params,
                                    struct nw_crypto_pairs **out,
                                    struct nw_error **err);

// Free a `nw_crypto_pairs` returned through `**out`. NULL is a no-op.
void nw_crypto_pairs_free(struct nw_crypto_pairs *value);

// Latest price of one crypto pair
//
// Stability: stable. `GET /v1/crypto/{pair}/price`.
// On success `*out` is owned by the caller; free it with `nw_crypto_price_model_free`.
enum nw_status nw_crypto_price(struct nw_client *client,
                               const char *pair,
                               struct nw_crypto_price_model **out,
                               struct nw_error **err);

// Free a `nw_crypto_price_model` returned through `**out`. NULL is a no-op.
void nw_crypto_price_model_free(struct nw_crypto_price_model *value);

// Latest prices for up to 50 crypto pairs
//
// Stability: stable. `GET /v1/crypto/prices`.
// On success `*out` is owned by the caller; free it with `nw_crypto_prices_model_free`.
enum nw_status nw_crypto_prices(struct nw_client *client,
                                const char *pairs,
                                struct nw_crypto_prices_model **out,
                                struct nw_error **err);

// Free a `nw_crypto_prices_model` returned through `**out`. NULL is a no-op.
void nw_crypto_prices_model_free(struct nw_crypto_prices_model *value);

// Free a `nw_dividends_calendar` returned through `**out`. NULL is a no-op.
void nw_dividends_calendar_free(struct nw_dividends_calendar *value);

// Free a `nw_economic_calendar` returned through `**out`. NULL is a no-op.
void nw_economic_calendar_free(struct nw_economic_calendar *value);

// Free a `nw_economic_release_detail` returned through `**out`. NULL is a no-op.
void nw_economic_release_detail_free(struct nw_economic_release_detail *value);

// Get Central Bank Balance Sheet
//
// Stability: stable. `GET /v1/economics/{country}/central-bank/balance-sheet`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_central_bank_balance_sheet_free`.
enum nw_status nw_economics_central_bank_balance_sheet(struct nw_client *client,
                                                       const char *country,
                                                       const struct nw_economics_central_bank_balance_sheet_params *params,
                                                       struct nw_central_bank_balance_sheet **out,
                                                       struct nw_error **err);

// Get Central Bank Reserves
//
// Stability: stable. `GET /v1/economics/{country}/central-bank/reserves`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_central_bank_reserves_free`.
enum nw_status nw_economics_central_bank_reserves(struct nw_client *client,
                                                  const char *country,
                                                  const struct nw_economics_central_bank_reserves_params *params,
                                                  struct nw_central_bank_reserves **out,
                                                  struct nw_error **err);

// Get Cpi
//
// Stability: stable. `GET /v1/economics/{country}/cpi`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_cpi_free`.
enum nw_status nw_economics_cpi(struct nw_client *client,
                                const char *country,
                                const struct nw_economics_cpi_params *params,
                                struct nw_cpi **out,
                                struct nw_error **err);

// Get Cpi History
//
// Stability: stable. `GET /v1/economics/{country}/cpi/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_cpi_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_cpi_history(struct nw_client *client,
                                        const char *country,
                                        const struct nw_economics_cpi_history_params *params,
                                        struct nw_cpi_history **out,
                                        struct nw_error **err);

// Get Equity Risk Premium History
//
// Stability: stable. `GET /v1/economics/{country}/equity-risk-premium/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_risk_premium_history_response_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_equity_risk_premium_history(struct nw_client *client,
                                                        const char *country,
                                                        const struct nw_economics_equity_risk_premium_history_params *params,
                                                        struct nw_equity_risk_premium_history_response **out,
                                                        struct nw_error **err);

// Get Gdp
//
// Stability: stable. `GET /v1/economics/{country}/gdp`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_gdp_free`.
enum nw_status nw_economics_gdp(struct nw_client *client,
                                const char *country,
                                const struct nw_economics_gdp_params *params,
                                struct nw_gdp **out,
                                struct nw_error **err);

// Get Gdp Growth
//
// Stability: stable. `GET /v1/economics/{country}/gdp-growth`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_gdp_growth_free`.
enum nw_status nw_economics_gdp_growth(struct nw_client *client,
                                       const char *country,
                                       const struct nw_economics_gdp_growth_params *params,
                                       struct nw_gdp_growth **out,
                                       struct nw_error **err);

// Get Gdp Growth History
//
// Stability: stable. `GET /v1/economics/{country}/gdp-growth/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_gdp_growth_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_gdp_growth_history(struct nw_client *client,
                                               const char *country,
                                               const struct nw_economics_gdp_growth_history_params *params,
                                               struct nw_gdp_growth_history **out,
                                               struct nw_error **err);

// Get Gdp History
//
// Stability: stable. `GET /v1/economics/{country}/gdp/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_gdp_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_gdp_history(struct nw_client *client,
                                        const char *country,
                                        const struct nw_economics_gdp_history_params *params,
                                        struct nw_gdp_history **out,
                                        struct nw_error **err);

// Get Equity Risk Premium
//
// Stability: stable. `GET /v1/economics/{country}/equity-risk-premium`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_risk_premium_free`.
enum nw_status nw_economics_get_equity_risk_premium(struct nw_client *client,
                                                    const char *country,
                                                    const struct nw_economics_get_equity_risk_premium_params *params,
                                                    struct nw_equity_risk_premium **out,
                                                    struct nw_error **err);

// Get Economic Release
//
// Stability: stable. `GET /v1/economics/calendar/{release_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_economic_release_detail_free`.
enum nw_status nw_economics_get_release(struct nw_client *client,
                                        const char *release_id,
                                        const struct nw_economics_get_release_params *params,
                                        struct nw_economic_release_detail **out,
                                        struct nw_error **err);

// Get Inflation
//
// Stability: stable. `GET /v1/economics/{country}/inflation`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_inflation_free`.
enum nw_status nw_economics_inflation(struct nw_client *client,
                                      const char *country,
                                      const struct nw_economics_inflation_params *params,
                                      struct nw_inflation **out,
                                      struct nw_error **err);

// Get Inflation History
//
// Stability: stable. `GET /v1/economics/{country}/inflation/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_inflation_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_inflation_history(struct nw_client *client,
                                              const char *country,
                                              const struct nw_economics_inflation_history_params *params,
                                              struct nw_inflation_history **out,
                                              struct nw_error **err);

// Get Labour Force Participation
//
// Stability: stable. `GET /v1/economics/{country}/labour-force-participation`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_labour_force_participation_free`.
enum nw_status nw_economics_labour_force_participation(struct nw_client *client,
                                                       const char *country,
                                                       const struct nw_economics_labour_force_participation_params *params,
                                                       struct nw_labour_force_participation **out,
                                                       struct nw_error **err);

// Get Labour Force Participation History
//
// Stability: stable. `GET /v1/economics/{country}/labour-force-participation/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_rate_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_labour_force_participation_history(struct nw_client *client,
                                                               const char *country,
                                                               const struct nw_economics_labour_force_participation_history_params *params,
                                                               struct nw_rate_history **out,
                                                               struct nw_error **err);

// List Equity Risk Premium
//
// Stability: stable. `GET /v1/economics/equity-risk-premium`.
// On success `*out` is owned by the caller; free it with `nw_equity_risk_premium_list_free`.
enum nw_status nw_economics_list_equity_risk_premium(struct nw_client *client,
                                                     struct nw_equity_risk_premium_list **out,
                                                     struct nw_error **err);

// List Economic Releases
//
// Stability: stable. `GET /v1/economics/calendar`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_economic_calendar_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_list_releases(struct nw_client *client,
                                          const struct nw_economics_list_releases_params *params,
                                          struct nw_economic_calendar **out,
                                          struct nw_error **err);

// Get Policy Rate
//
// Stability: stable. `GET /v1/economics/{country}/central-bank/policy-rate`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_policy_rate_free`.
enum nw_status nw_economics_policy_rate(struct nw_client *client,
                                        const char *country,
                                        const struct nw_economics_policy_rate_params *params,
                                        struct nw_policy_rate **out,
                                        struct nw_error **err);

// Get Policy Rate History
//
// Stability: stable. `GET /v1/economics/{country}/central-bank/policy-rate/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_policy_rate_history_response_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_policy_rate_history(struct nw_client *client,
                                                const char *country,
                                                const struct nw_economics_policy_rate_history_params *params,
                                                struct nw_policy_rate_history_response **out,
                                                struct nw_error **err);

// Read Catalogue Latest Batch
//
// Stability: stable. `GET /v1/economics/catalogue/latest`.
// On success `*out` is owned by the caller; free it with `nw_catalogue_bulk_latest_free`.
enum nw_status nw_economics_read_catalogue_latest_batch(struct nw_client *client,
                                                        const char *ids,
                                                        struct nw_catalogue_bulk_latest **out,
                                                        struct nw_error **err);

// Read Catalogue History
//
// Stability: stable. `GET /v1/economics/catalogue/series/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_catalogue_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_read_catalogue_observations(struct nw_client *client,
                                                        const char *id,
                                                        const struct nw_economics_read_catalogue_observations_params *params,
                                                        struct nw_catalogue_history **out,
                                                        struct nw_error **err);

// Read Catalogue Series
//
// Stability: stable. `GET /v1/economics/catalogue/series`.
// On success `*out` is owned by the caller; free it with `nw_catalogue_series_free`.
enum nw_status nw_economics_read_catalogue_series(struct nw_client *client,
                                                  const char *id,
                                                  struct nw_catalogue_series **out,
                                                  struct nw_error **err);

// Read Resource History
//
// Stability: stable. `GET /v1/economics/series/{series_id}/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_resource_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_read_series_observations(struct nw_client *client,
                                                     const char *series_id,
                                                     const struct nw_economics_read_series_observations_params *params,
                                                     struct nw_resource_history **out,
                                                     struct nw_error **err);

// Search Series Catalogue
//
// Stability: stable. `GET /v1/economics/catalogue`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_catalogue_search_page_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_search_catalogue(struct nw_client *client,
                                             const struct nw_economics_search_catalogue_params *params,
                                             struct nw_catalogue_search_page **out,
                                             struct nw_error **err);

// Get Trade
//
// Stability: stable. `GET /v1/economics/{country}/trade`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_trade_summary_free`.
enum nw_status nw_economics_trade(struct nw_client *client,
                                  const char *country,
                                  const struct nw_economics_trade_params *params,
                                  struct nw_trade_summary **out,
                                  struct nw_error **err);

// Get Trade History
//
// Stability: stable. `GET /v1/economics/{country}/trade/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_trade_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_trade_history(struct nw_client *client,
                                          const char *country,
                                          const struct nw_economics_trade_history_params *params,
                                          struct nw_trade_history **out,
                                          struct nw_error **err);

// Get Unemployment
//
// Stability: stable. `GET /v1/economics/{country}/unemployment`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_unemployment_free`.
enum nw_status nw_economics_unemployment(struct nw_client *client,
                                         const char *country,
                                         const struct nw_economics_unemployment_params *params,
                                         struct nw_unemployment **out,
                                         struct nw_error **err);

// Get Unemployment History
//
// Stability: stable. `GET /v1/economics/{country}/unemployment/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_rate_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_unemployment_history(struct nw_client *client,
                                                 const char *country,
                                                 const struct nw_economics_unemployment_history_params *params,
                                                 struct nw_rate_history **out,
                                                 struct nw_error **err);

// Get Yield Curve
//
// Stability: stable. `GET /v1/economics/{country}/yield-curve`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_yield_curve_free`.
enum nw_status nw_economics_yield_curve(struct nw_client *client,
                                        const char *country,
                                        const struct nw_economics_yield_curve_params *params,
                                        struct nw_yield_curve **out,
                                        struct nw_error **err);

// Get Yield Curve History
//
// Stability: stable. `GET /v1/economics/{country}/yield-curve/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_yield_curve_history_response_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_economics_yield_curve_history(struct nw_client *client,
                                                const char *country,
                                                const struct nw_economics_yield_curve_history_params *params,
                                                struct nw_yield_curve_history_response **out,
                                                struct nw_error **err);

// Price bars of one stock
//
// Stability: stable. `GET /v1/equities/{id}/bars`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_bars_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_equities_bars(struct nw_client *client,
                                const char *id,
                                const struct nw_equities_bars_params *params,
                                struct nw_equity_bars **out,
                                struct nw_error **err);

// Beta, cost of equity, cost of debt and WACC of one stock
//
// Stability: stable. `GET /v1/equities/{id}/cost-of-capital`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_cost_of_capital_free`.
enum nw_status nw_equities_cost_of_capital(struct nw_client *client,
                                           const char *id,
                                           const struct nw_equities_cost_of_capital_params *params,
                                           struct nw_equity_cost_of_capital **out,
                                           struct nw_error **err);

// Dividends of one stock
//
// Stability: stable. `GET /v1/equities/{id}/dividends`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_dividends_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_equities_dividends(struct nw_client *client,
                                     const char *id,
                                     const struct nw_equities_dividends_params *params,
                                     struct nw_equity_dividends **out,
                                     struct nw_error **err);

// Earnings of one stock
//
// Stability: stable. `GET /v1/equities/{id}/earnings`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_earnings_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_equities_earnings(struct nw_client *client,
                                    const char *id,
                                    const struct nw_equities_earnings_params *params,
                                    struct nw_equity_earnings **out,
                                    struct nw_error **err);

// Top US stock movers
//
// Stability: stable. `GET /v1/equities/movers`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_movers_free`.
enum nw_status nw_equities_movers(struct nw_client *client,
                                  const struct nw_equities_movers_params *params,
                                  struct nw_equity_movers **out,
                                  struct nw_error **err);

// Latest price of one stock
//
// Stability: stable. `GET /v1/equities/{id}/price`.
// On success `*out` is owned by the caller; free it with `nw_equity_price_free`.
enum nw_status nw_equities_price(struct nw_client *client,
                                 const char *id,
                                 struct nw_equity_price **out,
                                 struct nw_error **err);

// Latest prices for up to 50 stocks
//
// Stability: stable. `GET /v1/equities/prices`.
// On success `*out` is owned by the caller; free it with `nw_equity_prices_free`.
enum nw_status nw_equities_prices(struct nw_client *client,
                                  const char *ids,
                                  struct nw_equity_prices **out,
                                  struct nw_error **err);

// Profile of one stock
//
// Stability: stable. `GET /v1/equities/{id}`.
// On success `*out` is owned by the caller; free it with `nw_equity_profile_free`.
enum nw_status nw_equities_profile(struct nw_client *client,
                                   const char *id,
                                   struct nw_equity_profile **out,
                                   struct nw_error **err);

// Search stocks by ticker or name
//
// Stability: stable. `GET /v1/equities/search`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_search_results_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_equities_search(struct nw_client *client,
                                  const char *q,
                                  const struct nw_equities_search_params *params,
                                  struct nw_equity_search_results **out,
                                  struct nw_error **err);

// Splits of one stock
//
// Stability: stable. `GET /v1/equities/{id}/splits`.
// On success `*out` is owned by the caller; free it with `nw_equity_splits_free`.
enum nw_status nw_equities_splits(struct nw_client *client,
                                  const char *id,
                                  struct nw_equity_splits **out,
                                  struct nw_error **err);

// Market cap, P/E, P/S, P/B and dividend yield of one stock
//
// Stability: stable. `GET /v1/equities/{id}/valuation`.
// On success `*out` is owned by the caller; free it with `nw_equity_valuation_free`.
enum nw_status nw_equities_valuation(struct nw_client *client,
                                     const char *id,
                                     struct nw_equity_valuation **out,
                                     struct nw_error **err);

// Valuation history of one stock
//
// Stability: stable. `GET /v1/equities/{id}/valuation/history`.
// On success `*out` is owned by the caller; free it with `nw_equity_valuation_history_free`.
enum nw_status nw_equities_valuation_history(struct nw_client *client,
                                             const char *id,
                                             struct nw_equity_valuation_history **out,
                                             struct nw_error **err);

// Free a `nw_equity_bars` returned through `**out`. NULL is a no-op.
void nw_equity_bars_free(struct nw_equity_bars *value);

// Free a `nw_equity_cost_of_capital` returned through `**out`. NULL is a no-op.
void nw_equity_cost_of_capital_free(struct nw_equity_cost_of_capital *value);

// Free a `nw_equity_dividends` returned through `**out`. NULL is a no-op.
void nw_equity_dividends_free(struct nw_equity_dividends *value);

// Free a `nw_equity_earnings` returned through `**out`. NULL is a no-op.
void nw_equity_earnings_free(struct nw_equity_earnings *value);

// Free a `nw_equity_movers` returned through `**out`. NULL is a no-op.
void nw_equity_movers_free(struct nw_equity_movers *value);

// Free a `nw_equity_price` returned through `**out`. NULL is a no-op.
void nw_equity_price_free(struct nw_equity_price *value);

// Free a `nw_equity_prices` returned through `**out`. NULL is a no-op.
void nw_equity_prices_free(struct nw_equity_prices *value);

// Free a `nw_equity_profile` returned through `**out`. NULL is a no-op.
void nw_equity_profile_free(struct nw_equity_profile *value);

// Free a `nw_equity_risk_premium` returned through `**out`. NULL is a no-op.
void nw_equity_risk_premium_free(struct nw_equity_risk_premium *value);

// Free a `nw_equity_risk_premium_history_response` returned through `**out`. NULL is a no-op.
void nw_equity_risk_premium_history_response_free(struct nw_equity_risk_premium_history_response *value);

// Free a `nw_equity_risk_premium_list` returned through `**out`. NULL is a no-op.
void nw_equity_risk_premium_list_free(struct nw_equity_risk_premium_list *value);

// Free a `nw_equity_search_results` returned through `**out`. NULL is a no-op.
void nw_equity_search_results_free(struct nw_equity_search_results *value);

// Free a `nw_equity_splits` returned through `**out`. NULL is a no-op.
void nw_equity_splits_free(struct nw_equity_splits *value);

// Free a `nw_equity_valuation` returned through `**out`. NULL is a no-op.
void nw_equity_valuation_free(struct nw_equity_valuation *value);

// Free a `nw_equity_valuation_history` returned through `**out`. NULL is a no-op.
void nw_equity_valuation_history_free(struct nw_equity_valuation_history *value);

// API problem code, or the webhook verification reason; NULL for other errors.
const char *nw_error_code(const struct nw_error *error);

// API problem detail; NULL for other errors.
const char *nw_error_detail(const struct nw_error *error);

// Free an error. NULL is a no-op.
void nw_error_free(struct nw_error *error);

// Problem instance of an API error; NULL when absent.
const char *nw_error_instance(const struct nw_error *error);

// Error text. API errors read `HTTP <status> <code>: <title>`, plus
// ` (request_id <id>)` only when a request ID is present.
const char *nw_error_message(const struct nw_error *error);

// Original response bytes of an API or decode error, owned by the error. Sets `*len`
// when `len` is non-NULL. NULL with length 0 when there is no body.
const uint8_t *nw_error_raw_body(const struct nw_error *error, size_t *len);

// Request ID of an API error; NULL when absent or empty.
const char *nw_error_request_id(const struct nw_error *error);

// `Retry-After` seconds of an API error; -1 if absent.
int64_t nw_error_retry_after(const struct nw_error *error);

// HTTP status of an API error; 0 for other errors.
int nw_error_status(const struct nw_error *error);

// API problem title; NULL for other errors.
const char *nw_error_title(const struct nw_error *error);

// Free a `nw_event_detail` returned through `**out`. NULL is a no-op.
void nw_event_detail_free(struct nw_event_detail *value);

// Get Delivery Event
//
// Stability: stable. `GET /v1/events/{event_id}`.
// On success `*out` is owned by the caller; free it with `nw_event_detail_free`.
enum nw_status nw_events_get(struct nw_client *client,
                             const char *event_id,
                             struct nw_event_detail **out,
                             struct nw_error **err);

// List Delivery Events
//
// Stability: stable. `GET /v1/events`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_events_page_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_events_list(struct nw_client *client,
                              const struct nw_events_list_params *params,
                              struct nw_events_page **out,
                              struct nw_error **err);

// Free a `nw_events_page` returned through `**out`. NULL is a no-op.
void nw_events_page_free(struct nw_events_page *value);

// Free a `nw_extreme_event` returned through `**out`. NULL is a no-op.
void nw_extreme_event_free(struct nw_extreme_event *value);

// Free a `nw_filing` returned through `**out`. NULL is a no-op.
void nw_filing_free(struct nw_filing *value);

// Get As Reported
//
// Stability: stable. `GET /v1/filings/{filing_id}/financials/as-reported`.
// On success `*out` is owned by the caller; free it with `nw_as_reported_page_free`.
enum nw_status nw_filings_as_reported(struct nw_client *client,
                                      const char *filing_id,
                                      struct nw_as_reported_page **out,
                                      struct nw_error **err);

// Get Company
//
// Stability: stable. `GET /v1/companies/{company_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_company_with_listings_free`.
enum nw_status nw_filings_company(struct nw_client *client,
                                  const char *company_id,
                                  const struct nw_filings_company_params *params,
                                  struct nw_company_with_listings **out,
                                  struct nw_error **err);

// Get Filing
//
// Stability: stable. `GET /v1/filings/{filing_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_filing_free`.
enum nw_status nw_filings_filing(struct nw_client *client,
                                 const char *filing_id,
                                 const struct nw_filings_filing_params *params,
                                 struct nw_filing **out,
                                 struct nw_error **err);

// Get Financials
//
// Stability: stable. `GET /v1/companies/{company_id}/financials`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_financials_free`.
enum nw_status nw_filings_financials(struct nw_client *client,
                                     const char *company_id,
                                     const struct nw_filings_financials_params *params,
                                     struct nw_financials **out,
                                     struct nw_error **err);

// List Company Filings
//
// Stability: stable. `GET /v1/companies/{company_id}/filings`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_company_filings_page_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_filings_list_company_filings(struct nw_client *client,
                                               const char *company_id,
                                               const struct nw_filings_list_company_filings_params *params,
                                               struct nw_company_filings_page **out,
                                               struct nw_error **err);

// List Holders
//
// Stability: stable. `GET /v1/companies/{company_id}/holders`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_holders_page_free`.
enum nw_status nw_filings_list_holders(struct nw_client *client,
                                       const char *company_id,
                                       const struct nw_filings_list_holders_params *params,
                                       struct nw_holders_page **out,
                                       struct nw_error **err);

// List Latest Filings
//
// Stability: stable. `GET /v1/filings/latest`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_latest_filings_page_free`.
enum nw_status nw_filings_list_latest_filings(struct nw_client *client,
                                              const struct nw_filings_list_latest_filings_params *params,
                                              struct nw_latest_filings_page **out,
                                              struct nw_error **err);

// Search Companies
//
// Stability: stable. `GET /v1/companies`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_company_search_page_free`.
enum nw_status nw_filings_search_companies(struct nw_client *client,
                                           const struct nw_filings_search_companies_params *params,
                                           struct nw_company_search_page **out,
                                           struct nw_error **err);

// Free a `nw_financials` returned through `**out`. NULL is a no-op.
void nw_financials_free(struct nw_financials *value);

// Free a `nw_fund_distributions` returned through `**out`. NULL is a no-op.
void nw_fund_distributions_free(struct nw_fund_distributions *value);

// Free a `nw_fund_price` returned through `**out`. NULL is a no-op.
void nw_fund_price_free(struct nw_fund_price *value);

// Free a `nw_fund_prices` returned through `**out`. NULL is a no-op.
void nw_fund_prices_free(struct nw_fund_prices *value);

// Free a `nw_fund_profile` returned through `**out`. NULL is a no-op.
void nw_fund_profile_free(struct nw_fund_profile *value);

// Free a `nw_fund_search_results` returned through `**out`. NULL is a no-op.
void nw_fund_search_results_free(struct nw_fund_search_results *value);

// Price bars of one fund
//
// Stability: stable. `GET /v1/funds/{id}/bars`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_equity_bars_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_funds_bars(struct nw_client *client,
                             const char *id,
                             const struct nw_funds_bars_params *params,
                             struct nw_equity_bars **out,
                             struct nw_error **err);

// Distributions of one fund
//
// Stability: stable. `GET /v1/funds/{id}/distributions`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_fund_distributions_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_funds_distributions(struct nw_client *client,
                                      const char *id,
                                      const struct nw_funds_distributions_params *params,
                                      struct nw_fund_distributions **out,
                                      struct nw_error **err);

// Latest price of one fund
//
// Stability: stable. `GET /v1/funds/{id}/price`.
// On success `*out` is owned by the caller; free it with `nw_fund_price_free`.
enum nw_status nw_funds_price(struct nw_client *client,
                              const char *id,
                              struct nw_fund_price **out,
                              struct nw_error **err);

// Latest prices for up to 50 funds
//
// Stability: stable. `GET /v1/funds/prices`.
// On success `*out` is owned by the caller; free it with `nw_fund_prices_free`.
enum nw_status nw_funds_prices(struct nw_client *client,
                               const char *ids,
                               struct nw_fund_prices **out,
                               struct nw_error **err);

// Profile of one fund
//
// Stability: stable. `GET /v1/funds/{id}`.
// On success `*out` is owned by the caller; free it with `nw_fund_profile_free`.
enum nw_status nw_funds_profile(struct nw_client *client,
                                const char *id,
                                struct nw_fund_profile **out,
                                struct nw_error **err);

// Search ETFs and mutual funds by ticker or name
//
// Stability: stable. `GET /v1/funds/search`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_fund_search_results_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_funds_search(struct nw_client *client,
                               const char *q,
                               const struct nw_funds_search_params *params,
                               struct nw_fund_search_results **out,
                               struct nw_error **err);

// Free a `nw_fx_bars` returned through `**out`. NULL is a no-op.
void nw_fx_bars_free(struct nw_fx_bars *value);

// Free a `nw_fx_conversion` returned through `**out`. NULL is a no-op.
void nw_fx_conversion_free(struct nw_fx_conversion *value);

// Convert Fx
//
// Stability: stable. `GET /v1/fx/convert`.
// On success `*out` is owned by the caller; free it with `nw_fx_conversion_free`.
enum nw_status nw_fx_convert_currency(struct nw_client *client,
                                      const char *amount,
                                      const char *from,
                                      const char *to,
                                      struct nw_fx_conversion **out,
                                      struct nw_error **err);

// Free a `nw_fx_history` returned through `**out`. NULL is a no-op.
void nw_fx_history_free(struct nw_fx_history *value);

// Get Fx Rate Bars
//
// Stability: stable. `GET /v1/fx/rates/{base}/bars`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_fx_bars_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_fx_rate_bars(struct nw_client *client,
                               const char *base,
                               const char *currency,
                               const char *end,
                               const char *start,
                               const struct nw_fx_rate_bars_params *params,
                               struct nw_fx_bars **out,
                               struct nw_error **err);

// Get Fx Rate History
//
// Stability: stable. `GET /v1/fx/rates/{base}/observations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_fx_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_fx_rate_history(struct nw_client *client,
                                  const char *base,
                                  const char *const *currencies,
                                  size_t currencies_len,
                                  const char *end_date,
                                  const char *start_date,
                                  const struct nw_fx_rate_history_params *params,
                                  struct nw_fx_history **out,
                                  struct nw_error **err);

// Get Fx Rates
//
// Stability: stable. `GET /v1/fx/rates/{base}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_fx_rates_model_free`.
enum nw_status nw_fx_rates(struct nw_client *client,
                           const char *base,
                           const struct nw_fx_rates_params *params,
                           struct nw_fx_rates_model **out,
                           struct nw_error **err);

// Free a `nw_fx_rates_model` returned through `**out`. NULL is a no-op.
void nw_fx_rates_model_free(struct nw_fx_rates_model *value);

// Free a `nw_gdp` returned through `**out`. NULL is a no-op.
void nw_gdp_free(struct nw_gdp *value);

// Free a `nw_gdp_growth` returned through `**out`. NULL is a no-op.
void nw_gdp_growth_free(struct nw_gdp_growth *value);

// Free a `nw_gdp_growth_history` returned through `**out`. NULL is a no-op.
void nw_gdp_growth_history_free(struct nw_gdp_growth_history *value);

// Free a `nw_gdp_history` returned through `**out`. NULL is a no-op.
void nw_gdp_history_free(struct nw_gdp_history *value);

// Free a `nw_holders_page` returned through `**out`. NULL is a no-op.
void nw_holders_page_free(struct nw_holders_page *value);

// Free a `nw_inflation` returned through `**out`. NULL is a no-op.
void nw_inflation_free(struct nw_inflation *value);

// Free a `nw_inflation_history` returned through `**out`. NULL is a no-op.
void nw_inflation_history_free(struct nw_inflation_history *value);

// Free a `nw_labour_force_participation` returned through `**out`. NULL is a no-op.
void nw_labour_force_participation_free(struct nw_labour_force_participation *value);

// Free a `nw_latest_filings_page` returned through `**out`. NULL is a no-op.
void nw_latest_filings_page_free(struct nw_latest_filings_page *value);

// Free a `nw_no_content` returned through `**out`. NULL is a no-op.
void nw_no_content_free(struct nw_no_content *value);

// Free a `nw_palmoil_history` returned through `**out`. NULL is a no-op.
void nw_palmoil_history_free(struct nw_palmoil_history *value);

// Free a `nw_palmoil_price` returned through `**out`. NULL is a no-op.
void nw_palmoil_price_free(struct nw_palmoil_price *value);

// Free a `nw_palmoil_series_discovery` returned through `**out`. NULL is a no-op.
void nw_palmoil_series_discovery_free(struct nw_palmoil_series_discovery *value);

// Free a `nw_physical_latest` returned through `**out`. NULL is a no-op.
void nw_physical_latest_free(struct nw_physical_latest *value);

// Free a `nw_physical_observations` returned through `**out`. NULL is a no-op.
void nw_physical_observations_free(struct nw_physical_observations *value);

// Free a `nw_physical_series` returned through `**out`. NULL is a no-op.
void nw_physical_series_free(struct nw_physical_series *value);

// Free a `nw_physical_series_page` returned through `**out`. NULL is a no-op.
void nw_physical_series_page_free(struct nw_physical_series_page *value);

// Free a `nw_policy_path` returned through `**out`. NULL is a no-op.
void nw_policy_path_free(struct nw_policy_path *value);

// Free a `nw_policy_path_history` returned through `**out`. NULL is a no-op.
void nw_policy_path_history_free(struct nw_policy_path_history *value);

// Free a `nw_policy_rate` returned through `**out`. NULL is a no-op.
void nw_policy_rate_free(struct nw_policy_rate *value);

// Free a `nw_policy_rate_history_response` returned through `**out`. NULL is a no-op.
void nw_policy_rate_history_response_free(struct nw_policy_rate_history_response *value);

// Free a `nw_prediction_events_page` returned through `**out`. NULL is a no-op.
void nw_prediction_events_page_free(struct nw_prediction_events_page *value);

// Free a `nw_prediction_history` returned through `**out`. NULL is a no-op.
void nw_prediction_history_free(struct nw_prediction_history *value);

// Free a `nw_prediction_topic_detail` returned through `**out`. NULL is a no-op.
void nw_prediction_topic_detail_free(struct nw_prediction_topic_detail *value);

// Free a `nw_prediction_topics` returned through `**out`. NULL is a no-op.
void nw_prediction_topics_free(struct nw_prediction_topics *value);

// Get Binary Prediction
//
// Stability: stable. `GET /v1/predictions/binary/{event_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_binary_event_free`.
enum nw_status nw_predictions_binary(struct nw_client *client,
                                     const char *event_id,
                                     const struct nw_predictions_binary_params *params,
                                     struct nw_binary_event **out,
                                     struct nw_error **err);

// Get Binary Prediction History
//
// Stability: stable. `GET /v1/predictions/binary/{event_id}/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_predictions_binary_history(struct nw_client *client,
                                             const char *event_id,
                                             const char *end,
                                             const char *start,
                                             const struct nw_predictions_binary_history_params *params,
                                             struct nw_prediction_history **out,
                                             struct nw_error **err);

// Get Extreme Prediction
//
// Stability: stable. `GET /v1/predictions/extreme/{event_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_extreme_event_free`.
enum nw_status nw_predictions_extreme(struct nw_client *client,
                                      const char *event_id,
                                      const struct nw_predictions_extreme_params *params,
                                      struct nw_extreme_event **out,
                                      struct nw_error **err);

// Get Extreme Prediction History
//
// Stability: stable. `GET /v1/predictions/extreme/{event_id}/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_predictions_extreme_history(struct nw_client *client,
                                              const char *event_id,
                                              const char *end,
                                              const char *start,
                                              const struct nw_predictions_extreme_history_params *params,
                                              struct nw_prediction_history **out,
                                              struct nw_error **err);

// Get Prediction Topic
//
// Stability: stable. `GET /v1/predictions/topics/{topic_id}`.
// On success `*out` is owned by the caller; free it with `nw_prediction_topic_detail_free`.
enum nw_status nw_predictions_get_topic(struct nw_client *client,
                                        const char *topic_id,
                                        struct nw_prediction_topic_detail **out,
                                        struct nw_error **err);

// List Prediction Events
//
// Stability: stable. `GET /v1/predictions/events`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_prediction_events_page_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_predictions_list_events(struct nw_client *client,
                                          const struct nw_predictions_list_events_params *params,
                                          struct nw_prediction_events_page **out,
                                          struct nw_error **err);

// List Prediction Topics
//
// Stability: stable. `GET /v1/predictions/topics`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_prediction_topics_free`.
enum nw_status nw_predictions_list_topics(struct nw_client *client,
                                          const struct nw_predictions_list_topics_params *params,
                                          struct nw_prediction_topics **out,
                                          struct nw_error **err);

// Get Policy Path
//
// Stability: stable. `GET /v1/predictions/policy-paths/{authority}`.
// On success `*out` is owned by the caller; free it with `nw_policy_path_free`.
enum nw_status nw_predictions_policy_path(struct nw_client *client,
                                          const char *authority,
                                          struct nw_policy_path **out,
                                          struct nw_error **err);

// Get Policy Path History
//
// Stability: stable. `GET /v1/predictions/policy-paths/{authority}/history`.
// On success `*out` is owned by the caller; free it with `nw_policy_path_history_free`.
enum nw_status nw_predictions_policy_path_history(struct nw_client *client,
                                                  const char *authority,
                                                  const char *end,
                                                  const char *start,
                                                  struct nw_policy_path_history **out,
                                                  struct nw_error **err);

// Get Scalar Prediction
//
// Stability: stable. `GET /v1/predictions/scalar/{event_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_scalar_event_free`.
enum nw_status nw_predictions_scalar(struct nw_client *client,
                                     const char *event_id,
                                     const struct nw_predictions_scalar_params *params,
                                     struct nw_scalar_event **out,
                                     struct nw_error **err);

// Get Scalar Prediction History
//
// Stability: stable. `GET /v1/predictions/scalar/{event_id}/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_predictions_scalar_history(struct nw_client *client,
                                             const char *event_id,
                                             const char *end,
                                             const char *start,
                                             const struct nw_predictions_scalar_history_params *params,
                                             struct nw_prediction_history **out,
                                             struct nw_error **err);

// Get Timing Prediction
//
// Stability: stable. `GET /v1/predictions/timing/{event_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_timing_event_free`.
enum nw_status nw_predictions_timing(struct nw_client *client,
                                     const char *event_id,
                                     const struct nw_predictions_timing_params *params,
                                     struct nw_timing_event **out,
                                     struct nw_error **err);

// Get Timing Prediction History
//
// Stability: stable. `GET /v1/predictions/timing/{event_id}/history`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_predictions_timing_history(struct nw_client *client,
                                             const char *event_id,
                                             const char *end,
                                             const char *start,
                                             const struct nw_predictions_timing_history_params *params,
                                             struct nw_prediction_history **out,
                                             struct nw_error **err);

// Free a `nw_rate_history` returned through `**out`. NULL is a no-op.
void nw_rate_history_free(struct nw_rate_history *value);

// Free a `nw_resource_history` returned through `**out`. NULL is a no-op.
void nw_resource_history_free(struct nw_resource_history *value);

// Free a `nw_scalar_event` returned through `**out`. NULL is a no-op.
void nw_scalar_event_free(struct nw_scalar_event *value);

// Free a `nw_splits_calendar` returned through `**out`. NULL is a no-op.
void nw_splits_calendar_free(struct nw_splits_calendar *value);

// Free a `nw_timing_event` returned through `**out`. NULL is a no-op.
void nw_timing_event_free(struct nw_timing_event *value);

// Free a `nw_trade_history` returned through `**out`. NULL is a no-op.
void nw_trade_history_free(struct nw_trade_history *value);

// Free a `nw_trade_summary` returned through `**out`. NULL is a no-op.
void nw_trade_summary_free(struct nw_trade_summary *value);

// Free a `nw_unemployment` returned through `**out`. NULL is a no-op.
void nw_unemployment_free(struct nw_unemployment *value);

// Library version, a static string. Never NULL.
const char *nw_version(void);

// Whether this library can serve a program compiled against header version
// `major.minor`: pass `NW_VERSION_MAJOR` and `NW_VERSION_MINOR`. Before 1.0 the minor
// version must match (a minor release may break); from 1.0 the major must match and the
// library minor must be at least `minor`. Returns false on panic.
bool nw_version_check(uint32_t major, uint32_t minor);

// Free a `nw_webhook_deliveries_response` returned through `**out`. NULL is a no-op.
void nw_webhook_deliveries_response_free(struct nw_webhook_deliveries_response *value);

// Free a `nw_webhook_destination_created` returned through `**out`. NULL is a no-op.
void nw_webhook_destination_created_free(struct nw_webhook_destination_created *value);

// Free a `nw_webhook_destination_list` returned through `**out`. NULL is a no-op.
void nw_webhook_destination_list_free(struct nw_webhook_destination_list *value);

// Free a `nw_webhook_destination_response` returned through `**out`. NULL is a no-op.
void nw_webhook_destination_response_free(struct nw_webhook_destination_response *value);

// Free a `nw_webhook_event` returned through `**out`. NULL is a no-op.
void nw_webhook_event_free(struct nw_webhook_event *value);

// Free a `nw_webhook_history_response` returned through `**out`. NULL is a no-op.
void nw_webhook_history_response_free(struct nw_webhook_history_response *value);

// Free a `nw_webhook_test_response` returned through `**out`. NULL is a no-op.
void nw_webhook_test_response_free(struct nw_webhook_test_response *value);

// Verify a webhook delivery over the exact raw body bytes, then decode it.
// A NULL header is missing (`missing_header`). `tolerance_seconds` must be >= 0;
// `now_unix` 0 uses the system clock. Verification failures return `NW_ERR_WEBHOOK`
// with the reason as `nw_error_code`. Free the event with `nw_webhook_event_free`.
enum nw_status nw_webhook_verify(const uint8_t *body,
                                 size_t body_len,
                                 const char *timestamp_header,
                                 const char *signature_header,
                                 const char *const *secrets,
                                 size_t secrets_len,
                                 int64_t tolerance_seconds,
                                 int64_t now_unix,
                                 struct nw_webhook_event **out,
                                 struct nw_error **err);

// Configuration History
//
// Stability: stable. `GET /v1/webhooks/destinations/{destination_id}/history`.
// On success `*out` is owned by the caller; free it with `nw_webhook_history_response_free`.
enum nw_status nw_webhooks_configuration_history(struct nw_client *client,
                                                 const char *destination_id,
                                                 struct nw_webhook_history_response **out,
                                                 struct nw_error **err);

// Create Destination
//
// Stability: stable. `POST /v1/webhooks/destinations`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_webhook_destination_created_free`.
enum nw_status nw_webhooks_create_destination(struct nw_client *client,
                                              const struct nw_webhooks_create_destination_params *params,
                                              struct nw_webhook_destination_created **out,
                                              struct nw_error **err);

// Delete Destination
//
// Stability: stable. `DELETE /v1/webhooks/destinations/{destination_id}`.
// On success `*out` is owned by the caller; free it with `nw_no_content_free`.
enum nw_status nw_webhooks_delete_destination(struct nw_client *client,
                                              const char *destination_id,
                                              struct nw_no_content **out,
                                              struct nw_error **err);

// Delivery History
//
// Stability: stable. `GET /v1/webhooks/destinations/{destination_id}/deliveries`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_webhook_deliveries_response_free`.
// `next_cursor` is NULL on the last page.
enum nw_status nw_webhooks_delivery_history(struct nw_client *client,
                                            const char *destination_id,
                                            const struct nw_webhooks_delivery_history_params *params,
                                            struct nw_webhook_deliveries_response **out,
                                            struct nw_error **err);

// Get Destination
//
// Stability: stable. `GET /v1/webhooks/destinations/{destination_id}`.
// On success `*out` is owned by the caller; free it with `nw_webhook_destination_response_free`.
enum nw_status nw_webhooks_get_destination(struct nw_client *client,
                                           const char *destination_id,
                                           struct nw_webhook_destination_response **out,
                                           struct nw_error **err);

// List Destinations
//
// Stability: stable. `GET /v1/webhooks/destinations`.
// On success `*out` is owned by the caller; free it with `nw_webhook_destination_list_free`.
enum nw_status nw_webhooks_list_destinations(struct nw_client *client,
                                             struct nw_webhook_destination_list **out,
                                             struct nw_error **err);

// Replay Delivery
//
// Stability: stable. `POST /v1/webhooks/destinations/{destination_id}/deliveries/{delivery_id}/replay`.
// On success `*out` is owned by the caller; free it with `nw_no_content_free`.
enum nw_status nw_webhooks_replay_delivery(struct nw_client *client,
                                           const char *delivery_id,
                                           const char *destination_id,
                                           struct nw_no_content **out,
                                           struct nw_error **err);

// Rotate Secret
//
// Stability: stable. `POST /v1/webhooks/destinations/{destination_id}/rotate-secret`.
// On success `*out` is owned by the caller; free it with `nw_webhook_destination_created_free`.
enum nw_status nw_webhooks_rotate_secret(struct nw_client *client,
                                         const char *destination_id,
                                         struct nw_webhook_destination_created **out,
                                         struct nw_error **err);

// Test Destination
//
// Stability: stable. `POST /v1/webhooks/destinations/{destination_id}/test`.
// On success `*out` is owned by the caller; free it with `nw_webhook_test_response_free`.
enum nw_status nw_webhooks_test_destination(struct nw_client *client,
                                            const char *destination_id,
                                            struct nw_webhook_test_response **out,
                                            struct nw_error **err);

// Update Destination
//
// Stability: stable. `PATCH /v1/webhooks/destinations/{destination_id}`.
// `params` may be NULL for defaults.
// On success `*out` is owned by the caller; free it with `nw_webhook_destination_response_free`.
enum nw_status nw_webhooks_update_destination(struct nw_client *client,
                                              const char *destination_id,
                                              const struct nw_webhooks_update_destination_params *params,
                                              struct nw_webhook_destination_response **out,
                                              struct nw_error **err);

// Free a `nw_yield_curve` returned through `**out`. NULL is a no-op.
void nw_yield_curve_free(struct nw_yield_curve *value);

// Free a `nw_yield_curve_history_response` returned through `**out`. NULL is a no-op.
void nw_yield_curve_history_response_free(struct nw_yield_curve_history_response *value);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* NARWHALAPI_H */

// Generated from spec/next/openapi.json sha256 5071bd97e9507eb56b344d00a24ba056ce1340fe1184faa4d755fe7e407e1cf6. Do not edit.
use crate::*;
use std::os::raw::c_char;
/// Major version of this header and library.
pub const NW_VERSION_MAJOR: u32 = 0;
/// Minor version of this header and library.
pub const NW_VERSION_MINOR: u32 = 2;
/// Patch version of this header and library.
pub const NW_VERSION_PATCH: u32 = 0;
/// `CatalogueLatestFrequency`: a value this version does not know.
pub const NW_CATALOGUE_LATEST_FREQUENCY_UNKNOWN: i32 = 0;
/// `CatalogueLatestFrequency`: `annual`.
pub const NW_CATALOGUE_LATEST_FREQUENCY_ANNUAL: i32 = 1;
/// `CatalogueLatestFrequency`: `daily`.
pub const NW_CATALOGUE_LATEST_FREQUENCY_DAILY: i32 = 2;
/// `CatalogueLatestFrequency`: `monthly`.
pub const NW_CATALOGUE_LATEST_FREQUENCY_MONTHLY: i32 = 3;
/// `CatalogueLatestFrequency`: `quarterly`.
pub const NW_CATALOGUE_LATEST_FREQUENCY_QUARTERLY: i32 = 4;
fn catalogue_latest_frequency_code(value: &str) -> i32 {
    match value {
        "annual" => NW_CATALOGUE_LATEST_FREQUENCY_ANNUAL,
        "daily" => NW_CATALOGUE_LATEST_FREQUENCY_DAILY,
        "monthly" => NW_CATALOGUE_LATEST_FREQUENCY_MONTHLY,
        "quarterly" => NW_CATALOGUE_LATEST_FREQUENCY_QUARTERLY,
        _ => NW_CATALOGUE_LATEST_FREQUENCY_UNKNOWN,
    }
}
/// `CatalogueLatestSeasonalAdjustment`: a value this version does not know.
pub const NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_UNKNOWN: i32 = 0;
/// `CatalogueLatestSeasonalAdjustment`: `not_applicable`.
pub const NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_APPLICABLE: i32 = 1;
/// `CatalogueLatestSeasonalAdjustment`: `not_seasonally_adjusted`.
pub const NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_SEASONALLY_ADJUSTED: i32 = 2;
/// `CatalogueLatestSeasonalAdjustment`: `not_stated`.
pub const NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_STATED: i32 = 3;
/// `CatalogueLatestSeasonalAdjustment`: `seasonally_adjusted`.
pub const NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_SEASONALLY_ADJUSTED: i32 = 4;
fn catalogue_latest_seasonal_adjustment_code(value: &str) -> i32 {
    match value {
        "not_applicable" => NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_APPLICABLE,
        "not_seasonally_adjusted" => {
            NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_SEASONALLY_ADJUSTED
        }
        "not_stated" => NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_NOT_STATED,
        "seasonally_adjusted" => NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_SEASONALLY_ADJUSTED,
        _ => NW_CATALOGUE_LATEST_SEASONAL_ADJUSTMENT_UNKNOWN,
    }
}
/// `CentralBankBalanceSheetScale`: a value this version does not know.
pub const NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_UNKNOWN: i32 = 0;
/// `CentralBankBalanceSheetScale`: `millions`.
pub const NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_MILLIONS: i32 = 1;
/// `CentralBankBalanceSheetScale`: `units`.
pub const NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_UNITS: i32 = 2;
fn central_bank_balance_sheet_scale_code(value: &str) -> i32 {
    match value {
        "millions" => NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_MILLIONS,
        "units" => NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_UNITS,
        _ => NW_CENTRAL_BANK_BALANCE_SHEET_SCALE_UNKNOWN,
    }
}
/// `CommodityUnit`: a value this version does not know.
pub const NW_COMMODITY_UNIT_UNKNOWN: i32 = 0;
/// `CommodityUnit`: `barrel`.
pub const NW_COMMODITY_UNIT_BARREL: i32 = 1;
/// `CommodityUnit`: `pound`.
pub const NW_COMMODITY_UNIT_POUND: i32 = 2;
/// `CommodityUnit`: `troy_ounce`.
pub const NW_COMMODITY_UNIT_TROY_OUNCE: i32 = 3;
fn commodity_unit_code(value: &str) -> i32 {
    match value {
        "barrel" => NW_COMMODITY_UNIT_BARREL,
        "pound" => NW_COMMODITY_UNIT_POUND,
        "troy_ounce" => NW_COMMODITY_UNIT_TROY_OUNCE,
        _ => NW_COMMODITY_UNIT_UNKNOWN,
    }
}
/// `CompanyRelationKind`: a value this version does not know.
pub const NW_COMPANY_RELATION_KIND_UNKNOWN: i32 = 0;
/// `CompanyRelationKind`: `cross_listed`.
pub const NW_COMPANY_RELATION_KIND_CROSS_LISTED: i32 = 1;
/// `CompanyRelationKind`: `predecessor`.
pub const NW_COMPANY_RELATION_KIND_PREDECESSOR: i32 = 2;
/// `CompanyRelationKind`: `successor`.
pub const NW_COMPANY_RELATION_KIND_SUCCESSOR: i32 = 3;
fn company_relation_kind_code(value: &str) -> i32 {
    match value {
        "cross_listed" => NW_COMPANY_RELATION_KIND_CROSS_LISTED,
        "predecessor" => NW_COMPANY_RELATION_KIND_PREDECESSOR,
        "successor" => NW_COMPANY_RELATION_KIND_SUCCESSOR,
        _ => NW_COMPANY_RELATION_KIND_UNKNOWN,
    }
}
/// `CompanySearchMatch`: a value this version does not know.
pub const NW_COMPANY_SEARCH_MATCH_UNKNOWN: i32 = 0;
/// `CompanySearchMatch`: `code`.
pub const NW_COMPANY_SEARCH_MATCH_CODE: i32 = 1;
/// `CompanySearchMatch`: `former_code`.
pub const NW_COMPANY_SEARCH_MATCH_FORMER_CODE: i32 = 2;
/// `CompanySearchMatch`: `former_name`.
pub const NW_COMPANY_SEARCH_MATCH_FORMER_NAME: i32 = 3;
/// `CompanySearchMatch`: `id`.
pub const NW_COMPANY_SEARCH_MATCH_ID: i32 = 4;
/// `CompanySearchMatch`: `name`.
pub const NW_COMPANY_SEARCH_MATCH_NAME: i32 = 5;
fn company_search_match_code(value: &str) -> i32 {
    match value {
        "code" => NW_COMPANY_SEARCH_MATCH_CODE,
        "former_code" => NW_COMPANY_SEARCH_MATCH_FORMER_CODE,
        "former_name" => NW_COMPANY_SEARCH_MATCH_FORMER_NAME,
        "id" => NW_COMPANY_SEARCH_MATCH_ID,
        "name" => NW_COMPANY_SEARCH_MATCH_NAME,
        _ => NW_COMPANY_SEARCH_MATCH_UNKNOWN,
    }
}
/// `CompanyStandard`: a value this version does not know.
pub const NW_COMPANY_STANDARD_UNKNOWN: i32 = 0;
/// `CompanyStandard`: `ifrs`.
pub const NW_COMPANY_STANDARD_IFRS: i32 = 1;
/// `CompanyStandard`: `j_gaap`.
pub const NW_COMPANY_STANDARD_J_GAAP: i32 = 2;
/// `CompanyStandard`: `jmis`.
pub const NW_COMPANY_STANDARD_JMIS: i32 = 3;
/// `CompanyStandard`: `k_gaap`.
pub const NW_COMPANY_STANDARD_K_GAAP: i32 = 4;
/// `CompanyStandard`: `us_gaap`.
pub const NW_COMPANY_STANDARD_US_GAAP: i32 = 5;
fn company_standard_code(value: &str) -> i32 {
    match value {
        "ifrs" => NW_COMPANY_STANDARD_IFRS,
        "j_gaap" => NW_COMPANY_STANDARD_J_GAAP,
        "jmis" => NW_COMPANY_STANDARD_JMIS,
        "k_gaap" => NW_COMPANY_STANDARD_K_GAAP,
        "us_gaap" => NW_COMPANY_STANDARD_US_GAAP,
        _ => NW_COMPANY_STANDARD_UNKNOWN,
    }
}
/// `CostEquityRiskPremiumBasis`: a value this version does not know.
pub const NW_COST_EQUITY_RISK_PREMIUM_BASIS_UNKNOWN: i32 = 0;
/// `CostEquityRiskPremiumBasis`: `modelled`.
pub const NW_COST_EQUITY_RISK_PREMIUM_BASIS_MODELLED: i32 = 1;
/// `CostEquityRiskPremiumBasis`: `observed`.
pub const NW_COST_EQUITY_RISK_PREMIUM_BASIS_OBSERVED: i32 = 2;
/// `CostEquityRiskPremiumBasis`: `table`.
pub const NW_COST_EQUITY_RISK_PREMIUM_BASIS_TABLE: i32 = 3;
fn cost_equity_risk_premium_basis_code(value: &str) -> i32 {
    match value {
        "modelled" => NW_COST_EQUITY_RISK_PREMIUM_BASIS_MODELLED,
        "observed" => NW_COST_EQUITY_RISK_PREMIUM_BASIS_OBSERVED,
        "table" => NW_COST_EQUITY_RISK_PREMIUM_BASIS_TABLE,
        _ => NW_COST_EQUITY_RISK_PREMIUM_BASIS_UNKNOWN,
    }
}
/// `CostEquityRiskPremiumSource`: a value this version does not know.
pub const NW_COST_EQUITY_RISK_PREMIUM_SOURCE_UNKNOWN: i32 = 0;
/// `CostEquityRiskPremiumSource`: `caller`.
pub const NW_COST_EQUITY_RISK_PREMIUM_SOURCE_CALLER: i32 = 1;
/// `CostEquityRiskPremiumSource`: `narwhal`.
pub const NW_COST_EQUITY_RISK_PREMIUM_SOURCE_NARWHAL: i32 = 2;
fn cost_equity_risk_premium_source_code(value: &str) -> i32 {
    match value {
        "caller" => NW_COST_EQUITY_RISK_PREMIUM_SOURCE_CALLER,
        "narwhal" => NW_COST_EQUITY_RISK_PREMIUM_SOURCE_NARWHAL,
        _ => NW_COST_EQUITY_RISK_PREMIUM_SOURCE_UNKNOWN,
    }
}
/// `CostOfCapitalBeta`: a value this version does not know.
pub const NW_COST_OF_CAPITAL_BETA_UNKNOWN: i32 = 0;
/// `CostOfCapitalBeta`: `adjusted`.
pub const NW_COST_OF_CAPITAL_BETA_ADJUSTED: i32 = 1;
/// `CostOfCapitalBeta`: `raw`.
pub const NW_COST_OF_CAPITAL_BETA_RAW: i32 = 2;
fn cost_of_capital_beta_code(value: &str) -> i32 {
    match value {
        "adjusted" => NW_COST_OF_CAPITAL_BETA_ADJUSTED,
        "raw" => NW_COST_OF_CAPITAL_BETA_RAW,
        _ => NW_COST_OF_CAPITAL_BETA_UNKNOWN,
    }
}
/// `CostOfCapitalFrequency`: a value this version does not know.
pub const NW_COST_OF_CAPITAL_FREQUENCY_UNKNOWN: i32 = 0;
/// `CostOfCapitalFrequency`: `monthly`.
pub const NW_COST_OF_CAPITAL_FREQUENCY_MONTHLY: i32 = 1;
/// `CostOfCapitalFrequency`: `weekly`.
pub const NW_COST_OF_CAPITAL_FREQUENCY_WEEKLY: i32 = 2;
fn cost_of_capital_frequency_code(value: &str) -> i32 {
    match value {
        "monthly" => NW_COST_OF_CAPITAL_FREQUENCY_MONTHLY,
        "weekly" => NW_COST_OF_CAPITAL_FREQUENCY_WEEKLY,
        _ => NW_COST_OF_CAPITAL_FREQUENCY_UNKNOWN,
    }
}
/// `CostOfCapitalPeriod`: a value this version does not know.
pub const NW_COST_OF_CAPITAL_PERIOD_UNKNOWN: i32 = 0;
/// `CostOfCapitalPeriod`: `annual`.
pub const NW_COST_OF_CAPITAL_PERIOD_ANNUAL: i32 = 1;
/// `CostOfCapitalPeriod`: `ttm`.
pub const NW_COST_OF_CAPITAL_PERIOD_TTM: i32 = 2;
fn cost_of_capital_period_code(value: &str) -> i32 {
    match value {
        "annual" => NW_COST_OF_CAPITAL_PERIOD_ANNUAL,
        "ttm" => NW_COST_OF_CAPITAL_PERIOD_TTM,
        _ => NW_COST_OF_CAPITAL_PERIOD_UNKNOWN,
    }
}
/// `CostOfCapitalWindow`: a value this version does not know.
pub const NW_COST_OF_CAPITAL_WINDOW_UNKNOWN: i32 = 0;
/// `CostOfCapitalWindow`: `2y`.
pub const NW_COST_OF_CAPITAL_WINDOW_2Y: i32 = 1;
/// `CostOfCapitalWindow`: `5y`.
pub const NW_COST_OF_CAPITAL_WINDOW_5Y: i32 = 2;
fn cost_of_capital_window_code(value: &str) -> i32 {
    match value {
        "2y" => NW_COST_OF_CAPITAL_WINDOW_2Y,
        "5y" => NW_COST_OF_CAPITAL_WINDOW_5Y,
        _ => NW_COST_OF_CAPITAL_WINDOW_UNKNOWN,
    }
}
/// `CostOfDebtBasis`: a value this version does not know.
pub const NW_COST_OF_DEBT_BASIS_UNKNOWN: i32 = 0;
/// `CostOfDebtBasis`: `ours`.
pub const NW_COST_OF_DEBT_BASIS_OURS: i32 = 1;
/// `CostOfDebtBasis`: `table`.
pub const NW_COST_OF_DEBT_BASIS_TABLE: i32 = 2;
fn cost_of_debt_basis_code(value: &str) -> i32 {
    match value {
        "ours" => NW_COST_OF_DEBT_BASIS_OURS,
        "table" => NW_COST_OF_DEBT_BASIS_TABLE,
        _ => NW_COST_OF_DEBT_BASIS_UNKNOWN,
    }
}
/// `CostRiskFreeBasis`: a value this version does not know.
pub const NW_COST_RISK_FREE_BASIS_UNKNOWN: i32 = 0;
/// `CostRiskFreeBasis`: `caller`.
pub const NW_COST_RISK_FREE_BASIS_CALLER: i32 = 1;
/// `CostRiskFreeBasis`: `ours`.
pub const NW_COST_RISK_FREE_BASIS_OURS: i32 = 2;
fn cost_risk_free_basis_code(value: &str) -> i32 {
    match value {
        "caller" => NW_COST_RISK_FREE_BASIS_CALLER,
        "ours" => NW_COST_RISK_FREE_BASIS_OURS,
        _ => NW_COST_RISK_FREE_BASIS_UNKNOWN,
    }
}
/// `CostRiskFreeSource`: a value this version does not know.
pub const NW_COST_RISK_FREE_SOURCE_UNKNOWN: i32 = 0;
/// `CostRiskFreeSource`: `caller`.
pub const NW_COST_RISK_FREE_SOURCE_CALLER: i32 = 1;
/// `CostRiskFreeSource`: `yield_curve`.
pub const NW_COST_RISK_FREE_SOURCE_YIELD_CURVE: i32 = 2;
fn cost_risk_free_source_code(value: &str) -> i32 {
    match value {
        "caller" => NW_COST_RISK_FREE_SOURCE_CALLER,
        "yield_curve" => NW_COST_RISK_FREE_SOURCE_YIELD_CURVE,
        _ => NW_COST_RISK_FREE_SOURCE_UNKNOWN,
    }
}
/// `CostTaxRateBasis`: a value this version does not know.
pub const NW_COST_TAX_RATE_BASIS_UNKNOWN: i32 = 0;
/// `CostTaxRateBasis`: `caller`.
pub const NW_COST_TAX_RATE_BASIS_CALLER: i32 = 1;
/// `CostTaxRateBasis`: `table`.
pub const NW_COST_TAX_RATE_BASIS_TABLE: i32 = 2;
fn cost_tax_rate_basis_code(value: &str) -> i32 {
    match value {
        "caller" => NW_COST_TAX_RATE_BASIS_CALLER,
        "table" => NW_COST_TAX_RATE_BASIS_TABLE,
        _ => NW_COST_TAX_RATE_BASIS_UNKNOWN,
    }
}
/// `CostTaxRateSource`: a value this version does not know.
pub const NW_COST_TAX_RATE_SOURCE_UNKNOWN: i32 = 0;
/// `CostTaxRateSource`: `caller`.
pub const NW_COST_TAX_RATE_SOURCE_CALLER: i32 = 1;
/// `CostTaxRateSource`: `country_table`.
pub const NW_COST_TAX_RATE_SOURCE_COUNTRY_TABLE: i32 = 3;
fn cost_tax_rate_source_code(value: &str) -> i32 {
    match value {
        "caller" => NW_COST_TAX_RATE_SOURCE_CALLER,
        "country_table" => NW_COST_TAX_RATE_SOURCE_COUNTRY_TABLE,
        _ => NW_COST_TAX_RATE_SOURCE_UNKNOWN,
    }
}
/// `CryptoPriceErrorCode`: a value this version does not know.
pub const NW_CRYPTO_PRICE_ERROR_CODE_UNKNOWN: i32 = 0;
/// `CryptoPriceErrorCode`: `not_covered`.
pub const NW_CRYPTO_PRICE_ERROR_CODE_NOT_COVERED: i32 = 1;
/// `CryptoPriceErrorCode`: `not_found`.
pub const NW_CRYPTO_PRICE_ERROR_CODE_NOT_FOUND: i32 = 2;
/// `CryptoPriceErrorCode`: `upstream_busy`.
pub const NW_CRYPTO_PRICE_ERROR_CODE_UPSTREAM_BUSY: i32 = 3;
fn crypto_price_error_code_code(value: &str) -> i32 {
    match value {
        "not_covered" => NW_CRYPTO_PRICE_ERROR_CODE_NOT_COVERED,
        "not_found" => NW_CRYPTO_PRICE_ERROR_CODE_NOT_FOUND,
        "upstream_busy" => NW_CRYPTO_PRICE_ERROR_CODE_UPSTREAM_BUSY,
        _ => NW_CRYPTO_PRICE_ERROR_CODE_UNKNOWN,
    }
}
/// `DeliveryFamily`: a value this version does not know.
pub const NW_DELIVERY_FAMILY_UNKNOWN: i32 = 0;
/// `DeliveryFamily`: `gdp`.
pub const NW_DELIVERY_FAMILY_GDP: i32 = 1;
/// `DeliveryFamily`: `holidays`.
pub const NW_DELIVERY_FAMILY_HOLIDAYS: i32 = 2;
/// `DeliveryFamily`: `labour`.
pub const NW_DELIVERY_FAMILY_LABOUR: i32 = 3;
/// `DeliveryFamily`: `prices`.
pub const NW_DELIVERY_FAMILY_PRICES: i32 = 4;
/// `DeliveryFamily`: `trade`.
pub const NW_DELIVERY_FAMILY_TRADE: i32 = 5;
/// `DeliveryFamily`: `yield_curve`.
pub const NW_DELIVERY_FAMILY_YIELD_CURVE: i32 = 6;
/// `DeliveryFamily`: `activity`.
pub const NW_DELIVERY_FAMILY_ACTIVITY: i32 = 7;
/// `DeliveryFamily`: `housing`.
pub const NW_DELIVERY_FAMILY_HOUSING: i32 = 8;
/// `DeliveryFamily`: `money`.
pub const NW_DELIVERY_FAMILY_MONEY: i32 = 9;
/// `DeliveryFamily`: `policy_rate`.
pub const NW_DELIVERY_FAMILY_POLICY_RATE: i32 = 10;
/// `DeliveryFamily`: `public_finance`.
pub const NW_DELIVERY_FAMILY_PUBLIC_FINANCE: i32 = 11;
/// `DeliveryFamily`: `survey`.
pub const NW_DELIVERY_FAMILY_SURVEY: i32 = 12;
fn delivery_family_code(value: &str) -> i32 {
    match value {
        "gdp" => NW_DELIVERY_FAMILY_GDP,
        "holidays" => NW_DELIVERY_FAMILY_HOLIDAYS,
        "labour" => NW_DELIVERY_FAMILY_LABOUR,
        "prices" => NW_DELIVERY_FAMILY_PRICES,
        "trade" => NW_DELIVERY_FAMILY_TRADE,
        "yield_curve" => NW_DELIVERY_FAMILY_YIELD_CURVE,
        "activity" => NW_DELIVERY_FAMILY_ACTIVITY,
        "housing" => NW_DELIVERY_FAMILY_HOUSING,
        "money" => NW_DELIVERY_FAMILY_MONEY,
        "policy_rate" => NW_DELIVERY_FAMILY_POLICY_RATE,
        "public_finance" => NW_DELIVERY_FAMILY_PUBLIC_FINANCE,
        "survey" => NW_DELIVERY_FAMILY_SURVEY,
        _ => NW_DELIVERY_FAMILY_UNKNOWN,
    }
}
/// `EconomicFamily`: a value this version does not know.
pub const NW_ECONOMIC_FAMILY_UNKNOWN: i32 = 0;
/// `EconomicFamily`: `gdp`.
pub const NW_ECONOMIC_FAMILY_GDP: i32 = 1;
/// `EconomicFamily`: `labour`.
pub const NW_ECONOMIC_FAMILY_LABOUR: i32 = 2;
/// `EconomicFamily`: `prices`.
pub const NW_ECONOMIC_FAMILY_PRICES: i32 = 3;
/// `EconomicFamily`: `trade`.
pub const NW_ECONOMIC_FAMILY_TRADE: i32 = 4;
/// `EconomicFamily`: `yield_curve`.
pub const NW_ECONOMIC_FAMILY_YIELD_CURVE: i32 = 5;
/// `EconomicFamily`: `activity`.
pub const NW_ECONOMIC_FAMILY_ACTIVITY: i32 = 6;
/// `EconomicFamily`: `housing`.
pub const NW_ECONOMIC_FAMILY_HOUSING: i32 = 7;
/// `EconomicFamily`: `money`.
pub const NW_ECONOMIC_FAMILY_MONEY: i32 = 8;
/// `EconomicFamily`: `policy_rate`.
pub const NW_ECONOMIC_FAMILY_POLICY_RATE: i32 = 9;
/// `EconomicFamily`: `public_finance`.
pub const NW_ECONOMIC_FAMILY_PUBLIC_FINANCE: i32 = 10;
/// `EconomicFamily`: `survey`.
pub const NW_ECONOMIC_FAMILY_SURVEY: i32 = 11;
fn economic_family_code(value: &str) -> i32 {
    match value {
        "gdp" => NW_ECONOMIC_FAMILY_GDP,
        "labour" => NW_ECONOMIC_FAMILY_LABOUR,
        "prices" => NW_ECONOMIC_FAMILY_PRICES,
        "trade" => NW_ECONOMIC_FAMILY_TRADE,
        "yield_curve" => NW_ECONOMIC_FAMILY_YIELD_CURVE,
        "activity" => NW_ECONOMIC_FAMILY_ACTIVITY,
        "housing" => NW_ECONOMIC_FAMILY_HOUSING,
        "money" => NW_ECONOMIC_FAMILY_MONEY,
        "policy_rate" => NW_ECONOMIC_FAMILY_POLICY_RATE,
        "public_finance" => NW_ECONOMIC_FAMILY_PUBLIC_FINANCE,
        "survey" => NW_ECONOMIC_FAMILY_SURVEY,
        _ => NW_ECONOMIC_FAMILY_UNKNOWN,
    }
}
/// `EconomicReleaseValueValueStatus`: a value this version does not know.
pub const NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_UNKNOWN: i32 = 0;
/// `EconomicReleaseValueValueStatus`: `pending_capture`.
pub const NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_PENDING_CAPTURE: i32 = 1;
/// `EconomicReleaseValueValueStatus`: `unavailable`.
pub const NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_UNAVAILABLE: i32 = 2;
fn economic_release_value_value_status_code(value: &str) -> i32 {
    match value {
        "pending_capture" => NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_PENDING_CAPTURE,
        "unavailable" => NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_UNAVAILABLE,
        _ => NW_ECONOMIC_RELEASE_VALUE_VALUE_STATUS_UNKNOWN,
    }
}
/// `EquityEarningTime`: a value this version does not know.
pub const NW_EQUITY_EARNING_TIME_UNKNOWN: i32 = 0;
/// `EquityEarningTime`: `after_close`.
pub const NW_EQUITY_EARNING_TIME_AFTER_CLOSE: i32 = 1;
/// `EquityEarningTime`: `before_open`.
pub const NW_EQUITY_EARNING_TIME_BEFORE_OPEN: i32 = 2;
/// `EquityEarningTime`: `during`.
pub const NW_EQUITY_EARNING_TIME_DURING: i32 = 3;
fn equity_earning_time_code(value: &str) -> i32 {
    match value {
        "after_close" => NW_EQUITY_EARNING_TIME_AFTER_CLOSE,
        "before_open" => NW_EQUITY_EARNING_TIME_BEFORE_OPEN,
        "during" => NW_EQUITY_EARNING_TIME_DURING,
        _ => NW_EQUITY_EARNING_TIME_UNKNOWN,
    }
}
/// `EquityMarketSession`: a value this version does not know.
pub const NW_EQUITY_MARKET_SESSION_UNKNOWN: i32 = 0;
/// `EquityMarketSession`: `closed`.
pub const NW_EQUITY_MARKET_SESSION_CLOSED: i32 = 1;
/// `EquityMarketSession`: `open`.
pub const NW_EQUITY_MARKET_SESSION_OPEN: i32 = 2;
/// `EquityMarketSession`: `post`.
pub const NW_EQUITY_MARKET_SESSION_POST: i32 = 3;
/// `EquityMarketSession`: `pre`.
pub const NW_EQUITY_MARKET_SESSION_PRE: i32 = 4;
fn equity_market_session_code(value: &str) -> i32 {
    match value {
        "closed" => NW_EQUITY_MARKET_SESSION_CLOSED,
        "open" => NW_EQUITY_MARKET_SESSION_OPEN,
        "post" => NW_EQUITY_MARKET_SESSION_POST,
        "pre" => NW_EQUITY_MARKET_SESSION_PRE,
        _ => NW_EQUITY_MARKET_SESSION_UNKNOWN,
    }
}
/// `EquityType`: a value this version does not know.
pub const NW_EQUITY_TYPE_UNKNOWN: i32 = 0;
/// `EquityType`: `closed_end_fund`.
pub const NW_EQUITY_TYPE_CLOSED_END_FUND: i32 = 1;
/// `EquityType`: `common_stock`.
pub const NW_EQUITY_TYPE_COMMON_STOCK: i32 = 2;
/// `EquityType`: `depositary_receipt`.
pub const NW_EQUITY_TYPE_DEPOSITARY_RECEIPT: i32 = 3;
/// `EquityType`: `other`.
pub const NW_EQUITY_TYPE_OTHER: i32 = 4;
/// `EquityType`: `preferred_stock`.
pub const NW_EQUITY_TYPE_PREFERRED_STOCK: i32 = 5;
/// `EquityType`: `reit`.
pub const NW_EQUITY_TYPE_REIT: i32 = 6;
/// `EquityType`: `right`.
pub const NW_EQUITY_TYPE_RIGHT: i32 = 7;
/// `EquityType`: `unit`.
pub const NW_EQUITY_TYPE_UNIT: i32 = 8;
/// `EquityType`: `warrant`.
pub const NW_EQUITY_TYPE_WARRANT: i32 = 9;
fn equity_type_code(value: &str) -> i32 {
    match value {
        "closed_end_fund" => NW_EQUITY_TYPE_CLOSED_END_FUND,
        "common_stock" => NW_EQUITY_TYPE_COMMON_STOCK,
        "depositary_receipt" => NW_EQUITY_TYPE_DEPOSITARY_RECEIPT,
        "other" => NW_EQUITY_TYPE_OTHER,
        "preferred_stock" => NW_EQUITY_TYPE_PREFERRED_STOCK,
        "reit" => NW_EQUITY_TYPE_REIT,
        "right" => NW_EQUITY_TYPE_RIGHT,
        "unit" => NW_EQUITY_TYPE_UNIT,
        "warrant" => NW_EQUITY_TYPE_WARRANT,
        _ => NW_EQUITY_TYPE_UNKNOWN,
    }
}
/// `ExtremeObservation`: a value this version does not know.
pub const NW_EXTREME_OBSERVATION_UNKNOWN: i32 = 0;
/// `ExtremeObservation`: `any_trade`.
pub const NW_EXTREME_OBSERVATION_ANY_TRADE: i32 = 1;
/// `ExtremeObservation`: `close`.
pub const NW_EXTREME_OBSERVATION_CLOSE: i32 = 2;
/// `ExtremeObservation`: `any_release`.
pub const NW_EXTREME_OBSERVATION_ANY_RELEASE: i32 = 3;
fn extreme_observation_code(value: &str) -> i32 {
    match value {
        "any_trade" => NW_EXTREME_OBSERVATION_ANY_TRADE,
        "close" => NW_EXTREME_OBSERVATION_CLOSE,
        "any_release" => NW_EXTREME_OBSERVATION_ANY_RELEASE,
        _ => NW_EXTREME_OBSERVATION_UNKNOWN,
    }
}
/// `FilingEventType`: a value this version does not know.
pub const NW_FILING_EVENT_TYPE_UNKNOWN: i32 = 0;
/// `FilingEventType`: `annual`.
pub const NW_FILING_EVENT_TYPE_ANNUAL: i32 = 1;
/// `FilingEventType`: `event`.
pub const NW_FILING_EVENT_TYPE_EVENT: i32 = 2;
/// `FilingEventType`: `ownership`.
pub const NW_FILING_EVENT_TYPE_OWNERSHIP: i32 = 3;
/// `FilingEventType`: `quarterly`.
pub const NW_FILING_EVENT_TYPE_QUARTERLY: i32 = 4;
/// `FilingEventType`: `semiannual`.
pub const NW_FILING_EVENT_TYPE_SEMIANNUAL: i32 = 5;
fn filing_event_type_code(value: &str) -> i32 {
    match value {
        "annual" => NW_FILING_EVENT_TYPE_ANNUAL,
        "event" => NW_FILING_EVENT_TYPE_EVENT,
        "ownership" => NW_FILING_EVENT_TYPE_OWNERSHIP,
        "quarterly" => NW_FILING_EVENT_TYPE_QUARTERLY,
        "semiannual" => NW_FILING_EVENT_TYPE_SEMIANNUAL,
        _ => NW_FILING_EVENT_TYPE_UNKNOWN,
    }
}
/// `FilingType`: a value this version does not know.
pub const NW_FILING_TYPE_UNKNOWN: i32 = 0;
/// `FilingType`: `annual`.
pub const NW_FILING_TYPE_ANNUAL: i32 = 1;
/// `FilingType`: `event`.
pub const NW_FILING_TYPE_EVENT: i32 = 2;
/// `FilingType`: `insider`.
pub const NW_FILING_TYPE_INSIDER: i32 = 3;
/// `FilingType`: `other`.
pub const NW_FILING_TYPE_OTHER: i32 = 4;
/// `FilingType`: `ownership`.
pub const NW_FILING_TYPE_OWNERSHIP: i32 = 5;
/// `FilingType`: `quarterly`.
pub const NW_FILING_TYPE_QUARTERLY: i32 = 6;
/// `FilingType`: `semiannual`.
pub const NW_FILING_TYPE_SEMIANNUAL: i32 = 7;
fn filing_type_code(value: &str) -> i32 {
    match value {
        "annual" => NW_FILING_TYPE_ANNUAL,
        "event" => NW_FILING_TYPE_EVENT,
        "insider" => NW_FILING_TYPE_INSIDER,
        "other" => NW_FILING_TYPE_OTHER,
        "ownership" => NW_FILING_TYPE_OWNERSHIP,
        "quarterly" => NW_FILING_TYPE_QUARTERLY,
        "semiannual" => NW_FILING_TYPE_SEMIANNUAL,
        _ => NW_FILING_TYPE_UNKNOWN,
    }
}
/// `FinancialsBasis`: a value this version does not know.
pub const NW_FINANCIALS_BASIS_UNKNOWN: i32 = 0;
/// `FinancialsBasis`: `consolidated`.
pub const NW_FINANCIALS_BASIS_CONSOLIDATED: i32 = 1;
/// `FinancialsBasis`: `standalone`.
pub const NW_FINANCIALS_BASIS_STANDALONE: i32 = 2;
fn financials_basis_code(value: &str) -> i32 {
    match value {
        "consolidated" => NW_FINANCIALS_BASIS_CONSOLIDATED,
        "standalone" => NW_FINANCIALS_BASIS_STANDALONE,
        _ => NW_FINANCIALS_BASIS_UNKNOWN,
    }
}
/// `FiscalPeriod`: a value this version does not know.
pub const NW_FISCAL_PERIOD_UNKNOWN: i32 = 0;
/// `FiscalPeriod`: `FY`.
pub const NW_FISCAL_PERIOD_FY: i32 = 1;
/// `FiscalPeriod`: `H1`.
pub const NW_FISCAL_PERIOD_H1: i32 = 2;
/// `FiscalPeriod`: `H2`.
pub const NW_FISCAL_PERIOD_H2: i32 = 3;
/// `FiscalPeriod`: `Q1`.
pub const NW_FISCAL_PERIOD_Q1: i32 = 4;
/// `FiscalPeriod`: `Q2`.
pub const NW_FISCAL_PERIOD_Q2: i32 = 5;
/// `FiscalPeriod`: `Q3`.
pub const NW_FISCAL_PERIOD_Q3: i32 = 6;
/// `FiscalPeriod`: `Q4`.
pub const NW_FISCAL_PERIOD_Q4: i32 = 7;
fn fiscal_period_code(value: &str) -> i32 {
    match value {
        "FY" => NW_FISCAL_PERIOD_FY,
        "H1" => NW_FISCAL_PERIOD_H1,
        "H2" => NW_FISCAL_PERIOD_H2,
        "Q1" => NW_FISCAL_PERIOD_Q1,
        "Q2" => NW_FISCAL_PERIOD_Q2,
        "Q3" => NW_FISCAL_PERIOD_Q3,
        "Q4" => NW_FISCAL_PERIOD_Q4,
        _ => NW_FISCAL_PERIOD_UNKNOWN,
    }
}
/// `FundDistributionKind`: a value this version does not know.
pub const NW_FUND_DISTRIBUTION_KIND_UNKNOWN: i32 = 0;
/// `FundDistributionKind`: `capital_gain`.
pub const NW_FUND_DISTRIBUTION_KIND_CAPITAL_GAIN: i32 = 1;
/// `FundDistributionKind`: `income`.
pub const NW_FUND_DISTRIBUTION_KIND_INCOME: i32 = 2;
fn fund_distribution_kind_code(value: &str) -> i32 {
    match value {
        "capital_gain" => NW_FUND_DISTRIBUTION_KIND_CAPITAL_GAIN,
        "income" => NW_FUND_DISTRIBUTION_KIND_INCOME,
        _ => NW_FUND_DISTRIBUTION_KIND_UNKNOWN,
    }
}
/// `FundType`: a value this version does not know.
pub const NW_FUND_TYPE_UNKNOWN: i32 = 0;
/// `FundType`: `etf`.
pub const NW_FUND_TYPE_ETF: i32 = 1;
/// `FundType`: `mutual_fund`.
pub const NW_FUND_TYPE_MUTUAL_FUND: i32 = 2;
fn fund_type_code(value: &str) -> i32 {
    match value {
        "etf" => NW_FUND_TYPE_ETF,
        "mutual_fund" => NW_FUND_TYPE_MUTUAL_FUND,
        _ => NW_FUND_TYPE_UNKNOWN,
    }
}
/// `HolidayScope`: a value this version does not know.
pub const NW_HOLIDAY_SCOPE_UNKNOWN: i32 = 0;
/// `HolidayScope`: `bank`.
pub const NW_HOLIDAY_SCOPE_BANK: i32 = 1;
/// `HolidayScope`: `central_government`.
pub const NW_HOLIDAY_SCOPE_CENTRAL_GOVERNMENT: i32 = 2;
/// `HolidayScope`: `collective_leave`.
pub const NW_HOLIDAY_SCOPE_COLLECTIVE_LEAVE: i32 = 3;
/// `HolidayScope`: `federal_government`.
pub const NW_HOLIDAY_SCOPE_FEDERAL_GOVERNMENT: i32 = 4;
/// `HolidayScope`: `national_public`.
pub const NW_HOLIDAY_SCOPE_NATIONAL_PUBLIC: i32 = 5;
/// `HolidayScope`: `private_sector`.
pub const NW_HOLIDAY_SCOPE_PRIVATE_SECTOR: i32 = 6;
/// `HolidayScope`: `public_sector`.
pub const NW_HOLIDAY_SCOPE_PUBLIC_SECTOR: i32 = 7;
/// `HolidayScope`: `regional_public`.
pub const NW_HOLIDAY_SCOPE_REGIONAL_PUBLIC: i32 = 8;
fn holiday_scope_code(value: &str) -> i32 {
    match value {
        "bank" => NW_HOLIDAY_SCOPE_BANK,
        "central_government" => NW_HOLIDAY_SCOPE_CENTRAL_GOVERNMENT,
        "collective_leave" => NW_HOLIDAY_SCOPE_COLLECTIVE_LEAVE,
        "federal_government" => NW_HOLIDAY_SCOPE_FEDERAL_GOVERNMENT,
        "national_public" => NW_HOLIDAY_SCOPE_NATIONAL_PUBLIC,
        "private_sector" => NW_HOLIDAY_SCOPE_PRIVATE_SECTOR,
        "public_sector" => NW_HOLIDAY_SCOPE_PUBLIC_SECTOR,
        "regional_public" => NW_HOLIDAY_SCOPE_REGIONAL_PUBLIC,
        _ => NW_HOLIDAY_SCOPE_UNKNOWN,
    }
}
/// `IndicatorUnit`: a value this version does not know.
pub const NW_INDICATOR_UNIT_UNKNOWN: i32 = 0;
/// `IndicatorUnit`: `balance`.
pub const NW_INDICATOR_UNIT_BALANCE: i32 = 1;
/// `IndicatorUnit`: `currency`.
pub const NW_INDICATOR_UNIT_CURRENCY: i32 = 2;
/// `IndicatorUnit`: `index`.
pub const NW_INDICATOR_UNIT_INDEX: i32 = 3;
/// `IndicatorUnit`: `percent`.
pub const NW_INDICATOR_UNIT_PERCENT: i32 = 4;
/// `IndicatorUnit`: `ratio`.
pub const NW_INDICATOR_UNIT_RATIO: i32 = 5;
/// `IndicatorUnit`: `thousands`.
pub const NW_INDICATOR_UNIT_THOUSANDS: i32 = 6;
fn indicator_unit_code(value: &str) -> i32 {
    match value {
        "balance" => NW_INDICATOR_UNIT_BALANCE,
        "currency" => NW_INDICATOR_UNIT_CURRENCY,
        "index" => NW_INDICATOR_UNIT_INDEX,
        "percent" => NW_INDICATOR_UNIT_PERCENT,
        "ratio" => NW_INDICATOR_UNIT_RATIO,
        "thousands" => NW_INDICATOR_UNIT_THOUSANDS,
        _ => NW_INDICATOR_UNIT_UNKNOWN,
    }
}
/// `MarketSession`: a value this version does not know.
pub const NW_MARKET_SESSION_UNKNOWN: i32 = 0;
/// `MarketSession`: `interbank_closed`.
pub const NW_MARKET_SESSION_INTERBANK_CLOSED: i32 = 1;
/// `MarketSession`: `open`.
pub const NW_MARKET_SESSION_OPEN: i32 = 2;
/// `MarketSession`: `weekend`.
pub const NW_MARKET_SESSION_WEEKEND: i32 = 3;
fn market_session_code(value: &str) -> i32 {
    match value {
        "interbank_closed" => NW_MARKET_SESSION_INTERBANK_CLOSED,
        "open" => NW_MARKET_SESSION_OPEN,
        "weekend" => NW_MARKET_SESSION_WEEKEND,
        _ => NW_MARKET_SESSION_UNKNOWN,
    }
}
/// `MoversDirection`: a value this version does not know.
pub const NW_MOVERS_DIRECTION_UNKNOWN: i32 = 0;
/// `MoversDirection`: `active`.
pub const NW_MOVERS_DIRECTION_ACTIVE: i32 = 1;
/// `MoversDirection`: `gainers`.
pub const NW_MOVERS_DIRECTION_GAINERS: i32 = 2;
/// `MoversDirection`: `losers`.
pub const NW_MOVERS_DIRECTION_LOSERS: i32 = 3;
fn movers_direction_code(value: &str) -> i32 {
    match value {
        "active" => NW_MOVERS_DIRECTION_ACTIVE,
        "gainers" => NW_MOVERS_DIRECTION_GAINERS,
        "losers" => NW_MOVERS_DIRECTION_LOSERS,
        _ => NW_MOVERS_DIRECTION_UNKNOWN,
    }
}
/// `PalmoilFrequency`: a value this version does not know.
pub const NW_PALMOIL_FREQUENCY_UNKNOWN: i32 = 0;
/// `PalmoilFrequency`: `daily`.
pub const NW_PALMOIL_FREQUENCY_DAILY: i32 = 1;
/// `PalmoilFrequency`: `fortnightly`.
pub const NW_PALMOIL_FREQUENCY_FORTNIGHTLY: i32 = 2;
/// `PalmoilFrequency`: `irregular`.
pub const NW_PALMOIL_FREQUENCY_IRREGULAR: i32 = 3;
/// `PalmoilFrequency`: `monthly`.
pub const NW_PALMOIL_FREQUENCY_MONTHLY: i32 = 4;
/// `PalmoilFrequency`: `weekly`.
pub const NW_PALMOIL_FREQUENCY_WEEKLY: i32 = 5;
fn palmoil_frequency_code(value: &str) -> i32 {
    match value {
        "daily" => NW_PALMOIL_FREQUENCY_DAILY,
        "fortnightly" => NW_PALMOIL_FREQUENCY_FORTNIGHTLY,
        "irregular" => NW_PALMOIL_FREQUENCY_IRREGULAR,
        "monthly" => NW_PALMOIL_FREQUENCY_MONTHLY,
        "weekly" => NW_PALMOIL_FREQUENCY_WEEKLY,
        _ => NW_PALMOIL_FREQUENCY_UNKNOWN,
    }
}
/// `PalmoilPeriodType`: a value this version does not know.
pub const NW_PALMOIL_PERIOD_TYPE_UNKNOWN: i32 = 0;
/// `PalmoilPeriodType`: `effective`.
pub const NW_PALMOIL_PERIOD_TYPE_EFFECTIVE: i32 = 1;
/// `PalmoilPeriodType`: `observation`.
pub const NW_PALMOIL_PERIOD_TYPE_OBSERVATION: i32 = 2;
fn palmoil_period_type_code(value: &str) -> i32 {
    match value {
        "effective" => NW_PALMOIL_PERIOD_TYPE_EFFECTIVE,
        "observation" => NW_PALMOIL_PERIOD_TYPE_OBSERVATION,
        _ => NW_PALMOIL_PERIOD_TYPE_UNKNOWN,
    }
}
/// `PalmoilProduct`: a value this version does not know.
pub const NW_PALMOIL_PRODUCT_UNKNOWN: i32 = 0;
/// `PalmoilProduct`: `cpo`.
pub const NW_PALMOIL_PRODUCT_CPO: i32 = 1;
/// `PalmoilProduct`: `ffb`.
pub const NW_PALMOIL_PRODUCT_FFB: i32 = 2;
fn palmoil_product_code(value: &str) -> i32 {
    match value {
        "cpo" => NW_PALMOIL_PRODUCT_CPO,
        "ffb" => NW_PALMOIL_PRODUCT_FFB,
        _ => NW_PALMOIL_PRODUCT_UNKNOWN,
    }
}
/// `PalmoilScalarPriceType`: a value this version does not know.
pub const NW_PALMOIL_SCALAR_PRICE_TYPE_UNKNOWN: i32 = 0;
/// `PalmoilScalarPriceType`: `government_reference`.
pub const NW_PALMOIL_SCALAR_PRICE_TYPE_GOVERNMENT_REFERENCE: i32 = 1;
/// `PalmoilScalarPriceType`: `market_average`.
pub const NW_PALMOIL_SCALAR_PRICE_TYPE_MARKET_AVERAGE: i32 = 2;
fn palmoil_scalar_price_type_code(value: &str) -> i32 {
    match value {
        "government_reference" => NW_PALMOIL_SCALAR_PRICE_TYPE_GOVERNMENT_REFERENCE,
        "market_average" => NW_PALMOIL_SCALAR_PRICE_TYPE_MARKET_AVERAGE,
        _ => NW_PALMOIL_SCALAR_PRICE_TYPE_UNKNOWN,
    }
}
/// `PalmoilScalarUnit`: a value this version does not know.
pub const NW_PALMOIL_SCALAR_UNIT_UNKNOWN: i32 = 0;
/// `PalmoilScalarUnit`: `metric_ton`.
pub const NW_PALMOIL_SCALAR_UNIT_METRIC_TON: i32 = 1;
/// `PalmoilScalarUnit`: `one_percent_oer`.
pub const NW_PALMOIL_SCALAR_UNIT_ONE_PERCENT_OER: i32 = 2;
fn palmoil_scalar_unit_code(value: &str) -> i32 {
    match value {
        "metric_ton" => NW_PALMOIL_SCALAR_UNIT_METRIC_TON,
        "one_percent_oer" => NW_PALMOIL_SCALAR_UNIT_ONE_PERCENT_OER,
        _ => NW_PALMOIL_SCALAR_UNIT_UNKNOWN,
    }
}
/// `PhysicalCategory`: a value this version does not know.
pub const NW_PHYSICAL_CATEGORY_UNKNOWN: i32 = 0;
/// `PhysicalCategory`: `emissions`.
pub const NW_PHYSICAL_CATEGORY_EMISSIONS: i32 = 1;
/// `PhysicalCategory`: `energy`.
pub const NW_PHYSICAL_CATEGORY_ENERGY: i32 = 2;
/// `PhysicalCategory`: `fertilizers`.
pub const NW_PHYSICAL_CATEGORY_FERTILIZERS: i32 = 3;
/// `PhysicalCategory`: `forestry`.
pub const NW_PHYSICAL_CATEGORY_FORESTRY: i32 = 4;
/// `PhysicalCategory`: `grains`.
pub const NW_PHYSICAL_CATEGORY_GRAINS: i32 = 5;
/// `PhysicalCategory`: `livestock-dairy`.
pub const NW_PHYSICAL_CATEGORY_LIVESTOCK_DAIRY: i32 = 6;
/// `PhysicalCategory`: `metals`.
pub const NW_PHYSICAL_CATEGORY_METALS: i32 = 7;
/// `PhysicalCategory`: `oils-oilseeds`.
pub const NW_PHYSICAL_CATEGORY_OILS_OILSEEDS: i32 = 8;
/// `PhysicalCategory`: `softs`.
pub const NW_PHYSICAL_CATEGORY_SOFTS: i32 = 9;
fn physical_category_code(value: &str) -> i32 {
    match value {
        "emissions" => NW_PHYSICAL_CATEGORY_EMISSIONS,
        "energy" => NW_PHYSICAL_CATEGORY_ENERGY,
        "fertilizers" => NW_PHYSICAL_CATEGORY_FERTILIZERS,
        "forestry" => NW_PHYSICAL_CATEGORY_FORESTRY,
        "grains" => NW_PHYSICAL_CATEGORY_GRAINS,
        "livestock-dairy" => NW_PHYSICAL_CATEGORY_LIVESTOCK_DAIRY,
        "metals" => NW_PHYSICAL_CATEGORY_METALS,
        "oils-oilseeds" => NW_PHYSICAL_CATEGORY_OILS_OILSEEDS,
        "softs" => NW_PHYSICAL_CATEGORY_SOFTS,
        _ => NW_PHYSICAL_CATEGORY_UNKNOWN,
    }
}
/// `PhysicalFrequency`: a value this version does not know.
pub const NW_PHYSICAL_FREQUENCY_UNKNOWN: i32 = 0;
/// `PhysicalFrequency`: `annual`.
pub const NW_PHYSICAL_FREQUENCY_ANNUAL: i32 = 1;
/// `PhysicalFrequency`: `daily`.
pub const NW_PHYSICAL_FREQUENCY_DAILY: i32 = 2;
/// `PhysicalFrequency`: `intraday`.
pub const NW_PHYSICAL_FREQUENCY_INTRADAY: i32 = 3;
/// `PhysicalFrequency`: `irregular`.
pub const NW_PHYSICAL_FREQUENCY_IRREGULAR: i32 = 4;
/// `PhysicalFrequency`: `monthly`.
pub const NW_PHYSICAL_FREQUENCY_MONTHLY: i32 = 5;
/// `PhysicalFrequency`: `quarterly`.
pub const NW_PHYSICAL_FREQUENCY_QUARTERLY: i32 = 6;
/// `PhysicalFrequency`: `semiannual`.
pub const NW_PHYSICAL_FREQUENCY_SEMIANNUAL: i32 = 7;
/// `PhysicalFrequency`: `three_times_monthly`.
pub const NW_PHYSICAL_FREQUENCY_THREE_TIMES_MONTHLY: i32 = 8;
/// `PhysicalFrequency`: `twice_monthly`.
pub const NW_PHYSICAL_FREQUENCY_TWICE_MONTHLY: i32 = 9;
/// `PhysicalFrequency`: `weekly`.
pub const NW_PHYSICAL_FREQUENCY_WEEKLY: i32 = 10;
fn physical_frequency_code(value: &str) -> i32 {
    match value {
        "annual" => NW_PHYSICAL_FREQUENCY_ANNUAL,
        "daily" => NW_PHYSICAL_FREQUENCY_DAILY,
        "intraday" => NW_PHYSICAL_FREQUENCY_INTRADAY,
        "irregular" => NW_PHYSICAL_FREQUENCY_IRREGULAR,
        "monthly" => NW_PHYSICAL_FREQUENCY_MONTHLY,
        "quarterly" => NW_PHYSICAL_FREQUENCY_QUARTERLY,
        "semiannual" => NW_PHYSICAL_FREQUENCY_SEMIANNUAL,
        "three_times_monthly" => NW_PHYSICAL_FREQUENCY_THREE_TIMES_MONTHLY,
        "twice_monthly" => NW_PHYSICAL_FREQUENCY_TWICE_MONTHLY,
        "weekly" => NW_PHYSICAL_FREQUENCY_WEEKLY,
        _ => NW_PHYSICAL_FREQUENCY_UNKNOWN,
    }
}
/// `PhysicalPriceType`: a value this version does not know.
pub const NW_PHYSICAL_PRICE_TYPE_UNKNOWN: i32 = 0;
/// `PhysicalPriceType`: `auction_clearing`.
pub const NW_PHYSICAL_PRICE_TYPE_AUCTION_CLEARING: i32 = 1;
/// `PhysicalPriceType`: `export_unit_value`.
pub const NW_PHYSICAL_PRICE_TYPE_EXPORT_UNIT_VALUE: i32 = 2;
/// `PhysicalPriceType`: `farmgate_average`.
pub const NW_PHYSICAL_PRICE_TYPE_FARMGATE_AVERAGE: i32 = 3;
/// `PhysicalPriceType`: `government_reference`.
pub const NW_PHYSICAL_PRICE_TYPE_GOVERNMENT_REFERENCE: i32 = 4;
/// `PhysicalPriceType`: `import_unit_value`.
pub const NW_PHYSICAL_PRICE_TYPE_IMPORT_UNIT_VALUE: i32 = 5;
/// `PhysicalPriceType`: `international_benchmark`.
pub const NW_PHYSICAL_PRICE_TYPE_INTERNATIONAL_BENCHMARK: i32 = 6;
/// `PhysicalPriceType`: `market_average`.
pub const NW_PHYSICAL_PRICE_TYPE_MARKET_AVERAGE: i32 = 7;
/// `PhysicalPriceType`: `official_buy`.
pub const NW_PHYSICAL_PRICE_TYPE_OFFICIAL_BUY: i32 = 8;
/// `PhysicalPriceType`: `official_reference_rate`.
pub const NW_PHYSICAL_PRICE_TYPE_OFFICIAL_REFERENCE_RATE: i32 = 9;
/// `PhysicalPriceType`: `official_sell`.
pub const NW_PHYSICAL_PRICE_TYPE_OFFICIAL_SELL: i32 = 10;
/// `PhysicalPriceType`: `regulated_average`.
pub const NW_PHYSICAL_PRICE_TYPE_REGULATED_AVERAGE: i32 = 11;
/// `PhysicalPriceType`: `retail_average`.
pub const NW_PHYSICAL_PRICE_TYPE_RETAIL_AVERAGE: i32 = 12;
/// `PhysicalPriceType`: `retail_buy`.
pub const NW_PHYSICAL_PRICE_TYPE_RETAIL_BUY: i32 = 13;
/// `PhysicalPriceType`: `retail_sell`.
pub const NW_PHYSICAL_PRICE_TYPE_RETAIL_SELL: i32 = 14;
/// `PhysicalPriceType`: `wholesale_average`.
pub const NW_PHYSICAL_PRICE_TYPE_WHOLESALE_AVERAGE: i32 = 15;
fn physical_price_type_code(value: &str) -> i32 {
    match value {
        "auction_clearing" => NW_PHYSICAL_PRICE_TYPE_AUCTION_CLEARING,
        "export_unit_value" => NW_PHYSICAL_PRICE_TYPE_EXPORT_UNIT_VALUE,
        "farmgate_average" => NW_PHYSICAL_PRICE_TYPE_FARMGATE_AVERAGE,
        "government_reference" => NW_PHYSICAL_PRICE_TYPE_GOVERNMENT_REFERENCE,
        "import_unit_value" => NW_PHYSICAL_PRICE_TYPE_IMPORT_UNIT_VALUE,
        "international_benchmark" => NW_PHYSICAL_PRICE_TYPE_INTERNATIONAL_BENCHMARK,
        "market_average" => NW_PHYSICAL_PRICE_TYPE_MARKET_AVERAGE,
        "official_buy" => NW_PHYSICAL_PRICE_TYPE_OFFICIAL_BUY,
        "official_reference_rate" => NW_PHYSICAL_PRICE_TYPE_OFFICIAL_REFERENCE_RATE,
        "official_sell" => NW_PHYSICAL_PRICE_TYPE_OFFICIAL_SELL,
        "regulated_average" => NW_PHYSICAL_PRICE_TYPE_REGULATED_AVERAGE,
        "retail_average" => NW_PHYSICAL_PRICE_TYPE_RETAIL_AVERAGE,
        "retail_buy" => NW_PHYSICAL_PRICE_TYPE_RETAIL_BUY,
        "retail_sell" => NW_PHYSICAL_PRICE_TYPE_RETAIL_SELL,
        "wholesale_average" => NW_PHYSICAL_PRICE_TYPE_WHOLESALE_AVERAGE,
        _ => NW_PHYSICAL_PRICE_TYPE_UNKNOWN,
    }
}
/// `PointPolicyRateQualifier`: a value this version does not know.
pub const NW_POINT_POLICY_RATE_QUALIFIER_UNKNOWN: i32 = 0;
/// `PointPolicyRateQualifier`: `around`.
pub const NW_POINT_POLICY_RATE_QUALIFIER_AROUND: i32 = 1;
/// `PointPolicyRateQualifier`: `exact`.
pub const NW_POINT_POLICY_RATE_QUALIFIER_EXACT: i32 = 2;
fn point_policy_rate_qualifier_code(value: &str) -> i32 {
    match value {
        "around" => NW_POINT_POLICY_RATE_QUALIFIER_AROUND,
        "exact" => NW_POINT_POLICY_RATE_QUALIFIER_EXACT,
        _ => NW_POINT_POLICY_RATE_QUALIFIER_UNKNOWN,
    }
}
/// `PredictionEventStatus`: a value this version does not know.
pub const NW_PREDICTION_EVENT_STATUS_UNKNOWN: i32 = 0;
/// `PredictionEventStatus`: `closed`.
pub const NW_PREDICTION_EVENT_STATUS_CLOSED: i32 = 1;
/// `PredictionEventStatus`: `settled`.
pub const NW_PREDICTION_EVENT_STATUS_SETTLED: i32 = 2;
/// `PredictionEventStatus`: `trading`.
pub const NW_PREDICTION_EVENT_STATUS_TRADING: i32 = 3;
/// `PredictionEventStatus`: `upcoming`.
pub const NW_PREDICTION_EVENT_STATUS_UPCOMING: i32 = 4;
/// `PredictionEventStatus`: `void`.
pub const NW_PREDICTION_EVENT_STATUS_VOID: i32 = 5;
fn prediction_event_status_code(value: &str) -> i32 {
    match value {
        "closed" => NW_PREDICTION_EVENT_STATUS_CLOSED,
        "settled" => NW_PREDICTION_EVENT_STATUS_SETTLED,
        "trading" => NW_PREDICTION_EVENT_STATUS_TRADING,
        "upcoming" => NW_PREDICTION_EVENT_STATUS_UPCOMING,
        "void" => NW_PREDICTION_EVENT_STATUS_VOID,
        _ => NW_PREDICTION_EVENT_STATUS_UNKNOWN,
    }
}
/// `PredictionShape`: a value this version does not know.
pub const NW_PREDICTION_SHAPE_UNKNOWN: i32 = 0;
/// `PredictionShape`: `binary`.
pub const NW_PREDICTION_SHAPE_BINARY: i32 = 1;
/// `PredictionShape`: `categorical`.
pub const NW_PREDICTION_SHAPE_CATEGORICAL: i32 = 2;
/// `PredictionShape`: `extreme`.
pub const NW_PREDICTION_SHAPE_EXTREME: i32 = 3;
/// `PredictionShape`: `scalar`.
pub const NW_PREDICTION_SHAPE_SCALAR: i32 = 4;
/// `PredictionShape`: `set`.
pub const NW_PREDICTION_SHAPE_SET: i32 = 5;
/// `PredictionShape`: `timing`.
pub const NW_PREDICTION_SHAPE_TIMING: i32 = 6;
fn prediction_shape_code(value: &str) -> i32 {
    match value {
        "binary" => NW_PREDICTION_SHAPE_BINARY,
        "categorical" => NW_PREDICTION_SHAPE_CATEGORICAL,
        "extreme" => NW_PREDICTION_SHAPE_EXTREME,
        "scalar" => NW_PREDICTION_SHAPE_SCALAR,
        "set" => NW_PREDICTION_SHAPE_SET,
        "timing" => NW_PREDICTION_SHAPE_TIMING,
        _ => NW_PREDICTION_SHAPE_UNKNOWN,
    }
}
/// `PredictionVenueSettlement`: a value this version does not know.
pub const NW_PREDICTION_VENUE_SETTLEMENT_UNKNOWN: i32 = 0;
/// `PredictionVenueSettlement`: `settled`.
pub const NW_PREDICTION_VENUE_SETTLEMENT_SETTLED: i32 = 1;
/// `PredictionVenueSettlement`: `void`.
pub const NW_PREDICTION_VENUE_SETTLEMENT_VOID: i32 = 2;
fn prediction_venue_settlement_code(value: &str) -> i32 {
    match value {
        "settled" => NW_PREDICTION_VENUE_SETTLEMENT_SETTLED,
        "void" => NW_PREDICTION_VENUE_SETTLEMENT_VOID,
        _ => NW_PREDICTION_VENUE_SETTLEMENT_UNKNOWN,
    }
}
/// `PredictionWebhookStatus`: a value this version does not know.
pub const NW_PREDICTION_WEBHOOK_STATUS_UNKNOWN: i32 = 0;
/// `PredictionWebhookStatus`: `closed`.
pub const NW_PREDICTION_WEBHOOK_STATUS_CLOSED: i32 = 1;
/// `PredictionWebhookStatus`: `settled`.
pub const NW_PREDICTION_WEBHOOK_STATUS_SETTLED: i32 = 2;
/// `PredictionWebhookStatus`: `void`.
pub const NW_PREDICTION_WEBHOOK_STATUS_VOID: i32 = 3;
fn prediction_webhook_status_code(value: &str) -> i32 {
    match value {
        "closed" => NW_PREDICTION_WEBHOOK_STATUS_CLOSED,
        "settled" => NW_PREDICTION_WEBHOOK_STATUS_SETTLED,
        "void" => NW_PREDICTION_WEBHOOK_STATUS_VOID,
        _ => NW_PREDICTION_WEBHOOK_STATUS_UNKNOWN,
    }
}
/// `ProducerArrangement`: a value this version does not know.
pub const NW_PRODUCER_ARRANGEMENT_UNKNOWN: i32 = 0;
/// `ProducerArrangement`: `purchase_agreement`.
pub const NW_PRODUCER_ARRANGEMENT_PURCHASE_AGREEMENT: i32 = 1;
/// `ProducerArrangement`: `supported_scheme`.
pub const NW_PRODUCER_ARRANGEMENT_SUPPORTED_SCHEME: i32 = 2;
fn producer_arrangement_code(value: &str) -> i32 {
    match value {
        "purchase_agreement" => NW_PRODUCER_ARRANGEMENT_PURCHASE_AGREEMENT,
        "supported_scheme" => NW_PRODUCER_ARRANGEMENT_SUPPORTED_SCHEME,
        _ => NW_PRODUCER_ARRANGEMENT_UNKNOWN,
    }
}
/// `Registry`: a value this version does not know.
pub const NW_REGISTRY_UNKNOWN: i32 = 0;
/// `Registry`: `jpn`.
pub const NW_REGISTRY_JPN: i32 = 1;
/// `Registry`: `kor`.
pub const NW_REGISTRY_KOR: i32 = 2;
/// `Registry`: `usa`.
pub const NW_REGISTRY_USA: i32 = 3;
/// `Registry`: `aut`.
pub const NW_REGISTRY_AUT: i32 = 4;
/// `Registry`: `bel`.
pub const NW_REGISTRY_BEL: i32 = 5;
/// `Registry`: `cyp`.
pub const NW_REGISTRY_CYP: i32 = 6;
/// `Registry`: `cze`.
pub const NW_REGISTRY_CZE: i32 = 7;
/// `Registry`: `dnk`.
pub const NW_REGISTRY_DNK: i32 = 8;
/// `Registry`: `esp`.
pub const NW_REGISTRY_ESP: i32 = 9;
/// `Registry`: `est`.
pub const NW_REGISTRY_EST: i32 = 10;
/// `Registry`: `fin`.
pub const NW_REGISTRY_FIN: i32 = 11;
/// `Registry`: `fra`.
pub const NW_REGISTRY_FRA: i32 = 12;
/// `Registry`: `gbr`.
pub const NW_REGISTRY_GBR: i32 = 13;
/// `Registry`: `grc`.
pub const NW_REGISTRY_GRC: i32 = 14;
/// `Registry`: `hrv`.
pub const NW_REGISTRY_HRV: i32 = 15;
/// `Registry`: `hun`.
pub const NW_REGISTRY_HUN: i32 = 16;
/// `Registry`: `isl`.
pub const NW_REGISTRY_ISL: i32 = 17;
/// `Registry`: `ita`.
pub const NW_REGISTRY_ITA: i32 = 18;
/// `Registry`: `ltu`.
pub const NW_REGISTRY_LTU: i32 = 19;
/// `Registry`: `lux`.
pub const NW_REGISTRY_LUX: i32 = 20;
/// `Registry`: `lva`.
pub const NW_REGISTRY_LVA: i32 = 21;
/// `Registry`: `mlt`.
pub const NW_REGISTRY_MLT: i32 = 22;
/// `Registry`: `nld`.
pub const NW_REGISTRY_NLD: i32 = 23;
/// `Registry`: `nor`.
pub const NW_REGISTRY_NOR: i32 = 24;
/// `Registry`: `pol`.
pub const NW_REGISTRY_POL: i32 = 25;
/// `Registry`: `prt`.
pub const NW_REGISTRY_PRT: i32 = 26;
/// `Registry`: `rou`.
pub const NW_REGISTRY_ROU: i32 = 27;
/// `Registry`: `svk`.
pub const NW_REGISTRY_SVK: i32 = 28;
/// `Registry`: `svn`.
pub const NW_REGISTRY_SVN: i32 = 29;
/// `Registry`: `swe`.
pub const NW_REGISTRY_SWE: i32 = 30;
/// `Registry`: `twn`.
pub const NW_REGISTRY_TWN: i32 = 31;
fn registry_code(value: &str) -> i32 {
    match value {
        "jpn" => NW_REGISTRY_JPN,
        "kor" => NW_REGISTRY_KOR,
        "usa" => NW_REGISTRY_USA,
        "aut" => NW_REGISTRY_AUT,
        "bel" => NW_REGISTRY_BEL,
        "cyp" => NW_REGISTRY_CYP,
        "cze" => NW_REGISTRY_CZE,
        "dnk" => NW_REGISTRY_DNK,
        "esp" => NW_REGISTRY_ESP,
        "est" => NW_REGISTRY_EST,
        "fin" => NW_REGISTRY_FIN,
        "fra" => NW_REGISTRY_FRA,
        "gbr" => NW_REGISTRY_GBR,
        "grc" => NW_REGISTRY_GRC,
        "hrv" => NW_REGISTRY_HRV,
        "hun" => NW_REGISTRY_HUN,
        "isl" => NW_REGISTRY_ISL,
        "ita" => NW_REGISTRY_ITA,
        "ltu" => NW_REGISTRY_LTU,
        "lux" => NW_REGISTRY_LUX,
        "lva" => NW_REGISTRY_LVA,
        "mlt" => NW_REGISTRY_MLT,
        "nld" => NW_REGISTRY_NLD,
        "nor" => NW_REGISTRY_NOR,
        "pol" => NW_REGISTRY_POL,
        "prt" => NW_REGISTRY_PRT,
        "rou" => NW_REGISTRY_ROU,
        "svk" => NW_REGISTRY_SVK,
        "svn" => NW_REGISTRY_SVN,
        "swe" => NW_REGISTRY_SWE,
        "twn" => NW_REGISTRY_TWN,
        _ => NW_REGISTRY_UNKNOWN,
    }
}
/// `ReleaseStage`: a value this version does not know.
pub const NW_RELEASE_STAGE_UNKNOWN: i32 = 0;
/// `ReleaseStage`: `advance`.
pub const NW_RELEASE_STAGE_ADVANCE: i32 = 1;
/// `ReleaseStage`: `final`.
pub const NW_RELEASE_STAGE_FINAL: i32 = 2;
/// `ReleaseStage`: `first`.
pub const NW_RELEASE_STAGE_FIRST: i32 = 3;
/// `ReleaseStage`: `flash`.
pub const NW_RELEASE_STAGE_FLASH: i32 = 4;
/// `ReleaseStage`: `preliminary`.
pub const NW_RELEASE_STAGE_PRELIMINARY: i32 = 5;
/// `ReleaseStage`: `second`.
pub const NW_RELEASE_STAGE_SECOND: i32 = 6;
/// `ReleaseStage`: `third`.
pub const NW_RELEASE_STAGE_THIRD: i32 = 7;
fn release_stage_code(value: &str) -> i32 {
    match value {
        "advance" => NW_RELEASE_STAGE_ADVANCE,
        "final" => NW_RELEASE_STAGE_FINAL,
        "first" => NW_RELEASE_STAGE_FIRST,
        "flash" => NW_RELEASE_STAGE_FLASH,
        "preliminary" => NW_RELEASE_STAGE_PRELIMINARY,
        "second" => NW_RELEASE_STAGE_SECOND,
        "third" => NW_RELEASE_STAGE_THIRD,
        _ => NW_RELEASE_STAGE_UNKNOWN,
    }
}
/// `ReleaseStatus`: a value this version does not know.
pub const NW_RELEASE_STATUS_UNKNOWN: i32 = 0;
/// `ReleaseStatus`: `cancelled`.
pub const NW_RELEASE_STATUS_CANCELLED: i32 = 1;
/// `ReleaseStatus`: `published`.
pub const NW_RELEASE_STATUS_PUBLISHED: i32 = 2;
/// `ReleaseStatus`: `scheduled`.
pub const NW_RELEASE_STATUS_SCHEDULED: i32 = 3;
fn release_status_code(value: &str) -> i32 {
    match value {
        "cancelled" => NW_RELEASE_STATUS_CANCELLED,
        "published" => NW_RELEASE_STATUS_PUBLISHED,
        "scheduled" => NW_RELEASE_STATUS_SCHEDULED,
        _ => NW_RELEASE_STATUS_UNKNOWN,
    }
}
/// `ReleaseTopic`: a value this version does not know.
pub const NW_RELEASE_TOPIC_UNKNOWN: i32 = 0;
/// `ReleaseTopic`: `balance_of_payments`.
pub const NW_RELEASE_TOPIC_BALANCE_OF_PAYMENTS: i32 = 1;
/// `ReleaseTopic`: `business_confidence`.
pub const NW_RELEASE_TOPIC_BUSINESS_CONFIDENCE: i32 = 2;
/// `ReleaseTopic`: `construction`.
pub const NW_RELEASE_TOPIC_CONSTRUCTION: i32 = 3;
/// `ReleaseTopic`: `consumer_confidence`.
pub const NW_RELEASE_TOPIC_CONSUMER_CONFIDENCE: i32 = 4;
/// `ReleaseTopic`: `consumer_prices`.
pub const NW_RELEASE_TOPIC_CONSUMER_PRICES: i32 = 5;
/// `ReleaseTopic`: `employment`.
pub const NW_RELEASE_TOPIC_EMPLOYMENT: i32 = 6;
/// `ReleaseTopic`: `energy`.
pub const NW_RELEASE_TOPIC_ENERGY: i32 = 7;
/// `ReleaseTopic`: `gdp`.
pub const NW_RELEASE_TOPIC_GDP: i32 = 8;
/// `ReleaseTopic`: `home_sales`.
pub const NW_RELEASE_TOPIC_HOME_SALES: i32 = 9;
/// `ReleaseTopic`: `house_prices`.
pub const NW_RELEASE_TOPIC_HOUSE_PRICES: i32 = 10;
/// `ReleaseTopic`: `household_spending`.
pub const NW_RELEASE_TOPIC_HOUSEHOLD_SPENDING: i32 = 11;
/// `ReleaseTopic`: `housing_construction`.
pub const NW_RELEASE_TOPIC_HOUSING_CONSTRUCTION: i32 = 12;
/// `ReleaseTopic`: `import_export_prices`.
pub const NW_RELEASE_TOPIC_IMPORT_EXPORT_PRICES: i32 = 13;
/// `ReleaseTopic`: `industrial_production`.
pub const NW_RELEASE_TOPIC_INDUSTRIAL_PRODUCTION: i32 = 14;
/// `ReleaseTopic`: `investment`.
pub const NW_RELEASE_TOPIC_INVESTMENT: i32 = 15;
/// `ReleaseTopic`: `jobless_claims`.
pub const NW_RELEASE_TOPIC_JOBLESS_CLAIMS: i32 = 16;
/// `ReleaseTopic`: `labour_force`.
pub const NW_RELEASE_TOPIC_LABOUR_FORCE: i32 = 17;
/// `ReleaseTopic`: `manufacturing`.
pub const NW_RELEASE_TOPIC_MANUFACTURING: i32 = 18;
/// `ReleaseTopic`: `merchandise_trade`.
pub const NW_RELEASE_TOPIC_MERCHANDISE_TRADE: i32 = 19;
/// `ReleaseTopic`: `money_credit`.
pub const NW_RELEASE_TOPIC_MONEY_CREDIT: i32 = 20;
/// `ReleaseTopic`: `other`.
pub const NW_RELEASE_TOPIC_OTHER: i32 = 21;
/// `ReleaseTopic`: `policy_communication`.
pub const NW_RELEASE_TOPIC_POLICY_COMMUNICATION: i32 = 22;
/// `ReleaseTopic`: `policy_minutes`.
pub const NW_RELEASE_TOPIC_POLICY_MINUTES: i32 = 23;
/// `ReleaseTopic`: `policy_rate`.
pub const NW_RELEASE_TOPIC_POLICY_RATE: i32 = 24;
/// `ReleaseTopic`: `producer_prices`.
pub const NW_RELEASE_TOPIC_PRODUCER_PRICES: i32 = 25;
/// `ReleaseTopic`: `public_finance`.
pub const NW_RELEASE_TOPIC_PUBLIC_FINANCE: i32 = 26;
/// `ReleaseTopic`: `retail_sales`.
pub const NW_RELEASE_TOPIC_RETAIL_SALES: i32 = 27;
/// `ReleaseTopic`: `services`.
pub const NW_RELEASE_TOPIC_SERVICES: i32 = 28;
/// `ReleaseTopic`: `tourism`.
pub const NW_RELEASE_TOPIC_TOURISM: i32 = 29;
/// `ReleaseTopic`: `wages`.
pub const NW_RELEASE_TOPIC_WAGES: i32 = 30;
/// `ReleaseTopic`: `wholesale_trade`.
pub const NW_RELEASE_TOPIC_WHOLESALE_TRADE: i32 = 31;
fn release_topic_code(value: &str) -> i32 {
    match value {
        "balance_of_payments" => NW_RELEASE_TOPIC_BALANCE_OF_PAYMENTS,
        "business_confidence" => NW_RELEASE_TOPIC_BUSINESS_CONFIDENCE,
        "construction" => NW_RELEASE_TOPIC_CONSTRUCTION,
        "consumer_confidence" => NW_RELEASE_TOPIC_CONSUMER_CONFIDENCE,
        "consumer_prices" => NW_RELEASE_TOPIC_CONSUMER_PRICES,
        "employment" => NW_RELEASE_TOPIC_EMPLOYMENT,
        "energy" => NW_RELEASE_TOPIC_ENERGY,
        "gdp" => NW_RELEASE_TOPIC_GDP,
        "home_sales" => NW_RELEASE_TOPIC_HOME_SALES,
        "house_prices" => NW_RELEASE_TOPIC_HOUSE_PRICES,
        "household_spending" => NW_RELEASE_TOPIC_HOUSEHOLD_SPENDING,
        "housing_construction" => NW_RELEASE_TOPIC_HOUSING_CONSTRUCTION,
        "import_export_prices" => NW_RELEASE_TOPIC_IMPORT_EXPORT_PRICES,
        "industrial_production" => NW_RELEASE_TOPIC_INDUSTRIAL_PRODUCTION,
        "investment" => NW_RELEASE_TOPIC_INVESTMENT,
        "jobless_claims" => NW_RELEASE_TOPIC_JOBLESS_CLAIMS,
        "labour_force" => NW_RELEASE_TOPIC_LABOUR_FORCE,
        "manufacturing" => NW_RELEASE_TOPIC_MANUFACTURING,
        "merchandise_trade" => NW_RELEASE_TOPIC_MERCHANDISE_TRADE,
        "money_credit" => NW_RELEASE_TOPIC_MONEY_CREDIT,
        "other" => NW_RELEASE_TOPIC_OTHER,
        "policy_communication" => NW_RELEASE_TOPIC_POLICY_COMMUNICATION,
        "policy_minutes" => NW_RELEASE_TOPIC_POLICY_MINUTES,
        "policy_rate" => NW_RELEASE_TOPIC_POLICY_RATE,
        "producer_prices" => NW_RELEASE_TOPIC_PRODUCER_PRICES,
        "public_finance" => NW_RELEASE_TOPIC_PUBLIC_FINANCE,
        "retail_sales" => NW_RELEASE_TOPIC_RETAIL_SALES,
        "services" => NW_RELEASE_TOPIC_SERVICES,
        "tourism" => NW_RELEASE_TOPIC_TOURISM,
        "wages" => NW_RELEASE_TOPIC_WAGES,
        "wholesale_trade" => NW_RELEASE_TOPIC_WHOLESALE_TRADE,
        _ => NW_RELEASE_TOPIC_UNKNOWN,
    }
}
/// `ResolutionKind`: a value this version does not know.
pub const NW_RESOLUTION_KIND_UNKNOWN: i32 = 0;
/// `ResolutionKind`: `occurrence`.
pub const NW_RESOLUTION_KIND_OCCURRENCE: i32 = 1;
/// `ResolutionKind`: `price_fix`.
pub const NW_RESOLUTION_KIND_PRICE_FIX: i32 = 2;
/// `ResolutionKind`: `price_window`.
pub const NW_RESOLUTION_KIND_PRICE_WINDOW: i32 = 3;
/// `ResolutionKind`: `release`.
pub const NW_RESOLUTION_KIND_RELEASE: i32 = 4;
fn resolution_kind_code(value: &str) -> i32 {
    match value {
        "occurrence" => NW_RESOLUTION_KIND_OCCURRENCE,
        "price_fix" => NW_RESOLUTION_KIND_PRICE_FIX,
        "price_window" => NW_RESOLUTION_KIND_PRICE_WINDOW,
        "release" => NW_RESOLUTION_KIND_RELEASE,
        _ => NW_RESOLUTION_KIND_UNKNOWN,
    }
}
/// `ResourceObservationStatus`: a value this version does not know.
pub const NW_RESOURCE_OBSERVATION_STATUS_UNKNOWN: i32 = 0;
/// `ResourceObservationStatus`: `break_in_series`.
pub const NW_RESOURCE_OBSERVATION_STATUS_BREAK_IN_SERIES: i32 = 1;
/// `ResourceObservationStatus`: `break_in_series_and_definition_differs`.
pub const NW_RESOURCE_OBSERVATION_STATUS_BREAK_IN_SERIES_AND_DEFINITION_DIFFERS: i32 = 2;
/// `ResourceObservationStatus`: `definition_differs`.
pub const NW_RESOURCE_OBSERVATION_STATUS_DEFINITION_DIFFERS: i32 = 3;
/// `ResourceObservationStatus`: `estimated`.
pub const NW_RESOURCE_OBSERVATION_STATUS_ESTIMATED: i32 = 4;
/// `ResourceObservationStatus`: `provisional`.
pub const NW_RESOURCE_OBSERVATION_STATUS_PROVISIONAL: i32 = 5;
fn resource_observation_status_code(value: &str) -> i32 {
    match value {
        "break_in_series" => NW_RESOURCE_OBSERVATION_STATUS_BREAK_IN_SERIES,
        "break_in_series_and_definition_differs" => {
            NW_RESOURCE_OBSERVATION_STATUS_BREAK_IN_SERIES_AND_DEFINITION_DIFFERS
        }
        "definition_differs" => NW_RESOURCE_OBSERVATION_STATUS_DEFINITION_DIFFERS,
        "estimated" => NW_RESOURCE_OBSERVATION_STATUS_ESTIMATED,
        "provisional" => NW_RESOURCE_OBSERVATION_STATUS_PROVISIONAL,
        _ => NW_RESOURCE_OBSERVATION_STATUS_UNKNOWN,
    }
}
/// `ResourceUnit`: a value this version does not know.
pub const NW_RESOURCE_UNIT_UNKNOWN: i32 = 0;
/// `ResourceUnit`: `currency`.
pub const NW_RESOURCE_UNIT_CURRENCY: i32 = 1;
/// `ResourceUnit`: `currency_per_hour`.
pub const NW_RESOURCE_UNIT_CURRENCY_PER_HOUR: i32 = 2;
/// `ResourceUnit`: `currency_per_month`.
pub const NW_RESOURCE_UNIT_CURRENCY_PER_MONTH: i32 = 3;
/// `ResourceUnit`: `currency_per_week`.
pub const NW_RESOURCE_UNIT_CURRENCY_PER_WEEK: i32 = 4;
/// `ResourceUnit`: `currency_per_year`.
pub const NW_RESOURCE_UNIT_CURRENCY_PER_YEAR: i32 = 5;
/// `ResourceUnit`: `index`.
pub const NW_RESOURCE_UNIT_INDEX: i32 = 6;
/// `ResourceUnit`: `job`.
pub const NW_RESOURCE_UNIT_JOB: i32 = 7;
/// `ResourceUnit`: `percent`.
pub const NW_RESOURCE_UNIT_PERCENT: i32 = 8;
/// `ResourceUnit`: `person`.
pub const NW_RESOURCE_UNIT_PERSON: i32 = 9;
fn resource_unit_code(value: &str) -> i32 {
    match value {
        "currency" => NW_RESOURCE_UNIT_CURRENCY,
        "currency_per_hour" => NW_RESOURCE_UNIT_CURRENCY_PER_HOUR,
        "currency_per_month" => NW_RESOURCE_UNIT_CURRENCY_PER_MONTH,
        "currency_per_week" => NW_RESOURCE_UNIT_CURRENCY_PER_WEEK,
        "currency_per_year" => NW_RESOURCE_UNIT_CURRENCY_PER_YEAR,
        "index" => NW_RESOURCE_UNIT_INDEX,
        "job" => NW_RESOURCE_UNIT_JOB,
        "percent" => NW_RESOURCE_UNIT_PERCENT,
        "person" => NW_RESOURCE_UNIT_PERSON,
        _ => NW_RESOURCE_UNIT_UNKNOWN,
    }
}
/// `ScalarEventAbove`: a value this version does not know.
pub const NW_SCALAR_EVENT_ABOVE_UNKNOWN: i32 = 0;
/// `ScalarEventAbove`: `median`.
pub const NW_SCALAR_EVENT_ABOVE_MEDIAN: i32 = 1;
/// `ScalarEventAbove`: `p10`.
pub const NW_SCALAR_EVENT_ABOVE_P10: i32 = 2;
/// `ScalarEventAbove`: `p90`.
pub const NW_SCALAR_EVENT_ABOVE_P90: i32 = 3;
fn scalar_event_above_code(value: &str) -> i32 {
    match value {
        "median" => NW_SCALAR_EVENT_ABOVE_MEDIAN,
        "p10" => NW_SCALAR_EVENT_ABOVE_P10,
        "p90" => NW_SCALAR_EVENT_ABOVE_P90,
        _ => NW_SCALAR_EVENT_ABOVE_UNKNOWN,
    }
}
/// `SecurityType`: a value this version does not know.
pub const NW_SECURITY_TYPE_UNKNOWN: i32 = 0;
/// `SecurityType`: `adr`.
pub const NW_SECURITY_TYPE_ADR: i32 = 1;
/// `SecurityType`: `common`.
pub const NW_SECURITY_TYPE_COMMON: i32 = 2;
/// `SecurityType`: `other`.
pub const NW_SECURITY_TYPE_OTHER: i32 = 3;
/// `SecurityType`: `preferred`.
pub const NW_SECURITY_TYPE_PREFERRED: i32 = 4;
fn security_type_code(value: &str) -> i32 {
    match value {
        "adr" => NW_SECURITY_TYPE_ADR,
        "common" => NW_SECURITY_TYPE_COMMON,
        "other" => NW_SECURITY_TYPE_OTHER,
        "preferred" => NW_SECURITY_TYPE_PREFERRED,
        _ => NW_SECURITY_TYPE_UNKNOWN,
    }
}
/// `SettlementStatus`: a value this version does not know.
pub const NW_SETTLEMENT_STATUS_UNKNOWN: i32 = 0;
/// `SettlementStatus`: `disputed`.
pub const NW_SETTLEMENT_STATUS_DISPUTED: i32 = 1;
/// `SettlementStatus`: `pending`.
pub const NW_SETTLEMENT_STATUS_PENDING: i32 = 2;
/// `SettlementStatus`: `settled`.
pub const NW_SETTLEMENT_STATUS_SETTLED: i32 = 3;
/// `SettlementStatus`: `void`.
pub const NW_SETTLEMENT_STATUS_VOID: i32 = 4;
fn settlement_status_code(value: &str) -> i32 {
    match value {
        "disputed" => NW_SETTLEMENT_STATUS_DISPUTED,
        "pending" => NW_SETTLEMENT_STATUS_PENDING,
        "settled" => NW_SETTLEMENT_STATUS_SETTLED,
        "void" => NW_SETTLEMENT_STATUS_VOID,
        _ => NW_SETTLEMENT_STATUS_UNKNOWN,
    }
}
/// `TimingStepKind`: a value this version does not know.
pub const NW_TIMING_STEP_KIND_UNKNOWN: i32 = 0;
/// `TimingStepKind`: `date`.
pub const NW_TIMING_STEP_KIND_DATE: i32 = 1;
/// `TimingStepKind`: `meeting`.
pub const NW_TIMING_STEP_KIND_MEETING: i32 = 2;
fn timing_step_kind_code(value: &str) -> i32 {
    match value {
        "date" => NW_TIMING_STEP_KIND_DATE,
        "meeting" => NW_TIMING_STEP_KIND_MEETING,
        _ => NW_TIMING_STEP_KIND_UNKNOWN,
    }
}
/// `TopicGroup`: a value this version does not know.
pub const NW_TOPIC_GROUP_UNKNOWN: i32 = 0;
/// `TopicGroup`: `central_banks`.
pub const NW_TOPIC_GROUP_CENTRAL_BANKS: i32 = 1;
/// `TopicGroup`: `energy`.
pub const NW_TOPIC_GROUP_ENERGY: i32 = 2;
/// `TopicGroup`: `metals`.
pub const NW_TOPIC_GROUP_METALS: i32 = 3;
/// `TopicGroup`: `us_activity`.
pub const NW_TOPIC_GROUP_US_ACTIVITY: i32 = 4;
/// `TopicGroup`: `us_jobs`.
pub const NW_TOPIC_GROUP_US_JOBS: i32 = 5;
/// `TopicGroup`: `us_prices`.
pub const NW_TOPIC_GROUP_US_PRICES: i32 = 6;
/// `TopicGroup`: `global_activity`.
pub const NW_TOPIC_GROUP_GLOBAL_ACTIVITY: i32 = 7;
/// `TopicGroup`: `global_prices`.
pub const NW_TOPIC_GROUP_GLOBAL_PRICES: i32 = 8;
fn topic_group_code(value: &str) -> i32 {
    match value {
        "central_banks" => NW_TOPIC_GROUP_CENTRAL_BANKS,
        "energy" => NW_TOPIC_GROUP_ENERGY,
        "metals" => NW_TOPIC_GROUP_METALS,
        "us_activity" => NW_TOPIC_GROUP_US_ACTIVITY,
        "us_jobs" => NW_TOPIC_GROUP_US_JOBS,
        "us_prices" => NW_TOPIC_GROUP_US_PRICES,
        "global_activity" => NW_TOPIC_GROUP_GLOBAL_ACTIVITY,
        "global_prices" => NW_TOPIC_GROUP_GLOBAL_PRICES,
        _ => NW_TOPIC_GROUP_UNKNOWN,
    }
}
/// `Venue`: a value this version does not know.
pub const NW_VENUE_UNKNOWN: i32 = 0;
/// `Venue`: `kalshi`.
pub const NW_VENUE_KALSHI: i32 = 1;
/// `Venue`: `polymarket`.
pub const NW_VENUE_POLYMARKET: i32 = 2;
fn venue_code(value: &str) -> i32 {
    match value {
        "kalshi" => NW_VENUE_KALSHI,
        "polymarket" => NW_VENUE_POLYMARKET,
        _ => NW_VENUE_UNKNOWN,
    }
}
/// `VenueDifference`: a value this version does not know.
pub const NW_VENUE_DIFFERENCE_UNKNOWN: i32 = 0;
/// `VenueDifference`: `outcome_set`.
pub const NW_VENUE_DIFFERENCE_OUTCOME_SET: i32 = 1;
/// `VenueDifference`: `rules`.
pub const NW_VENUE_DIFFERENCE_RULES: i32 = 2;
/// `VenueDifference`: `source`.
pub const NW_VENUE_DIFFERENCE_SOURCE: i32 = 3;
/// `VenueDifference`: `timing`.
pub const NW_VENUE_DIFFERENCE_TIMING: i32 = 4;
fn venue_difference_code(value: &str) -> i32 {
    match value {
        "outcome_set" => NW_VENUE_DIFFERENCE_OUTCOME_SET,
        "rules" => NW_VENUE_DIFFERENCE_RULES,
        "source" => NW_VENUE_DIFFERENCE_SOURCE,
        "timing" => NW_VENUE_DIFFERENCE_TIMING,
        _ => NW_VENUE_DIFFERENCE_UNKNOWN,
    }
}
/// `VenueExclusionReason`: a value this version does not know.
pub const NW_VENUE_EXCLUSION_REASON_UNKNOWN: i32 = 0;
/// `VenueExclusionReason`: `outcome_set`.
pub const NW_VENUE_EXCLUSION_REASON_OUTCOME_SET: i32 = 1;
/// `VenueExclusionReason`: `rules`.
pub const NW_VENUE_EXCLUSION_REASON_RULES: i32 = 2;
/// `VenueExclusionReason`: `source`.
pub const NW_VENUE_EXCLUSION_REASON_SOURCE: i32 = 3;
/// `VenueExclusionReason`: `stale`.
pub const NW_VENUE_EXCLUSION_REASON_STALE: i32 = 4;
/// `VenueExclusionReason`: `thin`.
pub const NW_VENUE_EXCLUSION_REASON_THIN: i32 = 5;
/// `VenueExclusionReason`: `timing`.
pub const NW_VENUE_EXCLUSION_REASON_TIMING: i32 = 6;
fn venue_exclusion_reason_code(value: &str) -> i32 {
    match value {
        "outcome_set" => NW_VENUE_EXCLUSION_REASON_OUTCOME_SET,
        "rules" => NW_VENUE_EXCLUSION_REASON_RULES,
        "source" => NW_VENUE_EXCLUSION_REASON_SOURCE,
        "stale" => NW_VENUE_EXCLUSION_REASON_STALE,
        "thin" => NW_VENUE_EXCLUSION_REASON_THIN,
        "timing" => NW_VENUE_EXCLUSION_REASON_TIMING,
        _ => NW_VENUE_EXCLUSION_REASON_UNKNOWN,
    }
}
/// `WebhookDeliveryState`: a value this version does not know.
pub const NW_WEBHOOK_DELIVERY_STATE_UNKNOWN: i32 = 0;
/// `WebhookDeliveryState`: `cancelled`.
pub const NW_WEBHOOK_DELIVERY_STATE_CANCELLED: i32 = 1;
/// `WebhookDeliveryState`: `failed`.
pub const NW_WEBHOOK_DELIVERY_STATE_FAILED: i32 = 2;
/// `WebhookDeliveryState`: `leased`.
pub const NW_WEBHOOK_DELIVERY_STATE_LEASED: i32 = 3;
/// `WebhookDeliveryState`: `pending`.
pub const NW_WEBHOOK_DELIVERY_STATE_PENDING: i32 = 4;
/// `WebhookDeliveryState`: `succeeded`.
pub const NW_WEBHOOK_DELIVERY_STATE_SUCCEEDED: i32 = 5;
fn webhook_delivery_state_code(value: &str) -> i32 {
    match value {
        "cancelled" => NW_WEBHOOK_DELIVERY_STATE_CANCELLED,
        "failed" => NW_WEBHOOK_DELIVERY_STATE_FAILED,
        "leased" => NW_WEBHOOK_DELIVERY_STATE_LEASED,
        "pending" => NW_WEBHOOK_DELIVERY_STATE_PENDING,
        "succeeded" => NW_WEBHOOK_DELIVERY_STATE_SUCCEEDED,
        _ => NW_WEBHOOK_DELIVERY_STATE_UNKNOWN,
    }
}
/// `WebhookDisabledReason`: a value this version does not know.
pub const NW_WEBHOOK_DISABLED_REASON_UNKNOWN: i32 = 0;
/// `WebhookDisabledReason`: `customer`.
pub const NW_WEBHOOK_DISABLED_REASON_CUSTOMER: i32 = 1;
/// `WebhookDisabledReason`: `plan_limit`.
pub const NW_WEBHOOK_DISABLED_REASON_PLAN_LIMIT: i32 = 2;
fn webhook_disabled_reason_code(value: &str) -> i32 {
    match value {
        "customer" => NW_WEBHOOK_DISABLED_REASON_CUSTOMER,
        "plan_limit" => NW_WEBHOOK_DISABLED_REASON_PLAN_LIMIT,
        _ => NW_WEBHOOK_DISABLED_REASON_UNKNOWN,
    }
}
/// `WebhookEventType`: a value this version does not know.
pub const NW_WEBHOOK_EVENT_TYPE_UNKNOWN: i32 = 0;
/// `WebhookEventType`: `dataset.published`.
pub const NW_WEBHOOK_EVENT_TYPE_DATASET_PUBLISHED: i32 = 1;
/// `WebhookEventType`: `dataset.revised`.
pub const NW_WEBHOOK_EVENT_TYPE_DATASET_REVISED: i32 = 2;
/// `WebhookEventType`: `holiday.calendar.revised`.
pub const NW_WEBHOOK_EVENT_TYPE_HOLIDAY_CALENDAR_REVISED: i32 = 3;
/// `WebhookEventType`: `prediction.event.closed`.
pub const NW_WEBHOOK_EVENT_TYPE_PREDICTION_EVENT_CLOSED: i32 = 4;
/// `WebhookEventType`: `prediction.event.settled`.
pub const NW_WEBHOOK_EVENT_TYPE_PREDICTION_EVENT_SETTLED: i32 = 5;
/// `WebhookEventType`: `release.cancelled`.
pub const NW_WEBHOOK_EVENT_TYPE_RELEASE_CANCELLED: i32 = 6;
/// `WebhookEventType`: `release.rescheduled`.
pub const NW_WEBHOOK_EVENT_TYPE_RELEASE_RESCHEDULED: i32 = 7;
/// `WebhookEventType`: `release.scheduled`.
pub const NW_WEBHOOK_EVENT_TYPE_RELEASE_SCHEDULED: i32 = 8;
/// `WebhookEventType`: `release.updated`.
pub const NW_WEBHOOK_EVENT_TYPE_RELEASE_UPDATED: i32 = 9;
/// `WebhookEventType`: `webhook.test`.
pub const NW_WEBHOOK_EVENT_TYPE_WEBHOOK_TEST: i32 = 10;
/// `WebhookEventType`: `yield_curve.published`.
pub const NW_WEBHOOK_EVENT_TYPE_YIELD_CURVE_PUBLISHED: i32 = 11;
/// `WebhookEventType`: `yield_curve.revised`.
pub const NW_WEBHOOK_EVENT_TYPE_YIELD_CURVE_REVISED: i32 = 12;
/// `WebhookEventType`: `filing.published`.
pub const NW_WEBHOOK_EVENT_TYPE_FILING_PUBLISHED: i32 = 13;
fn webhook_event_type_code(value: &str) -> i32 {
    match value {
        "dataset.published" => NW_WEBHOOK_EVENT_TYPE_DATASET_PUBLISHED,
        "dataset.revised" => NW_WEBHOOK_EVENT_TYPE_DATASET_REVISED,
        "holiday.calendar.revised" => NW_WEBHOOK_EVENT_TYPE_HOLIDAY_CALENDAR_REVISED,
        "prediction.event.closed" => NW_WEBHOOK_EVENT_TYPE_PREDICTION_EVENT_CLOSED,
        "prediction.event.settled" => NW_WEBHOOK_EVENT_TYPE_PREDICTION_EVENT_SETTLED,
        "release.cancelled" => NW_WEBHOOK_EVENT_TYPE_RELEASE_CANCELLED,
        "release.rescheduled" => NW_WEBHOOK_EVENT_TYPE_RELEASE_RESCHEDULED,
        "release.scheduled" => NW_WEBHOOK_EVENT_TYPE_RELEASE_SCHEDULED,
        "release.updated" => NW_WEBHOOK_EVENT_TYPE_RELEASE_UPDATED,
        "webhook.test" => NW_WEBHOOK_EVENT_TYPE_WEBHOOK_TEST,
        "yield_curve.published" => NW_WEBHOOK_EVENT_TYPE_YIELD_CURVE_PUBLISHED,
        "yield_curve.revised" => NW_WEBHOOK_EVENT_TYPE_YIELD_CURVE_REVISED,
        "filing.published" => NW_WEBHOOK_EVENT_TYPE_FILING_PUBLISHED,
        _ => NW_WEBHOOK_EVENT_TYPE_UNKNOWN,
    }
}
/// `WebhookFamily`: a value this version does not know.
pub const NW_WEBHOOK_FAMILY_UNKNOWN: i32 = 0;
/// `WebhookFamily`: `activity`.
pub const NW_WEBHOOK_FAMILY_ACTIVITY: i32 = 1;
/// `WebhookFamily`: `gdp`.
pub const NW_WEBHOOK_FAMILY_GDP: i32 = 2;
/// `WebhookFamily`: `holidays`.
pub const NW_WEBHOOK_FAMILY_HOLIDAYS: i32 = 3;
/// `WebhookFamily`: `housing`.
pub const NW_WEBHOOK_FAMILY_HOUSING: i32 = 4;
/// `WebhookFamily`: `labour`.
pub const NW_WEBHOOK_FAMILY_LABOUR: i32 = 5;
/// `WebhookFamily`: `money`.
pub const NW_WEBHOOK_FAMILY_MONEY: i32 = 6;
/// `WebhookFamily`: `policy_rate`.
pub const NW_WEBHOOK_FAMILY_POLICY_RATE: i32 = 7;
/// `WebhookFamily`: `predictions`.
pub const NW_WEBHOOK_FAMILY_PREDICTIONS: i32 = 8;
/// `WebhookFamily`: `prices`.
pub const NW_WEBHOOK_FAMILY_PRICES: i32 = 9;
/// `WebhookFamily`: `public_finance`.
pub const NW_WEBHOOK_FAMILY_PUBLIC_FINANCE: i32 = 10;
/// `WebhookFamily`: `survey`.
pub const NW_WEBHOOK_FAMILY_SURVEY: i32 = 11;
/// `WebhookFamily`: `trade`.
pub const NW_WEBHOOK_FAMILY_TRADE: i32 = 12;
/// `WebhookFamily`: `yield_curve`.
pub const NW_WEBHOOK_FAMILY_YIELD_CURVE: i32 = 13;
/// `WebhookFamily`: `filings`.
pub const NW_WEBHOOK_FAMILY_FILINGS: i32 = 14;
fn webhook_family_code(value: &str) -> i32 {
    match value {
        "activity" => NW_WEBHOOK_FAMILY_ACTIVITY,
        "gdp" => NW_WEBHOOK_FAMILY_GDP,
        "holidays" => NW_WEBHOOK_FAMILY_HOLIDAYS,
        "housing" => NW_WEBHOOK_FAMILY_HOUSING,
        "labour" => NW_WEBHOOK_FAMILY_LABOUR,
        "money" => NW_WEBHOOK_FAMILY_MONEY,
        "policy_rate" => NW_WEBHOOK_FAMILY_POLICY_RATE,
        "predictions" => NW_WEBHOOK_FAMILY_PREDICTIONS,
        "prices" => NW_WEBHOOK_FAMILY_PRICES,
        "public_finance" => NW_WEBHOOK_FAMILY_PUBLIC_FINANCE,
        "survey" => NW_WEBHOOK_FAMILY_SURVEY,
        "trade" => NW_WEBHOOK_FAMILY_TRADE,
        "yield_curve" => NW_WEBHOOK_FAMILY_YIELD_CURVE,
        "filings" => NW_WEBHOOK_FAMILY_FILINGS,
        _ => NW_WEBHOOK_FAMILY_UNKNOWN,
    }
}
/// `WebhookTestStatus`: a value this version does not know.
pub const NW_WEBHOOK_TEST_STATUS_UNKNOWN: i32 = 0;
/// `WebhookTestStatus`: `disabled`.
pub const NW_WEBHOOK_TEST_STATUS_DISABLED: i32 = 1;
/// `WebhookTestStatus`: `queued`.
pub const NW_WEBHOOK_TEST_STATUS_QUEUED: i32 = 2;
/// `WebhookTestStatus`: `rate_limited`.
pub const NW_WEBHOOK_TEST_STATUS_RATE_LIMITED: i32 = 3;
fn webhook_test_status_code(value: &str) -> i32 {
    match value {
        "disabled" => NW_WEBHOOK_TEST_STATUS_DISABLED,
        "queued" => NW_WEBHOOK_TEST_STATUS_QUEUED,
        "rate_limited" => NW_WEBHOOK_TEST_STATUS_RATE_LIMITED,
        _ => NW_WEBHOOK_TEST_STATUS_UNKNOWN,
    }
}
/// `YieldCurveType`: a value this version does not know.
pub const NW_YIELD_CURVE_TYPE_UNKNOWN: i32 = 0;
/// `YieldCurveType`: `constant_maturity_ytm`.
pub const NW_YIELD_CURVE_TYPE_CONSTANT_MATURITY_YTM: i32 = 1;
/// `YieldCurveType`: `nominal_par`.
pub const NW_YIELD_CURVE_TYPE_NOMINAL_PAR: i32 = 2;
/// `YieldCurveType`: `nominal_zero_coupon`.
pub const NW_YIELD_CURVE_TYPE_NOMINAL_ZERO_COUPON: i32 = 3;
/// `YieldCurveType`: `spot_svensson`.
pub const NW_YIELD_CURVE_TYPE_SPOT_SVENSSON: i32 = 4;
/// `YieldCurveType`: `zero_coupon`.
pub const NW_YIELD_CURVE_TYPE_ZERO_COUPON: i32 = 5;
fn yield_curve_type_code(value: &str) -> i32 {
    match value {
        "constant_maturity_ytm" => NW_YIELD_CURVE_TYPE_CONSTANT_MATURITY_YTM,
        "nominal_par" => NW_YIELD_CURVE_TYPE_NOMINAL_PAR,
        "nominal_zero_coupon" => NW_YIELD_CURVE_TYPE_NOMINAL_ZERO_COUPON,
        "spot_svensson" => NW_YIELD_CURVE_TYPE_SPOT_SVENSSON,
        "zero_coupon" => NW_YIELD_CURVE_TYPE_ZERO_COUPON,
        _ => NW_YIELD_CURVE_TYPE_UNKNOWN,
    }
}
/// Mirror of `AsReportedValue`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_as_reported_value {
    pub end: *const c_char,
    pub start: *const c_char,
    pub value: *const c_char,
}
pub(crate) fn mirror_as_reported_value(
    v: &sdk::models::AsReportedValue,
    s: &mut Store,
) -> Result<nw_as_reported_value, Unrepresentable> {
    Ok(nw_as_reported_value {
        end: s.str(&v.end)?,
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        value: s.str(v.value.as_str())?,
    })
}
/// Mirror of `AsReportedConcept`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_as_reported_concept {
    pub concept: *const c_char,
    pub label: *const c_char,
    pub taxonomy: *const c_char,
    pub unit: *const c_char,
    pub values: *const *const nw_as_reported_value,
    pub values_len: usize,
}
pub(crate) fn mirror_as_reported_concept(
    v: &sdk::models::AsReportedConcept,
    s: &mut Store,
) -> Result<nw_as_reported_concept, Unrepresentable> {
    let f4 = {
        let items = v
            .values
            .iter()
            .map(|x| {
                let m = mirror_as_reported_value(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_as_reported_concept {
        concept: s.str(&v.concept)?,
        label: match &v.label {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        taxonomy: s.str(&v.taxonomy)?,
        unit: s.str(&v.unit)?,
        values: f4.0,
        values_len: f4.1,
    })
}
/// Mirror of `AsReportedPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_as_reported_page {
    pub meta: *const nw_response_meta,
    pub concepts: *const *const nw_as_reported_concept,
    pub concepts_len: usize,
}
pub(crate) fn mirror_as_reported_page(
    v: &sdk::models::AsReportedPage,
    s: &mut Store,
) -> Result<nw_as_reported_page, Unrepresentable> {
    let f0 = {
        let items = v
            .concepts
            .iter()
            .map(|x| {
                let m = mirror_as_reported_concept(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_as_reported_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        concepts: f0.0,
        concepts_len: f0.1,
    })
}
/// Mirror of `BinaryLabels`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_binary_labels {
    pub no: *const c_char,
    pub yes: *const c_char,
}
pub(crate) fn mirror_binary_labels(
    v: &sdk::models::BinaryLabels,
    s: &mut Store,
) -> Result<nw_binary_labels, Unrepresentable> {
    Ok(nw_binary_labels {
        no: s.str(&v.no)?,
        yes: s.str(&v.yes)?,
    })
}
/// Mirror of `Quote`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_quote {
    pub market_ids: *const *const c_char,
    pub market_ids_len: usize,
    pub probability: *const c_char,
    pub venue: i32,
    pub venue_raw: *const c_char,
}
pub(crate) fn mirror_quote(
    v: &sdk::models::Quote,
    s: &mut Store,
) -> Result<nw_quote, Unrepresentable> {
    let f0 = {
        let items = v
            .market_ids
            .iter()
            .map(|x| s.str(x))
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_quote {
        market_ids: f0.0,
        market_ids_len: f0.1,
        probability: s.str(&v.probability)?,
        venue: venue_code(v.venue.as_str()),
        venue_raw: s.str(v.venue.as_str())?,
    })
}
/// Mirror of `PredictionResultMarket`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_result_market {
    pub market_id: *const c_char,
    pub outcome: *const c_char,
    pub value: *const c_char,
}
pub(crate) fn mirror_prediction_result_market(
    v: &sdk::models::PredictionResultMarket,
    s: &mut Store,
) -> Result<nw_prediction_result_market, Unrepresentable> {
    Ok(nw_prediction_result_market {
        market_id: s.str(&v.market_id)?,
        outcome: s.str(&v.outcome)?,
        value: match &v.value {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PredictionResultVenue`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_result_venue {
    pub amended_at: *const c_char,
    pub has_final_: bool,
    pub final_: bool,
    pub markets: *const *const nw_prediction_result_market,
    pub markets_len: usize,
    pub settled_at: *const c_char,
    pub has_unavailable: bool,
    pub unavailable: bool,
    pub venue: i32,
    pub venue_raw: *const c_char,
}
pub(crate) fn mirror_prediction_result_venue(
    v: &sdk::models::PredictionResultVenue,
    s: &mut Store,
) -> Result<nw_prediction_result_venue, Unrepresentable> {
    let f2 = {
        let items = v
            .markets
            .iter()
            .map(|x| {
                let m = mirror_prediction_result_market(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_result_venue {
        amended_at: match &v.amended_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_final_: v.final_.is_some(),
        final_: v.final_.unwrap_or_default(),
        markets: f2.0,
        markets_len: f2.1,
        settled_at: match &v.settled_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_unavailable: v.unavailable.is_some(),
        unavailable: v.unavailable.unwrap_or_default(),
        venue: venue_code(v.venue.as_str()),
        venue_raw: s.str(v.venue.as_str())?,
    })
}
/// Mirror of `PredictionResult`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_result {
    pub amended_at: *const c_char,
    pub final_: bool,
    pub settled_at: *const c_char,
    pub venues: *const *const nw_prediction_result_venue,
    pub venues_len: usize,
}
pub(crate) fn mirror_prediction_result(
    v: &sdk::models::PredictionResult,
    s: &mut Store,
) -> Result<nw_prediction_result, Unrepresentable> {
    let f3 = {
        let items = v
            .venues
            .iter()
            .map(|x| {
                let m = mirror_prediction_result_venue(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_result {
        amended_at: match &v.amended_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        final_: v.final_,
        settled_at: match &v.settled_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f3.0,
        venues_len: f3.1,
    })
}
/// Mirror of `VenueMatch`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_venue_match {
    pub differences: *const i32,
    pub differences_raw: *const *const c_char,
    pub differences_len: usize,
    pub note: *const c_char,
}
pub(crate) fn mirror_venue_match(
    v: &sdk::models::VenueMatch,
    s: &mut Store,
) -> Result<nw_venue_match, Unrepresentable> {
    let f0 = {
        let codes = v
            .differences
            .iter()
            .map(|x| venue_difference_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .differences
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    Ok(nw_venue_match {
        differences: f0.0,
        differences_raw: f0.1,
        differences_len: f0.2,
        note: match &v.note {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PredictionVenue`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_venue {
    pub excluded_reason: i32,
    pub excluded_reason_raw: *const c_char,
    pub match_: *const nw_venue_match,
    pub open_interest_usd: *const c_char,
    pub rules_text: *const c_char,
    pub rules_url: *const c_char,
    pub settlement_source: *const c_char,
    pub url: *const c_char,
    pub venue: i32,
    pub venue_raw: *const c_char,
    pub volume_24h_usd: *const c_char,
    pub weight: *const c_char,
    pub settlement: i32,
    pub settlement_raw: *const c_char,
}
pub(crate) fn mirror_prediction_venue(
    v: &sdk::models::PredictionVenue,
    s: &mut Store,
) -> Result<nw_prediction_venue, Unrepresentable> {
    Ok(nw_prediction_venue {
        excluded_reason: v
            .excluded_reason
            .as_ref()
            .map_or(0, |x| venue_exclusion_reason_code(x.as_str())),
        excluded_reason_raw: match &v.excluded_reason {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        match_: match &v.match_ {
            Some(x) => {
                let v = mirror_venue_match(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        open_interest_usd: match &v.open_interest_usd {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        rules_text: match &v.rules_text {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        rules_url: match &v.rules_url {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        settlement_source: match &v.settlement_source {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        url: s.str(&v.url)?,
        venue: venue_code(v.venue.as_str()),
        venue_raw: s.str(v.venue.as_str())?,
        volume_24h_usd: match &v.volume_24h_usd {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        weight: match &v.weight {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        settlement: v
            .settlement
            .as_ref()
            .map_or(0, |x| prediction_venue_settlement_code(x.as_str())),
        settlement_raw: match &v.settlement {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `BinaryEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_binary_event {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub closes_at: *const c_char,
    pub event_id: *const c_char,
    pub question: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub topic_id: *const c_char,
    pub unit: *const c_char,
    pub venues: *const *const nw_prediction_venue,
    pub venues_len: usize,
    pub labels: *const nw_binary_labels,
    pub probability: *const c_char,
    pub quotes: *const *const nw_quote,
    pub quotes_len: usize,
    pub resolution: *const c_char,
    pub resolves_with: *const c_char,
    pub result: *const nw_prediction_result,
    pub has_stale: bool,
    pub stale: bool,
}
pub(crate) fn mirror_binary_event(
    v: &sdk::models::BinaryEvent,
    s: &mut Store,
) -> Result<nw_binary_event, Unrepresentable> {
    let f6 = {
        let items = v
            .quotes
            .iter()
            .map(|x| {
                let m = mirror_quote(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f16 = {
        let items = v
            .venues
            .iter()
            .map(|x| {
                let m = mirror_prediction_venue(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_binary_event {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        question: s.str(&v.question)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        status: prediction_event_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        topic_id: s.str(&v.topic_id)?,
        unit: match &v.unit {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f16.0,
        venues_len: f16.1,
        labels: match &v.labels {
            Some(x) => {
                let v = mirror_binary_labels(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        probability: match &v.probability {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        quotes: f6.0,
        quotes_len: f6.1,
        resolution: s.str(&v.resolution)?,
        resolves_with: match &v.resolves_with {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        result: match &v.result {
            Some(x) => {
                let v = mirror_prediction_result(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
    })
}
/// Mirror of `CalendarCoverage`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_calendar_coverage {
    pub scope: i32,
    pub scope_raw: *const c_char,
    pub subdivision_codes: *const *const c_char,
    pub subdivision_codes_len: usize,
}
pub(crate) fn mirror_calendar_coverage(
    v: &sdk::models::CalendarCoverage,
    s: &mut Store,
) -> Result<nw_calendar_coverage, Unrepresentable> {
    let f1 = match &v.subdivision_codes {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    Ok(nw_calendar_coverage {
        scope: holiday_scope_code(v.scope.as_str()),
        scope_raw: s.str(v.scope.as_str())?,
        subdivision_codes: f1.0,
        subdivision_codes_len: f1.1,
    })
}
/// Mirror of `CalendarHoliday`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_calendar_holiday {
    pub date: *const c_char,
    pub local_name: *const c_char,
    pub name: *const c_char,
    pub has_observed: bool,
    pub observed: bool,
    pub scope: i32,
    pub scope_raw: *const c_char,
    pub subdivision_codes: *const *const c_char,
    pub subdivision_codes_len: usize,
}
pub(crate) fn mirror_calendar_holiday(
    v: &sdk::models::CalendarHoliday,
    s: &mut Store,
) -> Result<nw_calendar_holiday, Unrepresentable> {
    let f5 = match &v.subdivision_codes {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    Ok(nw_calendar_holiday {
        date: s.str(&v.date)?,
        local_name: match &v.local_name {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        name: s.str(&v.name)?,
        has_observed: v.observed.is_some(),
        observed: v.observed.unwrap_or_default(),
        scope: holiday_scope_code(v.scope.as_str()),
        scope_raw: s.str(v.scope.as_str())?,
        subdivision_codes: f5.0,
        subdivision_codes_len: f5.1,
    })
}
/// Mirror of `CalendarHolidays`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_calendar_holidays {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub coverage: *const *const nw_calendar_coverage,
    pub coverage_len: usize,
    pub holidays: *const *const nw_calendar_holiday,
    pub holidays_len: usize,
}
pub(crate) fn mirror_calendar_holidays(
    v: &sdk::models::CalendarHolidays,
    s: &mut Store,
) -> Result<nw_calendar_holidays, Unrepresentable> {
    let f1 = {
        let items = v
            .coverage
            .iter()
            .map(|x| {
                let m = mirror_calendar_coverage(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f2 = {
        let items = v
            .holidays
            .iter()
            .map(|x| {
                let m = mirror_calendar_holiday(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_calendar_holidays {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        coverage: f1.0,
        coverage_len: f1.1,
        holidays: f2.0,
        holidays_len: f2.1,
    })
}
/// Mirror of `CalendarHolidaysOnDate`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_calendar_holidays_on_date {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub coverage: *const *const nw_calendar_coverage,
    pub coverage_len: usize,
    pub holidays: *const *const nw_calendar_holiday,
    pub holidays_len: usize,
}
pub(crate) fn mirror_calendar_holidays_on_date(
    v: &sdk::models::CalendarHolidaysOnDate,
    s: &mut Store,
) -> Result<nw_calendar_holidays_on_date, Unrepresentable> {
    let f1 = {
        let items = v
            .coverage
            .iter()
            .map(|x| {
                let m = mirror_calendar_coverage(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f2 = {
        let items = v
            .holidays
            .iter()
            .map(|x| {
                let m = mirror_calendar_holiday(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_calendar_holidays_on_date {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        coverage: f1.0,
        coverage_len: f1.1,
        holidays: f2.0,
        holidays_len: f2.1,
    })
}
/// Mirror of `CalendarNextHoliday`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_calendar_next_holiday {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub coverage: *const *const nw_calendar_coverage,
    pub coverage_len: usize,
    pub holiday: *const nw_calendar_holiday,
}
pub(crate) fn mirror_calendar_next_holiday(
    v: &sdk::models::CalendarNextHoliday,
    s: &mut Store,
) -> Result<nw_calendar_next_holiday, Unrepresentable> {
    let f1 = {
        let items = v
            .coverage
            .iter()
            .map(|x| {
                let m = mirror_calendar_coverage(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_calendar_next_holiday {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        coverage: f1.0,
        coverage_len: f1.1,
        holiday: match &v.holiday {
            Some(x) => {
                let v = mirror_calendar_holiday(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CalendarSubscriptionCreated`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_calendar_subscription_created {
    pub meta: *const nw_response_meta,
    pub expires_at: *const c_char,
    pub subscription_id: *const c_char,
    pub url: *const c_char,
}
pub(crate) fn mirror_calendar_subscription_created(
    v: &sdk::models::CalendarSubscriptionCreated,
    s: &mut Store,
) -> Result<nw_calendar_subscription_created, Unrepresentable> {
    Ok(nw_calendar_subscription_created {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        expires_at: s.str(&v.expires_at)?,
        subscription_id: s.str(&v.subscription_id)?,
        url: s.str(&v.url)?,
    })
}
/// Mirror of `CatalogueLatest`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_catalogue_latest {
    pub attribution: *const c_char,
    pub base_period: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub fetched_at: *const c_char,
    pub first_reported_at: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub id: *const c_char,
    pub licence_url: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub scale: *const c_char,
    pub seasonal_adjustment: i32,
    pub seasonal_adjustment_raw: *const c_char,
    pub has_stale: bool,
    pub stale: bool,
    pub status: i32,
    pub status_raw: *const c_char,
    pub title: *const c_char,
    pub unit: *const c_char,
    pub unit_label: *const c_char,
    pub value: *const c_char,
}
pub(crate) fn mirror_catalogue_latest(
    v: &sdk::models::CatalogueLatest,
    s: &mut Store,
) -> Result<nw_catalogue_latest, Unrepresentable> {
    Ok(nw_catalogue_latest {
        attribution: s.str(&v.attribution)?,
        base_period: match &v.base_period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        fetched_at: s.str(&v.fetched_at)?,
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        frequency: catalogue_latest_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        id: s.str(&v.id)?,
        licence_url: s.str(&v.licence_url)?,
        period: s.str(&v.period)?,
        released_on: match &v.released_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        scale: match &v.scale {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        seasonal_adjustment: catalogue_latest_seasonal_adjustment_code(
            v.seasonal_adjustment.as_str(),
        ),
        seasonal_adjustment_raw: s.str(v.seasonal_adjustment.as_str())?,
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
        status: v
            .status
            .as_ref()
            .map_or(0, |x| resource_observation_status_code(x.as_str())),
        status_raw: match &v.status {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        title: s.str(&v.title)?,
        unit: s.str(&v.unit)?,
        unit_label: s.str(&v.unit_label)?,
        value: s.str(v.value.as_str())?,
    })
}
/// Mirror of `CatalogueBulkLatest`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_catalogue_bulk_latest {
    pub meta: *const nw_response_meta,
    pub missing: *const *const c_char,
    pub missing_len: usize,
    pub observations: *const *const nw_catalogue_latest,
    pub observations_len: usize,
}
pub(crate) fn mirror_catalogue_bulk_latest(
    v: &sdk::models::CatalogueBulkLatest,
    s: &mut Store,
) -> Result<nw_catalogue_bulk_latest, Unrepresentable> {
    let f0 = {
        let items = v
            .missing
            .iter()
            .map(|x| s.str(x))
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f1 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_catalogue_latest(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_catalogue_bulk_latest {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        missing: f0.0,
        missing_len: f0.1,
        observations: f1.0,
        observations_len: f1.1,
    })
}
/// Mirror of `ResourceObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_resource_observation {
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub value: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_resource_observation(
    v: &sdk::models::ResourceObservation,
    s: &mut Store,
) -> Result<nw_resource_observation, Unrepresentable> {
    Ok(nw_resource_observation {
        period: s.str(&v.period)?,
        released_on: match &v.released_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        status: v
            .status
            .as_ref()
            .map_or(0, |x| resource_observation_status_code(x.as_str())),
        status_raw: match &v.status {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        value: s.str(v.value.as_str())?,
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CatalogueHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_catalogue_history {
    pub meta: *const nw_response_meta,
    pub attribution: *const c_char,
    pub base_period: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub fetched_at: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub id: *const c_char,
    pub licence_url: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_resource_observation,
    pub observations_len: usize,
    pub scale: *const c_char,
    pub seasonal_adjustment: i32,
    pub seasonal_adjustment_raw: *const c_char,
    pub has_stale: bool,
    pub stale: bool,
    pub title: *const c_char,
    pub unit: *const c_char,
    pub unit_label: *const c_char,
}
pub(crate) fn mirror_catalogue_history(
    v: &sdk::models::CatalogueHistory,
    s: &mut Store,
) -> Result<nw_catalogue_history, Unrepresentable> {
    let f9 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_resource_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_catalogue_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        attribution: s.str(&v.attribution)?,
        base_period: match &v.base_period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        fetched_at: s.str(&v.fetched_at)?,
        frequency: catalogue_latest_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        id: s.str(&v.id)?,
        licence_url: s.str(&v.licence_url)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f9.0,
        observations_len: f9.1,
        scale: match &v.scale {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        seasonal_adjustment: catalogue_latest_seasonal_adjustment_code(
            v.seasonal_adjustment.as_str(),
        ),
        seasonal_adjustment_raw: s.str(v.seasonal_adjustment.as_str())?,
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
        title: s.str(&v.title)?,
        unit: s.str(&v.unit)?,
        unit_label: s.str(&v.unit_label)?,
    })
}
/// Mirror of `CatalogueEntry`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_catalogue_entry {
    pub country: *const c_char,
    pub first_period: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub id: *const c_char,
    pub id_template: *const c_char,
    pub last_period: *const c_char,
    pub source: *const c_char,
    pub title: *const c_char,
    pub unit_label: *const c_char,
}
pub(crate) fn mirror_catalogue_entry(
    v: &sdk::models::CatalogueEntry,
    s: &mut Store,
) -> Result<nw_catalogue_entry, Unrepresentable> {
    Ok(nw_catalogue_entry {
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        first_period: match &v.first_period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        frequency: v
            .frequency
            .as_ref()
            .map_or(0, |x| catalogue_latest_frequency_code(x.as_str())),
        frequency_raw: match &v.frequency {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        id: match &v.id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        id_template: match &v.id_template {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        last_period: match &v.last_period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        source: s.str(&v.source)?,
        title: s.str(&v.title)?,
        unit_label: match &v.unit_label {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CatalogueSearchPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_catalogue_search_page {
    pub meta: *const nw_response_meta,
    pub next_cursor: *const c_char,
    pub results: *const *const nw_catalogue_entry,
    pub results_len: usize,
}
pub(crate) fn mirror_catalogue_search_page(
    v: &sdk::models::CatalogueSearchPage,
    s: &mut Store,
) -> Result<nw_catalogue_search_page, Unrepresentable> {
    let f1 = {
        let items = v
            .results
            .iter()
            .map(|x| {
                let m = mirror_catalogue_entry(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_catalogue_search_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        results: f1.0,
        results_len: f1.1,
    })
}
/// Mirror of `CatalogueSeries`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_catalogue_series {
    pub meta: *const nw_response_meta,
    pub attribution: *const c_char,
    pub base_period: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub fetched_at: *const c_char,
    pub first_period: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub id: *const c_char,
    pub last_period: *const c_char,
    pub licence_url: *const c_char,
    pub scale: *const c_char,
    pub seasonal_adjustment: i32,
    pub seasonal_adjustment_raw: *const c_char,
    pub has_stale: bool,
    pub stale: bool,
    pub title: *const c_char,
    pub unit: *const c_char,
    pub unit_label: *const c_char,
}
pub(crate) fn mirror_catalogue_series(
    v: &sdk::models::CatalogueSeries,
    s: &mut Store,
) -> Result<nw_catalogue_series, Unrepresentable> {
    Ok(nw_catalogue_series {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        attribution: s.str(&v.attribution)?,
        base_period: match &v.base_period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        fetched_at: s.str(&v.fetched_at)?,
        first_period: s.str(&v.first_period)?,
        frequency: catalogue_latest_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        id: s.str(&v.id)?,
        last_period: s.str(&v.last_period)?,
        licence_url: s.str(&v.licence_url)?,
        scale: match &v.scale {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        seasonal_adjustment: catalogue_latest_seasonal_adjustment_code(
            v.seasonal_adjustment.as_str(),
        ),
        seasonal_adjustment_raw: s.str(v.seasonal_adjustment.as_str())?,
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
        title: s.str(&v.title)?,
        unit: s.str(&v.unit)?,
        unit_label: s.str(&v.unit_label)?,
    })
}
/// Mirror of `CentralBankBalanceSheet`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_central_bank_balance_sheet {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub scale: i32,
    pub scale_raw: *const c_char,
    pub value: *const c_char,
}
pub(crate) fn mirror_central_bank_balance_sheet(
    v: &sdk::models::CentralBankBalanceSheet,
    s: &mut Store,
) -> Result<nw_central_bank_balance_sheet, Unrepresentable> {
    Ok(nw_central_bank_balance_sheet {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        period: s.str(&v.period)?,
        released_on: s.str(&v.released_on)?,
        scale: central_bank_balance_sheet_scale_code(v.scale.as_str()),
        scale_raw: s.str(v.scale.as_str())?,
        value: s.str(v.value.as_str())?,
    })
}
/// Mirror of `CentralBankReserves`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_central_bank_reserves {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub scale: i32,
    pub scale_raw: *const c_char,
    pub value: *const c_char,
}
pub(crate) fn mirror_central_bank_reserves(
    v: &sdk::models::CentralBankReserves,
    s: &mut Store,
) -> Result<nw_central_bank_reserves, Unrepresentable> {
    Ok(nw_central_bank_reserves {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        period: s.str(&v.period)?,
        released_on: s.str(&v.released_on)?,
        scale: central_bank_balance_sheet_scale_code(v.scale.as_str()),
        scale_raw: s.str(v.scale.as_str())?,
        value: s.str(v.value.as_str())?,
    })
}
/// Mirror of `CommodityBar`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_commodity_bar {
    pub close: *const c_char,
    pub high: *const c_char,
    pub low: *const c_char,
    pub open: *const c_char,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_commodity_bar(
    v: &sdk::models::CommodityBar,
    s: &mut Store,
) -> Result<nw_commodity_bar, Unrepresentable> {
    Ok(nw_commodity_bar {
        close: s.str(v.close.as_str())?,
        high: match &v.high {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        low: match &v.low {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        open: match &v.open {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `CommodityBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_commodity_bars {
    pub meta: *const nw_response_meta,
    pub bars: *const *const nw_commodity_bar,
    pub bars_len: usize,
    pub currency: *const c_char,
    pub end: *const c_char,
    pub next_cursor: *const c_char,
    pub start: *const c_char,
    pub symbol: *const c_char,
    pub unit: i32,
    pub unit_raw: *const c_char,
}
pub(crate) fn mirror_commodity_bars(
    v: &sdk::models::CommodityBars,
    s: &mut Store,
) -> Result<nw_commodity_bars, Unrepresentable> {
    let f0 = {
        let items = v
            .bars
            .iter()
            .map(|x| {
                let m = mirror_commodity_bar(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_commodity_bars {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        bars: f0.0,
        bars_len: f0.1,
        currency: s.str(&v.currency)?,
        end: match &v.end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        symbol: s.str(&v.symbol)?,
        unit: commodity_unit_code(v.unit.as_str()),
        unit_raw: s.str(v.unit.as_str())?,
    })
}
/// Mirror of `CommoditySpot`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_commodity_spot {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub price: *const c_char,
    pub stale: bool,
    pub symbol: *const c_char,
    pub timestamp: *const c_char,
    pub unit: i32,
    pub unit_raw: *const c_char,
}
pub(crate) fn mirror_commodity_spot(
    v: &sdk::models::CommoditySpot,
    s: &mut Store,
) -> Result<nw_commodity_spot, Unrepresentable> {
    Ok(nw_commodity_spot {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        price: s.str(v.price.as_str())?,
        stale: v.stale,
        symbol: s.str(&v.symbol)?,
        timestamp: s.str(&v.timestamp)?,
        unit: commodity_unit_code(v.unit.as_str()),
        unit_raw: s.str(v.unit.as_str())?,
    })
}
/// Mirror of `ListedFiling`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_listed_filing {
    pub filed: *const c_char,
    pub form: *const c_char,
    pub id: *const c_char,
    pub period_end: *const c_char,
    pub title: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_listed_filing(
    v: &sdk::models::ListedFiling,
    s: &mut Store,
) -> Result<nw_listed_filing, Unrepresentable> {
    Ok(nw_listed_filing {
        filed: s.str(&v.filed)?,
        form: s.str(&v.form)?,
        id: s.str(&v.id)?,
        period_end: match &v.period_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        title: match &v.title {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: filing_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `CompanyFilingsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_company_filings_page {
    pub meta: *const nw_response_meta,
    pub filed_from: *const c_char,
    pub filings: *const *const nw_listed_filing,
    pub filings_len: usize,
    pub id: *const c_char,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_company_filings_page(
    v: &sdk::models::CompanyFilingsPage,
    s: &mut Store,
) -> Result<nw_company_filings_page, Unrepresentable> {
    let f1 = {
        let items = v
            .filings
            .iter()
            .map(|x| {
                let m = mirror_listed_filing(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_company_filings_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        filed_from: match &v.filed_from {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        filings: f1.0,
        filings_len: f1.1,
        id: s.str(&v.id)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CompanySearchHit`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_company_search_hit {
    pub code: *const c_char,
    pub country: *const c_char,
    pub has_delisted: bool,
    pub delisted: bool,
    pub has_former_code: bool,
    pub former_code: bool,
    pub former_name: *const c_char,
    pub id: *const c_char,
    pub matched: i32,
    pub matched_raw: *const c_char,
    pub mic: *const c_char,
    pub name: *const c_char,
}
pub(crate) fn mirror_company_search_hit(
    v: &sdk::models::CompanySearchHit,
    s: &mut Store,
) -> Result<nw_company_search_hit, Unrepresentable> {
    Ok(nw_company_search_hit {
        code: match &v.code {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_delisted: v.delisted.is_some(),
        delisted: v.delisted.unwrap_or_default(),
        has_former_code: v.former_code.is_some(),
        former_code: v.former_code.unwrap_or_default(),
        former_name: match &v.former_name {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        id: s.str(&v.id)?,
        matched: v
            .matched
            .as_ref()
            .map_or(0, |x| company_search_match_code(x.as_str())),
        matched_raw: match &v.matched {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        mic: match &v.mic {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        name: s.str(&v.name)?,
    })
}
/// Mirror of `RegistryCoverage`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_registry_coverage {
    pub companies: i64,
    pub registry: i32,
    pub registry_raw: *const c_char,
}
pub(crate) fn mirror_registry_coverage(
    v: &sdk::models::RegistryCoverage,
    s: &mut Store,
) -> Result<nw_registry_coverage, Unrepresentable> {
    Ok(nw_registry_coverage {
        companies: v.companies,
        registry: registry_code(v.registry.as_str()),
        registry_raw: s.str(v.registry.as_str())?,
    })
}
/// Mirror of `CompanySearchPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_company_search_page {
    pub meta: *const nw_response_meta,
    pub companies: *const *const nw_company_search_hit,
    pub companies_len: usize,
    pub coverage: *const *const nw_registry_coverage,
    pub coverage_len: usize,
}
pub(crate) fn mirror_company_search_page(
    v: &sdk::models::CompanySearchPage,
    s: &mut Store,
) -> Result<nw_company_search_page, Unrepresentable> {
    let f0 = {
        let items = v
            .companies
            .iter()
            .map(|x| {
                let m = mirror_company_search_hit(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f1 = match &v.coverage {
        Some(a) => {
            let items = a
                .iter()
                .map(|x| {
                    let m = mirror_registry_coverage(x, s)?;
                    Ok::<_, Unrepresentable>(s.boxed(m))
                })
                .collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    Ok(nw_company_search_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        companies: f0.0,
        companies_len: f0.1,
        coverage: f1.0,
        coverage_len: f1.1,
    })
}
/// Mirror of `CompanyIndustry`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_company_industry {
    pub industry: *const c_char,
    pub isic: *const c_char,
    pub sector: *const c_char,
    pub source_code: *const c_char,
    pub source_scheme: *const c_char,
}
pub(crate) fn mirror_company_industry(
    v: &sdk::models::CompanyIndustry,
    s: &mut Store,
) -> Result<nw_company_industry, Unrepresentable> {
    Ok(nw_company_industry {
        industry: match &v.industry {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        isic: match &v.isic {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        sector: match &v.sector {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        source_code: match &v.source_code {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        source_scheme: match &v.source_scheme {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CompanyNameSpan`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_company_name_span {
    pub from: *const c_char,
    pub name: *const c_char,
    pub to: *const c_char,
}
pub(crate) fn mirror_company_name_span(
    v: &sdk::models::CompanyNameSpan,
    s: &mut Store,
) -> Result<nw_company_name_span, Unrepresentable> {
    Ok(nw_company_name_span {
        from: match &v.from {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        name: s.str(&v.name)?,
        to: match &v.to {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CompanyRelation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_company_relation {
    pub id: *const c_char,
    pub kind: i32,
    pub kind_raw: *const c_char,
}
pub(crate) fn mirror_company_relation(
    v: &sdk::models::CompanyRelation,
    s: &mut Store,
) -> Result<nw_company_relation, Unrepresentable> {
    Ok(nw_company_relation {
        id: s.str(&v.id)?,
        kind: company_relation_kind_code(v.kind.as_str()),
        kind_raw: s.str(v.kind.as_str())?,
    })
}
/// Mirror of `SecurityListing`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_security_listing {
    pub code: *const c_char,
    pub from: *const c_char,
    pub mic: *const c_char,
    pub to: *const c_char,
}
pub(crate) fn mirror_security_listing(
    v: &sdk::models::SecurityListing,
    s: &mut Store,
) -> Result<nw_security_listing, Unrepresentable> {
    Ok(nw_security_listing {
        code: s.str(&v.code)?,
        from: match &v.from {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        mic: s.str(&v.mic)?,
        to: match &v.to {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CompanySecurity`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_company_security {
    pub adr_ratio: *const c_char,
    pub class_: *const c_char,
    pub figi: *const c_char,
    pub isin: *const c_char,
    pub listings: *const *const nw_security_listing,
    pub listings_len: usize,
    pub has_primary: bool,
    pub primary: bool,
    pub title: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_company_security(
    v: &sdk::models::CompanySecurity,
    s: &mut Store,
) -> Result<nw_company_security, Unrepresentable> {
    let f4 = {
        let items = v
            .listings
            .iter()
            .map(|x| {
                let m = mirror_security_listing(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_company_security {
        adr_ratio: match &v.adr_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        class_: s.str(&v.class)?,
        figi: match &v.figi {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        isin: match &v.isin {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        listings: f4.0,
        listings_len: f4.1,
        has_primary: v.primary.is_some(),
        primary: v.primary.unwrap_or_default(),
        title: match &v.title {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: security_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `CompanyWithListings`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_company_with_listings {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub has_delisted: bool,
    pub delisted: bool,
    pub fiscal_year_end: *const c_char,
    pub id: *const c_char,
    pub industry: *const nw_company_industry,
    pub jp_corporate_number: *const c_char,
    pub lei: *const c_char,
    pub listings: *const *const c_char,
    pub listings_len: usize,
    pub name: *const c_char,
    pub names: *const *const nw_company_name_span,
    pub names_len: usize,
    pub related: *const *const nw_company_relation,
    pub related_len: usize,
    pub securities: *const *const nw_company_security,
    pub securities_len: usize,
    pub standard: i32,
    pub standard_raw: *const c_char,
    pub standard_variant: *const c_char,
    pub status_date: *const c_char,
}
pub(crate) fn mirror_company_with_listings(
    v: &sdk::models::CompanyWithListings,
    s: &mut Store,
) -> Result<nw_company_with_listings, Unrepresentable> {
    let f7 = match &v.listings {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f9 = {
        let items = v
            .names
            .iter()
            .map(|x| {
                let m = mirror_company_name_span(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f10 = {
        let items = v
            .related
            .iter()
            .map(|x| {
                let m = mirror_company_relation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f11 = {
        let items = v
            .securities
            .iter()
            .map(|x| {
                let m = mirror_company_security(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_company_with_listings {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_delisted: v.delisted.is_some(),
        delisted: v.delisted.unwrap_or_default(),
        fiscal_year_end: match &v.fiscal_year_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        id: s.str(&v.id)?,
        industry: match &v.industry {
            Some(x) => {
                let v = mirror_company_industry(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        jp_corporate_number: match &v.jp_corporate_number {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        lei: match &v.lei {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        listings: f7.0,
        listings_len: f7.1,
        name: s.str(&v.name)?,
        names: f9.0,
        names_len: f9.1,
        related: f10.0,
        related_len: f10.1,
        securities: f11.0,
        securities_len: f11.1,
        standard: v
            .standard
            .as_ref()
            .map_or(0, |x| company_standard_code(x.as_str())),
        standard_raw: match &v.standard {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        standard_variant: match &v.standard_variant {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        status_date: match &v.status_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `SeriesSource`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_series_source {
    pub distributor: *const c_char,
    pub publisher: *const c_char,
}
pub(crate) fn mirror_series_source(
    v: &sdk::models::SeriesSource,
    s: &mut Store,
) -> Result<nw_series_source, Unrepresentable> {
    Ok(nw_series_source {
        distributor: s.str(&v.distributor)?,
        publisher: s.str(&v.publisher)?,
    })
}
/// Mirror of `Cpi`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_cpi {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
    pub value: *const c_char,
    pub index_base: *const c_char,
}
pub(crate) fn mirror_cpi(v: &sdk::models::Cpi, s: &mut Store) -> Result<nw_cpi, Unrepresentable> {
    Ok(nw_cpi {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        period: s.str(&v.period)?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        value: s.str(v.value.as_str())?,
        index_base: s.str(&v.index_base)?,
    })
}
/// Mirror of `CpiHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cpi_history_observation {
    pub period: *const c_char,
    pub value: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_cpi_history_observation(
    v: &sdk::models::CpiHistoryObservation,
    s: &mut Store,
) -> Result<nw_cpi_history_observation, Unrepresentable> {
    Ok(nw_cpi_history_observation {
        period: s.str(&v.period)?,
        value: s.str(v.value.as_str())?,
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CpiHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_cpi_history {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub index_base: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_cpi_history_observation,
    pub observations_len: usize,
    pub source: *const nw_series_source,
    pub start: *const c_char,
}
pub(crate) fn mirror_cpi_history(
    v: &sdk::models::CpiHistory,
    s: &mut Store,
) -> Result<nw_cpi_history, Unrepresentable> {
    let f4 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_cpi_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_cpi_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        index_base: s.str(&v.index_base)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f4.0,
        observations_len: f4.1,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CryptoBar`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_crypto_bar {
    pub close: *const c_char,
    pub high: *const c_char,
    pub low: *const c_char,
    pub open: *const c_char,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_crypto_bar(
    v: &sdk::models::CryptoBar,
    s: &mut Store,
) -> Result<nw_crypto_bar, Unrepresentable> {
    Ok(nw_crypto_bar {
        close: s.str(v.close.as_str())?,
        high: s.str(v.high.as_str())?,
        low: s.str(v.low.as_str())?,
        open: s.str(v.open.as_str())?,
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `CryptoBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_crypto_bars_model {
    pub meta: *const nw_response_meta,
    pub bars: *const *const nw_crypto_bar,
    pub bars_len: usize,
    pub end: *const c_char,
    pub next_cursor: *const c_char,
    pub pair: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_crypto_bars(
    v: &sdk::models::CryptoBars,
    s: &mut Store,
) -> Result<nw_crypto_bars_model, Unrepresentable> {
    let f0 = {
        let items = v
            .bars
            .iter()
            .map(|x| {
                let m = mirror_crypto_bar(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_crypto_bars_model {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        bars: f0.0,
        bars_len: f0.1,
        end: match &v.end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        pair: s.str(&v.pair)?,
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CryptoPair`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_crypto_pair {
    pub base: *const c_char,
    pub name: *const c_char,
    pub pair: *const c_char,
    pub quote: *const c_char,
}
pub(crate) fn mirror_crypto_pair(
    v: &sdk::models::CryptoPair,
    s: &mut Store,
) -> Result<nw_crypto_pair, Unrepresentable> {
    Ok(nw_crypto_pair {
        base: s.str(&v.base)?,
        name: s.str(&v.name)?,
        pair: s.str(&v.pair)?,
        quote: s.str(&v.quote)?,
    })
}
/// Mirror of `CryptoPairs`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_crypto_pairs {
    pub meta: *const nw_response_meta,
    pub pairs: *const *const nw_crypto_pair,
    pub pairs_len: usize,
}
pub(crate) fn mirror_crypto_pairs(
    v: &sdk::models::CryptoPairs,
    s: &mut Store,
) -> Result<nw_crypto_pairs, Unrepresentable> {
    let f0 = {
        let items = v
            .pairs
            .iter()
            .map(|x| {
                let m = mirror_crypto_pair(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_crypto_pairs {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        pairs: f0.0,
        pairs_len: f0.1,
    })
}
/// Mirror of `CryptoPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_crypto_price_model {
    pub meta: *const nw_response_meta,
    pub base: *const c_char,
    pub change_24h: *const c_char,
    pub change_24h_percent: *const c_char,
    pub has_delayed: bool,
    pub delayed: bool,
    pub pair: *const c_char,
    pub price: *const c_char,
    pub quote: *const c_char,
    pub stale: bool,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_crypto_price(
    v: &sdk::models::CryptoPrice,
    s: &mut Store,
) -> Result<nw_crypto_price_model, Unrepresentable> {
    Ok(nw_crypto_price_model {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        base: s.str(&v.base)?,
        change_24h: match &v.change_24h {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        change_24h_percent: match &v.change_24h_percent {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        has_delayed: v.delayed.is_some(),
        delayed: v.delayed.unwrap_or_default(),
        pair: s.str(&v.pair)?,
        price: s.str(v.price.as_str())?,
        quote: s.str(&v.quote)?,
        stale: v.stale,
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `CryptoPriceError`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_crypto_price_error {
    pub code: i32,
    pub code_raw: *const c_char,
    pub id: *const c_char,
}
pub(crate) fn mirror_crypto_price_error(
    v: &sdk::models::CryptoPriceError,
    s: &mut Store,
) -> Result<nw_crypto_price_error, Unrepresentable> {
    Ok(nw_crypto_price_error {
        code: crypto_price_error_code_code(v.code.as_str()),
        code_raw: s.str(v.code.as_str())?,
        id: s.str(&v.id)?,
    })
}
/// Mirror of `CryptoPrices`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_crypto_prices_model {
    pub meta: *const nw_response_meta,
    pub errors: *const *const nw_crypto_price_error,
    pub errors_len: usize,
    pub prices: *const *const nw_crypto_price_model,
    pub prices_len: usize,
}
pub(crate) fn mirror_crypto_prices(
    v: &sdk::models::CryptoPrices,
    s: &mut Store,
) -> Result<nw_crypto_prices_model, Unrepresentable> {
    let f0 = {
        let items = v
            .errors
            .iter()
            .map(|x| {
                let m = mirror_crypto_price_error(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f1 = {
        let items = v
            .prices
            .iter()
            .map(|x| {
                let m = mirror_crypto_price(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_crypto_prices_model {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        errors: f0.0,
        errors_len: f0.1,
        prices: f1.0,
        prices_len: f1.1,
    })
}
/// Mirror of `DividendsCalendarEntry`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_dividends_calendar_entry {
    pub amount: *const c_char,
    pub currency: *const c_char,
    pub ex_date: *const c_char,
    pub id: *const c_char,
    pub name: *const c_char,
    pub symbol: *const c_char,
}
pub(crate) fn mirror_dividends_calendar_entry(
    v: &sdk::models::DividendsCalendarEntry,
    s: &mut Store,
) -> Result<nw_dividends_calendar_entry, Unrepresentable> {
    Ok(nw_dividends_calendar_entry {
        amount: s.str(v.amount.as_str())?,
        currency: s.str(&v.currency)?,
        ex_date: s.str(&v.ex_date)?,
        id: s.str(&v.id)?,
        name: s.str(&v.name)?,
        symbol: s.str(&v.symbol)?,
    })
}
/// Mirror of `DividendsCalendar`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_dividends_calendar {
    pub meta: *const nw_response_meta,
    pub entries: *const *const nw_dividends_calendar_entry,
    pub entries_len: usize,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_dividends_calendar(
    v: &sdk::models::DividendsCalendar,
    s: &mut Store,
) -> Result<nw_dividends_calendar, Unrepresentable> {
    let f0 = {
        let items = v
            .entries
            .iter()
            .map(|x| {
                let m = mirror_dividends_calendar_entry(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_dividends_calendar {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        entries: f0.0,
        entries_len: f0.1,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EconomicReleaseValue`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_economic_release_value {
    pub actual: *const c_char,
    pub has_annualized: bool,
    pub annualized: bool,
    pub currency: *const c_char,
    pub index_base: *const c_char,
    pub measure: *const c_char,
    pub has_not_seasonally_adjusted: bool,
    pub not_seasonally_adjusted: bool,
    pub period: *const c_char,
    pub previous: *const c_char,
    pub previous_revised_from: *const c_char,
    pub scale: *const c_char,
    pub transformation: *const c_char,
    pub unit: i32,
    pub unit_raw: *const c_char,
    pub value_status: i32,
    pub value_status_raw: *const c_char,
}
pub(crate) fn mirror_economic_release_value(
    v: &sdk::models::EconomicReleaseValue,
    s: &mut Store,
) -> Result<nw_economic_release_value, Unrepresentable> {
    Ok(nw_economic_release_value {
        actual: match &v.actual {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        has_annualized: v.annualized.is_some(),
        annualized: v.annualized.unwrap_or_default(),
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        index_base: match &v.index_base {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        measure: s.str(&v.measure)?,
        has_not_seasonally_adjusted: v.not_seasonally_adjusted.is_some(),
        not_seasonally_adjusted: v.not_seasonally_adjusted.unwrap_or_default(),
        period: match &v.period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        previous: match &v.previous {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        previous_revised_from: match &v.previous_revised_from {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        scale: match &v.scale {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        transformation: match &v.transformation {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        unit: indicator_unit_code(v.unit.as_str()),
        unit_raw: s.str(v.unit.as_str())?,
        value_status: v
            .value_status
            .as_ref()
            .map_or(0, |x| economic_release_value_value_status_code(x.as_str())),
        value_status_raw: match &v.value_status {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EconomicReleaseSummary`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_economic_release_summary {
    pub at: *const c_char,
    pub between: *const *const c_char,
    pub between_len: usize,
    pub by: *const c_char,
    pub country: *const c_char,
    pub date: *const c_char,
    pub family: i32,
    pub family_raw: *const c_char,
    pub name: *const c_char,
    pub notice: *const c_char,
    pub period: *const c_char,
    pub release_id: *const c_char,
    pub revision: i64,
    pub series_id: *const c_char,
    pub stage: i32,
    pub stage_raw: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub timezone: *const c_char,
    pub topic: i32,
    pub topic_raw: *const c_char,
    pub values: *const *const nw_economic_release_value,
    pub values_len: usize,
}
pub(crate) fn mirror_economic_release_summary(
    v: &sdk::models::EconomicReleaseSummary,
    s: &mut Store,
) -> Result<nw_economic_release_summary, Unrepresentable> {
    let f1 = match &v.between {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f16 = {
        let items = v
            .values
            .iter()
            .map(|x| {
                let m = mirror_economic_release_value(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_economic_release_summary {
        at: match &v.at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        between: f1.0,
        between_len: f1.1,
        by: match &v.by {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        date: match &v.date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        family: economic_family_code(v.family.as_str()),
        family_raw: s.str(v.family.as_str())?,
        name: s.str(&v.name)?,
        notice: match &v.notice {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        period: match &v.period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        release_id: s.str(&v.release_id)?,
        revision: v.revision,
        series_id: match &v.series_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        stage: v
            .stage
            .as_ref()
            .map_or(0, |x| release_stage_code(x.as_str())),
        stage_raw: match &v.stage {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        status: release_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        timezone: match &v.timezone {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        topic: v
            .topic
            .as_ref()
            .map_or(0, |x| release_topic_code(x.as_str())),
        topic_raw: match &v.topic {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        values: f16.0,
        values_len: f16.1,
    })
}
/// Mirror of `EconomicCalendar`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_economic_calendar {
    pub meta: *const nw_response_meta,
    pub next_cursor: *const c_char,
    pub releases_len: usize,
    pub releases: *const *const nw_economic_release_summary,
}
pub(crate) fn mirror_economic_calendar(
    v: &sdk::models::EconomicCalendar,
    s: &mut Store,
) -> Result<nw_economic_calendar, Unrepresentable> {
    let f1 = {
        let items = v
            .releases
            .iter()
            .map(|x| {
                let m = mirror_economic_release_summary(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_economic_calendar {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        releases_len: f1.1,
        releases: f1.0,
    })
}
/// Mirror of `EconomicReleaseDetail`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_economic_release_detail {
    pub meta: *const nw_response_meta,
    pub at: *const c_char,
    pub between: *const *const c_char,
    pub between_len: usize,
    pub by: *const c_char,
    pub country: *const c_char,
    pub date: *const c_char,
    pub family: i32,
    pub family_raw: *const c_char,
    pub name: *const c_char,
    pub notice: *const c_char,
    pub observed_at: *const c_char,
    pub period: *const c_char,
    pub release_id: *const c_char,
    pub revision: i64,
    pub series_id: *const c_char,
    pub stage: i32,
    pub stage_raw: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub timezone: *const c_char,
    pub topic: i32,
    pub topic_raw: *const c_char,
    pub values: *const *const nw_economic_release_value,
    pub values_len: usize,
    pub version: *const c_char,
}
pub(crate) fn mirror_economic_release_detail(
    v: &sdk::models::EconomicReleaseDetail,
    s: &mut Store,
) -> Result<nw_economic_release_detail, Unrepresentable> {
    let f1 = match &v.between {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f17 = {
        let items = v
            .values
            .iter()
            .map(|x| {
                let m = mirror_economic_release_value(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_economic_release_detail {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        at: match &v.at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        between: f1.0,
        between_len: f1.1,
        by: match &v.by {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        date: match &v.date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        family: economic_family_code(v.family.as_str()),
        family_raw: s.str(v.family.as_str())?,
        name: s.str(&v.name)?,
        notice: match &v.notice {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observed_at: match &v.observed_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        period: match &v.period {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        release_id: s.str(&v.release_id)?,
        revision: v.revision,
        series_id: match &v.series_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        stage: v
            .stage
            .as_ref()
            .map_or(0, |x| release_stage_code(x.as_str())),
        stage_raw: match &v.stage {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        status: release_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        timezone: match &v.timezone {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        topic: v
            .topic
            .as_ref()
            .map_or(0, |x| release_topic_code(x.as_str())),
        topic_raw: match &v.topic {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        values: f17.0,
        values_len: f17.1,
        version: s.str(&v.version)?,
    })
}
/// Mirror of `EquityBar`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_bar {
    pub close: *const c_char,
    pub date: *const c_char,
    pub high: *const c_char,
    pub low: *const c_char,
    pub open: *const c_char,
    pub timestamp: *const c_char,
    pub volume: *const c_char,
}
pub(crate) fn mirror_equity_bar(
    v: &sdk::models::EquityBar,
    s: &mut Store,
) -> Result<nw_equity_bar, Unrepresentable> {
    Ok(nw_equity_bar {
        close: s.str(v.close.as_str())?,
        date: match &v.date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        high: s.str(v.high.as_str())?,
        low: s.str(v.low.as_str())?,
        open: s.str(v.open.as_str())?,
        timestamp: s.str(&v.timestamp)?,
        volume: match &v.volume {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_bars {
    pub meta: *const nw_response_meta,
    pub adjusted: bool,
    pub bars: *const *const nw_equity_bar,
    pub bars_len: usize,
    pub currency: *const c_char,
    pub end: *const c_char,
    pub id: *const c_char,
    pub next_cursor: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_equity_bars(
    v: &sdk::models::EquityBars,
    s: &mut Store,
) -> Result<nw_equity_bars, Unrepresentable> {
    let f1 = {
        let items = v
            .bars
            .iter()
            .map(|x| {
                let m = mirror_equity_bar(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_bars {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        adjusted: v.adjusted,
        bars: f1.0,
        bars_len: f1.1,
        currency: s.str(&v.currency)?,
        end: match &v.end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        id: s.str(&v.id)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CostBenchmark`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_benchmark {
    pub has_default_: bool,
    pub default_: bool,
    pub id: *const c_char,
}
pub(crate) fn mirror_cost_benchmark(
    v: &sdk::models::CostBenchmark,
    s: &mut Store,
) -> Result<nw_cost_benchmark, Unrepresentable> {
    Ok(nw_cost_benchmark {
        has_default_: v.default.is_some(),
        default_: v.default.unwrap_or_default(),
        id: s.str(&v.id)?,
    })
}
/// Mirror of `CostBeta`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_beta {
    pub adjusted: *const c_char,
    pub benchmark: *const nw_cost_benchmark,
    pub end: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub observations: i64,
    pub r_squared: *const c_char,
    pub raw: *const c_char,
    pub start: *const c_char,
    pub std_error: *const c_char,
    pub used: i32,
    pub used_raw: *const c_char,
    pub window: i32,
    pub window_raw: *const c_char,
}
pub(crate) fn mirror_cost_beta(
    v: &sdk::models::CostBeta,
    s: &mut Store,
) -> Result<nw_cost_beta, Unrepresentable> {
    Ok(nw_cost_beta {
        adjusted: s.str(v.adjusted.as_str())?,
        benchmark: {
            let v = mirror_cost_benchmark(&v.benchmark, s)?;
            s.boxed(v)
        },
        end: s.str(&v.end)?,
        frequency: cost_of_capital_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        observations: v.observations,
        r_squared: s.str(v.r_squared.as_str())?,
        raw: s.str(v.raw.as_str())?,
        start: s.str(&v.start)?,
        std_error: s.str(v.std_error.as_str())?,
        used: cost_of_capital_beta_code(v.used.as_str()),
        used_raw: s.str(v.used.as_str())?,
        window: cost_of_capital_window_code(v.window.as_str()),
        window_raw: s.str(v.window.as_str())?,
    })
}
/// Mirror of `CostOfDebt`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_of_debt {
    pub after_tax: *const c_char,
    pub average_debt: *const c_char,
    pub closing_debt: *const c_char,
    pub interest_expense: *const c_char,
    pub opening_debt: *const c_char,
    pub period: i32,
    pub period_raw: *const c_char,
    pub period_end: *const c_char,
    pub pre_tax: *const c_char,
    pub basis: i32,
    pub basis_raw: *const c_char,
    pub default_spread: *const c_char,
    pub interest_coverage: *const c_char,
    pub table: *const c_char,
    pub table_update: *const c_char,
    pub table_url: *const c_char,
}
pub(crate) fn mirror_cost_of_debt(
    v: &sdk::models::CostOfDebt,
    s: &mut Store,
) -> Result<nw_cost_of_debt, Unrepresentable> {
    Ok(nw_cost_of_debt {
        after_tax: match &v.after_tax {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        average_debt: s.str(v.average_debt.as_str())?,
        closing_debt: s.str(v.closing_debt.as_str())?,
        interest_expense: s.str(v.interest_expense.as_str())?,
        opening_debt: match &v.opening_debt {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        period: cost_of_capital_period_code(v.period.as_str()),
        period_raw: s.str(v.period.as_str())?,
        period_end: s.str(&v.period_end)?,
        pre_tax: s.str(v.pre_tax.as_str())?,
        basis: cost_of_debt_basis_code(v.basis.as_str()),
        basis_raw: s.str(v.basis.as_str())?,
        default_spread: match &v.default_spread {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        interest_coverage: match &v.interest_coverage {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        table: match &v.table {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        table_update: match &v.table_update {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        table_url: match &v.table_url {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CostEquityRiskPremium`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_equity_risk_premium {
    pub rate: *const c_char,
    pub as_of: *const c_char,
    pub basis: i32,
    pub basis_raw: *const c_char,
    pub country: *const c_char,
    pub source: i32,
    pub source_raw: *const c_char,
}
pub(crate) fn mirror_cost_equity_risk_premium(
    v: &sdk::models::CostEquityRiskPremium,
    s: &mut Store,
) -> Result<nw_cost_equity_risk_premium, Unrepresentable> {
    Ok(nw_cost_equity_risk_premium {
        rate: s.str(v.rate.as_str())?,
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        basis: v
            .basis
            .as_ref()
            .map_or(0, |x| cost_equity_risk_premium_basis_code(x.as_str())),
        basis_raw: match &v.basis {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        source: cost_equity_risk_premium_source_code(v.source.as_str()),
        source_raw: s.str(v.source.as_str())?,
    })
}
/// Mirror of `CostRiskFree`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_risk_free {
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub curve_type: *const c_char,
    pub maturity: *const c_char,
    pub rate: *const c_char,
    pub source: i32,
    pub source_raw: *const c_char,
    pub basis: i32,
    pub basis_raw: *const c_char,
}
pub(crate) fn mirror_cost_risk_free(
    v: &sdk::models::CostRiskFree,
    s: &mut Store,
) -> Result<nw_cost_risk_free, Unrepresentable> {
    Ok(nw_cost_risk_free {
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        curve_type: match &v.curve_type {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        maturity: match &v.maturity {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        rate: s.str(v.rate.as_str())?,
        source: cost_risk_free_source_code(v.source.as_str()),
        source_raw: s.str(v.source.as_str())?,
        basis: cost_risk_free_basis_code(v.basis.as_str()),
        basis_raw: s.str(v.basis.as_str())?,
    })
}
/// Mirror of `CostTaxRate`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_tax_rate {
    pub rate: *const c_char,
    pub source: i32,
    pub source_raw: *const c_char,
    pub basis: i32,
    pub basis_raw: *const c_char,
    pub country: *const c_char,
    pub table: *const c_char,
    pub table_update: *const c_char,
    pub table_url: *const c_char,
}
pub(crate) fn mirror_cost_tax_rate(
    v: &sdk::models::CostTaxRate,
    s: &mut Store,
) -> Result<nw_cost_tax_rate, Unrepresentable> {
    Ok(nw_cost_tax_rate {
        rate: s.str(v.rate.as_str())?,
        source: cost_tax_rate_source_code(v.source.as_str()),
        source_raw: s.str(v.source.as_str())?,
        basis: cost_tax_rate_basis_code(v.basis.as_str()),
        basis_raw: s.str(v.basis.as_str())?,
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        table: match &v.table {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        table_update: match &v.table_update {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        table_url: match &v.table_url {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `CostWarning`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_warning {
    pub code: *const c_char,
    pub message: *const c_char,
}
pub(crate) fn mirror_cost_warning(
    v: &sdk::models::CostWarning,
    s: &mut Store,
) -> Result<nw_cost_warning, Unrepresentable> {
    Ok(nw_cost_warning {
        code: s.str(&v.code)?,
        message: s.str(&v.message)?,
    })
}
/// Mirror of `CostWeights`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_cost_weights {
    pub debt: *const c_char,
    pub equity: *const c_char,
    pub market_cap: *const c_char,
    pub market_cap_as_of: *const c_char,
    pub total_debt: *const c_char,
    pub total_debt_as_of: *const c_char,
}
pub(crate) fn mirror_cost_weights(
    v: &sdk::models::CostWeights,
    s: &mut Store,
) -> Result<nw_cost_weights, Unrepresentable> {
    Ok(nw_cost_weights {
        debt: s.str(v.debt.as_str())?,
        equity: s.str(v.equity.as_str())?,
        market_cap: s.str(v.market_cap.as_str())?,
        market_cap_as_of: s.str(&v.market_cap_as_of)?,
        total_debt: s.str(v.total_debt.as_str())?,
        total_debt_as_of: s.str(&v.total_debt_as_of)?,
    })
}
/// Mirror of `EquityCostOfCapital`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_cost_of_capital {
    pub meta: *const nw_response_meta,
    pub beta: *const nw_cost_beta,
    pub company_id: *const c_char,
    pub cost_of_debt: *const nw_cost_of_debt,
    pub cost_of_equity: *const c_char,
    pub currency: *const c_char,
    pub equity_risk_premium: *const nw_cost_equity_risk_premium,
    pub id: *const c_char,
    pub methodology_url: *const c_char,
    pub risk_free: *const nw_cost_risk_free,
    pub tax_rate: *const nw_cost_tax_rate,
    pub wacc: *const c_char,
    pub warnings: *const *const nw_cost_warning,
    pub warnings_len: usize,
    pub weights: *const nw_cost_weights,
}
pub(crate) fn mirror_equity_cost_of_capital(
    v: &sdk::models::EquityCostOfCapital,
    s: &mut Store,
) -> Result<nw_equity_cost_of_capital, Unrepresentable> {
    let f11 = {
        let items = v
            .warnings
            .iter()
            .map(|x| {
                let m = mirror_cost_warning(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_cost_of_capital {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        beta: {
            let v = mirror_cost_beta(&v.beta, s)?;
            s.boxed(v)
        },
        company_id: s.str(&v.company_id)?,
        cost_of_debt: match &v.cost_of_debt {
            Some(x) => {
                let v = mirror_cost_of_debt(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        cost_of_equity: s.str(v.cost_of_equity.as_str())?,
        currency: s.str(&v.currency)?,
        equity_risk_premium: {
            let v = mirror_cost_equity_risk_premium(&v.equity_risk_premium, s)?;
            s.boxed(v)
        },
        id: s.str(&v.id)?,
        methodology_url: s.str(&v.methodology_url)?,
        risk_free: {
            let v = mirror_cost_risk_free(&v.risk_free, s)?;
            s.boxed(v)
        },
        tax_rate: match &v.tax_rate {
            Some(x) => {
                let v = mirror_cost_tax_rate(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        wacc: match &v.wacc {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        warnings: f11.0,
        warnings_len: f11.1,
        weights: {
            let v = mirror_cost_weights(&v.weights, s)?;
            s.boxed(v)
        },
    })
}
/// Mirror of `EquityDividend`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_dividend {
    pub amount: *const c_char,
    pub currency: *const c_char,
    pub ex_date: *const c_char,
    pub pay_date: *const c_char,
    pub record_date: *const c_char,
}
pub(crate) fn mirror_equity_dividend(
    v: &sdk::models::EquityDividend,
    s: &mut Store,
) -> Result<nw_equity_dividend, Unrepresentable> {
    Ok(nw_equity_dividend {
        amount: s.str(v.amount.as_str())?,
        currency: s.str(&v.currency)?,
        ex_date: s.str(&v.ex_date)?,
        pay_date: match &v.pay_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        record_date: match &v.record_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityDividends`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_dividends {
    pub meta: *const nw_response_meta,
    pub dividends: *const *const nw_equity_dividend,
    pub dividends_len: usize,
    pub id: *const c_char,
    pub next_cursor: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_equity_dividends(
    v: &sdk::models::EquityDividends,
    s: &mut Store,
) -> Result<nw_equity_dividends, Unrepresentable> {
    let f0 = {
        let items = v
            .dividends
            .iter()
            .map(|x| {
                let m = mirror_equity_dividend(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_dividends {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        dividends: f0.0,
        dividends_len: f0.1,
        id: s.str(&v.id)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityEarning`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_earning {
    pub date: *const c_char,
    pub eps_actual: *const c_char,
    pub eps_estimate: *const c_char,
    pub filing_id: *const c_char,
    pub surprise: *const c_char,
    pub surprise_percent: *const c_char,
    pub time: i32,
    pub time_raw: *const c_char,
}
pub(crate) fn mirror_equity_earning(
    v: &sdk::models::EquityEarning,
    s: &mut Store,
) -> Result<nw_equity_earning, Unrepresentable> {
    Ok(nw_equity_earning {
        date: s.str(&v.date)?,
        eps_actual: match &v.eps_actual {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        eps_estimate: match &v.eps_estimate {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        filing_id: match &v.filing_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        surprise: match &v.surprise {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        surprise_percent: match &v.surprise_percent {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        time: v
            .time
            .as_ref()
            .map_or(0, |x| equity_earning_time_code(x.as_str())),
        time_raw: match &v.time {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityEarnings`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_earnings {
    pub meta: *const nw_response_meta,
    pub earnings: *const *const nw_equity_earning,
    pub earnings_len: usize,
    pub id: *const c_char,
    pub next_cursor: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_equity_earnings(
    v: &sdk::models::EquityEarnings,
    s: &mut Store,
) -> Result<nw_equity_earnings, Unrepresentable> {
    let f0 = {
        let items = v
            .earnings
            .iter()
            .map(|x| {
                let m = mirror_equity_earning(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_earnings {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        earnings: f0.0,
        earnings_len: f0.1,
        id: s.str(&v.id)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityMover`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_mover {
    pub change_percent: *const c_char,
    pub id: *const c_char,
    pub name: *const c_char,
    pub price: *const c_char,
    pub symbol: *const c_char,
    pub volume: *const c_char,
}
pub(crate) fn mirror_equity_mover(
    v: &sdk::models::EquityMover,
    s: &mut Store,
) -> Result<nw_equity_mover, Unrepresentable> {
    Ok(nw_equity_mover {
        change_percent: s.str(v.change_percent.as_str())?,
        id: s.str(&v.id)?,
        name: s.str(&v.name)?,
        price: s.str(v.price.as_str())?,
        symbol: s.str(&v.symbol)?,
        volume: s.str(v.volume.as_str())?,
    })
}
/// Mirror of `EquityMovers`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_movers {
    pub meta: *const nw_response_meta,
    pub direction: i32,
    pub direction_raw: *const c_char,
    pub market: *const c_char,
    pub movers: *const *const nw_equity_mover,
    pub movers_len: usize,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_equity_movers(
    v: &sdk::models::EquityMovers,
    s: &mut Store,
) -> Result<nw_equity_movers, Unrepresentable> {
    let f2 = {
        let items = v
            .movers
            .iter()
            .map(|x| {
                let m = mirror_equity_mover(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_movers {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        direction: movers_direction_code(v.direction.as_str()),
        direction_raw: s.str(v.direction.as_str())?,
        market: s.str(&v.market)?,
        movers: f2.0,
        movers_len: f2.1,
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `EquityPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_price {
    pub meta: *const nw_response_meta,
    pub change: *const c_char,
    pub change_percent: *const c_char,
    pub currency: *const c_char,
    pub has_delayed: bool,
    pub delayed: bool,
    pub id: *const c_char,
    pub market_session: i32,
    pub market_session_raw: *const c_char,
    pub previous_close: *const c_char,
    pub price: *const c_char,
    pub stale: bool,
    pub symbol: *const c_char,
    pub timestamp: *const c_char,
    pub volume: *const c_char,
}
pub(crate) fn mirror_equity_price(
    v: &sdk::models::EquityPrice,
    s: &mut Store,
) -> Result<nw_equity_price, Unrepresentable> {
    Ok(nw_equity_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        change: match &v.change {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        change_percent: match &v.change_percent {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        has_delayed: v.delayed.is_some(),
        delayed: v.delayed.unwrap_or_default(),
        id: s.str(&v.id)?,
        market_session: equity_market_session_code(v.market_session.as_str()),
        market_session_raw: s.str(v.market_session.as_str())?,
        previous_close: match &v.previous_close {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        price: s.str(v.price.as_str())?,
        stale: v.stale,
        symbol: s.str(&v.symbol)?,
        timestamp: s.str(&v.timestamp)?,
        volume: match &v.volume {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityPriceError`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_price_error {
    pub code: i32,
    pub code_raw: *const c_char,
    pub id: *const c_char,
}
pub(crate) fn mirror_equity_price_error(
    v: &sdk::models::EquityPriceError,
    s: &mut Store,
) -> Result<nw_equity_price_error, Unrepresentable> {
    Ok(nw_equity_price_error {
        code: crypto_price_error_code_code(v.code.as_str()),
        code_raw: s.str(v.code.as_str())?,
        id: s.str(&v.id)?,
    })
}
/// Mirror of `EquityPrices`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_prices {
    pub meta: *const nw_response_meta,
    pub errors: *const *const nw_equity_price_error,
    pub errors_len: usize,
    pub prices: *const *const nw_equity_price,
    pub prices_len: usize,
}
pub(crate) fn mirror_equity_prices(
    v: &sdk::models::EquityPrices,
    s: &mut Store,
) -> Result<nw_equity_prices, Unrepresentable> {
    let f0 = {
        let items = v
            .errors
            .iter()
            .map(|x| {
                let m = mirror_equity_price_error(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f1 = {
        let items = v
            .prices
            .iter()
            .map(|x| {
                let m = mirror_equity_price(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_prices {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        errors: f0.0,
        errors_len: f0.1,
        prices: f1.0,
        prices_len: f1.1,
    })
}
/// Mirror of `EquityProfile`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_profile {
    pub meta: *const nw_response_meta,
    pub company_id: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub id: *const c_char,
    pub industry: *const c_char,
    pub logo_url: *const c_char,
    pub mic: *const c_char,
    pub name: *const c_char,
    pub sector: *const c_char,
    pub symbol: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
    pub website: *const c_char,
}
pub(crate) fn mirror_equity_profile(
    v: &sdk::models::EquityProfile,
    s: &mut Store,
) -> Result<nw_equity_profile, Unrepresentable> {
    Ok(nw_equity_profile {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        company_id: match &v.company_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        id: s.str(&v.id)?,
        industry: match &v.industry {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        logo_url: match &v.logo_url {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        mic: s.str(&v.mic)?,
        name: s.str(&v.name)?,
        sector: match &v.sector {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        symbol: s.str(&v.symbol)?,
        type_: equity_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
        website: match &v.website {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityRiskPremiumBasis`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_risk_premium_basis {
    pub country_default_spread: i32,
    pub country_default_spread_raw: *const c_char,
    pub country_risk_premium: i32,
    pub country_risk_premium_raw: *const c_char,
    pub mature_erp: i32,
    pub mature_erp_raw: *const c_char,
    pub total_erp: i32,
    pub total_erp_raw: *const c_char,
    pub volatility_ratio: i32,
    pub volatility_ratio_raw: *const c_char,
}
pub(crate) fn mirror_equity_risk_premium_basis(
    v: &sdk::models::EquityRiskPremiumBasis,
    s: &mut Store,
) -> Result<nw_equity_risk_premium_basis, Unrepresentable> {
    Ok(nw_equity_risk_premium_basis {
        country_default_spread: cost_equity_risk_premium_basis_code(
            v.country_default_spread.as_str(),
        ),
        country_default_spread_raw: s.str(v.country_default_spread.as_str())?,
        country_risk_premium: cost_equity_risk_premium_basis_code(v.country_risk_premium.as_str()),
        country_risk_premium_raw: s.str(v.country_risk_premium.as_str())?,
        mature_erp: cost_equity_risk_premium_basis_code(v.mature_erp.as_str()),
        mature_erp_raw: s.str(v.mature_erp.as_str())?,
        total_erp: cost_equity_risk_premium_basis_code(v.total_erp.as_str()),
        total_erp_raw: s.str(v.total_erp.as_str())?,
        volatility_ratio: cost_equity_risk_premium_basis_code(v.volatility_ratio.as_str()),
        volatility_ratio_raw: s.str(v.volatility_ratio.as_str())?,
    })
}
/// Mirror of `EquityRiskPremium`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_risk_premium {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub basis: *const nw_equity_risk_premium_basis,
    pub country: *const c_char,
    pub country_default_spread: *const c_char,
    pub country_risk_premium: *const c_char,
    pub mature_erp: *const c_char,
    pub methodology_url: *const c_char,
    pub total_erp: *const c_char,
    pub unit: *const c_char,
    pub volatility_ratio: *const c_char,
}
pub(crate) fn mirror_equity_risk_premium(
    v: &sdk::models::EquityRiskPremium,
    s: &mut Store,
) -> Result<nw_equity_risk_premium, Unrepresentable> {
    Ok(nw_equity_risk_premium {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: s.str(&v.as_of)?,
        basis: {
            let v = mirror_equity_risk_premium_basis(&v.basis, s)?;
            s.boxed(v)
        },
        country: s.str(&v.country)?,
        country_default_spread: s.str(v.country_default_spread.as_str())?,
        country_risk_premium: s.str(v.country_risk_premium.as_str())?,
        mature_erp: s.str(v.mature_erp.as_str())?,
        methodology_url: match &v.methodology_url {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        total_erp: s.str(v.total_erp.as_str())?,
        unit: s.str(&v.unit)?,
        volatility_ratio: match &v.volatility_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityRiskPremiumHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_risk_premium_history_response {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_equity_risk_premium,
    pub observations_len: usize,
}
pub(crate) fn mirror_equity_risk_premium_history_response(
    v: &sdk::models::EquityRiskPremiumHistoryResponse,
    s: &mut Store,
) -> Result<nw_equity_risk_premium_history_response, Unrepresentable> {
    let f2 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_equity_risk_premium(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_risk_premium_history_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f2.0,
        observations_len: f2.1,
    })
}
/// Mirror of `EquityRiskPremiumList`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_risk_premium_list {
    pub meta: *const nw_response_meta,
    pub countries: *const *const nw_equity_risk_premium,
    pub countries_len: usize,
}
pub(crate) fn mirror_equity_risk_premium_list(
    v: &sdk::models::EquityRiskPremiumList,
    s: &mut Store,
) -> Result<nw_equity_risk_premium_list, Unrepresentable> {
    let f0 = {
        let items = v
            .countries
            .iter()
            .map(|x| {
                let m = mirror_equity_risk_premium(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_risk_premium_list {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        countries: f0.0,
        countries_len: f0.1,
    })
}
/// Mirror of `EquitySearchResult`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_search_result {
    pub company_id: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub id: *const c_char,
    pub mic: *const c_char,
    pub name: *const c_char,
    pub symbol: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_equity_search_result(
    v: &sdk::models::EquitySearchResult,
    s: &mut Store,
) -> Result<nw_equity_search_result, Unrepresentable> {
    Ok(nw_equity_search_result {
        company_id: match &v.company_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        id: s.str(&v.id)?,
        mic: s.str(&v.mic)?,
        name: s.str(&v.name)?,
        symbol: s.str(&v.symbol)?,
        type_: equity_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `EquitySearchResults`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_search_results {
    pub meta: *const nw_response_meta,
    pub next_cursor: *const c_char,
    pub results: *const *const nw_equity_search_result,
    pub results_len: usize,
}
pub(crate) fn mirror_equity_search_results(
    v: &sdk::models::EquitySearchResults,
    s: &mut Store,
) -> Result<nw_equity_search_results, Unrepresentable> {
    let f1 = {
        let items = v
            .results
            .iter()
            .map(|x| {
                let m = mirror_equity_search_result(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_search_results {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        results: f1.0,
        results_len: f1.1,
    })
}
/// Mirror of `EquitySplit`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_split {
    pub date: *const c_char,
    pub ratio: *const c_char,
}
pub(crate) fn mirror_equity_split(
    v: &sdk::models::EquitySplit,
    s: &mut Store,
) -> Result<nw_equity_split, Unrepresentable> {
    Ok(nw_equity_split {
        date: s.str(&v.date)?,
        ratio: s.str(&v.ratio)?,
    })
}
/// Mirror of `EquitySplits`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_splits {
    pub meta: *const nw_response_meta,
    pub id: *const c_char,
    pub splits: *const *const nw_equity_split,
    pub splits_len: usize,
    pub start: *const c_char,
}
pub(crate) fn mirror_equity_splits(
    v: &sdk::models::EquitySplits,
    s: &mut Store,
) -> Result<nw_equity_splits, Unrepresentable> {
    let f1 = {
        let items = v
            .splits
            .iter()
            .map(|x| {
                let m = mirror_equity_split(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_splits {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        id: s.str(&v.id)?,
        splits: f1.0,
        splits_len: f1.1,
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityValuation`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_valuation {
    pub meta: *const nw_response_meta,
    pub company_id: *const c_char,
    pub currency: *const c_char,
    pub dividend_per_share_ttm: *const c_char,
    pub dividend_yield: *const c_char,
    pub dividends_period_end: *const c_char,
    pub eps_period_end: *const c_char,
    pub eps_ttm: *const c_char,
    pub equity_as_of: *const c_char,
    pub id: *const c_char,
    pub market_cap: *const c_char,
    pub pb_ratio: *const c_char,
    pub pe_ratio: *const c_char,
    pub ps_ratio: *const c_char,
    pub revenue_period_end: *const c_char,
    pub revenue_ttm: *const c_char,
    pub shares_as_of: *const c_char,
    pub shares_outstanding: *const c_char,
    pub timestamp: *const c_char,
    pub total_equity: *const c_char,
}
pub(crate) fn mirror_equity_valuation(
    v: &sdk::models::EquityValuation,
    s: &mut Store,
) -> Result<nw_equity_valuation, Unrepresentable> {
    Ok(nw_equity_valuation {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        company_id: s.str(&v.company_id)?,
        currency: s.str(&v.currency)?,
        dividend_per_share_ttm: match &v.dividend_per_share_ttm {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        dividend_yield: match &v.dividend_yield {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        dividends_period_end: match &v.dividends_period_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        eps_period_end: match &v.eps_period_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        eps_ttm: match &v.eps_ttm {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        equity_as_of: match &v.equity_as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        id: s.str(&v.id)?,
        market_cap: match &v.market_cap {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        pb_ratio: match &v.pb_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        pe_ratio: match &v.pe_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        ps_ratio: match &v.ps_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        revenue_period_end: match &v.revenue_period_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        revenue_ttm: match &v.revenue_ttm {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        shares_as_of: match &v.shares_as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        shares_outstanding: match &v.shares_outstanding {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        timestamp: s.str(&v.timestamp)?,
        total_equity: match &v.total_equity {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityValuationPoint`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_equity_valuation_point {
    pub date: *const c_char,
    pub dividend_yield: *const c_char,
    pub market_cap: *const c_char,
    pub pb_ratio: *const c_char,
    pub pe_ratio: *const c_char,
    pub price: *const c_char,
    pub price_date: *const c_char,
    pub ps_ratio: *const c_char,
}
pub(crate) fn mirror_equity_valuation_point(
    v: &sdk::models::EquityValuationPoint,
    s: &mut Store,
) -> Result<nw_equity_valuation_point, Unrepresentable> {
    Ok(nw_equity_valuation_point {
        date: s.str(&v.date)?,
        dividend_yield: match &v.dividend_yield {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        market_cap: match &v.market_cap {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        pb_ratio: match &v.pb_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        pe_ratio: match &v.pe_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        price: match &v.price {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        price_date: match &v.price_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        ps_ratio: match &v.ps_ratio {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EquityValuationHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_equity_valuation_history {
    pub meta: *const nw_response_meta,
    pub company_id: *const c_char,
    pub currency: *const c_char,
    pub id: *const c_char,
    pub points: *const *const nw_equity_valuation_point,
    pub points_len: usize,
    pub start: *const c_char,
}
pub(crate) fn mirror_equity_valuation_history(
    v: &sdk::models::EquityValuationHistory,
    s: &mut Store,
) -> Result<nw_equity_valuation_history, Unrepresentable> {
    let f3 = {
        let items = v
            .points
            .iter()
            .map(|x| {
                let m = mirror_equity_valuation_point(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_equity_valuation_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        company_id: s.str(&v.company_id)?,
        currency: s.str(&v.currency)?,
        id: s.str(&v.id)?,
        points: f3.0,
        points_len: f3.1,
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `EventDetail`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_event_detail {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub cursor: *const c_char,
    pub event_id: *const c_char,
    pub family: i32,
    pub family_raw: *const c_char,
    pub has_holiday_generation: bool,
    pub holiday_generation: i64,
    pub has_holiday_year: bool,
    pub holiday_year: i64,
    pub recorded_at: *const c_char,
    pub release_id: *const c_char,
    pub resource_id: *const c_char,
    pub resource_version: *const c_char,
    pub type_: *const c_char,
}
pub(crate) fn mirror_event_detail(
    v: &sdk::models::EventDetail,
    s: &mut Store,
) -> Result<nw_event_detail, Unrepresentable> {
    Ok(nw_event_detail {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        cursor: s.str(&v.cursor)?,
        event_id: s.str(&v.event_id)?,
        family: delivery_family_code(v.family.as_str()),
        family_raw: s.str(v.family.as_str())?,
        has_holiday_generation: v.holiday_generation.is_some(),
        holiday_generation: v.holiday_generation.unwrap_or_default(),
        has_holiday_year: v.holiday_year.is_some(),
        holiday_year: v.holiday_year.unwrap_or_default(),
        recorded_at: s.str(&v.recorded_at)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resource_id: match &v.resource_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resource_version: match &v.resource_version {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: s.str(&v.type_)?,
    })
}
/// Mirror of `DeliveryEvent`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_delivery_event {
    pub country: *const c_char,
    pub cursor: *const c_char,
    pub event_id: *const c_char,
    pub family: i32,
    pub family_raw: *const c_char,
    pub has_holiday_generation: bool,
    pub holiday_generation: i64,
    pub has_holiday_year: bool,
    pub holiday_year: i64,
    pub recorded_at: *const c_char,
    pub release_id: *const c_char,
    pub resource_id: *const c_char,
    pub resource_version: *const c_char,
    pub type_: *const c_char,
}
pub(crate) fn mirror_delivery_event(
    v: &sdk::models::DeliveryEvent,
    s: &mut Store,
) -> Result<nw_delivery_event, Unrepresentable> {
    Ok(nw_delivery_event {
        country: s.str(&v.country)?,
        cursor: s.str(&v.cursor)?,
        event_id: s.str(&v.event_id)?,
        family: delivery_family_code(v.family.as_str()),
        family_raw: s.str(v.family.as_str())?,
        has_holiday_generation: v.holiday_generation.is_some(),
        holiday_generation: v.holiday_generation.unwrap_or_default(),
        has_holiday_year: v.holiday_year.is_some(),
        holiday_year: v.holiday_year.unwrap_or_default(),
        recorded_at: s.str(&v.recorded_at)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resource_id: match &v.resource_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resource_version: match &v.resource_version {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: s.str(&v.type_)?,
    })
}
/// Mirror of `EventsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_events_page {
    pub meta: *const nw_response_meta,
    pub cursor: *const c_char,
    pub events: *const *const nw_delivery_event,
    pub events_len: usize,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_events_page(
    v: &sdk::models::EventsPage,
    s: &mut Store,
) -> Result<nw_events_page, Unrepresentable> {
    let f1 = {
        let items = v
            .events
            .iter()
            .map(|x| {
                let m = mirror_delivery_event(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_events_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        cursor: s.str(&v.cursor)?,
        events: f1.0,
        events_len: f1.1,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `ExtremeLevel`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_extreme_level {
    pub key: *const c_char,
    pub level: *const c_char,
    pub probability: *const c_char,
    pub quotes: *const *const nw_quote,
    pub quotes_len: usize,
}
pub(crate) fn mirror_extreme_level(
    v: &sdk::models::ExtremeLevel,
    s: &mut Store,
) -> Result<nw_extreme_level, Unrepresentable> {
    let f3 = {
        let items = v
            .quotes
            .iter()
            .map(|x| {
                let m = mirror_quote(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_extreme_level {
        key: s.str(&v.key)?,
        level: s.str(v.level.as_str())?,
        probability: match &v.probability {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        quotes: f3.0,
        quotes_len: f3.1,
    })
}
/// Mirror of `ExtremeWindow`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_extreme_window {
    pub end: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_extreme_window(
    v: &sdk::models::ExtremeWindow,
    s: &mut Store,
) -> Result<nw_extreme_window, Unrepresentable> {
    Ok(nw_extreme_window {
        end: s.str(&v.end)?,
        start: s.str(&v.start)?,
    })
}
/// Mirror of `ExtremeEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_extreme_event {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub closes_at: *const c_char,
    pub event_id: *const c_char,
    pub question: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub topic_id: *const c_char,
    pub unit: *const c_char,
    pub venues: *const *const nw_prediction_venue,
    pub venues_len: usize,
    pub down: *const *const nw_extreme_level,
    pub down_len: usize,
    pub observation: i32,
    pub observation_raw: *const c_char,
    pub resolution: *const c_char,
    pub resolves_with: *const c_char,
    pub result: *const nw_prediction_result,
    pub has_stale: bool,
    pub stale: bool,
    pub up: *const *const nw_extreme_level,
    pub up_len: usize,
    pub window: *const nw_extreme_window,
}
pub(crate) fn mirror_extreme_event(
    v: &sdk::models::ExtremeEvent,
    s: &mut Store,
) -> Result<nw_extreme_event, Unrepresentable> {
    let f2 = {
        let items = v
            .down
            .iter()
            .map(|x| {
                let m = mirror_extreme_level(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f15 = {
        let items =
            v.up.iter()
                .map(|x| {
                    let m = mirror_extreme_level(x, s)?;
                    Ok::<_, Unrepresentable>(s.boxed(m))
                })
                .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f16 = {
        let items = v
            .venues
            .iter()
            .map(|x| {
                let m = mirror_prediction_venue(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_extreme_event {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        question: s.str(&v.question)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        status: prediction_event_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        topic_id: s.str(&v.topic_id)?,
        unit: match &v.unit {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f16.0,
        venues_len: f16.1,
        down: f2.0,
        down_len: f2.1,
        observation: extreme_observation_code(v.observation.as_str()),
        observation_raw: s.str(v.observation.as_str())?,
        resolution: s.str(&v.resolution)?,
        resolves_with: match &v.resolves_with {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        result: match &v.result {
            Some(x) => {
                let v = mirror_prediction_result(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
        up: f15.0,
        up_len: f15.1,
        window: {
            let v = mirror_extreme_window(&v.window, s)?;
            s.boxed(v)
        },
    })
}
/// Mirror of `Filing`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_filing {
    pub meta: *const nw_response_meta,
    pub amends: *const c_char,
    pub company_id: *const c_char,
    pub filed_at: *const c_char,
    pub filer_id: *const c_char,
    pub form: *const c_char,
    pub fp: i32,
    pub fp_raw: *const c_char,
    pub has_fy: bool,
    pub fy: i64,
    pub id: *const c_char,
    pub period_end: *const c_char,
    pub title: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
    pub url: *const c_char,
    pub has_withdrawn: bool,
    pub withdrawn: bool,
}
pub(crate) fn mirror_filing(
    v: &sdk::models::Filing,
    s: &mut Store,
) -> Result<nw_filing, Unrepresentable> {
    Ok(nw_filing {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        amends: match &v.amends {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        company_id: s.str(&v.company_id)?,
        filed_at: s.str(&v.filed_at)?,
        filer_id: match &v.filer_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        form: s.str(&v.form)?,
        fp: v.fp.as_ref().map_or(0, |x| fiscal_period_code(x.as_str())),
        fp_raw: match &v.fp {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        has_fy: v.fy.is_some(),
        fy: v.fy.unwrap_or_default(),
        id: s.str(&v.id)?,
        period_end: match &v.period_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        title: match &v.title {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: filing_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
        url: match &v.url {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_withdrawn: v.withdrawn.is_some(),
        withdrawn: v.withdrawn.unwrap_or_default(),
    })
}
/// Mirror of `FinancialRow`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_financial_row {
    pub currency: *const c_char,
    pub derived: *const *const c_char,
    pub derived_len: usize,
    pub end: *const c_char,
    pub has_fh: bool,
    pub fh: i64,
    pub has_fq: bool,
    pub fq: i64,
    pub fy: i64,
}
pub(crate) fn mirror_financial_row(
    v: &sdk::models::FinancialRow,
    s: &mut Store,
) -> Result<nw_financial_row, Unrepresentable> {
    let f1 = match &v.derived {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    Ok(nw_financial_row {
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        derived: f1.0,
        derived_len: f1.1,
        end: s.str(&v.end)?,
        has_fh: v.fh.is_some(),
        fh: v.fh.unwrap_or_default(),
        has_fq: v.fq.is_some(),
        fq: v.fq.unwrap_or_default(),
        fy: v.fy,
    })
}
/// Mirror of `Financials`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_financials {
    pub meta: *const nw_response_meta,
    pub annual: *const *const nw_financial_row,
    pub annual_len: usize,
    pub as_of: *const c_char,
    pub basis: i32,
    pub basis_raw: *const c_char,
    pub currency: *const c_char,
    pub has_from_fy: bool,
    pub from_fy: i64,
    pub id: *const c_char,
    pub quarterly: *const *const nw_financial_row,
    pub quarterly_len: usize,
    pub semiannual: *const *const nw_financial_row,
    pub semiannual_len: usize,
}
pub(crate) fn mirror_financials(
    v: &sdk::models::Financials,
    s: &mut Store,
) -> Result<nw_financials, Unrepresentable> {
    let f0 = match &v.annual {
        Some(a) => {
            let items = a
                .iter()
                .map(|x| {
                    let m = mirror_financial_row(x, s)?;
                    Ok::<_, Unrepresentable>(s.boxed(m))
                })
                .collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f6 = match &v.quarterly {
        Some(a) => {
            let items = a
                .iter()
                .map(|x| {
                    let m = mirror_financial_row(x, s)?;
                    Ok::<_, Unrepresentable>(s.boxed(m))
                })
                .collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f7 = match &v.semiannual {
        Some(a) => {
            let items = a
                .iter()
                .map(|x| {
                    let m = mirror_financial_row(x, s)?;
                    Ok::<_, Unrepresentable>(s.boxed(m))
                })
                .collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    Ok(nw_financials {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        annual: f0.0,
        annual_len: f0.1,
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        basis: v
            .basis
            .as_ref()
            .map_or(0, |x| financials_basis_code(x.as_str())),
        basis_raw: match &v.basis {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_from_fy: v.from_fy.is_some(),
        from_fy: v.from_fy.unwrap_or_default(),
        id: s.str(&v.id)?,
        quarterly: f6.0,
        quarterly_len: f6.1,
        semiannual: f7.0,
        semiannual_len: f7.1,
    })
}
/// Mirror of `FundDistribution`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fund_distribution {
    pub amount: *const c_char,
    pub currency: *const c_char,
    pub ex_date: *const c_char,
    pub kind: i32,
    pub kind_raw: *const c_char,
    pub pay_date: *const c_char,
    pub record_date: *const c_char,
}
pub(crate) fn mirror_fund_distribution(
    v: &sdk::models::FundDistribution,
    s: &mut Store,
) -> Result<nw_fund_distribution, Unrepresentable> {
    Ok(nw_fund_distribution {
        amount: s.str(v.amount.as_str())?,
        currency: s.str(&v.currency)?,
        ex_date: s.str(&v.ex_date)?,
        kind: v
            .kind
            .as_ref()
            .map_or(0, |x| fund_distribution_kind_code(x.as_str())),
        kind_raw: match &v.kind {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        pay_date: match &v.pay_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        record_date: match &v.record_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `FundDistributions`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fund_distributions {
    pub meta: *const nw_response_meta,
    pub distributions: *const *const nw_fund_distribution,
    pub distributions_len: usize,
    pub id: *const c_char,
    pub next_cursor: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_fund_distributions(
    v: &sdk::models::FundDistributions,
    s: &mut Store,
) -> Result<nw_fund_distributions, Unrepresentable> {
    let f0 = {
        let items = v
            .distributions
            .iter()
            .map(|x| {
                let m = mirror_fund_distribution(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fund_distributions {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        distributions: f0.0,
        distributions_len: f0.1,
        id: s.str(&v.id)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `FundEtfPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fund_etf_price {
    pub meta: *const nw_response_meta,
    pub change: *const c_char,
    pub change_percent: *const c_char,
    pub currency: *const c_char,
    pub has_delayed: bool,
    pub delayed: bool,
    pub id: *const c_char,
    pub market_session: i32,
    pub market_session_raw: *const c_char,
    pub previous_close: *const c_char,
    pub price: *const c_char,
    pub stale: bool,
    pub symbol: *const c_char,
    pub timestamp: *const c_char,
    pub type_: *const c_char,
    pub volume: *const c_char,
}
pub(crate) fn mirror_fund_etf_price(
    v: &sdk::models::FundEtfPrice,
    s: &mut Store,
) -> Result<nw_fund_etf_price, Unrepresentable> {
    Ok(nw_fund_etf_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        change: match &v.change {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        change_percent: match &v.change_percent {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        has_delayed: v.delayed.is_some(),
        delayed: v.delayed.unwrap_or_default(),
        id: s.str(&v.id)?,
        market_session: equity_market_session_code(v.market_session.as_str()),
        market_session_raw: s.str(v.market_session.as_str())?,
        previous_close: match &v.previous_close {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        price: s.str(v.price.as_str())?,
        stale: v.stale,
        symbol: s.str(&v.symbol)?,
        timestamp: s.str(&v.timestamp)?,
        type_: s.str(&v.type_)?,
        volume: match &v.volume {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `FundNavPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fund_nav_price {
    pub meta: *const nw_response_meta,
    pub change: *const c_char,
    pub change_percent: *const c_char,
    pub currency: *const c_char,
    pub date: *const c_char,
    pub id: *const c_char,
    pub nav: *const c_char,
    pub previous_nav: *const c_char,
    pub stale: bool,
    pub symbol: *const c_char,
    pub type_: *const c_char,
}
pub(crate) fn mirror_fund_nav_price(
    v: &sdk::models::FundNavPrice,
    s: &mut Store,
) -> Result<nw_fund_nav_price, Unrepresentable> {
    Ok(nw_fund_nav_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        change: match &v.change {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        change_percent: match &v.change_percent {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        date: s.str(&v.date)?,
        id: s.str(&v.id)?,
        nav: s.str(v.nav.as_str())?,
        previous_nav: match &v.previous_nav {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        stale: v.stale,
        symbol: s.str(&v.symbol)?,
        type_: s.str(&v.type_)?,
    })
}
/// Union `FundPrice`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_fund_price {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub fund_etf_price: *const nw_fund_etf_price,
    pub fund_nav_price: *const nw_fund_nav_price,
}
pub(crate) fn mirror_fund_price(
    v: &sdk::models::FundPrice,
    s: &mut Store,
) -> Result<nw_fund_price, Unrepresentable> {
    let mut head = nw_fund_price {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        fund_etf_price: std::ptr::null(),
        fund_nav_price: std::ptr::null(),
    };
    match v {
        sdk::models::FundPrice::FundEtfPrice(x) => {
            head.kind = c"FundEtfPrice".as_ptr();
            let m = mirror_fund_etf_price(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.fund_etf_price = s.boxed(m);
        }
        sdk::models::FundPrice::FundNavPrice(x) => {
            head.kind = c"FundNavPrice".as_ptr();
            let m = mirror_fund_nav_price(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.fund_nav_price = s.boxed(m);
        }
        sdk::models::FundPrice::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Mirror of `FundPriceError`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fund_price_error {
    pub code: i32,
    pub code_raw: *const c_char,
    pub id: *const c_char,
}
pub(crate) fn mirror_fund_price_error(
    v: &sdk::models::FundPriceError,
    s: &mut Store,
) -> Result<nw_fund_price_error, Unrepresentable> {
    Ok(nw_fund_price_error {
        code: crypto_price_error_code_code(v.code.as_str()),
        code_raw: s.str(v.code.as_str())?,
        id: s.str(&v.id)?,
    })
}
/// Union `FundPriceItem`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_fund_price_item {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub fund_etf_price: *const nw_fund_etf_price,
    pub fund_nav_price: *const nw_fund_nav_price,
}
pub(crate) fn mirror_fund_price_item(
    v: &sdk::models::FundPriceItem,
    s: &mut Store,
) -> Result<nw_fund_price_item, Unrepresentable> {
    let mut head = nw_fund_price_item {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        fund_etf_price: std::ptr::null(),
        fund_nav_price: std::ptr::null(),
    };
    match v {
        sdk::models::FundPriceItem::FundEtfPrice(x) => {
            head.kind = c"FundEtfPrice".as_ptr();
            let m = mirror_fund_etf_price(x, s)?;
            head.fund_etf_price = s.boxed(m);
        }
        sdk::models::FundPriceItem::FundNavPrice(x) => {
            head.kind = c"FundNavPrice".as_ptr();
            let m = mirror_fund_nav_price(x, s)?;
            head.fund_nav_price = s.boxed(m);
        }
        sdk::models::FundPriceItem::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Mirror of `FundPrices`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fund_prices {
    pub meta: *const nw_response_meta,
    pub errors: *const *const nw_fund_price_error,
    pub errors_len: usize,
    pub prices_len: usize,
    pub prices: *const *const nw_fund_price_item,
}
pub(crate) fn mirror_fund_prices(
    v: &sdk::models::FundPrices,
    s: &mut Store,
) -> Result<nw_fund_prices, Unrepresentable> {
    let f0 = {
        let items = v
            .errors
            .iter()
            .map(|x| {
                let m = mirror_fund_price_error(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f1 = {
        let items = v
            .prices
            .iter()
            .map(|x| {
                let m = mirror_fund_price_item(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fund_prices {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        errors: f0.0,
        errors_len: f0.1,
        prices_len: f1.1,
        prices: f1.0,
    })
}
/// Mirror of `FundProfile`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fund_profile {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub id: *const c_char,
    pub mic: *const c_char,
    pub name: *const c_char,
    pub symbol: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_fund_profile(
    v: &sdk::models::FundProfile,
    s: &mut Store,
) -> Result<nw_fund_profile, Unrepresentable> {
    Ok(nw_fund_profile {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        id: s.str(&v.id)?,
        mic: match &v.mic {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        name: s.str(&v.name)?,
        symbol: s.str(&v.symbol)?,
        type_: fund_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `FundSearchResult`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fund_search_result {
    pub country: *const c_char,
    pub currency: *const c_char,
    pub id: *const c_char,
    pub mic: *const c_char,
    pub name: *const c_char,
    pub symbol: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_fund_search_result(
    v: &sdk::models::FundSearchResult,
    s: &mut Store,
) -> Result<nw_fund_search_result, Unrepresentable> {
    Ok(nw_fund_search_result {
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        currency: s.str(&v.currency)?,
        id: s.str(&v.id)?,
        mic: match &v.mic {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        name: s.str(&v.name)?,
        symbol: s.str(&v.symbol)?,
        type_: fund_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `FundSearchResults`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fund_search_results {
    pub meta: *const nw_response_meta,
    pub next_cursor: *const c_char,
    pub results: *const *const nw_fund_search_result,
    pub results_len: usize,
}
pub(crate) fn mirror_fund_search_results(
    v: &sdk::models::FundSearchResults,
    s: &mut Store,
) -> Result<nw_fund_search_results, Unrepresentable> {
    let f1 = {
        let items = v
            .results
            .iter()
            .map(|x| {
                let m = mirror_fund_search_result(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fund_search_results {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        results: f1.0,
        results_len: f1.1,
    })
}
/// Mirror of `FxBar`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fx_bar {
    pub close: *const c_char,
    pub high: *const c_char,
    pub low: *const c_char,
    pub open: *const c_char,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_fx_bar(
    v: &sdk::models::FxBar,
    s: &mut Store,
) -> Result<nw_fx_bar, Unrepresentable> {
    Ok(nw_fx_bar {
        close: s.str(v.close.as_str())?,
        high: s.str(v.high.as_str())?,
        low: s.str(v.low.as_str())?,
        open: s.str(v.open.as_str())?,
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `FxBars`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fx_bars {
    pub meta: *const nw_response_meta,
    pub bars: *const *const nw_fx_bar,
    pub bars_len: usize,
    pub base: *const c_char,
    pub currency: *const c_char,
    pub end: *const c_char,
    pub next_cursor: *const c_char,
    pub start: *const c_char,
}
pub(crate) fn mirror_fx_bars(
    v: &sdk::models::FxBars,
    s: &mut Store,
) -> Result<nw_fx_bars, Unrepresentable> {
    let f0 = {
        let items = v
            .bars
            .iter()
            .map(|x| {
                let m = mirror_fx_bar(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fx_bars {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        bars: f0.0,
        bars_len: f0.1,
        base: s.str(&v.base)?,
        currency: s.str(&v.currency)?,
        end: match &v.end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `FxConversion`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fx_conversion {
    pub meta: *const nw_response_meta,
    pub converted: *const c_char,
    pub from: *const c_char,
    pub market_session: i32,
    pub market_session_raw: *const c_char,
    pub rate: *const c_char,
    pub stale: bool,
    pub timestamp: *const c_char,
    pub to: *const c_char,
}
pub(crate) fn mirror_fx_conversion(
    v: &sdk::models::FxConversion,
    s: &mut Store,
) -> Result<nw_fx_conversion, Unrepresentable> {
    Ok(nw_fx_conversion {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        converted: s.str(v.converted.as_str())?,
        from: s.str(&v.from)?,
        market_session: market_session_code(v.market_session.as_str()),
        market_session_raw: s.str(v.market_session.as_str())?,
        rate: s.str(v.rate.as_str())?,
        stale: v.stale,
        timestamp: s.str(&v.timestamp)?,
        to: s.str(&v.to)?,
    })
}
/// Mirror of `FxHistoricalRate`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fx_historical_rate {
    pub currency: *const c_char,
    pub rate: *const c_char,
}
pub(crate) fn mirror_fx_historical_rate(
    v: &sdk::models::FxHistoricalRate,
    s: &mut Store,
) -> Result<nw_fx_historical_rate, Unrepresentable> {
    Ok(nw_fx_historical_rate {
        currency: s.str(&v.currency)?,
        rate: s.str(v.rate.as_str())?,
    })
}
/// Mirror of `FxHistoricalObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fx_historical_observation {
    pub date: *const c_char,
    pub rates: *const *const nw_fx_historical_rate,
    pub rates_len: usize,
}
pub(crate) fn mirror_fx_historical_observation(
    v: &sdk::models::FxHistoricalObservation,
    s: &mut Store,
) -> Result<nw_fx_historical_observation, Unrepresentable> {
    let f1 = {
        let items = v
            .rates
            .iter()
            .map(|x| {
                let m = mirror_fx_historical_rate(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fx_historical_observation {
        date: s.str(&v.date)?,
        rates: f1.0,
        rates_len: f1.1,
    })
}
/// Mirror of `FxHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fx_history {
    pub meta: *const nw_response_meta,
    pub base: *const c_char,
    pub end_date: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_fx_historical_observation,
    pub observations_len: usize,
    pub start_date: *const c_char,
}
pub(crate) fn mirror_fx_history(
    v: &sdk::models::FxHistory,
    s: &mut Store,
) -> Result<nw_fx_history, Unrepresentable> {
    let f3 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_fx_historical_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fx_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        base: s.str(&v.base)?,
        end_date: match &v.end_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f3.0,
        observations_len: f3.1,
        start_date: match &v.start_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `FxRate`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_fx_rate {
    pub currency: *const c_char,
    pub rate: *const c_char,
    pub stale: bool,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_fx_rate(
    v: &sdk::models::FxRate,
    s: &mut Store,
) -> Result<nw_fx_rate, Unrepresentable> {
    Ok(nw_fx_rate {
        currency: s.str(&v.currency)?,
        rate: s.str(v.rate.as_str())?,
        stale: v.stale,
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `FxRates`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_fx_rates_model {
    pub meta: *const nw_response_meta,
    pub base: *const c_char,
    pub market_session: i32,
    pub market_session_raw: *const c_char,
    pub rates: *const *const nw_fx_rate,
    pub rates_len: usize,
}
pub(crate) fn mirror_fx_rates(
    v: &sdk::models::FxRates,
    s: &mut Store,
) -> Result<nw_fx_rates_model, Unrepresentable> {
    let f2 = {
        let items = v
            .rates
            .iter()
            .map(|x| {
                let m = mirror_fx_rate(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_fx_rates_model {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        base: s.str(&v.base)?,
        market_session: market_session_code(v.market_session.as_str()),
        market_session_raw: s.str(v.market_session.as_str())?,
        rates: f2.0,
        rates_len: f2.1,
    })
}
/// Mirror of `Gdp`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_gdp {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
    pub value: *const c_char,
    pub has_annualized: bool,
    pub annualized: bool,
    pub has_not_seasonally_adjusted: bool,
    pub not_seasonally_adjusted: bool,
}
pub(crate) fn mirror_gdp(v: &sdk::models::Gdp, s: &mut Store) -> Result<nw_gdp, Unrepresentable> {
    Ok(nw_gdp {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        period: s.str(&v.period)?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        value: s.str(v.value.as_str())?,
        has_annualized: v.annualized.is_some(),
        annualized: v.annualized.unwrap_or_default(),
        has_not_seasonally_adjusted: v.not_seasonally_adjusted.is_some(),
        not_seasonally_adjusted: v.not_seasonally_adjusted.unwrap_or_default(),
    })
}
/// Mirror of `GdpGrowth`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_gdp_growth {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub period: *const c_char,
    pub quarter_over_quarter: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
    pub year_over_year: *const c_char,
    pub year_to_date: *const c_char,
    pub has_annualized: bool,
    pub annualized: bool,
    pub has_not_seasonally_adjusted: bool,
    pub not_seasonally_adjusted: bool,
    pub has_quarter_over_quarter_not_seasonally_adjusted: bool,
    pub quarter_over_quarter_not_seasonally_adjusted: bool,
    pub has_year_over_year_not_seasonally_adjusted: bool,
    pub year_over_year_not_seasonally_adjusted: bool,
}
pub(crate) fn mirror_gdp_growth(
    v: &sdk::models::GdpGrowth,
    s: &mut Store,
) -> Result<nw_gdp_growth, Unrepresentable> {
    Ok(nw_gdp_growth {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        period: s.str(&v.period)?,
        quarter_over_quarter: s.str(v.quarter_over_quarter.as_str())?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        year_over_year: s.str(v.year_over_year.as_str())?,
        year_to_date: match &v.year_to_date {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        has_annualized: v.annualized.is_some(),
        annualized: v.annualized.unwrap_or_default(),
        has_not_seasonally_adjusted: v.not_seasonally_adjusted.is_some(),
        not_seasonally_adjusted: v.not_seasonally_adjusted.unwrap_or_default(),
        has_quarter_over_quarter_not_seasonally_adjusted: v
            .quarter_over_quarter_not_seasonally_adjusted
            .is_some(),
        quarter_over_quarter_not_seasonally_adjusted: v
            .quarter_over_quarter_not_seasonally_adjusted
            .unwrap_or_default(),
        has_year_over_year_not_seasonally_adjusted: v
            .year_over_year_not_seasonally_adjusted
            .is_some(),
        year_over_year_not_seasonally_adjusted: v
            .year_over_year_not_seasonally_adjusted
            .unwrap_or_default(),
    })
}
/// Mirror of `GdpGrowthHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_gdp_growth_history_observation {
    pub period: *const c_char,
    pub quarter_over_quarter: *const c_char,
    pub year_over_year: *const c_char,
    pub year_to_date: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_gdp_growth_history_observation(
    v: &sdk::models::GdpGrowthHistoryObservation,
    s: &mut Store,
) -> Result<nw_gdp_growth_history_observation, Unrepresentable> {
    Ok(nw_gdp_growth_history_observation {
        period: s.str(&v.period)?,
        quarter_over_quarter: match &v.quarter_over_quarter {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        year_over_year: match &v.year_over_year {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        year_to_date: match &v.year_to_date {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `GdpGrowthHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_gdp_growth_history {
    pub meta: *const nw_response_meta,
    pub has_annualized: bool,
    pub annualized: bool,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub next_cursor: *const c_char,
    pub has_not_seasonally_adjusted: bool,
    pub not_seasonally_adjusted: bool,
    pub observations: *const *const nw_gdp_growth_history_observation,
    pub observations_len: usize,
    pub has_quarter_over_quarter_not_seasonally_adjusted: bool,
    pub quarter_over_quarter_not_seasonally_adjusted: bool,
    pub source: *const nw_series_source,
    pub start: *const c_char,
    pub has_year_over_year_not_seasonally_adjusted: bool,
    pub year_over_year_not_seasonally_adjusted: bool,
}
pub(crate) fn mirror_gdp_growth_history(
    v: &sdk::models::GdpGrowthHistory,
    s: &mut Store,
) -> Result<nw_gdp_growth_history, Unrepresentable> {
    let f5 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_gdp_growth_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_gdp_growth_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        has_annualized: v.annualized.is_some(),
        annualized: v.annualized.unwrap_or_default(),
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_not_seasonally_adjusted: v.not_seasonally_adjusted.is_some(),
        not_seasonally_adjusted: v.not_seasonally_adjusted.unwrap_or_default(),
        observations: f5.0,
        observations_len: f5.1,
        has_quarter_over_quarter_not_seasonally_adjusted: v
            .quarter_over_quarter_not_seasonally_adjusted
            .is_some(),
        quarter_over_quarter_not_seasonally_adjusted: v
            .quarter_over_quarter_not_seasonally_adjusted
            .unwrap_or_default(),
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_year_over_year_not_seasonally_adjusted: v
            .year_over_year_not_seasonally_adjusted
            .is_some(),
        year_over_year_not_seasonally_adjusted: v
            .year_over_year_not_seasonally_adjusted
            .unwrap_or_default(),
    })
}
/// Mirror of `GdpHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_gdp_history_observation {
    pub period: *const c_char,
    pub value: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_gdp_history_observation(
    v: &sdk::models::GdpHistoryObservation,
    s: &mut Store,
) -> Result<nw_gdp_history_observation, Unrepresentable> {
    Ok(nw_gdp_history_observation {
        period: s.str(&v.period)?,
        value: s.str(v.value.as_str())?,
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `GdpHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_gdp_history {
    pub meta: *const nw_response_meta,
    pub has_annualized: bool,
    pub annualized: bool,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub has_not_seasonally_adjusted: bool,
    pub not_seasonally_adjusted: bool,
    pub observations: *const *const nw_gdp_history_observation,
    pub observations_len: usize,
    pub source: *const nw_series_source,
    pub start: *const c_char,
}
pub(crate) fn mirror_gdp_history(
    v: &sdk::models::GdpHistory,
    s: &mut Store,
) -> Result<nw_gdp_history, Unrepresentable> {
    let f6 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_gdp_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_gdp_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        has_annualized: v.annualized.is_some(),
        annualized: v.annualized.unwrap_or_default(),
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_not_seasonally_adjusted: v.not_seasonally_adjusted.is_some(),
        not_seasonally_adjusted: v.not_seasonally_adjusted.unwrap_or_default(),
        observations: f6.0,
        observations_len: f6.1,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `HolderSecurity`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_holder_security {
    pub class_: *const c_char,
    pub isin: *const c_char,
}
pub(crate) fn mirror_holder_security(
    v: &sdk::models::HolderSecurity,
    s: &mut Store,
) -> Result<nw_holder_security, Unrepresentable> {
    Ok(nw_holder_security {
        class_: match &v.class {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        isin: match &v.isin {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `Holder`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_holder {
    pub event_date: *const c_char,
    pub filed: *const c_char,
    pub filing_id: *const c_char,
    pub holder: *const c_char,
    pub percent: *const c_char,
    pub security: *const nw_holder_security,
    pub shares: *const c_char,
}
pub(crate) fn mirror_holder(
    v: &sdk::models::Holder,
    s: &mut Store,
) -> Result<nw_holder, Unrepresentable> {
    Ok(nw_holder {
        event_date: match &v.event_date {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        filed: s.str(&v.filed)?,
        filing_id: s.str(&v.filing_id)?,
        holder: s.str(&v.holder)?,
        percent: match &v.percent {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        security: match &v.security {
            Some(x) => {
                let v = mirror_holder_security(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        shares: match &v.shares {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `HoldersPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_holders_page {
    pub meta: *const nw_response_meta,
    pub filed_from: *const c_char,
    pub holders: *const *const nw_holder,
    pub holders_len: usize,
    pub id: *const c_char,
}
pub(crate) fn mirror_holders_page(
    v: &sdk::models::HoldersPage,
    s: &mut Store,
) -> Result<nw_holders_page, Unrepresentable> {
    let f1 = {
        let items = v
            .holders
            .iter()
            .map(|x| {
                let m = mirror_holder(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_holders_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        filed_from: match &v.filed_from {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        holders: f1.0,
        holders_len: f1.1,
        id: s.str(&v.id)?,
    })
}
/// Mirror of `Inflation`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_inflation {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub month_over_month: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
    pub year_over_year: *const c_char,
    pub year_to_date: *const c_char,
}
pub(crate) fn mirror_inflation(
    v: &sdk::models::Inflation,
    s: &mut Store,
) -> Result<nw_inflation, Unrepresentable> {
    Ok(nw_inflation {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        month_over_month: s.str(v.month_over_month.as_str())?,
        period: s.str(&v.period)?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        year_over_year: s.str(v.year_over_year.as_str())?,
        year_to_date: match &v.year_to_date {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `InflationHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_inflation_history_observation {
    pub month_over_month: *const c_char,
    pub period: *const c_char,
    pub year_over_year: *const c_char,
    pub year_to_date: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_inflation_history_observation(
    v: &sdk::models::InflationHistoryObservation,
    s: &mut Store,
) -> Result<nw_inflation_history_observation, Unrepresentable> {
    Ok(nw_inflation_history_observation {
        month_over_month: match &v.month_over_month {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        period: s.str(&v.period)?,
        year_over_year: match &v.year_over_year {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        year_to_date: match &v.year_to_date {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `InflationHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_inflation_history {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_inflation_history_observation,
    pub observations_len: usize,
    pub source: *const nw_series_source,
    pub start: *const c_char,
}
pub(crate) fn mirror_inflation_history(
    v: &sdk::models::InflationHistory,
    s: &mut Store,
) -> Result<nw_inflation_history, Unrepresentable> {
    let f3 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_inflation_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_inflation_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f3.0,
        observations_len: f3.1,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `LabourForceParticipation`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_labour_force_participation {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub period: *const c_char,
    pub rate: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
}
pub(crate) fn mirror_labour_force_participation(
    v: &sdk::models::LabourForceParticipation,
    s: &mut Store,
) -> Result<nw_labour_force_participation, Unrepresentable> {
    Ok(nw_labour_force_participation {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        period: s.str(&v.period)?,
        rate: s.str(v.rate.as_str())?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `LatestFiling`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_latest_filing {
    pub company_id: *const c_char,
    pub filed: *const c_char,
    pub form: *const c_char,
    pub id: *const c_char,
    pub title: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_latest_filing(
    v: &sdk::models::LatestFiling,
    s: &mut Store,
) -> Result<nw_latest_filing, Unrepresentable> {
    Ok(nw_latest_filing {
        company_id: s.str(&v.company_id)?,
        filed: s.str(&v.filed)?,
        form: s.str(&v.form)?,
        id: s.str(&v.id)?,
        title: match &v.title {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: filing_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `LatestFilingsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_latest_filings_page {
    pub meta: *const nw_response_meta,
    pub filings: *const *const nw_latest_filing,
    pub filings_len: usize,
}
pub(crate) fn mirror_latest_filings_page(
    v: &sdk::models::LatestFilingsPage,
    s: &mut Store,
) -> Result<nw_latest_filings_page, Unrepresentable> {
    let f0 = {
        let items = v
            .filings
            .iter()
            .map(|x| {
                let m = mirror_latest_filing(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_latest_filings_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        filings: f0.0,
        filings_len: f0.1,
    })
}
/// Mirror of `NoContent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_no_content {
    pub meta: *const nw_response_meta,
}
pub(crate) fn mirror_no_content(
    v: &sdk::models::NoContent,
    s: &mut Store,
) -> Result<nw_no_content, Unrepresentable> {
    Ok(nw_no_content {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
    })
}
/// Mirror of `CommodityOilPalmFfbAgePrice`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_commodity_oil_palm_ffb_age_price {
    pub price: *const c_char,
    pub tree_age_max_years: i64,
    pub tree_age_min_years: i64,
}
pub(crate) fn mirror_commodity_oil_palm_ffb_age_price(
    v: &sdk::models::CommodityOilPalmFfbAgePrice,
    s: &mut Store,
) -> Result<nw_commodity_oil_palm_ffb_age_price, Unrepresentable> {
    Ok(nw_commodity_oil_palm_ffb_age_price {
        price: s.str(v.price.as_str())?,
        tree_age_max_years: v.tree_age_max_years,
        tree_age_min_years: v.tree_age_min_years,
    })
}
/// Mirror of `PalmoilScheduleObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_palmoil_schedule_observation {
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub prices: *const *const nw_commodity_oil_palm_ffb_age_price,
    pub prices_len: usize,
    pub revision: i64,
}
pub(crate) fn mirror_palmoil_schedule_observation(
    v: &sdk::models::PalmoilScheduleObservation,
    s: &mut Store,
) -> Result<nw_palmoil_schedule_observation, Unrepresentable> {
    let f2 = {
        let items = v
            .prices
            .iter()
            .map(|x| {
                let m = mirror_commodity_oil_palm_ffb_age_price(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_schedule_observation {
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        prices: f2.0,
        prices_len: f2.1,
        revision: v.revision,
    })
}
/// Mirror of `PalmoilScheduleHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_schedule_history {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_palmoil_schedule_observation,
    pub observations_len: usize,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_schedule_history(
    v: &sdk::models::PalmoilScheduleHistory,
    s: &mut Store,
) -> Result<nw_palmoil_schedule_history, Unrepresentable> {
    let f2 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_palmoil_schedule_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_schedule_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f2.0,
        observations_len: f2.1,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilMarketObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_palmoil_market_observation {
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub price_month: *const c_char,
    pub revision: i64,
}
pub(crate) fn mirror_palmoil_market_observation(
    v: &sdk::models::PalmoilMarketObservation,
    s: &mut Store,
) -> Result<nw_palmoil_market_observation, Unrepresentable> {
    Ok(nw_palmoil_market_observation {
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        price_month: s.str(&v.price_month)?,
        revision: v.revision,
    })
}
/// Mirror of `PalmoilMarketHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_market_history {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_palmoil_market_observation,
    pub observations_len: usize,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_market_history(
    v: &sdk::models::PalmoilMarketHistory,
    s: &mut Store,
) -> Result<nw_palmoil_market_history, Unrepresentable> {
    let f2 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_palmoil_market_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_market_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f2.0,
        observations_len: f2.1,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilScalarObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_palmoil_scalar_observation {
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub revision: i64,
}
pub(crate) fn mirror_palmoil_scalar_observation(
    v: &sdk::models::PalmoilScalarObservation,
    s: &mut Store,
) -> Result<nw_palmoil_scalar_observation, Unrepresentable> {
    Ok(nw_palmoil_scalar_observation {
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        revision: v.revision,
    })
}
/// Mirror of `PalmoilScalarHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_scalar_history {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_palmoil_scalar_observation,
    pub observations_len: usize,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_scalar_history(
    v: &sdk::models::PalmoilScalarHistory,
    s: &mut Store,
) -> Result<nw_palmoil_scalar_history, Unrepresentable> {
    let f2 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_palmoil_scalar_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_scalar_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f2.0,
        observations_len: f2.1,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilFfbObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_palmoil_ffb_observation {
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub revision: i64,
}
pub(crate) fn mirror_palmoil_ffb_observation(
    v: &sdk::models::PalmoilFfbObservation,
    s: &mut Store,
) -> Result<nw_palmoil_ffb_observation, Unrepresentable> {
    Ok(nw_palmoil_ffb_observation {
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        revision: v.revision,
    })
}
/// Mirror of `PalmoilFfbMarketHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_ffb_market_history {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_palmoil_ffb_observation,
    pub observations_len: usize,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_ffb_market_history(
    v: &sdk::models::PalmoilFfbMarketHistory,
    s: &mut Store,
) -> Result<nw_palmoil_ffb_market_history, Unrepresentable> {
    let f2 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_palmoil_ffb_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_ffb_market_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f2.0,
        observations_len: f2.1,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Union `PalmoilHistory`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_palmoil_history {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub palmoil_schedule_history: *const nw_palmoil_schedule_history,
    pub palmoil_market_history: *const nw_palmoil_market_history,
    pub palmoil_scalar_history: *const nw_palmoil_scalar_history,
    pub palmoil_ffb_market_history: *const nw_palmoil_ffb_market_history,
}
pub(crate) fn mirror_palmoil_history(
    v: &sdk::models::PalmoilHistory,
    s: &mut Store,
) -> Result<nw_palmoil_history, Unrepresentable> {
    let mut head = nw_palmoil_history {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        palmoil_schedule_history: std::ptr::null(),
        palmoil_market_history: std::ptr::null(),
        palmoil_scalar_history: std::ptr::null(),
        palmoil_ffb_market_history: std::ptr::null(),
    };
    match v {
        sdk::models::PalmoilHistory::PalmoilScheduleHistory(x) => {
            head.kind = c"PalmoilScheduleHistory".as_ptr();
            let m = mirror_palmoil_schedule_history(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_schedule_history = s.boxed(m);
        }
        sdk::models::PalmoilHistory::PalmoilMarketHistory(x) => {
            head.kind = c"PalmoilMarketHistory".as_ptr();
            let m = mirror_palmoil_market_history(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_market_history = s.boxed(m);
        }
        sdk::models::PalmoilHistory::PalmoilScalarHistory(x) => {
            head.kind = c"PalmoilScalarHistory".as_ptr();
            let m = mirror_palmoil_scalar_history(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_scalar_history = s.boxed(m);
        }
        sdk::models::PalmoilHistory::PalmoilFfbMarketHistory(x) => {
            head.kind = c"PalmoilFfbMarketHistory".as_ptr();
            let m = mirror_palmoil_ffb_market_history(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_ffb_market_history = s.boxed(m);
        }
        sdk::models::PalmoilHistory::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Mirror of `PalmoilSchedulePrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_schedule_price {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub prices: *const *const nw_commodity_oil_palm_ffb_age_price,
    pub prices_len: usize,
    pub revision: i64,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_schedule_price(
    v: &sdk::models::PalmoilSchedulePrice,
    s: &mut Store,
) -> Result<nw_palmoil_schedule_price, Unrepresentable> {
    let f3 = {
        let items = v
            .prices
            .iter()
            .map(|x| {
                let m = mirror_commodity_oil_palm_ffb_age_price(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_schedule_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        prices: f3.0,
        prices_len: f3.1,
        revision: v.revision,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilMarketPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_market_price {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub price_month: *const c_char,
    pub revision: i64,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_market_price(
    v: &sdk::models::PalmoilMarketPrice,
    s: &mut Store,
) -> Result<nw_palmoil_market_price, Unrepresentable> {
    Ok(nw_palmoil_market_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        price_month: s.str(&v.price_month)?,
        revision: v.revision,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilScalarPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_scalar_price {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub revision: i64,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_scalar_price(
    v: &sdk::models::PalmoilScalarPrice,
    s: &mut Store,
) -> Result<nw_palmoil_scalar_price, Unrepresentable> {
    Ok(nw_palmoil_scalar_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        revision: v.revision,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilFfbMarketPrice`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_ffb_market_price {
    pub meta: *const nw_response_meta,
    pub currency: *const c_char,
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub revision: i64,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_ffb_market_price(
    v: &sdk::models::PalmoilFfbMarketPrice,
    s: &mut Store,
) -> Result<nw_palmoil_ffb_market_price, Unrepresentable> {
    Ok(nw_palmoil_ffb_market_price {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        currency: s.str(&v.currency)?,
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        revision: v.revision,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Union `PalmoilPrice`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_palmoil_price {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub palmoil_schedule_price: *const nw_palmoil_schedule_price,
    pub palmoil_market_price: *const nw_palmoil_market_price,
    pub palmoil_scalar_price: *const nw_palmoil_scalar_price,
    pub palmoil_ffb_market_price: *const nw_palmoil_ffb_market_price,
}
pub(crate) fn mirror_palmoil_price(
    v: &sdk::models::PalmoilPrice,
    s: &mut Store,
) -> Result<nw_palmoil_price, Unrepresentable> {
    let mut head = nw_palmoil_price {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        palmoil_schedule_price: std::ptr::null(),
        palmoil_market_price: std::ptr::null(),
        palmoil_scalar_price: std::ptr::null(),
        palmoil_ffb_market_price: std::ptr::null(),
    };
    match v {
        sdk::models::PalmoilPrice::PalmoilSchedulePrice(x) => {
            head.kind = c"PalmoilSchedulePrice".as_ptr();
            let m = mirror_palmoil_schedule_price(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_schedule_price = s.boxed(m);
        }
        sdk::models::PalmoilPrice::PalmoilMarketPrice(x) => {
            head.kind = c"PalmoilMarketPrice".as_ptr();
            let m = mirror_palmoil_market_price(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_market_price = s.boxed(m);
        }
        sdk::models::PalmoilPrice::PalmoilScalarPrice(x) => {
            head.kind = c"PalmoilScalarPrice".as_ptr();
            let m = mirror_palmoil_scalar_price(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_scalar_price = s.boxed(m);
        }
        sdk::models::PalmoilPrice::PalmoilFfbMarketPrice(x) => {
            head.kind = c"PalmoilFfbMarketPrice".as_ptr();
            let m = mirror_palmoil_ffb_market_price(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.palmoil_ffb_market_price = s.boxed(m);
        }
        sdk::models::PalmoilPrice::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Mirror of `PalmoilScheduleSeries`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_palmoil_schedule_series {
    pub area: *const c_char,
    pub country: *const c_char,
    pub coverage_end: *const c_char,
    pub coverage_start: *const c_char,
    pub currency: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub observation_type: *const c_char,
    pub period_type: *const c_char,
    pub price_type: *const c_char,
    pub producer_arrangement: i32,
    pub producer_arrangement_raw: *const c_char,
    pub product: *const c_char,
    pub series_id: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_palmoil_schedule_series(
    v: &sdk::models::PalmoilScheduleSeries,
    s: &mut Store,
) -> Result<nw_palmoil_schedule_series, Unrepresentable> {
    Ok(nw_palmoil_schedule_series {
        area: match &v.area {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        coverage_end: s.str(&v.coverage_end)?,
        coverage_start: s.str(&v.coverage_start)?,
        currency: s.str(&v.currency)?,
        frequency: palmoil_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        observation_type: s.str(&v.observation_type)?,
        period_type: s.str(&v.period_type)?,
        price_type: s.str(&v.price_type)?,
        producer_arrangement: v
            .producer_arrangement
            .as_ref()
            .map_or(0, |x| producer_arrangement_code(x.as_str())),
        producer_arrangement_raw: match &v.producer_arrangement {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        product: s.str(&v.product)?,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `PalmoilScalarSeries`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_palmoil_scalar_series {
    pub area: *const c_char,
    pub country: *const c_char,
    pub coverage_end: *const c_char,
    pub coverage_start: *const c_char,
    pub currency: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub observation_type: *const c_char,
    pub period_type: i32,
    pub period_type_raw: *const c_char,
    pub price_type: i32,
    pub price_type_raw: *const c_char,
    pub product: i32,
    pub product_raw: *const c_char,
    pub series_id: *const c_char,
    pub unit: i32,
    pub unit_raw: *const c_char,
}
pub(crate) fn mirror_palmoil_scalar_series(
    v: &sdk::models::PalmoilScalarSeries,
    s: &mut Store,
) -> Result<nw_palmoil_scalar_series, Unrepresentable> {
    Ok(nw_palmoil_scalar_series {
        area: match &v.area {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        coverage_end: s.str(&v.coverage_end)?,
        coverage_start: s.str(&v.coverage_start)?,
        currency: s.str(&v.currency)?,
        frequency: palmoil_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        observation_type: s.str(&v.observation_type)?,
        period_type: palmoil_period_type_code(v.period_type.as_str()),
        period_type_raw: s.str(v.period_type.as_str())?,
        price_type: palmoil_scalar_price_type_code(v.price_type.as_str()),
        price_type_raw: s.str(v.price_type.as_str())?,
        product: palmoil_product_code(v.product.as_str()),
        product_raw: s.str(v.product.as_str())?,
        series_id: s.str(&v.series_id)?,
        unit: palmoil_scalar_unit_code(v.unit.as_str()),
        unit_raw: s.str(v.unit.as_str())?,
    })
}
/// Union `PalmoilSeries`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_palmoil_series {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub palmoil_schedule_series: *const nw_palmoil_schedule_series,
    pub palmoil_scalar_series: *const nw_palmoil_scalar_series,
}
pub(crate) fn mirror_palmoil_series(
    v: &sdk::models::PalmoilSeries,
    s: &mut Store,
) -> Result<nw_palmoil_series, Unrepresentable> {
    let mut head = nw_palmoil_series {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        palmoil_schedule_series: std::ptr::null(),
        palmoil_scalar_series: std::ptr::null(),
    };
    match v {
        sdk::models::PalmoilSeries::PalmoilScheduleSeries(x) => {
            head.kind = c"PalmoilScheduleSeries".as_ptr();
            let m = mirror_palmoil_schedule_series(x, s)?;
            head.palmoil_schedule_series = s.boxed(m);
        }
        sdk::models::PalmoilSeries::PalmoilScalarSeries(x) => {
            head.kind = c"PalmoilScalarSeries".as_ptr();
            let m = mirror_palmoil_scalar_series(x, s)?;
            head.palmoil_scalar_series = s.boxed(m);
        }
        sdk::models::PalmoilSeries::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Mirror of `PalmoilSeriesDiscovery`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_palmoil_series_discovery {
    pub meta: *const nw_response_meta,
    pub next_cursor: *const c_char,
    pub series: *const *const nw_palmoil_series,
    pub series_len: usize,
}
pub(crate) fn mirror_palmoil_series_discovery(
    v: &sdk::models::PalmoilSeriesDiscovery,
    s: &mut Store,
) -> Result<nw_palmoil_series_discovery, Unrepresentable> {
    let f1 = {
        let items = v
            .series
            .iter()
            .map(|x| {
                let m = mirror_palmoil_series(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_palmoil_series_discovery {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        series: f1.0,
        series_len: f1.1,
    })
}
/// Mirror of `PhysicalLatest`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_physical_latest {
    pub meta: *const nw_response_meta,
    pub attribution: *const c_char,
    pub currency: *const c_char,
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub published_on: *const c_char,
    pub revision: i64,
    pub series_id: *const c_char,
    pub unit: *const c_char,
    pub unit_quantity: *const c_char,
}
pub(crate) fn mirror_physical_latest(
    v: &sdk::models::PhysicalLatest,
    s: &mut Store,
) -> Result<nw_physical_latest, Unrepresentable> {
    Ok(nw_physical_latest {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        attribution: s.str(&v.attribution)?,
        currency: s.str(&v.currency)?,
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        published_on: match &v.published_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        revision: v.revision,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
        unit_quantity: s.str(v.unit_quantity.as_str())?,
    })
}
/// Mirror of `PhysicalObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_physical_observation {
    pub period_end: *const c_char,
    pub period_start: *const c_char,
    pub price: *const c_char,
    pub published_on: *const c_char,
    pub revision: i64,
}
pub(crate) fn mirror_physical_observation(
    v: &sdk::models::PhysicalObservation,
    s: &mut Store,
) -> Result<nw_physical_observation, Unrepresentable> {
    Ok(nw_physical_observation {
        period_end: s.str(&v.period_end)?,
        period_start: s.str(&v.period_start)?,
        price: s.str(v.price.as_str())?,
        published_on: match &v.published_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        revision: v.revision,
    })
}
/// Mirror of `PhysicalObservations`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_physical_observations {
    pub meta: *const nw_response_meta,
    pub attribution: *const c_char,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_physical_observation,
    pub observations_len: usize,
    pub series_id: *const c_char,
    pub unit: *const c_char,
    pub unit_quantity: *const c_char,
}
pub(crate) fn mirror_physical_observations(
    v: &sdk::models::PhysicalObservations,
    s: &mut Store,
) -> Result<nw_physical_observations, Unrepresentable> {
    let f3 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_physical_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_physical_observations {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        attribution: s.str(&v.attribution)?,
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f3.0,
        observations_len: f3.1,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
        unit_quantity: s.str(v.unit_quantity.as_str())?,
    })
}
/// Mirror of `PhysicalSeries`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_physical_series {
    pub meta: *const nw_response_meta,
    pub area: *const c_char,
    pub attribution: *const c_char,
    pub basis: *const c_char,
    pub category: i32,
    pub category_raw: *const c_char,
    pub commodity: *const c_char,
    pub coverage_end: *const c_char,
    pub coverage_start: *const c_char,
    pub currency: *const c_char,
    pub description: *const c_char,
    pub frequency: i32,
    pub frequency_raw: *const c_char,
    pub price_type: i32,
    pub price_type_raw: *const c_char,
    pub product: *const c_char,
    pub series_id: *const c_char,
    pub unit: *const c_char,
    pub unit_quantity: *const c_char,
    pub country: *const c_char,
}
pub(crate) fn mirror_physical_series(
    v: &sdk::models::PhysicalSeries,
    s: &mut Store,
) -> Result<nw_physical_series, Unrepresentable> {
    Ok(nw_physical_series {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        area: match &v.area {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        attribution: s.str(&v.attribution)?,
        basis: match &v.basis {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        category: physical_category_code(v.category.as_str()),
        category_raw: s.str(v.category.as_str())?,
        commodity: s.str(&v.commodity)?,
        coverage_end: s.str(&v.coverage_end)?,
        coverage_start: s.str(&v.coverage_start)?,
        currency: s.str(&v.currency)?,
        description: s.str(&v.description)?,
        frequency: physical_frequency_code(v.frequency.as_str()),
        frequency_raw: s.str(v.frequency.as_str())?,
        price_type: physical_price_type_code(v.price_type.as_str()),
        price_type_raw: s.str(v.price_type.as_str())?,
        product: s.str(&v.product)?,
        series_id: s.str(&v.series_id)?,
        unit: s.str(&v.unit)?,
        unit_quantity: s.str(v.unit_quantity.as_str())?,
        country: s.str(&v.country)?,
    })
}
/// Mirror of `PhysicalSeriesPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_physical_series_page {
    pub meta: *const nw_response_meta,
    pub next_cursor: *const c_char,
    pub series: *const *const nw_physical_series,
    pub series_len: usize,
}
pub(crate) fn mirror_physical_series_page(
    v: &sdk::models::PhysicalSeriesPage,
    s: &mut Store,
) -> Result<nw_physical_series_page, Unrepresentable> {
    let f1 = {
        let items = v
            .series
            .iter()
            .map(|x| {
                let m = mirror_physical_series(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_physical_series_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        series: f1.0,
        series_len: f1.1,
    })
}
/// Mirror of `PolicyPathMeeting`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_policy_path_meeting {
    pub cumulative_change_bps: *const c_char,
    pub event_id: *const c_char,
    pub expected_change_bps: *const c_char,
    pub p_cut: *const c_char,
    pub p_hike: *const c_char,
    pub p_hold: *const c_char,
    pub resolves_at: *const c_char,
}
pub(crate) fn mirror_policy_path_meeting(
    v: &sdk::models::PolicyPathMeeting,
    s: &mut Store,
) -> Result<nw_policy_path_meeting, Unrepresentable> {
    Ok(nw_policy_path_meeting {
        cumulative_change_bps: s.str(v.cumulative_change_bps.as_str())?,
        event_id: s.str(&v.event_id)?,
        expected_change_bps: s.str(v.expected_change_bps.as_str())?,
        p_cut: s.str(&v.p_cut)?,
        p_hike: s.str(&v.p_hike)?,
        p_hold: s.str(&v.p_hold)?,
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PolicyPath`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_policy_path {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub meetings: *const *const nw_policy_path_meeting,
    pub meetings_len: usize,
}
pub(crate) fn mirror_policy_path(
    v: &sdk::models::PolicyPath,
    s: &mut Store,
) -> Result<nw_policy_path, Unrepresentable> {
    let f1 = {
        let items = v
            .meetings
            .iter()
            .map(|x| {
                let m = mirror_policy_path_meeting(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_policy_path {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        meetings: f1.0,
        meetings_len: f1.1,
    })
}
/// Mirror of `PolicyPathOdds`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_policy_path_odds {
    pub cumulative_change_bps: *const c_char,
    pub event_id: *const c_char,
    pub expected_change_bps: *const c_char,
    pub p_cut: *const c_char,
    pub p_hike: *const c_char,
    pub p_hold: *const c_char,
}
pub(crate) fn mirror_policy_path_odds(
    v: &sdk::models::PolicyPathOdds,
    s: &mut Store,
) -> Result<nw_policy_path_odds, Unrepresentable> {
    Ok(nw_policy_path_odds {
        cumulative_change_bps: s.str(v.cumulative_change_bps.as_str())?,
        event_id: s.str(&v.event_id)?,
        expected_change_bps: s.str(v.expected_change_bps.as_str())?,
        p_cut: s.str(&v.p_cut)?,
        p_hike: s.str(&v.p_hike)?,
        p_hold: s.str(&v.p_hold)?,
    })
}
/// Mirror of `PolicyPathDay`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_policy_path_day {
    pub date: *const c_char,
    pub meetings_len: usize,
    pub meetings: *const *const nw_policy_path_odds,
}
pub(crate) fn mirror_policy_path_day(
    v: &sdk::models::PolicyPathDay,
    s: &mut Store,
) -> Result<nw_policy_path_day, Unrepresentable> {
    let f1 = {
        let items = v
            .meetings
            .iter()
            .map(|x| {
                let m = mirror_policy_path_odds(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_policy_path_day {
        date: s.str(&v.date)?,
        meetings_len: f1.1,
        meetings: f1.0,
    })
}
/// Mirror of `PolicyPathMeetingDate`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_policy_path_meeting_date {
    pub event_id: *const c_char,
    pub resolves_at: *const c_char,
}
pub(crate) fn mirror_policy_path_meeting_date(
    v: &sdk::models::PolicyPathMeetingDate,
    s: &mut Store,
) -> Result<nw_policy_path_meeting_date, Unrepresentable> {
    Ok(nw_policy_path_meeting_date {
        event_id: s.str(&v.event_id)?,
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PolicyPathHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_policy_path_history {
    pub meta: *const nw_response_meta,
    pub days: *const *const nw_policy_path_day,
    pub days_len: usize,
    pub meeting_dates: *const *const nw_policy_path_meeting_date,
    pub meeting_dates_len: usize,
}
pub(crate) fn mirror_policy_path_history(
    v: &sdk::models::PolicyPathHistory,
    s: &mut Store,
) -> Result<nw_policy_path_history, Unrepresentable> {
    let f0 = {
        let items = v
            .days
            .iter()
            .map(|x| {
                let m = mirror_policy_path_day(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f1 = {
        let items = v
            .meeting_dates
            .iter()
            .map(|x| {
                let m = mirror_policy_path_meeting_date(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_policy_path_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        days: f0.0,
        days_len: f0.1,
        meeting_dates: f1.0,
        meeting_dates_len: f1.1,
    })
}
/// Mirror of `PointPolicyRate`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_point_policy_rate {
    pub meta: *const nw_response_meta,
    pub announced_on: *const c_char,
    pub authority: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub effective_on: *const c_char,
    pub instrument: *const c_char,
    pub observed_on: *const c_char,
    pub qualifier: i32,
    pub qualifier_raw: *const c_char,
    pub rate: *const c_char,
    pub rate_type: *const c_char,
    pub unit: *const c_char,
}
pub(crate) fn mirror_point_policy_rate(
    v: &sdk::models::PointPolicyRate,
    s: &mut Store,
) -> Result<nw_point_policy_rate, Unrepresentable> {
    Ok(nw_point_policy_rate {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        announced_on: match &v.announced_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        authority: s.str(&v.authority)?,
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        effective_on: match &v.effective_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        instrument: s.str(&v.instrument)?,
        observed_on: match &v.observed_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        qualifier: point_policy_rate_qualifier_code(v.qualifier.as_str()),
        qualifier_raw: s.str(v.qualifier.as_str())?,
        rate: s.str(v.rate.as_str())?,
        rate_type: s.str(&v.rate_type)?,
        unit: s.str(&v.unit)?,
    })
}
/// Mirror of `RangePolicyRate`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_range_policy_rate {
    pub meta: *const nw_response_meta,
    pub announced_on: *const c_char,
    pub authority: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub effective_on: *const c_char,
    pub instrument: *const c_char,
    pub lower_bound: *const c_char,
    pub observed_on: *const c_char,
    pub rate_type: *const c_char,
    pub unit: *const c_char,
    pub upper_bound: *const c_char,
}
pub(crate) fn mirror_range_policy_rate(
    v: &sdk::models::RangePolicyRate,
    s: &mut Store,
) -> Result<nw_range_policy_rate, Unrepresentable> {
    Ok(nw_range_policy_rate {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        announced_on: match &v.announced_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        authority: s.str(&v.authority)?,
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        effective_on: match &v.effective_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        instrument: s.str(&v.instrument)?,
        lower_bound: s.str(v.lower_bound.as_str())?,
        observed_on: match &v.observed_on {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        rate_type: s.str(&v.rate_type)?,
        unit: s.str(&v.unit)?,
        upper_bound: s.str(v.upper_bound.as_str())?,
    })
}
/// Union `PolicyRate`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_policy_rate {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub point_policy_rate: *const nw_point_policy_rate,
    pub range_policy_rate: *const nw_range_policy_rate,
}
pub(crate) fn mirror_policy_rate(
    v: &sdk::models::PolicyRate,
    s: &mut Store,
) -> Result<nw_policy_rate, Unrepresentable> {
    let mut head = nw_policy_rate {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        point_policy_rate: std::ptr::null(),
        range_policy_rate: std::ptr::null(),
    };
    match v {
        sdk::models::PolicyRate::PointPolicyRate(x) => {
            head.kind = c"PointPolicyRate".as_ptr();
            let m = mirror_point_policy_rate(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.point_policy_rate = s.boxed(m);
        }
        sdk::models::PolicyRate::RangePolicyRate(x) => {
            head.kind = c"RangePolicyRate".as_ptr();
            let m = mirror_range_policy_rate(x, s)?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
            head.range_policy_rate = s.boxed(m);
        }
        sdk::models::PolicyRate::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
            let meta = s.meta(&x.meta)?;
            head.meta = s.boxed(meta);
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Union `PolicyRateObservation`. `kind` is the variant model name, or "unknown". Exactly one variant pointer is non-NULL; for "unknown" it is `unknown_raw`, the exact JSON text. `meta` is set when the union is a response, else NULL. `kind`, `unknown_raw` and `meta` come first; variants follow in lock order.
#[repr(C)]
pub struct nw_policy_rate_observation {
    pub kind: *const c_char,
    pub unknown_raw: *const c_char,
    pub meta: *const nw_response_meta,
    pub point_policy_rate: *const nw_point_policy_rate,
    pub range_policy_rate: *const nw_range_policy_rate,
}
pub(crate) fn mirror_policy_rate_observation(
    v: &sdk::models::PolicyRateObservation,
    s: &mut Store,
) -> Result<nw_policy_rate_observation, Unrepresentable> {
    let mut head = nw_policy_rate_observation {
        kind: c"unknown".as_ptr(),
        unknown_raw: std::ptr::null(),
        meta: std::ptr::null(),
        point_policy_rate: std::ptr::null(),
        range_policy_rate: std::ptr::null(),
    };
    match v {
        sdk::models::PolicyRateObservation::PointPolicyRate(x) => {
            head.kind = c"PointPolicyRate".as_ptr();
            let m = mirror_point_policy_rate(x, s)?;
            head.point_policy_rate = s.boxed(m);
        }
        sdk::models::PolicyRateObservation::RangePolicyRate(x) => {
            head.kind = c"RangePolicyRate".as_ptr();
            let m = mirror_range_policy_rate(x, s)?;
            head.range_policy_rate = s.boxed(m);
        }
        sdk::models::PolicyRateObservation::Unknown(x) => {
            head.unknown_raw = s.str(x.raw.get())?;
        }
        _ => return Err(Unrepresentable::Variant),
    }
    Ok(head)
}
/// Mirror of `PolicyRateHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_policy_rate_history_response {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_policy_rate_observation,
    pub observations_len: usize,
}
pub(crate) fn mirror_policy_rate_history_response(
    v: &sdk::models::PolicyRateHistoryResponse,
    s: &mut Store,
) -> Result<nw_policy_rate_history_response, Unrepresentable> {
    let f2 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_policy_rate_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_policy_rate_history_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f2.0,
        observations_len: f2.1,
    })
}
/// Mirror of `PredictionResultSummary`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_result_summary {
    pub amended_at: *const c_char,
    pub final_: bool,
    pub settled_at: *const c_char,
}
pub(crate) fn mirror_prediction_result_summary(
    v: &sdk::models::PredictionResultSummary,
    s: &mut Store,
) -> Result<nw_prediction_result_summary, Unrepresentable> {
    Ok(nw_prediction_result_summary {
        amended_at: match &v.amended_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        final_: v.final_,
        settled_at: match &v.settled_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PredictionEventSummary`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_event_summary {
    pub closes_at: *const c_char,
    pub event_id: *const c_char,
    pub question: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub shape: i32,
    pub shape_raw: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub topic_id: *const c_char,
    pub result: *const nw_prediction_result_summary,
}
pub(crate) fn mirror_prediction_event_summary(
    v: &sdk::models::PredictionEventSummary,
    s: &mut Store,
) -> Result<nw_prediction_event_summary, Unrepresentable> {
    Ok(nw_prediction_event_summary {
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        question: s.str(&v.question)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        shape: prediction_shape_code(v.shape.as_str()),
        shape_raw: s.str(v.shape.as_str())?,
        status: prediction_event_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        topic_id: s.str(&v.topic_id)?,
        result: match &v.result {
            Some(x) => {
                let v = mirror_prediction_result_summary(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PredictionEventsPage`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_prediction_events_page {
    pub meta: *const nw_response_meta,
    pub events: *const *const nw_prediction_event_summary,
    pub events_len: usize,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_prediction_events_page(
    v: &sdk::models::PredictionEventsPage,
    s: &mut Store,
) -> Result<nw_prediction_events_page, Unrepresentable> {
    let f0 = {
        let items = v
            .events
            .iter()
            .map(|x| {
                let m = mirror_prediction_event_summary(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_events_page {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        events: f0.0,
        events_len: f0.1,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `PredictionBar`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_bar {
    pub close: *const c_char,
    pub high: *const c_char,
    pub low: *const c_char,
    pub open: *const c_char,
    pub timestamp: *const c_char,
}
pub(crate) fn mirror_prediction_bar(
    v: &sdk::models::PredictionBar,
    s: &mut Store,
) -> Result<nw_prediction_bar, Unrepresentable> {
    Ok(nw_prediction_bar {
        close: s.str(&v.close)?,
        high: match &v.high {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        low: match &v.low {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        open: match &v.open {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        timestamp: s.str(&v.timestamp)?,
    })
}
/// Mirror of `PredictionSeries`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_series {
    pub active_from: *const c_char,
    pub active_to: *const c_char,
    pub bars: *const *const nw_prediction_bar,
    pub bars_len: usize,
    pub key: *const c_char,
}
pub(crate) fn mirror_prediction_series(
    v: &sdk::models::PredictionSeries,
    s: &mut Store,
) -> Result<nw_prediction_series, Unrepresentable> {
    let f2 = {
        let items = v
            .bars
            .iter()
            .map(|x| {
                let m = mirror_prediction_bar(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_series {
        active_from: s.str(&v.active_from)?,
        active_to: match &v.active_to {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        bars: f2.0,
        bars_len: f2.1,
        key: s.str(&v.key)?,
    })
}
/// Mirror of `PredictionHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_prediction_history {
    pub meta: *const nw_response_meta,
    pub backfill_venue: i32,
    pub backfill_venue_raw: *const c_char,
    pub end: *const c_char,
    pub series: *const *const nw_prediction_series,
    pub series_len: usize,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_prediction_history(
    v: &sdk::models::PredictionHistory,
    s: &mut Store,
) -> Result<nw_prediction_history, Unrepresentable> {
    let f3 = {
        let items = v
            .series
            .iter()
            .map(|x| {
                let m = mirror_prediction_series(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        backfill_venue: v
            .backfill_venue
            .as_ref()
            .map_or(0, |x| venue_code(x.as_str())),
        backfill_venue_raw: match &v.backfill_venue {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        end: match &v.end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        series: f3.0,
        series_len: f3.1,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `TopicCalendar`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_topic_calendar {
    pub country: *const c_char,
    pub family: *const c_char,
}
pub(crate) fn mirror_topic_calendar(
    v: &sdk::models::TopicCalendar,
    s: &mut Store,
) -> Result<nw_topic_calendar, Unrepresentable> {
    Ok(nw_topic_calendar {
        country: s.str(&v.country)?,
        family: s.str(&v.family)?,
    })
}
/// Mirror of `PredictionTopicEvent`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_topic_event {
    pub closes_at: *const c_char,
    pub event_id: *const c_char,
    pub question: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub result: *const nw_prediction_result_summary,
    pub status: i32,
    pub status_raw: *const c_char,
}
pub(crate) fn mirror_prediction_topic_event(
    v: &sdk::models::PredictionTopicEvent,
    s: &mut Store,
) -> Result<nw_prediction_topic_event, Unrepresentable> {
    Ok(nw_prediction_topic_event {
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        question: s.str(&v.question)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        result: match &v.result {
            Some(x) => {
                let v = mirror_prediction_result_summary(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        status: prediction_event_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
    })
}
/// Mirror of `PredictionTopicDetail`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_prediction_topic_detail {
    pub meta: *const nw_response_meta,
    pub calendar: *const nw_topic_calendar,
    pub events_len: usize,
    pub group: i32,
    pub group_raw: *const c_char,
    pub resolution_kind: i32,
    pub resolution_kind_raw: *const c_char,
    pub shape: i32,
    pub shape_raw: *const c_char,
    pub title: *const c_char,
    pub topic_id: *const c_char,
    pub unit: *const c_char,
    pub venues: *const i32,
    pub venues_raw: *const *const c_char,
    pub venues_len: usize,
    pub events: *const *const nw_prediction_topic_event,
}
pub(crate) fn mirror_prediction_topic_detail(
    v: &sdk::models::PredictionTopicDetail,
    s: &mut Store,
) -> Result<nw_prediction_topic_detail, Unrepresentable> {
    let f1 = {
        let items = v
            .events
            .iter()
            .map(|x| {
                let m = mirror_prediction_topic_event(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f8 = {
        let codes = v
            .venues
            .iter()
            .map(|x| venue_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .venues
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    Ok(nw_prediction_topic_detail {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        calendar: match &v.calendar {
            Some(x) => {
                let v = mirror_topic_calendar(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        events_len: f1.1,
        group: topic_group_code(v.group.as_str()),
        group_raw: s.str(v.group.as_str())?,
        resolution_kind: resolution_kind_code(v.resolution_kind.as_str()),
        resolution_kind_raw: s.str(v.resolution_kind.as_str())?,
        shape: prediction_shape_code(v.shape.as_str()),
        shape_raw: s.str(v.shape.as_str())?,
        title: s.str(&v.title)?,
        topic_id: s.str(&v.topic_id)?,
        unit: match &v.unit {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f8.0,
        venues_raw: f8.1,
        venues_len: f8.2,
        events: f1.0,
    })
}
/// Mirror of `PredictionTopic`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_topic {
    pub calendar: *const nw_topic_calendar,
    pub group: i32,
    pub group_raw: *const c_char,
    pub next_event_id: *const c_char,
    pub resolution_kind: i32,
    pub resolution_kind_raw: *const c_char,
    pub shape: i32,
    pub shape_raw: *const c_char,
    pub title: *const c_char,
    pub topic_id: *const c_char,
    pub unit: *const c_char,
    pub venues: *const i32,
    pub venues_raw: *const *const c_char,
    pub venues_len: usize,
}
pub(crate) fn mirror_prediction_topic(
    v: &sdk::models::PredictionTopic,
    s: &mut Store,
) -> Result<nw_prediction_topic, Unrepresentable> {
    let f8 = {
        let codes = v
            .venues
            .iter()
            .map(|x| venue_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .venues
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    Ok(nw_prediction_topic {
        calendar: match &v.calendar {
            Some(x) => {
                let v = mirror_topic_calendar(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        group: topic_group_code(v.group.as_str()),
        group_raw: s.str(v.group.as_str())?,
        next_event_id: match &v.next_event_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolution_kind: resolution_kind_code(v.resolution_kind.as_str()),
        resolution_kind_raw: s.str(v.resolution_kind.as_str())?,
        shape: prediction_shape_code(v.shape.as_str()),
        shape_raw: s.str(v.shape.as_str())?,
        title: s.str(&v.title)?,
        topic_id: s.str(&v.topic_id)?,
        unit: match &v.unit {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f8.0,
        venues_raw: f8.1,
        venues_len: f8.2,
    })
}
/// Mirror of `PredictionTopics`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_prediction_topics {
    pub meta: *const nw_response_meta,
    pub topics: *const *const nw_prediction_topic,
    pub topics_len: usize,
}
pub(crate) fn mirror_prediction_topics(
    v: &sdk::models::PredictionTopics,
    s: &mut Store,
) -> Result<nw_prediction_topics, Unrepresentable> {
    let f0 = {
        let items = v
            .topics
            .iter()
            .map(|x| {
                let m = mirror_prediction_topic(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_topics {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        topics: f0.0,
        topics_len: f0.1,
    })
}
/// Mirror of `RateHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_rate_history_observation {
    pub period: *const c_char,
    pub rate: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_rate_history_observation(
    v: &sdk::models::RateHistoryObservation,
    s: &mut Store,
) -> Result<nw_rate_history_observation, Unrepresentable> {
    Ok(nw_rate_history_observation {
        period: s.str(&v.period)?,
        rate: s.str(v.rate.as_str())?,
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `RateHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_rate_history {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_rate_history_observation,
    pub observations_len: usize,
    pub source: *const nw_series_source,
    pub start: *const c_char,
}
pub(crate) fn mirror_rate_history(
    v: &sdk::models::RateHistory,
    s: &mut Store,
) -> Result<nw_rate_history, Unrepresentable> {
    let f3 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_rate_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_rate_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f3.0,
        observations_len: f3.1,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `ResourceHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_resource_history {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_resource_observation,
    pub observations_len: usize,
    pub series_id: *const c_char,
    pub source: *const nw_series_source,
    pub unit: i32,
    pub unit_raw: *const c_char,
    pub as_of: *const c_char,
}
pub(crate) fn mirror_resource_history(
    v: &sdk::models::ResourceHistory,
    s: &mut Store,
) -> Result<nw_resource_history, Unrepresentable> {
    let f4 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_resource_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_resource_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        currency: match &v.currency {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f4.0,
        observations_len: f4.1,
        series_id: s.str(&v.series_id)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        unit: resource_unit_code(v.unit.as_str()),
        unit_raw: s.str(v.unit.as_str())?,
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `ScalarOutcome`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_scalar_outcome {
    pub key: *const c_char,
    pub label: *const c_char,
    pub lower: *const c_char,
    pub probability: *const c_char,
    pub quotes: *const *const nw_quote,
    pub quotes_len: usize,
    pub upper: *const c_char,
}
pub(crate) fn mirror_scalar_outcome(
    v: &sdk::models::ScalarOutcome,
    s: &mut Store,
) -> Result<nw_scalar_outcome, Unrepresentable> {
    let f4 = {
        let items = v
            .quotes
            .iter()
            .map(|x| {
                let m = mirror_quote(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_scalar_outcome {
        key: s.str(&v.key)?,
        label: s.str(&v.label)?,
        lower: match &v.lower {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        probability: match &v.probability {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        quotes: f4.0,
        quotes_len: f4.1,
        upper: match &v.upper {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `ScalarEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_scalar_event {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub closes_at: *const c_char,
    pub event_id: *const c_char,
    pub question: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub topic_id: *const c_char,
    pub unit: *const c_char,
    pub venues: *const *const nw_prediction_venue,
    pub venues_len: usize,
    pub above: *const i32,
    pub above_raw: *const *const c_char,
    pub above_len: usize,
    pub median: *const c_char,
    pub outcomes: *const *const nw_scalar_outcome,
    pub outcomes_len: usize,
    pub p10: *const c_char,
    pub p90: *const c_char,
    pub resolution: *const c_char,
    pub resolves_with: *const c_char,
    pub result: *const nw_prediction_result,
    pub has_stale: bool,
    pub stale: bool,
}
pub(crate) fn mirror_scalar_event(
    v: &sdk::models::ScalarEvent,
    s: &mut Store,
) -> Result<nw_scalar_event, Unrepresentable> {
    let f0 = {
        let codes = v
            .above
            .iter()
            .map(|x| scalar_event_above_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .above
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    let f5 = {
        let items = v
            .outcomes
            .iter()
            .map(|x| {
                let m = mirror_scalar_outcome(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f18 = {
        let items = v
            .venues
            .iter()
            .map(|x| {
                let m = mirror_prediction_venue(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_scalar_event {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        question: s.str(&v.question)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        status: prediction_event_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        topic_id: s.str(&v.topic_id)?,
        unit: match &v.unit {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f18.0,
        venues_len: f18.1,
        above: f0.0,
        above_raw: f0.1,
        above_len: f0.2,
        median: match &v.median {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        outcomes: f5.0,
        outcomes_len: f5.1,
        p10: match &v.p10 {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        p90: match &v.p90 {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        resolution: s.str(&v.resolution)?,
        resolves_with: match &v.resolves_with {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        result: match &v.result {
            Some(x) => {
                let v = mirror_prediction_result(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
    })
}
/// Mirror of `SplitsCalendarEntry`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_splits_calendar_entry {
    pub date: *const c_char,
    pub id: *const c_char,
    pub name: *const c_char,
    pub ratio: *const c_char,
    pub symbol: *const c_char,
}
pub(crate) fn mirror_splits_calendar_entry(
    v: &sdk::models::SplitsCalendarEntry,
    s: &mut Store,
) -> Result<nw_splits_calendar_entry, Unrepresentable> {
    Ok(nw_splits_calendar_entry {
        date: s.str(&v.date)?,
        id: s.str(&v.id)?,
        name: s.str(&v.name)?,
        ratio: s.str(&v.ratio)?,
        symbol: s.str(&v.symbol)?,
    })
}
/// Mirror of `SplitsCalendar`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_splits_calendar {
    pub meta: *const nw_response_meta,
    pub entries: *const *const nw_splits_calendar_entry,
    pub entries_len: usize,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_splits_calendar(
    v: &sdk::models::SplitsCalendar,
    s: &mut Store,
) -> Result<nw_splits_calendar, Unrepresentable> {
    let f0 = {
        let items = v
            .entries
            .iter()
            .map(|x| {
                let m = mirror_splits_calendar_entry(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_splits_calendar {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        entries: f0.0,
        entries_len: f0.1,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `TimingStep`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_timing_step {
    pub by: *const c_char,
    pub key: *const c_char,
    pub label: *const c_char,
    pub probability: *const c_char,
    pub quotes: *const *const nw_quote,
    pub quotes_len: usize,
    pub kind: i32,
    pub kind_raw: *const c_char,
}
pub(crate) fn mirror_timing_step(
    v: &sdk::models::TimingStep,
    s: &mut Store,
) -> Result<nw_timing_step, Unrepresentable> {
    let f5 = {
        let items = v
            .quotes
            .iter()
            .map(|x| {
                let m = mirror_quote(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_timing_step {
        by: s.str(&v.by)?,
        key: s.str(&v.key)?,
        label: s.str(&v.label)?,
        probability: match &v.probability {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        quotes: f5.0,
        quotes_len: f5.1,
        kind: timing_step_kind_code(v.kind.as_str()),
        kind_raw: s.str(v.kind.as_str())?,
    })
}
/// Mirror of `TimingEvent`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_timing_event {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub closes_at: *const c_char,
    pub event_id: *const c_char,
    pub question: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub topic_id: *const c_char,
    pub unit: *const c_char,
    pub venues: *const *const nw_prediction_venue,
    pub venues_len: usize,
    pub not_within_window: *const c_char,
    pub resolution: *const c_char,
    pub resolves_with: *const c_char,
    pub result: *const nw_prediction_result,
    pub has_stale: bool,
    pub stale: bool,
    pub steps: *const *const nw_timing_step,
    pub steps_len: usize,
}
pub(crate) fn mirror_timing_event(
    v: &sdk::models::TimingEvent,
    s: &mut Store,
) -> Result<nw_timing_event, Unrepresentable> {
    let f12 = {
        let items = v
            .steps
            .iter()
            .map(|x| {
                let m = mirror_timing_step(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f15 = {
        let items = v
            .venues
            .iter()
            .map(|x| {
                let m = mirror_prediction_venue(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_timing_event {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        question: s.str(&v.question)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        status: prediction_event_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        topic_id: s.str(&v.topic_id)?,
        unit: match &v.unit {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        venues: f15.0,
        venues_len: f15.1,
        not_within_window: match &v.not_within_window {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolution: s.str(&v.resolution)?,
        resolves_with: match &v.resolves_with {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        result: match &v.result {
            Some(x) => {
                let v = mirror_prediction_result(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        has_stale: v.stale.is_some(),
        stale: v.stale.unwrap_or_default(),
        steps: f12.0,
        steps_len: f12.1,
    })
}
/// Mirror of `TradeHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_trade_history_observation {
    pub exports: *const c_char,
    pub imports: *const c_char,
    pub period: *const c_char,
    pub first_reported_at: *const c_char,
}
pub(crate) fn mirror_trade_history_observation(
    v: &sdk::models::TradeHistoryObservation,
    s: &mut Store,
) -> Result<nw_trade_history_observation, Unrepresentable> {
    Ok(nw_trade_history_observation {
        exports: match &v.exports {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        imports: match &v.imports {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        period: s.str(&v.period)?,
        first_reported_at: match &v.first_reported_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `TradeHistory`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_trade_history {
    pub meta: *const nw_response_meta,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_trade_history_observation,
    pub observations_len: usize,
    pub source: *const nw_series_source,
    pub start: *const c_char,
}
pub(crate) fn mirror_trade_history(
    v: &sdk::models::TradeHistory,
    s: &mut Store,
) -> Result<nw_trade_history, Unrepresentable> {
    let f4 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_trade_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_trade_history {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        as_of: match &v.as_of {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f4.0,
        observations_len: f4.1,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        start: match &v.start {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `TradeSummary`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_trade_summary {
    pub meta: *const nw_response_meta,
    pub balance: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub exports: *const c_char,
    pub imports: *const c_char,
    pub period: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
}
pub(crate) fn mirror_trade_summary(
    v: &sdk::models::TradeSummary,
    s: &mut Store,
) -> Result<nw_trade_summary, Unrepresentable> {
    Ok(nw_trade_summary {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        balance: s.str(v.balance.as_str())?,
        country: s.str(&v.country)?,
        currency: s.str(&v.currency)?,
        exports: s.str(v.exports.as_str())?,
        imports: s.str(v.imports.as_str())?,
        period: s.str(&v.period)?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `Unemployment`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_unemployment {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub period: *const c_char,
    pub rate: *const c_char,
    pub released_on: *const c_char,
    pub source: *const nw_series_source,
}
pub(crate) fn mirror_unemployment(
    v: &sdk::models::Unemployment,
    s: &mut Store,
) -> Result<nw_unemployment, Unrepresentable> {
    Ok(nw_unemployment {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        period: s.str(&v.period)?,
        rate: s.str(v.rate.as_str())?,
        released_on: s.str(&v.released_on)?,
        source: match &v.source {
            Some(x) => {
                let v = mirror_series_source(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `WebhookDelivery`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_webhook_delivery {
    pub attempts: i64,
    pub config_version: i64,
    pub created_at: *const c_char,
    pub delivery_id: *const c_char,
    pub has_event_cursor: bool,
    pub event_cursor: i64,
    pub has_is_test: bool,
    pub is_test: bool,
    pub last_error: *const c_char,
    pub has_last_status: bool,
    pub last_status: i64,
    pub next_attempt_at: *const c_char,
    pub replays: i64,
    pub retry_until: *const c_char,
    pub state: i32,
    pub state_raw: *const c_char,
    pub updated_at: *const c_char,
    pub has_replays: bool,
}
pub(crate) fn mirror_webhook_delivery(
    v: &sdk::models::WebhookDelivery,
    s: &mut Store,
) -> Result<nw_webhook_delivery, Unrepresentable> {
    Ok(nw_webhook_delivery {
        attempts: v.attempts,
        config_version: v.config_version,
        created_at: s.str(&v.created_at)?,
        delivery_id: s.str(&v.delivery_id)?,
        has_event_cursor: v.event_cursor.is_some(),
        event_cursor: v.event_cursor.unwrap_or_default(),
        has_is_test: v.is_test.is_some(),
        is_test: v.is_test.unwrap_or_default(),
        last_error: match &v.last_error {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_last_status: v.last_status.is_some(),
        last_status: v.last_status.unwrap_or_default(),
        next_attempt_at: match &v.next_attempt_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        replays: v.replays.unwrap_or_default(),
        retry_until: match &v.retry_until {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        state: webhook_delivery_state_code(v.state.as_str()),
        state_raw: s.str(v.state.as_str())?,
        updated_at: s.str(&v.updated_at)?,
        has_replays: v.replays.is_some(),
    })
}
/// Mirror of `WebhookDeliveriesResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_webhook_deliveries_response {
    pub meta: *const nw_response_meta,
    pub deliveries: *const *const nw_webhook_delivery,
    pub deliveries_len: usize,
    pub next_cursor: *const c_char,
}
pub(crate) fn mirror_webhook_deliveries_response(
    v: &sdk::models::WebhookDeliveriesResponse,
    s: &mut Store,
) -> Result<nw_webhook_deliveries_response, Unrepresentable> {
    let f0 = {
        let items = v
            .deliveries
            .iter()
            .map(|x| {
                let m = mirror_webhook_delivery(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_webhook_deliveries_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        deliveries: f0.0,
        deliveries_len: f0.1,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
    })
}
/// Mirror of `WebhookDestinationCreated`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_webhook_destination_created {
    pub meta: *const nw_response_meta,
    pub config_version: i64,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub created_at: *const c_char,
    pub destination_id: *const c_char,
    pub enabled: bool,
    pub families: *const i32,
    pub families_raw: *const *const c_char,
    pub families_len: usize,
    pub secret: *const c_char,
    pub secret_version: i64,
    pub updated_at: *const c_char,
    pub url: *const c_char,
    pub disabled_reason: i32,
    pub disabled_reason_raw: *const c_char,
    pub companies: *const *const c_char,
    pub companies_len: usize,
}
pub(crate) fn mirror_webhook_destination_created(
    v: &sdk::models::WebhookDestinationCreated,
    s: &mut Store,
) -> Result<nw_webhook_destination_created, Unrepresentable> {
    let f0 = match &v.companies {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f2 = {
        let items = v
            .countries
            .iter()
            .map(|x| s.str(x))
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f7 = {
        let codes = v
            .families
            .iter()
            .map(|x| webhook_family_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .families
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    Ok(nw_webhook_destination_created {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        config_version: v.config_version,
        countries: f2.0,
        countries_len: f2.1,
        created_at: s.str(&v.created_at)?,
        destination_id: s.str(&v.destination_id)?,
        enabled: v.enabled,
        families: f7.0,
        families_raw: f7.1,
        families_len: f7.2,
        secret: s.str(&v.secret)?,
        secret_version: v.secret_version,
        updated_at: s.str(&v.updated_at)?,
        url: s.str(&v.url)?,
        disabled_reason: v
            .disabled_reason
            .as_ref()
            .map_or(0, |x| webhook_disabled_reason_code(x.as_str())),
        disabled_reason_raw: match &v.disabled_reason {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        companies: f0.0,
        companies_len: f0.1,
    })
}
/// Mirror of `WebhookDestinationResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_webhook_destination_response {
    pub meta: *const nw_response_meta,
    pub config_version: i64,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub created_at: *const c_char,
    pub destination_id: *const c_char,
    pub enabled: bool,
    pub families: *const i32,
    pub families_raw: *const *const c_char,
    pub families_len: usize,
    pub secret_version: i64,
    pub updated_at: *const c_char,
    pub url: *const c_char,
    pub disabled_reason: i32,
    pub disabled_reason_raw: *const c_char,
    pub companies: *const *const c_char,
    pub companies_len: usize,
}
pub(crate) fn mirror_webhook_destination_response(
    v: &sdk::models::WebhookDestinationResponse,
    s: &mut Store,
) -> Result<nw_webhook_destination_response, Unrepresentable> {
    let f0 = match &v.companies {
        Some(a) => {
            let items = a.iter().map(|x| s.str(x)).collect::<Result<Vec<_>, _>>()?;
            s.array(items)
        }
        None => (std::ptr::null(), 0),
    };
    let f2 = {
        let items = v
            .countries
            .iter()
            .map(|x| s.str(x))
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f7 = {
        let codes = v
            .families
            .iter()
            .map(|x| webhook_family_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .families
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    Ok(nw_webhook_destination_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        config_version: v.config_version,
        countries: f2.0,
        countries_len: f2.1,
        created_at: s.str(&v.created_at)?,
        destination_id: s.str(&v.destination_id)?,
        enabled: v.enabled,
        families: f7.0,
        families_raw: f7.1,
        families_len: f7.2,
        secret_version: v.secret_version,
        updated_at: s.str(&v.updated_at)?,
        url: s.str(&v.url)?,
        disabled_reason: v
            .disabled_reason
            .as_ref()
            .map_or(0, |x| webhook_disabled_reason_code(x.as_str())),
        disabled_reason_raw: match &v.disabled_reason {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        companies: f0.0,
        companies_len: f0.1,
    })
}
/// Mirror of `WebhookDestinationList`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_webhook_destination_list {
    pub meta: *const nw_response_meta,
    pub destinations: *const *const nw_webhook_destination_response,
    pub destinations_len: usize,
    pub destination_limit: i64,
}
pub(crate) fn mirror_webhook_destination_list(
    v: &sdk::models::WebhookDestinationList,
    s: &mut Store,
) -> Result<nw_webhook_destination_list, Unrepresentable> {
    let f1 = {
        let items = v
            .destinations
            .iter()
            .map(|x| {
                let m = mirror_webhook_destination_response(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_webhook_destination_list {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        destinations: f1.0,
        destinations_len: f1.1,
        destination_limit: v.destination_limit,
    })
}
/// Mirror of `WebhookConfigurationHistory`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_webhook_configuration_history {
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub enabled: bool,
    pub families: *const i32,
    pub families_raw: *const *const c_char,
    pub families_len: usize,
    pub recorded_at: *const c_char,
    pub url: *const c_char,
    pub config_version: i64,
}
pub(crate) fn mirror_webhook_configuration_history(
    v: &sdk::models::WebhookConfigurationHistory,
    s: &mut Store,
) -> Result<nw_webhook_configuration_history, Unrepresentable> {
    let f1 = {
        let items = v
            .countries
            .iter()
            .map(|x| s.str(x))
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    let f3 = {
        let codes = v
            .families
            .iter()
            .map(|x| webhook_family_code(x.as_str()))
            .collect::<Vec<_>>();
        let raw = v
            .families
            .iter()
            .map(|x| s.str(x.as_str()))
            .collect::<Result<Vec<_>, _>>()?;
        let (codes, len) = s.array(codes);
        (codes, s.array(raw).0, len)
    };
    Ok(nw_webhook_configuration_history {
        countries: f1.0,
        countries_len: f1.1,
        enabled: v.enabled,
        families: f3.0,
        families_raw: f3.1,
        families_len: f3.2,
        recorded_at: s.str(&v.recorded_at)?,
        url: s.str(&v.url)?,
        config_version: v.config_version,
    })
}
/// Mirror of `WebhookHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_webhook_history_response {
    pub meta: *const nw_response_meta,
    pub history: *const *const nw_webhook_configuration_history,
    pub history_len: usize,
}
pub(crate) fn mirror_webhook_history_response(
    v: &sdk::models::WebhookHistoryResponse,
    s: &mut Store,
) -> Result<nw_webhook_history_response, Unrepresentable> {
    let f0 = {
        let items = v
            .history
            .iter()
            .map(|x| {
                let m = mirror_webhook_configuration_history(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_webhook_history_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        history: f0.0,
        history_len: f0.1,
    })
}
/// Mirror of `WebhookTestResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_webhook_test_response {
    pub meta: *const nw_response_meta,
    pub delivery_id: *const c_char,
    pub has_retry_after_seconds: bool,
    pub retry_after_seconds: i64,
    pub status: i32,
    pub status_raw: *const c_char,
}
pub(crate) fn mirror_webhook_test_response(
    v: &sdk::models::WebhookTestResponse,
    s: &mut Store,
) -> Result<nw_webhook_test_response, Unrepresentable> {
    Ok(nw_webhook_test_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        delivery_id: match &v.delivery_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        has_retry_after_seconds: v.retry_after_seconds.is_some(),
        retry_after_seconds: v.retry_after_seconds.unwrap_or_default(),
        status: webhook_test_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
    })
}
/// Mirror of `YieldPoint`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_yield_point {
    pub maturity: *const c_char,
    pub yield_percent: *const c_char,
}
pub(crate) fn mirror_yield_point(
    v: &sdk::models::YieldPoint,
    s: &mut Store,
) -> Result<nw_yield_point, Unrepresentable> {
    Ok(nw_yield_point {
        maturity: s.str(&v.maturity)?,
        yield_percent: s.str(v.yield_percent.as_str())?,
    })
}
/// Mirror of `YieldCurve`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_yield_curve {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub curve_type: i32,
    pub curve_type_raw: *const c_char,
    pub date: *const c_char,
    pub yields: *const *const nw_yield_point,
    pub yields_len: usize,
}
pub(crate) fn mirror_yield_curve(
    v: &sdk::models::YieldCurve,
    s: &mut Store,
) -> Result<nw_yield_curve, Unrepresentable> {
    let f3 = {
        let items = v
            .yields
            .iter()
            .map(|x| {
                let m = mirror_yield_point(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_yield_curve {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        curve_type: yield_curve_type_code(v.curve_type.as_str()),
        curve_type_raw: s.str(v.curve_type.as_str())?,
        date: s.str(&v.date)?,
        yields: f3.0,
        yields_len: f3.1,
    })
}
/// Mirror of `YieldCurveHistoryObservation`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_yield_curve_history_observation {
    pub date: *const c_char,
    pub yields: *const *const nw_yield_point,
    pub yields_len: usize,
}
pub(crate) fn mirror_yield_curve_history_observation(
    v: &sdk::models::YieldCurveHistoryObservation,
    s: &mut Store,
) -> Result<nw_yield_curve_history_observation, Unrepresentable> {
    let f1 = {
        let items = v
            .yields
            .iter()
            .map(|x| {
                let m = mirror_yield_point(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_yield_curve_history_observation {
        date: s.str(&v.date)?,
        yields: f1.0,
        yields_len: f1.1,
    })
}
/// Mirror of `YieldCurveHistoryResponse`. Owned by the enclosing response. `meta` is never NULL; it carries header metadata for responses.
#[repr(C)]
pub struct nw_yield_curve_history_response {
    pub meta: *const nw_response_meta,
    pub country: *const c_char,
    pub curve_type: i32,
    pub curve_type_raw: *const c_char,
    pub next_cursor: *const c_char,
    pub observations: *const *const nw_yield_curve_history_observation,
    pub observations_len: usize,
}
pub(crate) fn mirror_yield_curve_history_response(
    v: &sdk::models::YieldCurveHistoryResponse,
    s: &mut Store,
) -> Result<nw_yield_curve_history_response, Unrepresentable> {
    let f3 = {
        let items = v
            .observations
            .iter()
            .map(|x| {
                let m = mirror_yield_curve_history_observation(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_yield_curve_history_response {
        meta: {
            let m = s.meta(&v.meta)?;
            s.boxed(m)
        },
        country: s.str(&v.country)?,
        curve_type: yield_curve_type_code(v.curve_type.as_str()),
        curve_type_raw: s.str(v.curve_type.as_str())?,
        next_cursor: match &v.next_cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        observations: f3.0,
        observations_len: f3.1,
    })
}
/// Mirror of `FilingWebhookEvent`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_filing_webhook_event {
    pub company_id: *const c_char,
    pub filed_at: *const c_char,
    pub filing_id: *const c_char,
    pub form: *const c_char,
    pub fp: i32,
    pub fp_raw: *const c_char,
    pub has_fy: bool,
    pub fy: i64,
    pub period_end: *const c_char,
    pub subject_id: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
}
pub(crate) fn mirror_filing_webhook_event(
    v: &sdk::models::FilingWebhookEvent,
    s: &mut Store,
) -> Result<nw_filing_webhook_event, Unrepresentable> {
    Ok(nw_filing_webhook_event {
        company_id: s.str(&v.company_id)?,
        filed_at: s.str(&v.filed_at)?,
        filing_id: s.str(&v.filing_id)?,
        form: s.str(&v.form)?,
        fp: v.fp.as_ref().map_or(0, |x| fiscal_period_code(x.as_str())),
        fp_raw: match &v.fp {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        has_fy: v.fy.is_some(),
        fy: v.fy.unwrap_or_default(),
        period_end: match &v.period_end {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        subject_id: match &v.subject_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        type_: filing_event_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
    })
}
/// Mirror of `PredictionWebhookSettlement`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_webhook_settlement {
    pub status: i32,
    pub status_raw: *const c_char,
    pub venue: i32,
    pub venue_raw: *const c_char,
}
pub(crate) fn mirror_prediction_webhook_settlement(
    v: &sdk::models::PredictionWebhookSettlement,
    s: &mut Store,
) -> Result<nw_prediction_webhook_settlement, Unrepresentable> {
    Ok(nw_prediction_webhook_settlement {
        status: settlement_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        venue: venue_code(v.venue.as_str()),
        venue_raw: s.str(v.venue.as_str())?,
    })
}
/// Mirror of `PredictionWebhookEvent`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_prediction_webhook_event {
    pub closes_at: *const c_char,
    pub release_id: *const c_char,
    pub resolves_at: *const c_char,
    pub settlements: *const *const nw_prediction_webhook_settlement,
    pub settlements_len: usize,
    pub shape: i32,
    pub shape_raw: *const c_char,
    pub status: i32,
    pub status_raw: *const c_char,
    pub topic_id: *const c_char,
    pub prediction_event_id: *const c_char,
}
pub(crate) fn mirror_prediction_webhook_event(
    v: &sdk::models::PredictionWebhookEvent,
    s: &mut Store,
) -> Result<nw_prediction_webhook_event, Unrepresentable> {
    let f4 = {
        let items = v
            .settlements
            .iter()
            .map(|x| {
                let m = mirror_prediction_webhook_settlement(x, s)?;
                Ok::<_, Unrepresentable>(s.boxed(m))
            })
            .collect::<Result<Vec<_>, _>>()?;
        s.array(items)
    };
    Ok(nw_prediction_webhook_event {
        closes_at: match &v.closes_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resolves_at: match &v.resolves_at {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        settlements: f4.0,
        settlements_len: f4.1,
        shape: prediction_shape_code(v.shape.as_str()),
        shape_raw: s.str(v.shape.as_str())?,
        status: prediction_webhook_status_code(v.status.as_str()),
        status_raw: s.str(v.status.as_str())?,
        topic_id: s.str(&v.topic_id)?,
        prediction_event_id: s.str(&v.prediction_event_id)?,
    })
}
/// Mirror of `WebhookEvent`. Owned by the enclosing response.
#[repr(C)]
pub struct nw_webhook_event {
    pub country: *const c_char,
    pub cursor: *const c_char,
    pub event_id: *const c_char,
    pub family: i32,
    pub family_raw: *const c_char,
    pub has_holiday_generation: bool,
    pub holiday_generation: i64,
    pub has_holiday_year: bool,
    pub holiday_year: i64,
    pub prediction: *const nw_prediction_webhook_event,
    pub recorded_at: *const c_char,
    pub release_id: *const c_char,
    pub resource_id: *const c_char,
    pub resource_version: *const c_char,
    pub schema_version: *const c_char,
    pub type_: i32,
    pub type_raw: *const c_char,
    pub filing: *const nw_filing_webhook_event,
}
pub(crate) fn mirror_webhook_event(
    v: &sdk::models::WebhookEvent,
    s: &mut Store,
) -> Result<nw_webhook_event, Unrepresentable> {
    Ok(nw_webhook_event {
        country: match &v.country {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        cursor: match &v.cursor {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        event_id: s.str(&v.event_id)?,
        family: v
            .family
            .as_ref()
            .map_or(0, |x| delivery_family_code(x.as_str())),
        family_raw: match &v.family {
            Some(x) => s.str(x.as_str())?,
            None => std::ptr::null(),
        },
        has_holiday_generation: v.holiday_generation.is_some(),
        holiday_generation: v.holiday_generation.unwrap_or_default(),
        has_holiday_year: v.holiday_year.is_some(),
        holiday_year: v.holiday_year.unwrap_or_default(),
        prediction: match &v.prediction {
            Some(x) => {
                let v = mirror_prediction_webhook_event(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
        recorded_at: s.str(&v.recorded_at)?,
        release_id: match &v.release_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resource_id: match &v.resource_id {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        resource_version: match &v.resource_version {
            Some(x) => s.str(x)?,
            None => std::ptr::null(),
        },
        schema_version: s.str(&v.schema_version)?,
        type_: webhook_event_type_code(v.type_.as_str()),
        type_raw: s.str(v.type_.as_str())?,
        filing: match &v.filing {
            Some(x) => {
                let v = mirror_filing_webhook_event(x, s)?;
                s.boxed(v)
            }
            None => std::ptr::null(),
        },
    })
}
/// Free a `nw_as_reported_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_as_reported_page_free(value: *mut nw_as_reported_page) {
    release(value)
}
/// Free a `nw_binary_event` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_binary_event_free(value: *mut nw_binary_event) {
    release(value)
}
/// Free a `nw_calendar_holidays` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_calendar_holidays_free(value: *mut nw_calendar_holidays) {
    release(value)
}
/// Free a `nw_calendar_holidays_on_date` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_calendar_holidays_on_date_free(
    value: *mut nw_calendar_holidays_on_date,
) {
    release(value)
}
/// Free a `nw_calendar_next_holiday` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_calendar_next_holiday_free(value: *mut nw_calendar_next_holiday) {
    release(value)
}
/// Free a `nw_calendar_subscription_created` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_calendar_subscription_created_free(
    value: *mut nw_calendar_subscription_created,
) {
    release(value)
}
/// Free a `nw_catalogue_bulk_latest` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_catalogue_bulk_latest_free(value: *mut nw_catalogue_bulk_latest) {
    release(value)
}
/// Free a `nw_catalogue_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_catalogue_history_free(value: *mut nw_catalogue_history) {
    release(value)
}
/// Free a `nw_catalogue_search_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_catalogue_search_page_free(value: *mut nw_catalogue_search_page) {
    release(value)
}
/// Free a `nw_catalogue_series` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_catalogue_series_free(value: *mut nw_catalogue_series) {
    release(value)
}
/// Free a `nw_central_bank_balance_sheet` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_central_bank_balance_sheet_free(
    value: *mut nw_central_bank_balance_sheet,
) {
    release(value)
}
/// Free a `nw_central_bank_reserves` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_central_bank_reserves_free(value: *mut nw_central_bank_reserves) {
    release(value)
}
/// Free a `nw_commodity_bars` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_commodity_bars_free(value: *mut nw_commodity_bars) {
    release(value)
}
/// Free a `nw_commodity_spot` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_commodity_spot_free(value: *mut nw_commodity_spot) {
    release(value)
}
/// Free a `nw_company_filings_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_company_filings_page_free(value: *mut nw_company_filings_page) {
    release(value)
}
/// Free a `nw_company_search_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_company_search_page_free(value: *mut nw_company_search_page) {
    release(value)
}
/// Free a `nw_company_with_listings` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_company_with_listings_free(value: *mut nw_company_with_listings) {
    release(value)
}
/// Free a `nw_cpi` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_cpi_free(value: *mut nw_cpi) {
    release(value)
}
/// Free a `nw_cpi_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_cpi_history_free(value: *mut nw_cpi_history) {
    release(value)
}
/// Free a `nw_crypto_bars_model` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_bars_model_free(value: *mut nw_crypto_bars_model) {
    release(value)
}
/// Free a `nw_crypto_pairs` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_pairs_free(value: *mut nw_crypto_pairs) {
    release(value)
}
/// Free a `nw_crypto_price_model` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_price_model_free(value: *mut nw_crypto_price_model) {
    release(value)
}
/// Free a `nw_crypto_prices_model` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_prices_model_free(value: *mut nw_crypto_prices_model) {
    release(value)
}
/// Free a `nw_dividends_calendar` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_dividends_calendar_free(value: *mut nw_dividends_calendar) {
    release(value)
}
/// Free a `nw_economic_calendar` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_economic_calendar_free(value: *mut nw_economic_calendar) {
    release(value)
}
/// Free a `nw_economic_release_detail` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_economic_release_detail_free(value: *mut nw_economic_release_detail) {
    release(value)
}
/// Free a `nw_equity_bars` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_bars_free(value: *mut nw_equity_bars) {
    release(value)
}
/// Free a `nw_equity_cost_of_capital` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_cost_of_capital_free(value: *mut nw_equity_cost_of_capital) {
    release(value)
}
/// Free a `nw_equity_dividends` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_dividends_free(value: *mut nw_equity_dividends) {
    release(value)
}
/// Free a `nw_equity_earnings` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_earnings_free(value: *mut nw_equity_earnings) {
    release(value)
}
/// Free a `nw_equity_movers` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_movers_free(value: *mut nw_equity_movers) {
    release(value)
}
/// Free a `nw_equity_price` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_price_free(value: *mut nw_equity_price) {
    release(value)
}
/// Free a `nw_equity_prices` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_prices_free(value: *mut nw_equity_prices) {
    release(value)
}
/// Free a `nw_equity_profile` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_profile_free(value: *mut nw_equity_profile) {
    release(value)
}
/// Free a `nw_equity_risk_premium` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_risk_premium_free(value: *mut nw_equity_risk_premium) {
    release(value)
}
/// Free a `nw_equity_risk_premium_history_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_risk_premium_history_response_free(
    value: *mut nw_equity_risk_premium_history_response,
) {
    release(value)
}
/// Free a `nw_equity_risk_premium_list` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_risk_premium_list_free(value: *mut nw_equity_risk_premium_list) {
    release(value)
}
/// Free a `nw_equity_search_results` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_search_results_free(value: *mut nw_equity_search_results) {
    release(value)
}
/// Free a `nw_equity_splits` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_splits_free(value: *mut nw_equity_splits) {
    release(value)
}
/// Free a `nw_equity_valuation` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_valuation_free(value: *mut nw_equity_valuation) {
    release(value)
}
/// Free a `nw_equity_valuation_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_equity_valuation_history_free(value: *mut nw_equity_valuation_history) {
    release(value)
}
/// Free a `nw_event_detail` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_event_detail_free(value: *mut nw_event_detail) {
    release(value)
}
/// Free a `nw_events_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_events_page_free(value: *mut nw_events_page) {
    release(value)
}
/// Free a `nw_extreme_event` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_extreme_event_free(value: *mut nw_extreme_event) {
    release(value)
}
/// Free a `nw_filing` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_filing_free(value: *mut nw_filing) {
    release(value)
}
/// Free a `nw_financials` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_financials_free(value: *mut nw_financials) {
    release(value)
}
/// Free a `nw_fund_distributions` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fund_distributions_free(value: *mut nw_fund_distributions) {
    release(value)
}
/// Free a `nw_fund_price` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fund_price_free(value: *mut nw_fund_price) {
    release(value)
}
/// Free a `nw_fund_prices` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fund_prices_free(value: *mut nw_fund_prices) {
    release(value)
}
/// Free a `nw_fund_profile` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fund_profile_free(value: *mut nw_fund_profile) {
    release(value)
}
/// Free a `nw_fund_search_results` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fund_search_results_free(value: *mut nw_fund_search_results) {
    release(value)
}
/// Free a `nw_fx_bars` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_bars_free(value: *mut nw_fx_bars) {
    release(value)
}
/// Free a `nw_fx_conversion` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_conversion_free(value: *mut nw_fx_conversion) {
    release(value)
}
/// Free a `nw_fx_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_history_free(value: *mut nw_fx_history) {
    release(value)
}
/// Free a `nw_fx_rates_model` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_rates_model_free(value: *mut nw_fx_rates_model) {
    release(value)
}
/// Free a `nw_gdp` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_gdp_free(value: *mut nw_gdp) {
    release(value)
}
/// Free a `nw_gdp_growth` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_gdp_growth_free(value: *mut nw_gdp_growth) {
    release(value)
}
/// Free a `nw_gdp_growth_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_gdp_growth_history_free(value: *mut nw_gdp_growth_history) {
    release(value)
}
/// Free a `nw_gdp_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_gdp_history_free(value: *mut nw_gdp_history) {
    release(value)
}
/// Free a `nw_holders_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_holders_page_free(value: *mut nw_holders_page) {
    release(value)
}
/// Free a `nw_inflation` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_inflation_free(value: *mut nw_inflation) {
    release(value)
}
/// Free a `nw_inflation_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_inflation_history_free(value: *mut nw_inflation_history) {
    release(value)
}
/// Free a `nw_labour_force_participation` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_labour_force_participation_free(
    value: *mut nw_labour_force_participation,
) {
    release(value)
}
/// Free a `nw_latest_filings_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_latest_filings_page_free(value: *mut nw_latest_filings_page) {
    release(value)
}
/// Free a `nw_no_content` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_no_content_free(value: *mut nw_no_content) {
    release(value)
}
/// Free a `nw_palmoil_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_palmoil_history_free(value: *mut nw_palmoil_history) {
    release(value)
}
/// Free a `nw_palmoil_price` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_palmoil_price_free(value: *mut nw_palmoil_price) {
    release(value)
}
/// Free a `nw_palmoil_series_discovery` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_palmoil_series_discovery_free(value: *mut nw_palmoil_series_discovery) {
    release(value)
}
/// Free a `nw_physical_latest` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_physical_latest_free(value: *mut nw_physical_latest) {
    release(value)
}
/// Free a `nw_physical_observations` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_physical_observations_free(value: *mut nw_physical_observations) {
    release(value)
}
/// Free a `nw_physical_series` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_physical_series_free(value: *mut nw_physical_series) {
    release(value)
}
/// Free a `nw_physical_series_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_physical_series_page_free(value: *mut nw_physical_series_page) {
    release(value)
}
/// Free a `nw_policy_path` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_policy_path_free(value: *mut nw_policy_path) {
    release(value)
}
/// Free a `nw_policy_path_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_policy_path_history_free(value: *mut nw_policy_path_history) {
    release(value)
}
/// Free a `nw_policy_rate` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_policy_rate_free(value: *mut nw_policy_rate) {
    release(value)
}
/// Free a `nw_policy_rate_history_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_policy_rate_history_response_free(
    value: *mut nw_policy_rate_history_response,
) {
    release(value)
}
/// Free a `nw_prediction_events_page` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_prediction_events_page_free(value: *mut nw_prediction_events_page) {
    release(value)
}
/// Free a `nw_prediction_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_prediction_history_free(value: *mut nw_prediction_history) {
    release(value)
}
/// Free a `nw_prediction_topic_detail` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_prediction_topic_detail_free(value: *mut nw_prediction_topic_detail) {
    release(value)
}
/// Free a `nw_prediction_topics` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_prediction_topics_free(value: *mut nw_prediction_topics) {
    release(value)
}
/// Free a `nw_rate_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_rate_history_free(value: *mut nw_rate_history) {
    release(value)
}
/// Free a `nw_resource_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_resource_history_free(value: *mut nw_resource_history) {
    release(value)
}
/// Free a `nw_scalar_event` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_scalar_event_free(value: *mut nw_scalar_event) {
    release(value)
}
/// Free a `nw_splits_calendar` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_splits_calendar_free(value: *mut nw_splits_calendar) {
    release(value)
}
/// Free a `nw_timing_event` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_timing_event_free(value: *mut nw_timing_event) {
    release(value)
}
/// Free a `nw_trade_history` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_trade_history_free(value: *mut nw_trade_history) {
    release(value)
}
/// Free a `nw_trade_summary` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_trade_summary_free(value: *mut nw_trade_summary) {
    release(value)
}
/// Free a `nw_unemployment` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_unemployment_free(value: *mut nw_unemployment) {
    release(value)
}
/// Free a `nw_webhook_deliveries_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_deliveries_response_free(
    value: *mut nw_webhook_deliveries_response,
) {
    release(value)
}
/// Free a `nw_webhook_destination_created` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_destination_created_free(
    value: *mut nw_webhook_destination_created,
) {
    release(value)
}
/// Free a `nw_webhook_destination_list` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_destination_list_free(value: *mut nw_webhook_destination_list) {
    release(value)
}
/// Free a `nw_webhook_destination_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_destination_response_free(
    value: *mut nw_webhook_destination_response,
) {
    release(value)
}
/// Free a `nw_webhook_event` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_event_free(value: *mut nw_webhook_event) {
    release(value)
}
/// Free a `nw_webhook_history_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_history_response_free(value: *mut nw_webhook_history_response) {
    release(value)
}
/// Free a `nw_webhook_test_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_webhook_test_response_free(value: *mut nw_webhook_test_response) {
    release(value)
}
/// Free a `nw_yield_curve` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_yield_curve_free(value: *mut nw_yield_curve) {
    release(value)
}
/// Free a `nw_yield_curve_history_response` returned through `**out`. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_yield_curve_history_response_free(
    value: *mut nw_yield_curve_history_response,
) {
    release(value)
}
/// Optional inputs for `nw_calendar_subscriptions_create`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_calendar_subscriptions_create_params {
    pub size: usize,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub end_date: *const c_char,
    pub families: *const *const c_char,
    pub families_len: usize,
    pub has_include_holidays: bool,
    pub include_holidays: bool,
    pub has_include_releases: bool,
    pub include_releases: bool,
    pub scope: *const c_char,
    pub start_date: *const c_char,
}
/// Create Calendar Subscription
///
/// Stability: stable. `POST /v1/calendars/subscriptions`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_calendar_subscription_created_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_calendar_subscriptions_create(
    client: *mut nw_client,
    params: *const nw_calendar_subscriptions_create_params,
    out: *mut *mut nw_calendar_subscription_created,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.calendar_subscriptions().create();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_calendar_subscriptions_create_params,
                        countries,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_calendar_subscriptions_create_params,
                        countries_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.countries(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendar_subscriptions_create_params,
                    end_date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end_date(v);
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_calendar_subscriptions_create_params,
                        families,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_calendar_subscriptions_create_params,
                        families_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.families(v);
                }
                if field!(
                    params,
                    size,
                    nw_calendar_subscriptions_create_params,
                    has_include_holidays,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_calendar_subscriptions_create_params,
                        include_holidays,
                        bool
                    ) {
                        request = request.include_holidays(v);
                    }
                }
                if field!(
                    params,
                    size,
                    nw_calendar_subscriptions_create_params,
                    has_include_releases,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_calendar_subscriptions_create_params,
                        include_releases,
                        bool
                    ) {
                        request = request.include_releases(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendar_subscriptions_create_params,
                    scope,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.scope(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendar_subscriptions_create_params,
                    start_date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start_date(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_calendar_subscription_created,
    )
}
/// Revoke Calendar Subscription
///
/// Stability: stable. `DELETE /v1/calendars/subscriptions/{subscription_id}`.
/// On success `*out` is owned by the caller; free it with `nw_no_content_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_calendar_subscriptions_revoke(
    client: *mut nw_client,
    subscription_id: *const c_char,
    out: *mut *mut nw_no_content,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .calendar_subscriptions()
                .revoke(arg(subscription_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_no_content,
    )
}
/// Optional inputs for `nw_calendars_holidays_on_date`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_calendars_holidays_on_date_params {
    pub size: usize,
    pub scope: *const c_char,
}
/// Get Calendar Holidays On Date
///
/// Stability: stable. `GET /v1/calendars/holidays/{country}/on/{date}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_calendar_holidays_on_date_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_calendars_holidays_on_date(
    client: *mut nw_client,
    country: *const c_char,
    date: *const c_char,
    params: *const nw_calendars_holidays_on_date_params,
    out: *mut *mut nw_calendar_holidays_on_date,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .calendars()
                .holidays_on_date(arg(country)?, arg(date)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendars_holidays_on_date_params,
                    scope,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.scope(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_calendar_holidays_on_date,
    )
}
/// Optional inputs for `nw_calendars_next_holiday`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_calendars_next_holiday_params {
    pub size: usize,
    pub scope: *const c_char,
}
/// Get Calendar Next Holiday
///
/// Stability: stable. `GET /v1/calendars/holidays/{country}/next`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_calendar_next_holiday_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_calendars_next_holiday(
    client: *mut nw_client,
    country: *const c_char,
    from_date: *const c_char,
    params: *const nw_calendars_next_holiday_params,
    out: *mut *mut nw_calendar_next_holiday,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .calendars()
                .next_holiday(arg(country)?, arg(from_date)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendars_next_holiday_params,
                    scope,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.scope(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_calendar_next_holiday,
    )
}
/// Optional inputs for `nw_calendars_list_holidays`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_calendars_list_holidays_params {
    pub size: usize,
    pub scope: *const c_char,
    pub version_id: *const c_char,
}
/// List Calendar Holidays
///
/// Stability: stable. `GET /v1/calendars/holidays/{country}/{year}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_calendar_holidays_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_calendars_list_holidays(
    client: *mut nw_client,
    country: *const c_char,
    year: i64,
    params: *const nw_calendars_list_holidays_params,
    out: *mut *mut nw_calendar_holidays,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.calendars().list_holidays(arg(country)?, year);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendars_list_holidays_params,
                    scope,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.scope(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_calendars_list_holidays_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_calendar_holidays,
    )
}
/// Optional inputs for `nw_commodities_palmoil_physical_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_commodities_palmoil_physical_observations_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub price_month: *const c_char,
    pub start: *const c_char,
}
/// Get Palmoil Observations
///
/// Stability: stable. `GET /v1/commodities/palmoil/physical/{series_id}/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_palmoil_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_palmoil_physical_observations(
    client: *mut nw_client,
    series_id: *const c_char,
    params: *const nw_commodities_palmoil_physical_observations_params,
    out: *mut *mut nw_palmoil_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .commodities()
                .palmoil_physical_observations(arg(series_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_palmoil_physical_observations_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_palmoil_physical_observations_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_commodities_palmoil_physical_observations_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_commodities_palmoil_physical_observations_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_palmoil_physical_observations_params,
                    price_month,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.price_month(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_palmoil_physical_observations_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_palmoil_history,
    )
}
/// Optional inputs for `nw_commodities_palmoil_physical_price`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_commodities_palmoil_physical_price_params {
    pub size: usize,
    pub price_month: *const c_char,
}
/// Get Palmoil Physical
///
/// Stability: stable. `GET /v1/commodities/palmoil/physical/{series_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_palmoil_price_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_palmoil_physical_price(
    client: *mut nw_client,
    series_id: *const c_char,
    params: *const nw_commodities_palmoil_physical_price_params,
    out: *mut *mut nw_palmoil_price,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().palmoil_physical_price(arg(series_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_palmoil_physical_price_params,
                    price_month,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.price_month(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_palmoil_price,
    )
}
/// Get Physical Latest
///
/// Stability: stable. `GET /v1/commodities/physical/series/{series_id}/latest`.
/// On success `*out` is owned by the caller; free it with `nw_physical_latest_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_physical_latest(
    client: *mut nw_client,
    series_id: *const c_char,
    out: *mut *mut nw_physical_latest,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().physical_latest(arg(series_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_physical_latest,
    )
}
/// Optional inputs for `nw_commodities_physical_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_commodities_physical_observations_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
}
/// Get Physical Observations
///
/// Stability: stable. `GET /v1/commodities/physical/series/{series_id}/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_physical_observations_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_physical_observations(
    client: *mut nw_client,
    series_id: *const c_char,
    params: *const nw_commodities_physical_observations_params,
    out: *mut *mut nw_physical_observations,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().physical_observations(arg(series_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_physical_observations_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_physical_observations_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_commodities_physical_observations_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_commodities_physical_observations_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_physical_observations_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_physical_observations,
    )
}
/// Get Physical Series
///
/// Stability: stable. `GET /v1/commodities/physical/series/{series_id}`.
/// On success `*out` is owned by the caller; free it with `nw_physical_series_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_get_physical_series(
    client: *mut nw_client,
    series_id: *const c_char,
    out: *mut *mut nw_physical_series,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().get_physical_series(arg(series_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_physical_series,
    )
}
/// Optional inputs for `nw_commodities_price_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_commodities_price_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub interval: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
}
/// Get Commodity Price History
///
/// Stability: stable. `GET /v1/commodities/spot/{commodity}/{currency}/bars`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_commodity_bars_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_price_history(
    client: *mut nw_client,
    commodity: *const c_char,
    currency: *const c_char,
    end: *const c_char,
    start: *const c_char,
    params: *const nw_commodities_price_history_params,
    out: *mut *mut nw_commodity_bars,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().price_history(
                arg(commodity)?,
                arg(currency)?,
                arg(end)?,
                arg(start)?,
            );
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_price_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_price_history_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
                if field!(
                    params,
                    size,
                    nw_commodities_price_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_commodities_price_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_commodity_bars,
    )
}
/// Get Commodity Spot
///
/// Stability: stable. `GET /v1/commodities/spot/{commodity}/{currency}`.
/// On success `*out` is owned by the caller; free it with `nw_commodity_spot_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_spot_price(
    client: *mut nw_client,
    commodity: *const c_char,
    currency: *const c_char,
    out: *mut *mut nw_commodity_spot,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .commodities()
                .spot_price(arg(commodity)?, arg(currency)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_commodity_spot,
    )
}
/// Optional inputs for `nw_commodities_list_palmoil_physical_series`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_commodities_list_palmoil_physical_series_params {
    pub size: usize,
    pub area: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub cursor: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub price_type: *const c_char,
    pub producer_arrangement: *const c_char,
    pub producer_type: *const c_char,
    pub product: *const c_char,
}
/// List Palmoil Physical
///
/// Stability: stable. `GET /v1/commodities/palmoil/physical`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_palmoil_series_discovery_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_list_palmoil_physical_series(
    client: *mut nw_client,
    params: *const nw_commodities_list_palmoil_physical_series_params,
    out: *mut *mut nw_palmoil_series_discovery,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().list_palmoil_physical_series();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    area,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.area(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    currency,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.currency(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_commodities_list_palmoil_physical_series_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    price_type,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.price_type(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    producer_arrangement,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.producer_arrangement(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    producer_type,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.producer_type(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_palmoil_physical_series_params,
                    product,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.product(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_palmoil_series_discovery,
    )
}
/// Optional inputs for `nw_commodities_list_physical_series`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_commodities_list_physical_series_params {
    pub size: usize,
    pub category: *const c_char,
    pub commodity: *const c_char,
    pub country: *const c_char,
    pub currency: *const c_char,
    pub cursor: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub price_type: *const c_char,
}
/// List Physical Series
///
/// Stability: stable. `GET /v1/commodities/physical/series`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_physical_series_page_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_commodities_list_physical_series(
    client: *mut nw_client,
    params: *const nw_commodities_list_physical_series_params,
    out: *mut *mut nw_physical_series_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.commodities().list_physical_series();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    category,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.category(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    commodity,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.commodity(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    currency,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.currency(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_commodities_list_physical_series_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_commodities_list_physical_series_params,
                    price_type,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.price_type(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_physical_series_page,
    )
}
/// Optional inputs for `nw_company_calendars_dividends`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_company_calendars_dividends_params {
    pub size: usize,
    pub country: *const c_char,
    pub end: *const c_char,
    pub start: *const c_char,
}
/// Company dividends calendar
///
/// Stability: stable. `GET /v1/calendars/dividends`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_dividends_calendar_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_company_calendars_dividends(
    client: *mut nw_client,
    params: *const nw_company_calendars_dividends_params,
    out: *mut *mut nw_dividends_calendar,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.company_calendars().dividends();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_company_calendars_dividends_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_company_calendars_dividends_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_company_calendars_dividends_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_dividends_calendar,
    )
}
/// Optional inputs for `nw_company_calendars_splits`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_company_calendars_splits_params {
    pub size: usize,
    pub country: *const c_char,
    pub end: *const c_char,
    pub start: *const c_char,
}
/// Company splits calendar
///
/// Stability: stable. `GET /v1/calendars/splits`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_splits_calendar_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_company_calendars_splits(
    client: *mut nw_client,
    params: *const nw_company_calendars_splits_params,
    out: *mut *mut nw_splits_calendar,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.company_calendars().splits();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_company_calendars_splits_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_company_calendars_splits_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_company_calendars_splits_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_splits_calendar,
    )
}
/// Optional inputs for `nw_crypto_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_crypto_bars_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub interval: *const c_char,
    pub start: *const c_char,
}
/// Price bars of one crypto pair
///
/// Stability: stable. `GET /v1/crypto/{pair}/bars`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_crypto_bars_model_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_bars(
    client: *mut nw_client,
    pair: *const c_char,
    params: *const nw_crypto_bars_params,
    out: *mut *mut nw_crypto_bars_model,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.crypto().bars(arg(pair)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_crypto_bars_params, cursor, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.cursor(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_crypto_bars_params, end, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.end(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_crypto_bars_params, interval, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.interval(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_crypto_bars_params, start, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_crypto_bars,
    )
}
/// Latest price of one crypto pair
///
/// Stability: stable. `GET /v1/crypto/{pair}/price`.
/// On success `*out` is owned by the caller; free it with `nw_crypto_price_model_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_price(
    client: *mut nw_client,
    pair: *const c_char,
    out: *mut *mut nw_crypto_price_model,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.crypto().price(arg(pair)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_crypto_price,
    )
}
/// Latest prices for up to 50 crypto pairs
///
/// Stability: stable. `GET /v1/crypto/prices`.
/// On success `*out` is owned by the caller; free it with `nw_crypto_prices_model_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_prices(
    client: *mut nw_client,
    pairs: *const c_char,
    out: *mut *mut nw_crypto_prices_model,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.crypto().prices(arg(pairs)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_crypto_prices,
    )
}
/// Optional inputs for `nw_crypto_list_pairs`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_crypto_list_pairs_params {
    pub size: usize,
    pub quote: *const c_char,
}
/// Crypto pairs we price
///
/// Stability: stable. `GET /v1/crypto/pairs`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_crypto_pairs_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_crypto_list_pairs(
    client: *mut nw_client,
    params: *const nw_crypto_list_pairs_params,
    out: *mut *mut nw_crypto_pairs,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.crypto().list_pairs();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_crypto_list_pairs_params,
                    quote,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.quote(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_crypto_pairs,
    )
}
/// Optional inputs for `nw_economics_central_bank_balance_sheet`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_central_bank_balance_sheet_params {
    pub size: usize,
    pub period: *const c_char,
}
/// Get Central Bank Balance Sheet
///
/// Stability: stable. `GET /v1/economics/{country}/central-bank/balance-sheet`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_central_bank_balance_sheet_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_central_bank_balance_sheet(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_central_bank_balance_sheet_params,
    out: *mut *mut nw_central_bank_balance_sheet,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().central_bank_balance_sheet(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_central_bank_balance_sheet_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_central_bank_balance_sheet,
    )
}
/// Optional inputs for `nw_economics_central_bank_reserves`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_central_bank_reserves_params {
    pub size: usize,
    pub period: *const c_char,
}
/// Get Central Bank Reserves
///
/// Stability: stable. `GET /v1/economics/{country}/central-bank/reserves`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_central_bank_reserves_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_central_bank_reserves(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_central_bank_reserves_params,
    out: *mut *mut nw_central_bank_reserves,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().central_bank_reserves(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_central_bank_reserves_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_central_bank_reserves,
    )
}
/// Optional inputs for `nw_economics_cpi`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_cpi_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Cpi
///
/// Stability: stable. `GET /v1/economics/{country}/cpi`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_cpi_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_cpi(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_cpi_params,
    out: *mut *mut nw_cpi,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().cpi(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_economics_cpi_params, period, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_cpi_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_cpi,
    )
}
/// Optional inputs for `nw_economics_cpi_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_cpi_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Cpi History
///
/// Stability: stable. `GET /v1/economics/{country}/cpi/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_cpi_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_cpi_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_cpi_history_params,
    out: *mut *mut nw_cpi_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().cpi_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_cpi_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_cpi_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_cpi_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_cpi_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_economics_cpi_history_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_cpi_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_cpi_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_cpi_history,
    )
}
/// Optional inputs for `nw_economics_get_equity_risk_premium`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_get_equity_risk_premium_params {
    pub size: usize,
    pub date: *const c_char,
}
/// Get Equity Risk Premium
///
/// Stability: stable. `GET /v1/economics/{country}/equity-risk-premium`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_risk_premium_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_get_equity_risk_premium(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_get_equity_risk_premium_params,
    out: *mut *mut nw_equity_risk_premium,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().get_equity_risk_premium(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_get_equity_risk_premium_params,
                    date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.date(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_risk_premium,
    )
}
/// Optional inputs for `nw_economics_equity_risk_premium_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_equity_risk_premium_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
}
/// Get Equity Risk Premium History
///
/// Stability: stable. `GET /v1/economics/{country}/equity-risk-premium/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_risk_premium_history_response_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_equity_risk_premium_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_equity_risk_premium_history_params,
    out: *mut *mut nw_equity_risk_premium_history_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .economics()
                .equity_risk_premium_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_equity_risk_premium_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_equity_risk_premium_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_equity_risk_premium_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_equity_risk_premium_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_equity_risk_premium_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_risk_premium_history_response,
    )
}
/// Optional inputs for `nw_economics_gdp`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_gdp_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Gdp
///
/// Stability: stable. `GET /v1/economics/{country}/gdp`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_gdp_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_gdp(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_gdp_params,
    out: *mut *mut nw_gdp,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().gdp(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_economics_gdp_params, period, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_gdp,
    )
}
/// Optional inputs for `nw_economics_gdp_growth`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_gdp_growth_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Gdp Growth
///
/// Stability: stable. `GET /v1/economics/{country}/gdp-growth`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_gdp_growth_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_gdp_growth(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_gdp_growth_params,
    out: *mut *mut nw_gdp_growth,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().gdp_growth(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_gdp_growth,
    )
}
/// Optional inputs for `nw_economics_gdp_growth_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_gdp_growth_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Gdp Growth History
///
/// Stability: stable. `GET /v1/economics/{country}/gdp-growth/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_gdp_growth_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_gdp_growth_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_gdp_growth_history_params,
    out: *mut *mut nw_gdp_growth_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().gdp_growth_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_gdp_growth_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_gdp_growth_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_growth_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_gdp_growth_history,
    )
}
/// Optional inputs for `nw_economics_gdp_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_gdp_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Gdp History
///
/// Stability: stable. `GET /v1/economics/{country}/gdp/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_gdp_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_gdp_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_gdp_history_params,
    out: *mut *mut nw_gdp_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().gdp_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_gdp_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_economics_gdp_history_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_gdp_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_gdp_history,
    )
}
/// Optional inputs for `nw_economics_inflation`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_inflation_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Inflation
///
/// Stability: stable. `GET /v1/economics/{country}/inflation`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_inflation_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_inflation(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_inflation_params,
    out: *mut *mut nw_inflation,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().inflation(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_inflation,
    )
}
/// Optional inputs for `nw_economics_inflation_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_inflation_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Inflation History
///
/// Stability: stable. `GET /v1/economics/{country}/inflation/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_inflation_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_inflation_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_inflation_history_params,
    out: *mut *mut nw_inflation_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().inflation_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_inflation_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_inflation_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_inflation_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_inflation_history,
    )
}
/// Optional inputs for `nw_economics_labour_force_participation`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_labour_force_participation_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Labour Force Participation
///
/// Stability: stable. `GET /v1/economics/{country}/labour-force-participation`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_labour_force_participation_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_labour_force_participation(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_labour_force_participation_params,
    out: *mut *mut nw_labour_force_participation,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().labour_force_participation(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_labour_force_participation,
    )
}
/// Optional inputs for `nw_economics_labour_force_participation_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_labour_force_participation_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Labour Force Participation History
///
/// Stability: stable. `GET /v1/economics/{country}/labour-force-participation/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_rate_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_labour_force_participation_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_labour_force_participation_history_params,
    out: *mut *mut nw_rate_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .economics()
                .labour_force_participation_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_labour_force_participation_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_labour_force_participation_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_rate_history,
    )
}
/// Optional inputs for `nw_economics_policy_rate`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_policy_rate_params {
    pub size: usize,
    pub date: *const c_char,
}
/// Get Policy Rate
///
/// Stability: stable. `GET /v1/economics/{country}/central-bank/policy-rate`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_policy_rate_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_policy_rate(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_policy_rate_params,
    out: *mut *mut nw_policy_rate,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().policy_rate(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_policy_rate_params,
                    date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.date(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_policy_rate,
    )
}
/// Optional inputs for `nw_economics_policy_rate_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_policy_rate_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
}
/// Get Policy Rate History
///
/// Stability: stable. `GET /v1/economics/{country}/central-bank/policy-rate/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_policy_rate_history_response_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_policy_rate_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_policy_rate_history_params,
    out: *mut *mut nw_policy_rate_history_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().policy_rate_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_policy_rate_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_policy_rate_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_policy_rate_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_policy_rate_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_policy_rate_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_policy_rate_history_response,
    )
}
/// Optional inputs for `nw_economics_get_release`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_get_release_params {
    pub size: usize,
    pub version_id: *const c_char,
}
/// Get Economic Release
///
/// Stability: stable. `GET /v1/economics/calendar/{release_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_economic_release_detail_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_get_release(
    client: *mut nw_client,
    release_id: *const c_char,
    params: *const nw_economics_get_release_params,
    out: *mut *mut nw_economic_release_detail,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().get_release(arg(release_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_get_release_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_economic_release_detail,
    )
}
/// Optional inputs for `nw_economics_trade`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_trade_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Trade
///
/// Stability: stable. `GET /v1/economics/{country}/trade`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_trade_summary_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_trade(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_trade_params,
    out: *mut *mut nw_trade_summary,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().trade(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_trade_summary,
    )
}
/// Optional inputs for `nw_economics_trade_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_trade_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Trade History
///
/// Stability: stable. `GET /v1/economics/{country}/trade/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_trade_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_trade_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_trade_history_params,
    out: *mut *mut nw_trade_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().trade_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_trade_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_economics_trade_history_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_trade_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_trade_history,
    )
}
/// Optional inputs for `nw_economics_unemployment`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_unemployment_params {
    pub size: usize,
    pub period: *const c_char,
    pub version_id: *const c_char,
}
/// Get Unemployment
///
/// Stability: stable. `GET /v1/economics/{country}/unemployment`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_unemployment_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_unemployment(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_unemployment_params,
    out: *mut *mut nw_unemployment,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().unemployment(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_unemployment,
    )
}
/// Optional inputs for `nw_economics_unemployment_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_unemployment_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Get Unemployment History
///
/// Stability: stable. `GET /v1/economics/{country}/unemployment/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_rate_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_unemployment_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_unemployment_history_params,
    out: *mut *mut nw_rate_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().unemployment_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_unemployment_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_unemployment_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_unemployment_history_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_rate_history,
    )
}
/// Optional inputs for `nw_economics_yield_curve`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_economics_yield_curve_params {
    pub size: usize,
    pub date: *const c_char,
    pub maturity: *const *const c_char,
    pub maturity_len: usize,
    pub version_id: *const c_char,
}
/// Get Yield Curve
///
/// Stability: stable. `GET /v1/economics/{country}/yield-curve`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_yield_curve_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_yield_curve(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_yield_curve_params,
    out: *mut *mut nw_yield_curve,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().yield_curve(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_yield_curve_params,
                    date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.date(v);
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_economics_yield_curve_params,
                        maturity,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_economics_yield_curve_params,
                        maturity_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.maturity(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_yield_curve_params,
                    version_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version_id(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_yield_curve,
    )
}
/// Optional inputs for `nw_economics_yield_curve_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_yield_curve_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub maturity: *const *const c_char,
    pub maturity_len: usize,
    pub start: *const c_char,
    pub as_of: *const c_char,
}
/// Get Yield Curve History
///
/// Stability: stable. `GET /v1/economics/{country}/yield-curve/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_yield_curve_history_response_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_yield_curve_history(
    client: *mut nw_client,
    country: *const c_char,
    params: *const nw_economics_yield_curve_history_params,
    out: *mut *mut nw_yield_curve_history_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().yield_curve_history(arg(country)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_yield_curve_history_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_yield_curve_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_yield_curve_history_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_yield_curve_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_yield_curve_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_economics_yield_curve_history_params,
                        maturity,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_economics_yield_curve_history_params,
                        maturity_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.maturity(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_yield_curve_history_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_yield_curve_history_response,
    )
}
/// List Equity Risk Premium
///
/// Stability: stable. `GET /v1/economics/equity-risk-premium`.
/// On success `*out` is owned by the caller; free it with `nw_equity_risk_premium_list_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_list_equity_risk_premium(
    client: *mut nw_client,
    out: *mut *mut nw_equity_risk_premium_list,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().list_equity_risk_premium();
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_risk_premium_list,
    )
}
/// Optional inputs for `nw_economics_list_releases`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_list_releases_params {
    pub size: usize,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub cursor: *const c_char,
    pub end_date: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start_date: *const c_char,
}
/// List Economic Releases
///
/// Stability: stable. `GET /v1/economics/calendar`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_economic_calendar_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_list_releases(
    client: *mut nw_client,
    params: *const nw_economics_list_releases_params,
    out: *mut *mut nw_economic_calendar,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().list_releases();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_economics_list_releases_params,
                        countries,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_economics_list_releases_params,
                        countries_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.countries(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_list_releases_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_list_releases_params,
                    end_date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end_date(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_list_releases_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_economics_list_releases_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_list_releases_params,
                    start_date,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start_date(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_economic_calendar,
    )
}
/// Read Catalogue Latest Batch
///
/// Stability: stable. `GET /v1/economics/catalogue/latest`.
/// On success `*out` is owned by the caller; free it with `nw_catalogue_bulk_latest_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_read_catalogue_latest_batch(
    client: *mut nw_client,
    ids: *const c_char,
    out: *mut *mut nw_catalogue_bulk_latest,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().read_catalogue_latest_batch(arg(ids)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_catalogue_bulk_latest,
    )
}
/// Optional inputs for `nw_economics_read_catalogue_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_read_catalogue_observations_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
}
/// Read Catalogue History
///
/// Stability: stable. `GET /v1/economics/catalogue/series/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_catalogue_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_read_catalogue_observations(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_economics_read_catalogue_observations_params,
    out: *mut *mut nw_catalogue_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().read_catalogue_observations(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_catalogue_observations_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_catalogue_observations_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_read_catalogue_observations_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_read_catalogue_observations_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_catalogue_observations_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_catalogue_history,
    )
}
/// Read Catalogue Series
///
/// Stability: stable. `GET /v1/economics/catalogue/series`.
/// On success `*out` is owned by the caller; free it with `nw_catalogue_series_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_read_catalogue_series(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_catalogue_series,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().read_catalogue_series(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_catalogue_series,
    )
}
/// Optional inputs for `nw_economics_read_series_observations`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_read_series_observations_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub start: *const c_char,
    pub as_of: *const c_char,
    pub version: *const c_char,
}
/// Read Resource History
///
/// Stability: stable. `GET /v1/economics/series/{series_id}/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_resource_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_read_series_observations(
    client: *mut nw_client,
    series_id: *const c_char,
    params: *const nw_economics_read_series_observations_params,
    out: *mut *mut nw_resource_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().read_series_observations(arg(series_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_series_observations_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_series_observations_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_series_observations_params,
                    end,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.end(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_read_series_observations_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_read_series_observations_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_series_observations_params,
                    start,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.start(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_read_series_observations_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_resource_history,
    )
}
/// Optional inputs for `nw_economics_search_catalogue`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_economics_search_catalogue_params {
    pub size: usize,
    pub country: *const c_char,
    pub cursor: *const c_char,
    pub frequency: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub q: *const c_char,
    pub source: *const c_char,
}
/// Search Series Catalogue
///
/// Stability: stable. `GET /v1/economics/catalogue`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_catalogue_search_page_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_economics_search_catalogue(
    client: *mut nw_client,
    params: *const nw_economics_search_catalogue_params,
    out: *mut *mut nw_catalogue_search_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.economics().search_catalogue();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_search_catalogue_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_search_catalogue_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_search_catalogue_params,
                    frequency,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.frequency(v);
                }
                if field!(
                    params,
                    size,
                    nw_economics_search_catalogue_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_economics_search_catalogue_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_search_catalogue_params,
                    q,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.q(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_economics_search_catalogue_params,
                    source,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.source(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_catalogue_search_page,
    )
}
/// Optional inputs for `nw_equities_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_equities_bars_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub interval: *const c_char,
    pub start: *const c_char,
}
/// Price bars of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/bars`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_bars_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_bars(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_equities_bars_params,
    out: *mut *mut nw_equity_bars,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().bars(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_equities_bars_params, cursor, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.cursor(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_equities_bars_params, end, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.end(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_bars_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_equities_bars_params, start, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_bars,
    )
}
/// Optional inputs for `nw_equities_cost_of_capital`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_equities_cost_of_capital_params {
    pub size: usize,
    pub benchmark: *const c_char,
    pub beta: *const c_char,
    pub erp: *const c_char,
    pub frequency: *const c_char,
    pub period: *const c_char,
    pub risk_free: *const c_char,
    pub tax_rate: *const c_char,
    pub window: *const c_char,
}
/// Beta, cost of equity, cost of debt and WACC of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/cost-of-capital`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_cost_of_capital_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_cost_of_capital(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_equities_cost_of_capital_params,
    out: *mut *mut nw_equity_cost_of_capital,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().cost_of_capital(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    benchmark,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.benchmark(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    beta,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.beta(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    erp,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.erp(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    frequency,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.frequency(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    risk_free,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.risk_free(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    tax_rate,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.tax_rate(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_cost_of_capital_params,
                    window,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.window(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_cost_of_capital,
    )
}
/// Optional inputs for `nw_equities_dividends`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_equities_dividends_params {
    pub size: usize,
    pub cursor: *const c_char,
}
/// Dividends of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/dividends`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_dividends_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_dividends(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_equities_dividends_params,
    out: *mut *mut nw_equity_dividends,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().dividends(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_dividends_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_dividends,
    )
}
/// Optional inputs for `nw_equities_earnings`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_equities_earnings_params {
    pub size: usize,
    pub cursor: *const c_char,
}
/// Earnings of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/earnings`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_earnings_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_earnings(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_equities_earnings_params,
    out: *mut *mut nw_equity_earnings,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().earnings(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_earnings_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_earnings,
    )
}
/// Optional inputs for `nw_equities_movers`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_equities_movers_params {
    pub size: usize,
    pub direction: *const c_char,
    pub market: *const c_char,
}
/// Top US stock movers
///
/// Stability: stable. `GET /v1/equities/movers`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_movers_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_movers(
    client: *mut nw_client,
    params: *const nw_equities_movers_params,
    out: *mut *mut nw_equity_movers,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().movers();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_movers_params,
                    direction,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.direction(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_movers_params,
                    market,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.market(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_movers,
    )
}
/// Latest price of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/price`.
/// On success `*out` is owned by the caller; free it with `nw_equity_price_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_price(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_equity_price,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().price(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_price,
    )
}
/// Latest prices for up to 50 stocks
///
/// Stability: stable. `GET /v1/equities/prices`.
/// On success `*out` is owned by the caller; free it with `nw_equity_prices_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_prices(
    client: *mut nw_client,
    ids: *const c_char,
    out: *mut *mut nw_equity_prices,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().prices(arg(ids)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_prices,
    )
}
/// Profile of one stock
///
/// Stability: stable. `GET /v1/equities/{id}`.
/// On success `*out` is owned by the caller; free it with `nw_equity_profile_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_profile(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_equity_profile,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().profile(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_profile,
    )
}
/// Splits of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/splits`.
/// On success `*out` is owned by the caller; free it with `nw_equity_splits_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_splits(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_equity_splits,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().splits(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_splits,
    )
}
/// Market cap, P/E, P/S, P/B and dividend yield of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/valuation`.
/// On success `*out` is owned by the caller; free it with `nw_equity_valuation_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_valuation(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_equity_valuation,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().valuation(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_valuation,
    )
}
/// Valuation history of one stock
///
/// Stability: stable. `GET /v1/equities/{id}/valuation/history`.
/// On success `*out` is owned by the caller; free it with `nw_equity_valuation_history_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_valuation_history(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_equity_valuation_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().valuation_history(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_valuation_history,
    )
}
/// Optional inputs for `nw_equities_search`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_equities_search_params {
    pub size: usize,
    pub country: *const c_char,
    pub cursor: *const c_char,
    pub type_: *const c_char,
}
/// Search stocks by ticker or name
///
/// Stability: stable. `GET /v1/equities/search`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_search_results_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_equities_search(
    client: *mut nw_client,
    q: *const c_char,
    params: *const nw_equities_search_params,
    out: *mut *mut nw_equity_search_results,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.equities().search(arg(q)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_search_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_search_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_equities_search_params,
                    type_,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.type_(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_search_results,
    )
}
/// Get Delivery Event
///
/// Stability: stable. `GET /v1/events/{event_id}`.
/// On success `*out` is owned by the caller; free it with `nw_event_detail_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_events_get(
    client: *mut nw_client,
    event_id: *const c_char,
    out: *mut *mut nw_event_detail,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.events().get(arg(event_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_event_detail,
    )
}
/// Optional inputs for `nw_events_list`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_events_list_params {
    pub size: usize,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub cursor: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
}
/// List Delivery Events
///
/// Stability: stable. `GET /v1/events`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_events_page_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_events_list(
    client: *mut nw_client,
    params: *const nw_events_list_params,
    out: *mut *mut nw_events_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.events().list();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_events_list_params,
                        countries,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(params, size, nw_events_list_params, countries_len, usize).unwrap_or(0),
                )? {
                    request = request.countries(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_events_list_params, cursor, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.cursor(v);
                }
                if field!(params, size, nw_events_list_params, has_limit, bool) == Some(true) {
                    if let Some(v) = field!(params, size, nw_events_list_params, limit, i64) {
                        request = request.limit(v);
                    }
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_events_page,
    )
}
/// Get As Reported
///
/// Stability: stable. `GET /v1/filings/{filing_id}/financials/as-reported`.
/// On success `*out` is owned by the caller; free it with `nw_as_reported_page_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_as_reported(
    client: *mut nw_client,
    filing_id: *const c_char,
    out: *mut *mut nw_as_reported_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().as_reported(arg(filing_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_as_reported_page,
    )
}
/// Optional inputs for `nw_filings_company`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_filings_company_params {
    pub size: usize,
    pub as_of: *const c_char,
    pub lang: *const c_char,
}
/// Get Company
///
/// Stability: stable. `GET /v1/companies/{company_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_company_with_listings_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_company(
    client: *mut nw_client,
    company_id: *const c_char,
    params: *const nw_filings_company_params,
    out: *mut *mut nw_company_with_listings,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().company(arg(company_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_company_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_filings_company_params, lang, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.lang(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_company_with_listings,
    )
}
/// Optional inputs for `nw_filings_filing`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_filings_filing_params {
    pub size: usize,
    pub lang: *const c_char,
}
/// Get Filing
///
/// Stability: stable. `GET /v1/filings/{filing_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_filing_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_filing(
    client: *mut nw_client,
    filing_id: *const c_char,
    params: *const nw_filings_filing_params,
    out: *mut *mut nw_filing,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().filing(arg(filing_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_filings_filing_params, lang, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.lang(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_filing,
    )
}
/// Optional inputs for `nw_filings_financials`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_filings_financials_params {
    pub size: usize,
    pub as_of: *const c_char,
    pub basis: *const c_char,
    pub fields: *const c_char,
    pub has_fiscal_year: bool,
    pub fiscal_year: i64,
    pub period: *const c_char,
    pub statement: *const c_char,
    pub version: *const c_char,
}
/// Get Financials
///
/// Stability: stable. `GET /v1/companies/{company_id}/financials`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_financials_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_financials(
    client: *mut nw_client,
    company_id: *const c_char,
    params: *const nw_filings_financials_params,
    out: *mut *mut nw_financials,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().financials(arg(company_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    basis,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.basis(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    fields,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.fields(v);
                }
                if field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    has_fiscal_year,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_filings_financials_params, fiscal_year, i64)
                    {
                        request = request.fiscal_year(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    period,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.period(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    statement,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.statement(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_financials_params,
                    version,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.version(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_financials,
    )
}
/// Optional inputs for `nw_filings_list_company_filings`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_filings_list_company_filings_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub filed_from: *const c_char,
    pub filed_to: *const c_char,
    pub form: *const c_char,
    pub has_include_amendments: bool,
    pub include_amendments: bool,
    pub lang: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub type_: *const c_char,
}
/// List Company Filings
///
/// Stability: stable. `GET /v1/companies/{company_id}/filings`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_company_filings_page_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_list_company_filings(
    client: *mut nw_client,
    company_id: *const c_char,
    params: *const nw_filings_list_company_filings_params,
    out: *mut *mut nw_company_filings_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().list_company_filings(arg(company_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    filed_from,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.filed_from(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    filed_to,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.filed_to(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    form,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.form(v);
                }
                if field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    has_include_amendments,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_filings_list_company_filings_params,
                        include_amendments,
                        bool
                    ) {
                        request = request.include_amendments(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    lang,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.lang(v);
                }
                if field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_filings_list_company_filings_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_company_filings_params,
                    type_,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.type_(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_company_filings_page,
    )
}
/// Optional inputs for `nw_filings_list_holders`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_filings_list_holders_params {
    pub size: usize,
    pub lang: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub security: *const c_char,
}
/// List Holders
///
/// Stability: stable. `GET /v1/companies/{company_id}/holders`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_holders_page_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_list_holders(
    client: *mut nw_client,
    company_id: *const c_char,
    params: *const nw_filings_list_holders_params,
    out: *mut *mut nw_holders_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().list_holders(arg(company_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_holders_params,
                    lang,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.lang(v);
                }
                if field!(
                    params,
                    size,
                    nw_filings_list_holders_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_filings_list_holders_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_holders_params,
                    security,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.security(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_holders_page,
    )
}
/// Optional inputs for `nw_filings_list_latest_filings`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_filings_list_latest_filings_params {
    pub size: usize,
    pub country: *const c_char,
    pub lang: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub type_: *const c_char,
}
/// List Latest Filings
///
/// Stability: stable. `GET /v1/filings/latest`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_latest_filings_page_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_list_latest_filings(
    client: *mut nw_client,
    params: *const nw_filings_list_latest_filings_params,
    out: *mut *mut nw_latest_filings_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().list_latest_filings();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_latest_filings_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_latest_filings_params,
                    lang,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.lang(v);
                }
                if field!(
                    params,
                    size,
                    nw_filings_list_latest_filings_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_filings_list_latest_filings_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_list_latest_filings_params,
                    type_,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.type_(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_latest_filings_page,
    )
}
/// Optional inputs for `nw_filings_search_companies`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_filings_search_companies_params {
    pub size: usize,
    pub as_of: *const c_char,
    pub country: *const c_char,
    pub lang: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub q: *const c_char,
    pub registry: *const c_char,
}
/// Search Companies
///
/// Stability: stable. `GET /v1/companies`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_company_search_page_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_filings_search_companies(
    client: *mut nw_client,
    params: *const nw_filings_search_companies_params,
    out: *mut *mut nw_company_search_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.filings().search_companies();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_search_companies_params,
                    as_of,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.as_of(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_search_companies_params,
                    country,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.country(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_search_companies_params,
                    lang,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.lang(v);
                }
                if field!(
                    params,
                    size,
                    nw_filings_search_companies_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_filings_search_companies_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_search_companies_params,
                    q,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.q(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_filings_search_companies_params,
                    registry,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.registry(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_company_search_page,
    )
}
/// Optional inputs for `nw_funds_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_funds_bars_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub end: *const c_char,
    pub interval: *const c_char,
    pub start: *const c_char,
}
/// Price bars of one fund
///
/// Stability: stable. `GET /v1/funds/{id}/bars`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_equity_bars_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_funds_bars(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_funds_bars_params,
    out: *mut *mut nw_equity_bars,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.funds().bars(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_bars_params, cursor, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.cursor(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_bars_params, end, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.end(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_bars_params, interval, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.interval(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_bars_params, start, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.start(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_equity_bars,
    )
}
/// Optional inputs for `nw_funds_distributions`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_funds_distributions_params {
    pub size: usize,
    pub cursor: *const c_char,
}
/// Distributions of one fund
///
/// Stability: stable. `GET /v1/funds/{id}/distributions`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_fund_distributions_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_funds_distributions(
    client: *mut nw_client,
    id: *const c_char,
    params: *const nw_funds_distributions_params,
    out: *mut *mut nw_fund_distributions,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.funds().distributions(arg(id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_funds_distributions_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_fund_distributions,
    )
}
/// Latest price of one fund
///
/// Stability: stable. `GET /v1/funds/{id}/price`.
/// On success `*out` is owned by the caller; free it with `nw_fund_price_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_funds_price(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_fund_price,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.funds().price(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_fund_price,
    )
}
/// Latest prices for up to 50 funds
///
/// Stability: stable. `GET /v1/funds/prices`.
/// On success `*out` is owned by the caller; free it with `nw_fund_prices_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_funds_prices(
    client: *mut nw_client,
    ids: *const c_char,
    out: *mut *mut nw_fund_prices,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.funds().prices(arg(ids)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_fund_prices,
    )
}
/// Profile of one fund
///
/// Stability: stable. `GET /v1/funds/{id}`.
/// On success `*out` is owned by the caller; free it with `nw_fund_profile_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_funds_profile(
    client: *mut nw_client,
    id: *const c_char,
    out: *mut *mut nw_fund_profile,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.funds().profile(arg(id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_fund_profile,
    )
}
/// Optional inputs for `nw_funds_search`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_funds_search_params {
    pub size: usize,
    pub country: *const c_char,
    pub cursor: *const c_char,
    pub type_: *const c_char,
}
/// Search ETFs and mutual funds by ticker or name
///
/// Stability: stable. `GET /v1/funds/search`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_fund_search_results_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_funds_search(
    client: *mut nw_client,
    q: *const c_char,
    params: *const nw_funds_search_params,
    out: *mut *mut nw_fund_search_results,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.funds().search(arg(q)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_search_params, country, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.country(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_search_params, cursor, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.cursor(v);
                }
                if let Some(v) =
                    opt(
                        field!(params, size, nw_funds_search_params, type_, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.type_(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_fund_search_results,
    )
}
/// Convert Fx
///
/// Stability: stable. `GET /v1/fx/convert`.
/// On success `*out` is owned by the caller; free it with `nw_fx_conversion_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_convert_currency(
    client: *mut nw_client,
    amount: *const c_char,
    from: *const c_char,
    to: *const c_char,
    out: *mut *mut nw_fx_conversion,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .fx()
                .convert_currency(arg(amount)?, arg(from)?, arg(to)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_fx_conversion,
    )
}
/// Optional inputs for `nw_fx_rate_bars`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_fx_rate_bars_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub interval: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
}
/// Get Fx Rate Bars
///
/// Stability: stable. `GET /v1/fx/rates/{base}/bars`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_fx_bars_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_rate_bars(
    client: *mut nw_client,
    base: *const c_char,
    currency: *const c_char,
    end: *const c_char,
    start: *const c_char,
    params: *const nw_fx_rate_bars_params,
    out: *mut *mut nw_fx_bars,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request =
                client
                    .fx()
                    .rate_bars(arg(base)?, arg(currency)?, arg(end)?, arg(start)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) =
                    opt(
                        field!(params, size, nw_fx_rate_bars_params, cursor, *const c_char)
                            .unwrap_or(std::ptr::null()),
                    )?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_fx_rate_bars_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
                if field!(params, size, nw_fx_rate_bars_params, has_limit, bool) == Some(true) {
                    if let Some(v) = field!(params, size, nw_fx_rate_bars_params, limit, i64) {
                        request = request.limit(v);
                    }
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_fx_bars,
    )
}
/// Optional inputs for `nw_fx_rate_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_fx_rate_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
}
/// Get Fx Rate History
///
/// Stability: stable. `GET /v1/fx/rates/{base}/observations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_fx_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_rate_history(
    client: *mut nw_client,
    base: *const c_char,
    currencies: *const *const c_char,
    currencies_len: usize,
    end_date: *const c_char,
    start_date: *const c_char,
    params: *const nw_fx_rate_history_params,
    out: *mut *mut nw_fx_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.fx().rate_history(
                arg(base)?,
                list(currencies, currencies_len)?,
                arg(end_date)?,
                arg(start_date)?,
            );
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_fx_rate_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if field!(params, size, nw_fx_rate_history_params, has_limit, bool) == Some(true) {
                    if let Some(v) = field!(params, size, nw_fx_rate_history_params, limit, i64) {
                        request = request.limit(v);
                    }
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_fx_history,
    )
}
/// Optional inputs for `nw_fx_rates`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_fx_rates_params {
    pub size: usize,
    pub currencies: *const *const c_char,
    pub currencies_len: usize,
}
/// Get Fx Rates
///
/// Stability: stable. `GET /v1/fx/rates/{base}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_fx_rates_model_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_fx_rates(
    client: *mut nw_client,
    base: *const c_char,
    params: *const nw_fx_rates_params,
    out: *mut *mut nw_fx_rates_model,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.fx().rates(arg(base)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_fx_rates_params,
                        currencies,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(params, size, nw_fx_rates_params, currencies_len, usize).unwrap_or(0),
                )? {
                    request = request.currencies(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_fx_rates,
    )
}
/// Optional inputs for `nw_predictions_binary`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_predictions_binary_params {
    pub size: usize,
    pub at: *const c_char,
}
/// Get Binary Prediction
///
/// Stability: stable. `GET /v1/predictions/binary/{event_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_binary_event_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_binary(
    client: *mut nw_client,
    event_id: *const c_char,
    params: *const nw_predictions_binary_params,
    out: *mut *mut nw_binary_event,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().binary(arg(event_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_binary_params,
                    at,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.at(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_binary_event,
    )
}
/// Optional inputs for `nw_predictions_binary_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_predictions_binary_history_params {
    pub size: usize,
    pub interval: *const c_char,
    pub cursor: *const c_char,
}
/// Get Binary Prediction History
///
/// Stability: stable. `GET /v1/predictions/binary/{event_id}/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_binary_history(
    client: *mut nw_client,
    event_id: *const c_char,
    end: *const c_char,
    start: *const c_char,
    params: *const nw_predictions_binary_history_params,
    out: *mut *mut nw_prediction_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request =
                client
                    .predictions()
                    .binary_history(arg(event_id)?, arg(end)?, arg(start)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_binary_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_binary_history_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_history,
    )
}
/// Optional inputs for `nw_predictions_extreme`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_predictions_extreme_params {
    pub size: usize,
    pub at: *const c_char,
}
/// Get Extreme Prediction
///
/// Stability: stable. `GET /v1/predictions/extreme/{event_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_extreme_event_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_extreme(
    client: *mut nw_client,
    event_id: *const c_char,
    params: *const nw_predictions_extreme_params,
    out: *mut *mut nw_extreme_event,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().extreme(arg(event_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_extreme_params,
                    at,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.at(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_extreme_event,
    )
}
/// Optional inputs for `nw_predictions_extreme_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_predictions_extreme_history_params {
    pub size: usize,
    pub interval: *const c_char,
    pub cursor: *const c_char,
}
/// Get Extreme Prediction History
///
/// Stability: stable. `GET /v1/predictions/extreme/{event_id}/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_extreme_history(
    client: *mut nw_client,
    event_id: *const c_char,
    end: *const c_char,
    start: *const c_char,
    params: *const nw_predictions_extreme_history_params,
    out: *mut *mut nw_prediction_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request =
                client
                    .predictions()
                    .extreme_history(arg(event_id)?, arg(end)?, arg(start)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_extreme_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_extreme_history_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_history,
    )
}
/// Get Policy Path
///
/// Stability: stable. `GET /v1/predictions/policy-paths/{authority}`.
/// On success `*out` is owned by the caller; free it with `nw_policy_path_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_policy_path(
    client: *mut nw_client,
    authority: *const c_char,
    out: *mut *mut nw_policy_path,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().policy_path(arg(authority)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_policy_path,
    )
}
/// Get Policy Path History
///
/// Stability: stable. `GET /v1/predictions/policy-paths/{authority}/history`.
/// On success `*out` is owned by the caller; free it with `nw_policy_path_history_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_policy_path_history(
    client: *mut nw_client,
    authority: *const c_char,
    end: *const c_char,
    start: *const c_char,
    out: *mut *mut nw_policy_path_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request =
                client
                    .predictions()
                    .policy_path_history(arg(authority)?, arg(end)?, arg(start)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_policy_path_history,
    )
}
/// Optional inputs for `nw_predictions_scalar`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_predictions_scalar_params {
    pub size: usize,
    pub at: *const c_char,
}
/// Get Scalar Prediction
///
/// Stability: stable. `GET /v1/predictions/scalar/{event_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_scalar_event_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_scalar(
    client: *mut nw_client,
    event_id: *const c_char,
    params: *const nw_predictions_scalar_params,
    out: *mut *mut nw_scalar_event,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().scalar(arg(event_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_scalar_params,
                    at,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.at(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_scalar_event,
    )
}
/// Optional inputs for `nw_predictions_scalar_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_predictions_scalar_history_params {
    pub size: usize,
    pub interval: *const c_char,
    pub cursor: *const c_char,
}
/// Get Scalar Prediction History
///
/// Stability: stable. `GET /v1/predictions/scalar/{event_id}/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_scalar_history(
    client: *mut nw_client,
    event_id: *const c_char,
    end: *const c_char,
    start: *const c_char,
    params: *const nw_predictions_scalar_history_params,
    out: *mut *mut nw_prediction_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request =
                client
                    .predictions()
                    .scalar_history(arg(event_id)?, arg(end)?, arg(start)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_scalar_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_scalar_history_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_history,
    )
}
/// Optional inputs for `nw_predictions_timing`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_predictions_timing_params {
    pub size: usize,
    pub at: *const c_char,
}
/// Get Timing Prediction
///
/// Stability: stable. `GET /v1/predictions/timing/{event_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_timing_event_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_timing(
    client: *mut nw_client,
    event_id: *const c_char,
    params: *const nw_predictions_timing_params,
    out: *mut *mut nw_timing_event,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().timing(arg(event_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_timing_params,
                    at,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.at(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_timing_event,
    )
}
/// Optional inputs for `nw_predictions_timing_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_predictions_timing_history_params {
    pub size: usize,
    pub interval: *const c_char,
    pub cursor: *const c_char,
}
/// Get Timing Prediction History
///
/// Stability: stable. `GET /v1/predictions/timing/{event_id}/history`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_prediction_history_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_timing_history(
    client: *mut nw_client,
    event_id: *const c_char,
    end: *const c_char,
    start: *const c_char,
    params: *const nw_predictions_timing_history_params,
    out: *mut *mut nw_prediction_history,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request =
                client
                    .predictions()
                    .timing_history(arg(event_id)?, arg(end)?, arg(start)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_timing_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_timing_history_params,
                    interval,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.interval(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_history,
    )
}
/// Get Prediction Topic
///
/// Stability: stable. `GET /v1/predictions/topics/{topic_id}`.
/// On success `*out` is owned by the caller; free it with `nw_prediction_topic_detail_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_get_topic(
    client: *mut nw_client,
    topic_id: *const c_char,
    out: *mut *mut nw_prediction_topic_detail,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().get_topic(arg(topic_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_topic_detail,
    )
}
/// Optional inputs for `nw_predictions_list_events`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_predictions_list_events_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub from: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
    pub release_id: *const c_char,
    pub shape: *const c_char,
    pub status: *const c_char,
    pub to: *const c_char,
    pub topic: *const c_char,
}
/// List Prediction Events
///
/// Stability: stable. `GET /v1/predictions/events`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_prediction_events_page_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_list_events(
    client: *mut nw_client,
    params: *const nw_predictions_list_events_params,
    out: *mut *mut nw_prediction_events_page,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().list_events();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    from,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.from(v);
                }
                if field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) =
                        field!(params, size, nw_predictions_list_events_params, limit, i64)
                    {
                        request = request.limit(v);
                    }
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    release_id,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.release_id(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    shape,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.shape(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    status,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.status(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    to,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.to(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_events_params,
                    topic,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.topic(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_events_page,
    )
}
/// Optional inputs for `nw_predictions_list_topics`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_predictions_list_topics_params {
    pub size: usize,
    pub group: *const c_char,
}
/// List Prediction Topics
///
/// Stability: stable. `GET /v1/predictions/topics`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_prediction_topics_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_predictions_list_topics(
    client: *mut nw_client,
    params: *const nw_predictions_list_topics_params,
    out: *mut *mut nw_prediction_topics,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.predictions().list_topics();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_predictions_list_topics_params,
                    group,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.group(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_prediction_topics,
    )
}
/// Configuration History
///
/// Stability: stable. `GET /v1/webhooks/destinations/{destination_id}/history`.
/// On success `*out` is owned by the caller; free it with `nw_webhook_history_response_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_configuration_history(
    client: *mut nw_client,
    destination_id: *const c_char,
    out: *mut *mut nw_webhook_history_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .webhooks()
                .configuration_history(arg(destination_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_history_response,
    )
}
/// Optional inputs for `nw_webhooks_create_destination`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_webhooks_create_destination_params {
    pub size: usize,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub has_enabled: bool,
    pub enabled: bool,
    pub families: *const *const c_char,
    pub families_len: usize,
    pub url: *const c_char,
    pub companies: *const *const c_char,
    pub companies_len: usize,
}
/// Create Destination
///
/// Stability: stable. `POST /v1/webhooks/destinations`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_webhook_destination_created_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_create_destination(
    client: *mut nw_client,
    params: *const nw_webhooks_create_destination_params,
    out: *mut *mut nw_webhook_destination_created,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().create_destination();
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        companies,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        companies_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.companies(v);
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        countries,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        countries_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.countries(v);
                }
                if field!(
                    params,
                    size,
                    nw_webhooks_create_destination_params,
                    has_enabled,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        enabled,
                        bool
                    ) {
                        request = request.enabled(v);
                    }
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        families,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_webhooks_create_destination_params,
                        families_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.families(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_webhooks_create_destination_params,
                    url,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.url(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_destination_created,
    )
}
/// Delete Destination
///
/// Stability: stable. `DELETE /v1/webhooks/destinations/{destination_id}`.
/// On success `*out` is owned by the caller; free it with `nw_no_content_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_delete_destination(
    client: *mut nw_client,
    destination_id: *const c_char,
    out: *mut *mut nw_no_content,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().delete_destination(arg(destination_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_no_content,
    )
}
/// Optional inputs for `nw_webhooks_delivery_history`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
/// Pass the previous page's `next_cursor` as `cursor`; it is sent exactly as given.
#[repr(C)]
pub struct nw_webhooks_delivery_history_params {
    pub size: usize,
    pub cursor: *const c_char,
    pub has_limit: bool,
    pub limit: i64,
}
/// Delivery History
///
/// Stability: stable. `GET /v1/webhooks/destinations/{destination_id}/deliveries`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_webhook_deliveries_response_free`.
/// `next_cursor` is NULL on the last page.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_delivery_history(
    client: *mut nw_client,
    destination_id: *const c_char,
    params: *const nw_webhooks_delivery_history_params,
    out: *mut *mut nw_webhook_deliveries_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().delivery_history(arg(destination_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_webhooks_delivery_history_params,
                    cursor,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.cursor(v);
                }
                if field!(
                    params,
                    size,
                    nw_webhooks_delivery_history_params,
                    has_limit,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_webhooks_delivery_history_params,
                        limit,
                        i64
                    ) {
                        request = request.limit(v);
                    }
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_deliveries_response,
    )
}
/// Get Destination
///
/// Stability: stable. `GET /v1/webhooks/destinations/{destination_id}`.
/// On success `*out` is owned by the caller; free it with `nw_webhook_destination_response_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_get_destination(
    client: *mut nw_client,
    destination_id: *const c_char,
    out: *mut *mut nw_webhook_destination_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().get_destination(arg(destination_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_destination_response,
    )
}
/// List Destinations
///
/// Stability: stable. `GET /v1/webhooks/destinations`.
/// On success `*out` is owned by the caller; free it with `nw_webhook_destination_list_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_list_destinations(
    client: *mut nw_client,
    out: *mut *mut nw_webhook_destination_list,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().list_destinations();
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_destination_list,
    )
}
/// Replay Delivery
///
/// Stability: stable. `POST /v1/webhooks/destinations/{destination_id}/deliveries/{delivery_id}/replay`.
/// On success `*out` is owned by the caller; free it with `nw_no_content_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_replay_delivery(
    client: *mut nw_client,
    delivery_id: *const c_char,
    destination_id: *const c_char,
    out: *mut *mut nw_no_content,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client
                .webhooks()
                .replay_delivery(arg(delivery_id)?, arg(destination_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_no_content,
    )
}
/// Rotate Secret
///
/// Stability: stable. `POST /v1/webhooks/destinations/{destination_id}/rotate-secret`.
/// On success `*out` is owned by the caller; free it with `nw_webhook_destination_created_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_rotate_secret(
    client: *mut nw_client,
    destination_id: *const c_char,
    out: *mut *mut nw_webhook_destination_created,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().rotate_secret(arg(destination_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_destination_created,
    )
}
/// Test Destination
///
/// Stability: stable. `POST /v1/webhooks/destinations/{destination_id}/test`.
/// On success `*out` is owned by the caller; free it with `nw_webhook_test_response_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_test_destination(
    client: *mut nw_client,
    destination_id: *const c_char,
    out: *mut *mut nw_webhook_test_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().test_destination(arg(destination_id)?);
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_test_response,
    )
}
/// Optional inputs for `nw_webhooks_update_destination`. Zero-initialize, set `size` to `sizeof` this struct, then set fields; NULL pointers and false `has_` flags leave a parameter unset.
#[repr(C)]
pub struct nw_webhooks_update_destination_params {
    pub size: usize,
    pub countries: *const *const c_char,
    pub countries_len: usize,
    pub has_enabled: bool,
    pub enabled: bool,
    pub families: *const *const c_char,
    pub families_len: usize,
    pub url: *const c_char,
    pub companies: *const *const c_char,
    pub companies_len: usize,
}
/// Update Destination
///
/// Stability: stable. `PATCH /v1/webhooks/destinations/{destination_id}`.
/// `params` may be NULL for defaults.
/// On success `*out` is owned by the caller; free it with `nw_webhook_destination_response_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_webhooks_update_destination(
    client: *mut nw_client,
    destination_id: *const c_char,
    params: *const nw_webhooks_update_destination_params,
    out: *mut *mut nw_webhook_destination_response,
    err: *mut *mut nw_error,
) -> nw_status {
    respond(
        out,
        err,
        || {
            let client = client_ref(client)?;
            #[allow(unused_mut)]
            let mut request = client.webhooks().update_destination(arg(destination_id)?);
            if !params.is_null() {
                let size = params_size(params)?;
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        companies,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        companies_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.companies(v);
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        countries,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        countries_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.countries(v);
                }
                if field!(
                    params,
                    size,
                    nw_webhooks_update_destination_params,
                    has_enabled,
                    bool
                ) == Some(true)
                {
                    if let Some(v) = field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        enabled,
                        bool
                    ) {
                        request = request.enabled(v);
                    }
                }
                if let Some(v) = opt_list(
                    field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        families,
                        *const *const c_char
                    )
                    .unwrap_or(std::ptr::null()),
                    field!(
                        params,
                        size,
                        nw_webhooks_update_destination_params,
                        families_len,
                        usize
                    )
                    .unwrap_or(0),
                )? {
                    request = request.families(v);
                }
                if let Some(v) = opt(field!(
                    params,
                    size,
                    nw_webhooks_update_destination_params,
                    url,
                    *const c_char
                )
                .unwrap_or(std::ptr::null()))?
                {
                    request = request.url(v);
                }
            }
            Ok(request.send_keeping_body()?)
        },
        mirror_webhook_destination_response,
    )
}

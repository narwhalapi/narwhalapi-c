// Generated from spec/next/openapi.json sha256 5071bd97e9507eb56b344d00a24ba056ce1340fe1184faa4d755fe7e407e1cf6. Do not edit.
use crate::{meta::Model, Decimal, ResponseMeta};
use serde::{Deserialize, Deserializer, Serialize};
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum BarInterval {
    V15m,
    V1d,
    V1h,
    V1m,
    V4h,
    V5m,
    Unknown(String),
}
impl BarInterval {
    pub fn as_str(&self) -> &str {
        match self {
            Self::V15m => "15m",
            Self::V1d => "1d",
            Self::V1h => "1h",
            Self::V1m => "1m",
            Self::V4h => "4h",
            Self::V5m => "5m",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for BarInterval {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for BarInterval {}
impl std::hash::Hash for BarInterval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for BarInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for BarInterval {
    fn from(value: &str) -> Self {
        match value {
            "15m" => Self::V15m,
            "1d" => Self::V1d,
            "1h" => Self::V1h,
            "1m" => Self::V1m,
            "4h" => Self::V4h,
            "5m" => Self::V5m,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for BarInterval {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for BarInterval {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CatalogueLatestFrequency {
    Annual,
    Daily,
    Monthly,
    Quarterly,
    Unknown(String),
}
impl CatalogueLatestFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Annual => "annual",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CatalogueLatestFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CatalogueLatestFrequency {}
impl std::hash::Hash for CatalogueLatestFrequency {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CatalogueLatestFrequency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CatalogueLatestFrequency {
    fn from(value: &str) -> Self {
        match value {
            "annual" => Self::Annual,
            "daily" => Self::Daily,
            "monthly" => Self::Monthly,
            "quarterly" => Self::Quarterly,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CatalogueLatestFrequency {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CatalogueLatestFrequency {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CatalogueLatestSeasonalAdjustment {
    NotApplicable,
    NotSeasonallyAdjusted,
    NotStated,
    SeasonallyAdjusted,
    Unknown(String),
}
impl CatalogueLatestSeasonalAdjustment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NotSeasonallyAdjusted => "not_seasonally_adjusted",
            Self::NotStated => "not_stated",
            Self::SeasonallyAdjusted => "seasonally_adjusted",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CatalogueLatestSeasonalAdjustment {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CatalogueLatestSeasonalAdjustment {}
impl std::hash::Hash for CatalogueLatestSeasonalAdjustment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CatalogueLatestSeasonalAdjustment {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CatalogueLatestSeasonalAdjustment {
    fn from(value: &str) -> Self {
        match value {
            "not_applicable" => Self::NotApplicable,
            "not_seasonally_adjusted" => Self::NotSeasonallyAdjusted,
            "not_stated" => Self::NotStated,
            "seasonally_adjusted" => Self::SeasonallyAdjusted,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CatalogueLatestSeasonalAdjustment {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CatalogueLatestSeasonalAdjustment {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CentralBankBalanceSheetScale {
    Millions,
    Units,
    Unknown(String),
}
impl CentralBankBalanceSheetScale {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Millions => "millions",
            Self::Units => "units",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CentralBankBalanceSheetScale {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CentralBankBalanceSheetScale {}
impl std::hash::Hash for CentralBankBalanceSheetScale {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CentralBankBalanceSheetScale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CentralBankBalanceSheetScale {
    fn from(value: &str) -> Self {
        match value {
            "millions" => Self::Millions,
            "units" => Self::Units,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CentralBankBalanceSheetScale {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CentralBankBalanceSheetScale {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CommodityUnit {
    Barrel,
    Pound,
    TroyOunce,
    Unknown(String),
}
impl CommodityUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Barrel => "barrel",
            Self::Pound => "pound",
            Self::TroyOunce => "troy_ounce",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CommodityUnit {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CommodityUnit {}
impl std::hash::Hash for CommodityUnit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CommodityUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CommodityUnit {
    fn from(value: &str) -> Self {
        match value {
            "barrel" => Self::Barrel,
            "pound" => Self::Pound,
            "troy_ounce" => Self::TroyOunce,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CommodityUnit {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CommodityUnit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CompanyRelationKind {
    CrossListed,
    Predecessor,
    Successor,
    Unknown(String),
}
impl CompanyRelationKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CrossListed => "cross_listed",
            Self::Predecessor => "predecessor",
            Self::Successor => "successor",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CompanyRelationKind {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CompanyRelationKind {}
impl std::hash::Hash for CompanyRelationKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CompanyRelationKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CompanyRelationKind {
    fn from(value: &str) -> Self {
        match value {
            "cross_listed" => Self::CrossListed,
            "predecessor" => Self::Predecessor,
            "successor" => Self::Successor,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CompanyRelationKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CompanyRelationKind {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CompanySearchMatch {
    Code,
    FormerCode,
    FormerName,
    Id,
    Name,
    Unknown(String),
}
impl CompanySearchMatch {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Code => "code",
            Self::FormerCode => "former_code",
            Self::FormerName => "former_name",
            Self::Id => "id",
            Self::Name => "name",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CompanySearchMatch {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CompanySearchMatch {}
impl std::hash::Hash for CompanySearchMatch {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CompanySearchMatch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CompanySearchMatch {
    fn from(value: &str) -> Self {
        match value {
            "code" => Self::Code,
            "former_code" => Self::FormerCode,
            "former_name" => Self::FormerName,
            "id" => Self::Id,
            "name" => Self::Name,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CompanySearchMatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CompanySearchMatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CompanyStandard {
    Ifrs,
    JGaap,
    Jmis,
    KGaap,
    UsGaap,
    Unknown(String),
}
impl CompanyStandard {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ifrs => "ifrs",
            Self::JGaap => "j_gaap",
            Self::Jmis => "jmis",
            Self::KGaap => "k_gaap",
            Self::UsGaap => "us_gaap",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CompanyStandard {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CompanyStandard {}
impl std::hash::Hash for CompanyStandard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CompanyStandard {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CompanyStandard {
    fn from(value: &str) -> Self {
        match value {
            "ifrs" => Self::Ifrs,
            "j_gaap" => Self::JGaap,
            "jmis" => Self::Jmis,
            "k_gaap" => Self::KGaap,
            "us_gaap" => Self::UsGaap,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CompanyStandard {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CompanyStandard {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostEquityRiskPremiumBasis {
    Modelled,
    Observed,
    Table,
    Unknown(String),
}
impl CostEquityRiskPremiumBasis {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Modelled => "modelled",
            Self::Observed => "observed",
            Self::Table => "table",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostEquityRiskPremiumBasis {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostEquityRiskPremiumBasis {}
impl std::hash::Hash for CostEquityRiskPremiumBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostEquityRiskPremiumBasis {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostEquityRiskPremiumBasis {
    fn from(value: &str) -> Self {
        match value {
            "modelled" => Self::Modelled,
            "observed" => Self::Observed,
            "table" => Self::Table,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostEquityRiskPremiumBasis {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostEquityRiskPremiumBasis {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostEquityRiskPremiumSource {
    Caller,
    Narwhal,
    Unknown(String),
}
impl CostEquityRiskPremiumSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Caller => "caller",
            Self::Narwhal => "narwhal",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostEquityRiskPremiumSource {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostEquityRiskPremiumSource {}
impl std::hash::Hash for CostEquityRiskPremiumSource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostEquityRiskPremiumSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostEquityRiskPremiumSource {
    fn from(value: &str) -> Self {
        match value {
            "caller" => Self::Caller,
            "narwhal" => Self::Narwhal,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostEquityRiskPremiumSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostEquityRiskPremiumSource {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostOfCapitalBeta {
    Adjusted,
    Raw,
    Unknown(String),
}
impl CostOfCapitalBeta {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Adjusted => "adjusted",
            Self::Raw => "raw",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostOfCapitalBeta {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostOfCapitalBeta {}
impl std::hash::Hash for CostOfCapitalBeta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostOfCapitalBeta {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostOfCapitalBeta {
    fn from(value: &str) -> Self {
        match value {
            "adjusted" => Self::Adjusted,
            "raw" => Self::Raw,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostOfCapitalBeta {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostOfCapitalBeta {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostOfCapitalFrequency {
    Monthly,
    Weekly,
    Unknown(String),
}
impl CostOfCapitalFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Monthly => "monthly",
            Self::Weekly => "weekly",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostOfCapitalFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostOfCapitalFrequency {}
impl std::hash::Hash for CostOfCapitalFrequency {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostOfCapitalFrequency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostOfCapitalFrequency {
    fn from(value: &str) -> Self {
        match value {
            "monthly" => Self::Monthly,
            "weekly" => Self::Weekly,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostOfCapitalFrequency {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostOfCapitalFrequency {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostOfCapitalPeriod {
    Annual,
    Ttm,
    Unknown(String),
}
impl CostOfCapitalPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Annual => "annual",
            Self::Ttm => "ttm",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostOfCapitalPeriod {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostOfCapitalPeriod {}
impl std::hash::Hash for CostOfCapitalPeriod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostOfCapitalPeriod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostOfCapitalPeriod {
    fn from(value: &str) -> Self {
        match value {
            "annual" => Self::Annual,
            "ttm" => Self::Ttm,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostOfCapitalPeriod {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostOfCapitalPeriod {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostOfCapitalWindow {
    V2y,
    V5y,
    Unknown(String),
}
impl CostOfCapitalWindow {
    pub fn as_str(&self) -> &str {
        match self {
            Self::V2y => "2y",
            Self::V5y => "5y",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostOfCapitalWindow {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostOfCapitalWindow {}
impl std::hash::Hash for CostOfCapitalWindow {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostOfCapitalWindow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostOfCapitalWindow {
    fn from(value: &str) -> Self {
        match value {
            "2y" => Self::V2y,
            "5y" => Self::V5y,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostOfCapitalWindow {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostOfCapitalWindow {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostOfDebtBasis {
    Ours,
    Table,
    Unknown(String),
}
impl CostOfDebtBasis {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ours => "ours",
            Self::Table => "table",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostOfDebtBasis {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostOfDebtBasis {}
impl std::hash::Hash for CostOfDebtBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostOfDebtBasis {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostOfDebtBasis {
    fn from(value: &str) -> Self {
        match value {
            "ours" => Self::Ours,
            "table" => Self::Table,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostOfDebtBasis {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostOfDebtBasis {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostRiskFreeBasis {
    Caller,
    Ours,
    Unknown(String),
}
impl CostRiskFreeBasis {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Caller => "caller",
            Self::Ours => "ours",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostRiskFreeBasis {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostRiskFreeBasis {}
impl std::hash::Hash for CostRiskFreeBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostRiskFreeBasis {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostRiskFreeBasis {
    fn from(value: &str) -> Self {
        match value {
            "caller" => Self::Caller,
            "ours" => Self::Ours,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostRiskFreeBasis {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostRiskFreeBasis {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostRiskFreeSource {
    Caller,
    YieldCurve,
    Unknown(String),
}
impl CostRiskFreeSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Caller => "caller",
            Self::YieldCurve => "yield_curve",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostRiskFreeSource {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostRiskFreeSource {}
impl std::hash::Hash for CostRiskFreeSource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostRiskFreeSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostRiskFreeSource {
    fn from(value: &str) -> Self {
        match value {
            "caller" => Self::Caller,
            "yield_curve" => Self::YieldCurve,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostRiskFreeSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostRiskFreeSource {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostTaxRateBasis {
    Caller,
    Table,
    Unknown(String),
}
impl CostTaxRateBasis {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Caller => "caller",
            Self::Table => "table",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostTaxRateBasis {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostTaxRateBasis {}
impl std::hash::Hash for CostTaxRateBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostTaxRateBasis {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostTaxRateBasis {
    fn from(value: &str) -> Self {
        match value {
            "caller" => Self::Caller,
            "table" => Self::Table,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostTaxRateBasis {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostTaxRateBasis {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CostTaxRateSource {
    Caller,
    CountryTable,
    Unknown(String),
}
impl CostTaxRateSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Caller => "caller",
            Self::CountryTable => "country_table",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CostTaxRateSource {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CostTaxRateSource {}
impl std::hash::Hash for CostTaxRateSource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CostTaxRateSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CostTaxRateSource {
    fn from(value: &str) -> Self {
        match value {
            "caller" => Self::Caller,
            "country_table" => Self::CountryTable,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CostTaxRateSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CostTaxRateSource {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CryptoBarInterval {
    V1d,
    V1m,
    Unknown(String),
}
impl CryptoBarInterval {
    pub fn as_str(&self) -> &str {
        match self {
            Self::V1d => "1d",
            Self::V1m => "1m",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CryptoBarInterval {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CryptoBarInterval {}
impl std::hash::Hash for CryptoBarInterval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CryptoBarInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CryptoBarInterval {
    fn from(value: &str) -> Self {
        match value {
            "1d" => Self::V1d,
            "1m" => Self::V1m,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CryptoBarInterval {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CryptoBarInterval {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CryptoPriceErrorCode {
    NotCovered,
    NotFound,
    UpstreamBusy,
    Unknown(String),
}
impl CryptoPriceErrorCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotCovered => "not_covered",
            Self::NotFound => "not_found",
            Self::UpstreamBusy => "upstream_busy",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CryptoPriceErrorCode {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CryptoPriceErrorCode {}
impl std::hash::Hash for CryptoPriceErrorCode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CryptoPriceErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CryptoPriceErrorCode {
    fn from(value: &str) -> Self {
        match value {
            "not_covered" => Self::NotCovered,
            "not_found" => Self::NotFound,
            "upstream_busy" => Self::UpstreamBusy,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CryptoPriceErrorCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CryptoPriceErrorCode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CryptoQuote {
    EUR,
    GBP,
    USD,
    Unknown(String),
}
impl CryptoQuote {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::USD => "USD",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for CryptoQuote {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CryptoQuote {}
impl std::hash::Hash for CryptoQuote {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for CryptoQuote {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for CryptoQuote {
    fn from(value: &str) -> Self {
        match value {
            "EUR" => Self::EUR,
            "GBP" => Self::GBP,
            "USD" => Self::USD,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for CryptoQuote {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CryptoQuote {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DeliveryFamily {
    Activity,
    Gdp,
    Holidays,
    Housing,
    Labour,
    Money,
    PolicyRate,
    Prices,
    PublicFinance,
    Survey,
    Trade,
    YieldCurve,
    Unknown(String),
}
impl DeliveryFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Activity => "activity",
            Self::Gdp => "gdp",
            Self::Holidays => "holidays",
            Self::Housing => "housing",
            Self::Labour => "labour",
            Self::Money => "money",
            Self::PolicyRate => "policy_rate",
            Self::Prices => "prices",
            Self::PublicFinance => "public_finance",
            Self::Survey => "survey",
            Self::Trade => "trade",
            Self::YieldCurve => "yield_curve",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for DeliveryFamily {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for DeliveryFamily {}
impl std::hash::Hash for DeliveryFamily {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for DeliveryFamily {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for DeliveryFamily {
    fn from(value: &str) -> Self {
        match value {
            "activity" => Self::Activity,
            "gdp" => Self::Gdp,
            "holidays" => Self::Holidays,
            "housing" => Self::Housing,
            "labour" => Self::Labour,
            "money" => Self::Money,
            "policy_rate" => Self::PolicyRate,
            "prices" => Self::Prices,
            "public_finance" => Self::PublicFinance,
            "survey" => Self::Survey,
            "trade" => Self::Trade,
            "yield_curve" => Self::YieldCurve,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for DeliveryFamily {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for DeliveryFamily {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EconomicFamily {
    Activity,
    Gdp,
    Housing,
    Labour,
    Money,
    PolicyRate,
    Prices,
    PublicFinance,
    Survey,
    Trade,
    YieldCurve,
    Unknown(String),
}
impl EconomicFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Activity => "activity",
            Self::Gdp => "gdp",
            Self::Housing => "housing",
            Self::Labour => "labour",
            Self::Money => "money",
            Self::PolicyRate => "policy_rate",
            Self::Prices => "prices",
            Self::PublicFinance => "public_finance",
            Self::Survey => "survey",
            Self::Trade => "trade",
            Self::YieldCurve => "yield_curve",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for EconomicFamily {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for EconomicFamily {}
impl std::hash::Hash for EconomicFamily {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for EconomicFamily {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for EconomicFamily {
    fn from(value: &str) -> Self {
        match value {
            "activity" => Self::Activity,
            "gdp" => Self::Gdp,
            "housing" => Self::Housing,
            "labour" => Self::Labour,
            "money" => Self::Money,
            "policy_rate" => Self::PolicyRate,
            "prices" => Self::Prices,
            "public_finance" => Self::PublicFinance,
            "survey" => Self::Survey,
            "trade" => Self::Trade,
            "yield_curve" => Self::YieldCurve,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for EconomicFamily {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for EconomicFamily {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EconomicReleaseValueValueStatus {
    PendingCapture,
    Unavailable,
    Unknown(String),
}
impl EconomicReleaseValueValueStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PendingCapture => "pending_capture",
            Self::Unavailable => "unavailable",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for EconomicReleaseValueValueStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for EconomicReleaseValueValueStatus {}
impl std::hash::Hash for EconomicReleaseValueValueStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for EconomicReleaseValueValueStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for EconomicReleaseValueValueStatus {
    fn from(value: &str) -> Self {
        match value {
            "pending_capture" => Self::PendingCapture,
            "unavailable" => Self::Unavailable,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for EconomicReleaseValueValueStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for EconomicReleaseValueValueStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EquityEarningTime {
    AfterClose,
    BeforeOpen,
    During,
    Unknown(String),
}
impl EquityEarningTime {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AfterClose => "after_close",
            Self::BeforeOpen => "before_open",
            Self::During => "during",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for EquityEarningTime {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for EquityEarningTime {}
impl std::hash::Hash for EquityEarningTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for EquityEarningTime {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for EquityEarningTime {
    fn from(value: &str) -> Self {
        match value {
            "after_close" => Self::AfterClose,
            "before_open" => Self::BeforeOpen,
            "during" => Self::During,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for EquityEarningTime {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for EquityEarningTime {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EquityMarketSession {
    Closed,
    Open,
    Post,
    Pre,
    Unknown(String),
}
impl EquityMarketSession {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::Post => "post",
            Self::Pre => "pre",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for EquityMarketSession {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for EquityMarketSession {}
impl std::hash::Hash for EquityMarketSession {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for EquityMarketSession {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for EquityMarketSession {
    fn from(value: &str) -> Self {
        match value {
            "closed" => Self::Closed,
            "open" => Self::Open,
            "post" => Self::Post,
            "pre" => Self::Pre,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for EquityMarketSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for EquityMarketSession {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EquityType {
    ClosedEndFund,
    CommonStock,
    DepositaryReceipt,
    Other,
    PreferredStock,
    Reit,
    Right,
    Unit,
    Warrant,
    Unknown(String),
}
impl EquityType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClosedEndFund => "closed_end_fund",
            Self::CommonStock => "common_stock",
            Self::DepositaryReceipt => "depositary_receipt",
            Self::Other => "other",
            Self::PreferredStock => "preferred_stock",
            Self::Reit => "reit",
            Self::Right => "right",
            Self::Unit => "unit",
            Self::Warrant => "warrant",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for EquityType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for EquityType {}
impl std::hash::Hash for EquityType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for EquityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for EquityType {
    fn from(value: &str) -> Self {
        match value {
            "closed_end_fund" => Self::ClosedEndFund,
            "common_stock" => Self::CommonStock,
            "depositary_receipt" => Self::DepositaryReceipt,
            "other" => Self::Other,
            "preferred_stock" => Self::PreferredStock,
            "reit" => Self::Reit,
            "right" => Self::Right,
            "unit" => Self::Unit,
            "warrant" => Self::Warrant,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for EquityType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for EquityType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ExtremeObservation {
    AnyRelease,
    AnyTrade,
    Close,
    Unknown(String),
}
impl ExtremeObservation {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AnyRelease => "any_release",
            Self::AnyTrade => "any_trade",
            Self::Close => "close",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ExtremeObservation {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ExtremeObservation {}
impl std::hash::Hash for ExtremeObservation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ExtremeObservation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ExtremeObservation {
    fn from(value: &str) -> Self {
        match value {
            "any_release" => Self::AnyRelease,
            "any_trade" => Self::AnyTrade,
            "close" => Self::Close,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ExtremeObservation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ExtremeObservation {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FilingEventType {
    Annual,
    Event,
    Ownership,
    Quarterly,
    Semiannual,
    Unknown(String),
}
impl FilingEventType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Annual => "annual",
            Self::Event => "event",
            Self::Ownership => "ownership",
            Self::Quarterly => "quarterly",
            Self::Semiannual => "semiannual",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FilingEventType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FilingEventType {}
impl std::hash::Hash for FilingEventType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FilingEventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FilingEventType {
    fn from(value: &str) -> Self {
        match value {
            "annual" => Self::Annual,
            "event" => Self::Event,
            "ownership" => Self::Ownership,
            "quarterly" => Self::Quarterly,
            "semiannual" => Self::Semiannual,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FilingEventType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FilingEventType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FilingType {
    Annual,
    Event,
    Insider,
    Other,
    Ownership,
    Quarterly,
    Semiannual,
    Unknown(String),
}
impl FilingType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Annual => "annual",
            Self::Event => "event",
            Self::Insider => "insider",
            Self::Other => "other",
            Self::Ownership => "ownership",
            Self::Quarterly => "quarterly",
            Self::Semiannual => "semiannual",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FilingType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FilingType {}
impl std::hash::Hash for FilingType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FilingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FilingType {
    fn from(value: &str) -> Self {
        match value {
            "annual" => Self::Annual,
            "event" => Self::Event,
            "insider" => Self::Insider,
            "other" => Self::Other,
            "ownership" => Self::Ownership,
            "quarterly" => Self::Quarterly,
            "semiannual" => Self::Semiannual,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FilingType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FilingType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FinancialStatement {
    Balance,
    CashFlow,
    Income,
    Other,
    PerShare,
    Shares,
    Unknown(String),
}
impl FinancialStatement {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Balance => "balance",
            Self::CashFlow => "cash_flow",
            Self::Income => "income",
            Self::Other => "other",
            Self::PerShare => "per_share",
            Self::Shares => "shares",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FinancialStatement {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FinancialStatement {}
impl std::hash::Hash for FinancialStatement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FinancialStatement {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FinancialStatement {
    fn from(value: &str) -> Self {
        match value {
            "balance" => Self::Balance,
            "cash_flow" => Self::CashFlow,
            "income" => Self::Income,
            "other" => Self::Other,
            "per_share" => Self::PerShare,
            "shares" => Self::Shares,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FinancialStatement {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FinancialStatement {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FinancialsBasis {
    Consolidated,
    Standalone,
    Unknown(String),
}
impl FinancialsBasis {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Consolidated => "consolidated",
            Self::Standalone => "standalone",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FinancialsBasis {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FinancialsBasis {}
impl std::hash::Hash for FinancialsBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FinancialsBasis {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FinancialsBasis {
    fn from(value: &str) -> Self {
        match value {
            "consolidated" => Self::Consolidated,
            "standalone" => Self::Standalone,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FinancialsBasis {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FinancialsBasis {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FinancialsPeriod {
    Annual,
    Quarterly,
    Semiannual,
    Unknown(String),
}
impl FinancialsPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Annual => "annual",
            Self::Quarterly => "quarterly",
            Self::Semiannual => "semiannual",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FinancialsPeriod {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FinancialsPeriod {}
impl std::hash::Hash for FinancialsPeriod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FinancialsPeriod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FinancialsPeriod {
    fn from(value: &str) -> Self {
        match value {
            "annual" => Self::Annual,
            "quarterly" => Self::Quarterly,
            "semiannual" => Self::Semiannual,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FinancialsPeriod {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FinancialsPeriod {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FinancialsVersion {
    FirstReported,
    Latest,
    Unknown(String),
}
impl FinancialsVersion {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FirstReported => "first_reported",
            Self::Latest => "latest",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FinancialsVersion {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FinancialsVersion {}
impl std::hash::Hash for FinancialsVersion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FinancialsVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FinancialsVersion {
    fn from(value: &str) -> Self {
        match value {
            "first_reported" => Self::FirstReported,
            "latest" => Self::Latest,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FinancialsVersion {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FinancialsVersion {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FiscalPeriod {
    FY,
    H1,
    H2,
    Q1,
    Q2,
    Q3,
    Q4,
    Unknown(String),
}
impl FiscalPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FY => "FY",
            Self::H1 => "H1",
            Self::H2 => "H2",
            Self::Q1 => "Q1",
            Self::Q2 => "Q2",
            Self::Q3 => "Q3",
            Self::Q4 => "Q4",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FiscalPeriod {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FiscalPeriod {}
impl std::hash::Hash for FiscalPeriod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FiscalPeriod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FiscalPeriod {
    fn from(value: &str) -> Self {
        match value {
            "FY" => Self::FY,
            "H1" => Self::H1,
            "H2" => Self::H2,
            "Q1" => Self::Q1,
            "Q2" => Self::Q2,
            "Q3" => Self::Q3,
            "Q4" => Self::Q4,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FiscalPeriod {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FiscalPeriod {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FundDistributionKind {
    CapitalGain,
    Income,
    Unknown(String),
}
impl FundDistributionKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CapitalGain => "capital_gain",
            Self::Income => "income",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FundDistributionKind {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FundDistributionKind {}
impl std::hash::Hash for FundDistributionKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FundDistributionKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FundDistributionKind {
    fn from(value: &str) -> Self {
        match value {
            "capital_gain" => Self::CapitalGain,
            "income" => Self::Income,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FundDistributionKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FundDistributionKind {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FundType {
    Etf,
    MutualFund,
    Unknown(String),
}
impl FundType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Etf => "etf",
            Self::MutualFund => "mutual_fund",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for FundType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FundType {}
impl std::hash::Hash for FundType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for FundType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for FundType {
    fn from(value: &str) -> Self {
        match value {
            "etf" => Self::Etf,
            "mutual_fund" => Self::MutualFund,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for FundType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FundType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HolidayScope {
    Bank,
    CentralGovernment,
    CollectiveLeave,
    FederalGovernment,
    NationalPublic,
    PrivateSector,
    PublicSector,
    RegionalPublic,
    Unknown(String),
}
impl HolidayScope {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bank => "bank",
            Self::CentralGovernment => "central_government",
            Self::CollectiveLeave => "collective_leave",
            Self::FederalGovernment => "federal_government",
            Self::NationalPublic => "national_public",
            Self::PrivateSector => "private_sector",
            Self::PublicSector => "public_sector",
            Self::RegionalPublic => "regional_public",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for HolidayScope {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HolidayScope {}
impl std::hash::Hash for HolidayScope {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for HolidayScope {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for HolidayScope {
    fn from(value: &str) -> Self {
        match value {
            "bank" => Self::Bank,
            "central_government" => Self::CentralGovernment,
            "collective_leave" => Self::CollectiveLeave,
            "federal_government" => Self::FederalGovernment,
            "national_public" => Self::NationalPublic,
            "private_sector" => Self::PrivateSector,
            "public_sector" => Self::PublicSector,
            "regional_public" => Self::RegionalPublic,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for HolidayScope {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for HolidayScope {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum IndicatorUnit {
    Balance,
    Currency,
    Index,
    Percent,
    Ratio,
    Thousands,
    Unknown(String),
}
impl IndicatorUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Balance => "balance",
            Self::Currency => "currency",
            Self::Index => "index",
            Self::Percent => "percent",
            Self::Ratio => "ratio",
            Self::Thousands => "thousands",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for IndicatorUnit {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for IndicatorUnit {}
impl std::hash::Hash for IndicatorUnit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for IndicatorUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for IndicatorUnit {
    fn from(value: &str) -> Self {
        match value {
            "balance" => Self::Balance,
            "currency" => Self::Currency,
            "index" => Self::Index,
            "percent" => Self::Percent,
            "ratio" => Self::Ratio,
            "thousands" => Self::Thousands,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for IndicatorUnit {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for IndicatorUnit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Language {
    En,
    Original,
    Unknown(String),
}
impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::En => "en",
            Self::Original => "original",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for Language {}
impl std::hash::Hash for Language {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "en" => Self::En,
            "original" => Self::Original,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for Language {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Language {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum MarketSession {
    InterbankClosed,
    Open,
    Weekend,
    Unknown(String),
}
impl MarketSession {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InterbankClosed => "interbank_closed",
            Self::Open => "open",
            Self::Weekend => "weekend",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for MarketSession {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for MarketSession {}
impl std::hash::Hash for MarketSession {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for MarketSession {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for MarketSession {
    fn from(value: &str) -> Self {
        match value {
            "interbank_closed" => Self::InterbankClosed,
            "open" => Self::Open,
            "weekend" => Self::Weekend,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for MarketSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for MarketSession {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum MoversDirection {
    Active,
    Gainers,
    Losers,
    Unknown(String),
}
impl MoversDirection {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Gainers => "gainers",
            Self::Losers => "losers",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for MoversDirection {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for MoversDirection {}
impl std::hash::Hash for MoversDirection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for MoversDirection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for MoversDirection {
    fn from(value: &str) -> Self {
        match value {
            "active" => Self::Active,
            "gainers" => Self::Gainers,
            "losers" => Self::Losers,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for MoversDirection {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for MoversDirection {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PalmoilFrequency {
    Daily,
    Fortnightly,
    Irregular,
    Monthly,
    Weekly,
    Unknown(String),
}
impl PalmoilFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Daily => "daily",
            Self::Fortnightly => "fortnightly",
            Self::Irregular => "irregular",
            Self::Monthly => "monthly",
            Self::Weekly => "weekly",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PalmoilFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PalmoilFrequency {}
impl std::hash::Hash for PalmoilFrequency {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PalmoilFrequency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PalmoilFrequency {
    fn from(value: &str) -> Self {
        match value {
            "daily" => Self::Daily,
            "fortnightly" => Self::Fortnightly,
            "irregular" => Self::Irregular,
            "monthly" => Self::Monthly,
            "weekly" => Self::Weekly,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PalmoilFrequency {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PalmoilFrequency {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PalmoilPeriodType {
    Effective,
    Observation,
    Unknown(String),
}
impl PalmoilPeriodType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Effective => "effective",
            Self::Observation => "observation",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PalmoilPeriodType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PalmoilPeriodType {}
impl std::hash::Hash for PalmoilPeriodType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PalmoilPeriodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PalmoilPeriodType {
    fn from(value: &str) -> Self {
        match value {
            "effective" => Self::Effective,
            "observation" => Self::Observation,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PalmoilPeriodType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PalmoilPeriodType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PalmoilPriceType {
    GovernmentReference,
    MarketAverage,
    OfficialPurchaseSchedule,
    Unknown(String),
}
impl PalmoilPriceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GovernmentReference => "government_reference",
            Self::MarketAverage => "market_average",
            Self::OfficialPurchaseSchedule => "official_purchase_schedule",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PalmoilPriceType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PalmoilPriceType {}
impl std::hash::Hash for PalmoilPriceType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PalmoilPriceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PalmoilPriceType {
    fn from(value: &str) -> Self {
        match value {
            "government_reference" => Self::GovernmentReference,
            "market_average" => Self::MarketAverage,
            "official_purchase_schedule" => Self::OfficialPurchaseSchedule,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PalmoilPriceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PalmoilPriceType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PalmoilProduct {
    Cpo,
    Ffb,
    Unknown(String),
}
impl PalmoilProduct {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cpo => "cpo",
            Self::Ffb => "ffb",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PalmoilProduct {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PalmoilProduct {}
impl std::hash::Hash for PalmoilProduct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PalmoilProduct {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PalmoilProduct {
    fn from(value: &str) -> Self {
        match value {
            "cpo" => Self::Cpo,
            "ffb" => Self::Ffb,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PalmoilProduct {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PalmoilProduct {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PalmoilScalarPriceType {
    GovernmentReference,
    MarketAverage,
    Unknown(String),
}
impl PalmoilScalarPriceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GovernmentReference => "government_reference",
            Self::MarketAverage => "market_average",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PalmoilScalarPriceType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PalmoilScalarPriceType {}
impl std::hash::Hash for PalmoilScalarPriceType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PalmoilScalarPriceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PalmoilScalarPriceType {
    fn from(value: &str) -> Self {
        match value {
            "government_reference" => Self::GovernmentReference,
            "market_average" => Self::MarketAverage,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PalmoilScalarPriceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PalmoilScalarPriceType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PalmoilScalarUnit {
    MetricTon,
    OnePercentOer,
    Unknown(String),
}
impl PalmoilScalarUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MetricTon => "metric_ton",
            Self::OnePercentOer => "one_percent_oer",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PalmoilScalarUnit {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PalmoilScalarUnit {}
impl std::hash::Hash for PalmoilScalarUnit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PalmoilScalarUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PalmoilScalarUnit {
    fn from(value: &str) -> Self {
        match value {
            "metric_ton" => Self::MetricTon,
            "one_percent_oer" => Self::OnePercentOer,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PalmoilScalarUnit {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PalmoilScalarUnit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PhysicalCategory {
    Emissions,
    Energy,
    Fertilizers,
    Forestry,
    Grains,
    LivestockDairy,
    Metals,
    OilsOilseeds,
    Softs,
    Unknown(String),
}
impl PhysicalCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Emissions => "emissions",
            Self::Energy => "energy",
            Self::Fertilizers => "fertilizers",
            Self::Forestry => "forestry",
            Self::Grains => "grains",
            Self::LivestockDairy => "livestock-dairy",
            Self::Metals => "metals",
            Self::OilsOilseeds => "oils-oilseeds",
            Self::Softs => "softs",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PhysicalCategory {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PhysicalCategory {}
impl std::hash::Hash for PhysicalCategory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PhysicalCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PhysicalCategory {
    fn from(value: &str) -> Self {
        match value {
            "emissions" => Self::Emissions,
            "energy" => Self::Energy,
            "fertilizers" => Self::Fertilizers,
            "forestry" => Self::Forestry,
            "grains" => Self::Grains,
            "livestock-dairy" => Self::LivestockDairy,
            "metals" => Self::Metals,
            "oils-oilseeds" => Self::OilsOilseeds,
            "softs" => Self::Softs,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PhysicalCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PhysicalCategory {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PhysicalFrequency {
    Annual,
    Daily,
    Intraday,
    Irregular,
    Monthly,
    Quarterly,
    Semiannual,
    ThreeTimesMonthly,
    TwiceMonthly,
    Weekly,
    Unknown(String),
}
impl PhysicalFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Annual => "annual",
            Self::Daily => "daily",
            Self::Intraday => "intraday",
            Self::Irregular => "irregular",
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::Semiannual => "semiannual",
            Self::ThreeTimesMonthly => "three_times_monthly",
            Self::TwiceMonthly => "twice_monthly",
            Self::Weekly => "weekly",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PhysicalFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PhysicalFrequency {}
impl std::hash::Hash for PhysicalFrequency {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PhysicalFrequency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PhysicalFrequency {
    fn from(value: &str) -> Self {
        match value {
            "annual" => Self::Annual,
            "daily" => Self::Daily,
            "intraday" => Self::Intraday,
            "irregular" => Self::Irregular,
            "monthly" => Self::Monthly,
            "quarterly" => Self::Quarterly,
            "semiannual" => Self::Semiannual,
            "three_times_monthly" => Self::ThreeTimesMonthly,
            "twice_monthly" => Self::TwiceMonthly,
            "weekly" => Self::Weekly,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PhysicalFrequency {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PhysicalFrequency {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PhysicalPriceType {
    AuctionClearing,
    ExportUnitValue,
    FarmgateAverage,
    GovernmentReference,
    ImportUnitValue,
    InternationalBenchmark,
    MarketAverage,
    OfficialBuy,
    OfficialReferenceRate,
    OfficialSell,
    RegulatedAverage,
    RetailAverage,
    RetailBuy,
    RetailSell,
    WholesaleAverage,
    Unknown(String),
}
impl PhysicalPriceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AuctionClearing => "auction_clearing",
            Self::ExportUnitValue => "export_unit_value",
            Self::FarmgateAverage => "farmgate_average",
            Self::GovernmentReference => "government_reference",
            Self::ImportUnitValue => "import_unit_value",
            Self::InternationalBenchmark => "international_benchmark",
            Self::MarketAverage => "market_average",
            Self::OfficialBuy => "official_buy",
            Self::OfficialReferenceRate => "official_reference_rate",
            Self::OfficialSell => "official_sell",
            Self::RegulatedAverage => "regulated_average",
            Self::RetailAverage => "retail_average",
            Self::RetailBuy => "retail_buy",
            Self::RetailSell => "retail_sell",
            Self::WholesaleAverage => "wholesale_average",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PhysicalPriceType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PhysicalPriceType {}
impl std::hash::Hash for PhysicalPriceType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PhysicalPriceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PhysicalPriceType {
    fn from(value: &str) -> Self {
        match value {
            "auction_clearing" => Self::AuctionClearing,
            "export_unit_value" => Self::ExportUnitValue,
            "farmgate_average" => Self::FarmgateAverage,
            "government_reference" => Self::GovernmentReference,
            "import_unit_value" => Self::ImportUnitValue,
            "international_benchmark" => Self::InternationalBenchmark,
            "market_average" => Self::MarketAverage,
            "official_buy" => Self::OfficialBuy,
            "official_reference_rate" => Self::OfficialReferenceRate,
            "official_sell" => Self::OfficialSell,
            "regulated_average" => Self::RegulatedAverage,
            "retail_average" => Self::RetailAverage,
            "retail_buy" => Self::RetailBuy,
            "retail_sell" => Self::RetailSell,
            "wholesale_average" => Self::WholesaleAverage,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PhysicalPriceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PhysicalPriceType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PointPolicyRateQualifier {
    Around,
    Exact,
    Unknown(String),
}
impl PointPolicyRateQualifier {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Around => "around",
            Self::Exact => "exact",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PointPolicyRateQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PointPolicyRateQualifier {}
impl std::hash::Hash for PointPolicyRateQualifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PointPolicyRateQualifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PointPolicyRateQualifier {
    fn from(value: &str) -> Self {
        match value {
            "around" => Self::Around,
            "exact" => Self::Exact,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PointPolicyRateQualifier {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PointPolicyRateQualifier {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PredictionEventStatus {
    Closed,
    Settled,
    Trading,
    Upcoming,
    Void,
    Unknown(String),
}
impl PredictionEventStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Closed => "closed",
            Self::Settled => "settled",
            Self::Trading => "trading",
            Self::Upcoming => "upcoming",
            Self::Void => "void",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PredictionEventStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PredictionEventStatus {}
impl std::hash::Hash for PredictionEventStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PredictionEventStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PredictionEventStatus {
    fn from(value: &str) -> Self {
        match value {
            "closed" => Self::Closed,
            "settled" => Self::Settled,
            "trading" => Self::Trading,
            "upcoming" => Self::Upcoming,
            "void" => Self::Void,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PredictionEventStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PredictionEventStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PredictionShape {
    Binary,
    Categorical,
    Extreme,
    Scalar,
    Set,
    Timing,
    Unknown(String),
}
impl PredictionShape {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Binary => "binary",
            Self::Categorical => "categorical",
            Self::Extreme => "extreme",
            Self::Scalar => "scalar",
            Self::Set => "set",
            Self::Timing => "timing",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PredictionShape {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PredictionShape {}
impl std::hash::Hash for PredictionShape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PredictionShape {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PredictionShape {
    fn from(value: &str) -> Self {
        match value {
            "binary" => Self::Binary,
            "categorical" => Self::Categorical,
            "extreme" => Self::Extreme,
            "scalar" => Self::Scalar,
            "set" => Self::Set,
            "timing" => Self::Timing,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PredictionShape {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PredictionShape {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PredictionVenueSettlement {
    Settled,
    Void,
    Unknown(String),
}
impl PredictionVenueSettlement {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Settled => "settled",
            Self::Void => "void",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PredictionVenueSettlement {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PredictionVenueSettlement {}
impl std::hash::Hash for PredictionVenueSettlement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PredictionVenueSettlement {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PredictionVenueSettlement {
    fn from(value: &str) -> Self {
        match value {
            "settled" => Self::Settled,
            "void" => Self::Void,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PredictionVenueSettlement {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PredictionVenueSettlement {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PredictionWebhookStatus {
    Closed,
    Settled,
    Void,
    Unknown(String),
}
impl PredictionWebhookStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Closed => "closed",
            Self::Settled => "settled",
            Self::Void => "void",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for PredictionWebhookStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PredictionWebhookStatus {}
impl std::hash::Hash for PredictionWebhookStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for PredictionWebhookStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for PredictionWebhookStatus {
    fn from(value: &str) -> Self {
        match value {
            "closed" => Self::Closed,
            "settled" => Self::Settled,
            "void" => Self::Void,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for PredictionWebhookStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PredictionWebhookStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ProducerArrangement {
    PurchaseAgreement,
    SupportedScheme,
    Unknown(String),
}
impl ProducerArrangement {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PurchaseAgreement => "purchase_agreement",
            Self::SupportedScheme => "supported_scheme",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ProducerArrangement {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ProducerArrangement {}
impl std::hash::Hash for ProducerArrangement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ProducerArrangement {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ProducerArrangement {
    fn from(value: &str) -> Self {
        match value {
            "purchase_agreement" => Self::PurchaseAgreement,
            "supported_scheme" => Self::SupportedScheme,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ProducerArrangement {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ProducerArrangement {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Registry {
    Aut,
    Bel,
    Cyp,
    Cze,
    Dnk,
    Esp,
    Est,
    Fin,
    Fra,
    Gbr,
    Grc,
    Hrv,
    Hun,
    Isl,
    Ita,
    Jpn,
    Kor,
    Ltu,
    Lux,
    Lva,
    Mlt,
    Nld,
    Nor,
    Pol,
    Prt,
    Rou,
    Svk,
    Svn,
    Swe,
    Twn,
    Usa,
    Unknown(String),
}
impl Registry {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Aut => "aut",
            Self::Bel => "bel",
            Self::Cyp => "cyp",
            Self::Cze => "cze",
            Self::Dnk => "dnk",
            Self::Esp => "esp",
            Self::Est => "est",
            Self::Fin => "fin",
            Self::Fra => "fra",
            Self::Gbr => "gbr",
            Self::Grc => "grc",
            Self::Hrv => "hrv",
            Self::Hun => "hun",
            Self::Isl => "isl",
            Self::Ita => "ita",
            Self::Jpn => "jpn",
            Self::Kor => "kor",
            Self::Ltu => "ltu",
            Self::Lux => "lux",
            Self::Lva => "lva",
            Self::Mlt => "mlt",
            Self::Nld => "nld",
            Self::Nor => "nor",
            Self::Pol => "pol",
            Self::Prt => "prt",
            Self::Rou => "rou",
            Self::Svk => "svk",
            Self::Svn => "svn",
            Self::Swe => "swe",
            Self::Twn => "twn",
            Self::Usa => "usa",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for Registry {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for Registry {}
impl std::hash::Hash for Registry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for Registry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for Registry {
    fn from(value: &str) -> Self {
        match value {
            "aut" => Self::Aut,
            "bel" => Self::Bel,
            "cyp" => Self::Cyp,
            "cze" => Self::Cze,
            "dnk" => Self::Dnk,
            "esp" => Self::Esp,
            "est" => Self::Est,
            "fin" => Self::Fin,
            "fra" => Self::Fra,
            "gbr" => Self::Gbr,
            "grc" => Self::Grc,
            "hrv" => Self::Hrv,
            "hun" => Self::Hun,
            "isl" => Self::Isl,
            "ita" => Self::Ita,
            "jpn" => Self::Jpn,
            "kor" => Self::Kor,
            "ltu" => Self::Ltu,
            "lux" => Self::Lux,
            "lva" => Self::Lva,
            "mlt" => Self::Mlt,
            "nld" => Self::Nld,
            "nor" => Self::Nor,
            "pol" => Self::Pol,
            "prt" => Self::Prt,
            "rou" => Self::Rou,
            "svk" => Self::Svk,
            "svn" => Self::Svn,
            "swe" => Self::Swe,
            "twn" => Self::Twn,
            "usa" => Self::Usa,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for Registry {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Registry {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ReleaseStage {
    Advance,
    Final,
    First,
    Flash,
    Preliminary,
    Second,
    Third,
    Unknown(String),
}
impl ReleaseStage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Advance => "advance",
            Self::Final => "final",
            Self::First => "first",
            Self::Flash => "flash",
            Self::Preliminary => "preliminary",
            Self::Second => "second",
            Self::Third => "third",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ReleaseStage {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ReleaseStage {}
impl std::hash::Hash for ReleaseStage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ReleaseStage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ReleaseStage {
    fn from(value: &str) -> Self {
        match value {
            "advance" => Self::Advance,
            "final" => Self::Final,
            "first" => Self::First,
            "flash" => Self::Flash,
            "preliminary" => Self::Preliminary,
            "second" => Self::Second,
            "third" => Self::Third,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ReleaseStage {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ReleaseStage {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ReleaseStatus {
    Cancelled,
    Published,
    Scheduled,
    Unknown(String),
}
impl ReleaseStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cancelled => "cancelled",
            Self::Published => "published",
            Self::Scheduled => "scheduled",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ReleaseStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ReleaseStatus {}
impl std::hash::Hash for ReleaseStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ReleaseStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ReleaseStatus {
    fn from(value: &str) -> Self {
        match value {
            "cancelled" => Self::Cancelled,
            "published" => Self::Published,
            "scheduled" => Self::Scheduled,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ReleaseStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ReleaseStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ReleaseTopic {
    BalanceOfPayments,
    BusinessConfidence,
    Construction,
    ConsumerConfidence,
    ConsumerPrices,
    Employment,
    Energy,
    Gdp,
    HomeSales,
    HousePrices,
    HouseholdSpending,
    HousingConstruction,
    ImportExportPrices,
    IndustrialProduction,
    Investment,
    JoblessClaims,
    LabourForce,
    Manufacturing,
    MerchandiseTrade,
    MoneyCredit,
    Other,
    PolicyCommunication,
    PolicyMinutes,
    PolicyRate,
    ProducerPrices,
    PublicFinance,
    RetailSales,
    Services,
    Tourism,
    Wages,
    WholesaleTrade,
    Unknown(String),
}
impl ReleaseTopic {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BalanceOfPayments => "balance_of_payments",
            Self::BusinessConfidence => "business_confidence",
            Self::Construction => "construction",
            Self::ConsumerConfidence => "consumer_confidence",
            Self::ConsumerPrices => "consumer_prices",
            Self::Employment => "employment",
            Self::Energy => "energy",
            Self::Gdp => "gdp",
            Self::HomeSales => "home_sales",
            Self::HousePrices => "house_prices",
            Self::HouseholdSpending => "household_spending",
            Self::HousingConstruction => "housing_construction",
            Self::ImportExportPrices => "import_export_prices",
            Self::IndustrialProduction => "industrial_production",
            Self::Investment => "investment",
            Self::JoblessClaims => "jobless_claims",
            Self::LabourForce => "labour_force",
            Self::Manufacturing => "manufacturing",
            Self::MerchandiseTrade => "merchandise_trade",
            Self::MoneyCredit => "money_credit",
            Self::Other => "other",
            Self::PolicyCommunication => "policy_communication",
            Self::PolicyMinutes => "policy_minutes",
            Self::PolicyRate => "policy_rate",
            Self::ProducerPrices => "producer_prices",
            Self::PublicFinance => "public_finance",
            Self::RetailSales => "retail_sales",
            Self::Services => "services",
            Self::Tourism => "tourism",
            Self::Wages => "wages",
            Self::WholesaleTrade => "wholesale_trade",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ReleaseTopic {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ReleaseTopic {}
impl std::hash::Hash for ReleaseTopic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ReleaseTopic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ReleaseTopic {
    fn from(value: &str) -> Self {
        match value {
            "balance_of_payments" => Self::BalanceOfPayments,
            "business_confidence" => Self::BusinessConfidence,
            "construction" => Self::Construction,
            "consumer_confidence" => Self::ConsumerConfidence,
            "consumer_prices" => Self::ConsumerPrices,
            "employment" => Self::Employment,
            "energy" => Self::Energy,
            "gdp" => Self::Gdp,
            "home_sales" => Self::HomeSales,
            "house_prices" => Self::HousePrices,
            "household_spending" => Self::HouseholdSpending,
            "housing_construction" => Self::HousingConstruction,
            "import_export_prices" => Self::ImportExportPrices,
            "industrial_production" => Self::IndustrialProduction,
            "investment" => Self::Investment,
            "jobless_claims" => Self::JoblessClaims,
            "labour_force" => Self::LabourForce,
            "manufacturing" => Self::Manufacturing,
            "merchandise_trade" => Self::MerchandiseTrade,
            "money_credit" => Self::MoneyCredit,
            "other" => Self::Other,
            "policy_communication" => Self::PolicyCommunication,
            "policy_minutes" => Self::PolicyMinutes,
            "policy_rate" => Self::PolicyRate,
            "producer_prices" => Self::ProducerPrices,
            "public_finance" => Self::PublicFinance,
            "retail_sales" => Self::RetailSales,
            "services" => Self::Services,
            "tourism" => Self::Tourism,
            "wages" => Self::Wages,
            "wholesale_trade" => Self::WholesaleTrade,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ReleaseTopic {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ReleaseTopic {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ResolutionKind {
    Occurrence,
    PriceFix,
    PriceWindow,
    Release,
    Unknown(String),
}
impl ResolutionKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Occurrence => "occurrence",
            Self::PriceFix => "price_fix",
            Self::PriceWindow => "price_window",
            Self::Release => "release",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ResolutionKind {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ResolutionKind {}
impl std::hash::Hash for ResolutionKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ResolutionKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ResolutionKind {
    fn from(value: &str) -> Self {
        match value {
            "occurrence" => Self::Occurrence,
            "price_fix" => Self::PriceFix,
            "price_window" => Self::PriceWindow,
            "release" => Self::Release,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ResolutionKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ResolutionKind {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ResourceObservationStatus {
    BreakInSeries,
    BreakInSeriesAndDefinitionDiffers,
    DefinitionDiffers,
    Estimated,
    Provisional,
    Unknown(String),
}
impl ResourceObservationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BreakInSeries => "break_in_series",
            Self::BreakInSeriesAndDefinitionDiffers => "break_in_series_and_definition_differs",
            Self::DefinitionDiffers => "definition_differs",
            Self::Estimated => "estimated",
            Self::Provisional => "provisional",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ResourceObservationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ResourceObservationStatus {}
impl std::hash::Hash for ResourceObservationStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ResourceObservationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ResourceObservationStatus {
    fn from(value: &str) -> Self {
        match value {
            "break_in_series" => Self::BreakInSeries,
            "break_in_series_and_definition_differs" => Self::BreakInSeriesAndDefinitionDiffers,
            "definition_differs" => Self::DefinitionDiffers,
            "estimated" => Self::Estimated,
            "provisional" => Self::Provisional,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ResourceObservationStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ResourceObservationStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ResourceUnit {
    Currency,
    CurrencyPerHour,
    CurrencyPerMonth,
    CurrencyPerWeek,
    CurrencyPerYear,
    Index,
    Job,
    Percent,
    Person,
    Unknown(String),
}
impl ResourceUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Currency => "currency",
            Self::CurrencyPerHour => "currency_per_hour",
            Self::CurrencyPerMonth => "currency_per_month",
            Self::CurrencyPerWeek => "currency_per_week",
            Self::CurrencyPerYear => "currency_per_year",
            Self::Index => "index",
            Self::Job => "job",
            Self::Percent => "percent",
            Self::Person => "person",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ResourceUnit {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ResourceUnit {}
impl std::hash::Hash for ResourceUnit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ResourceUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ResourceUnit {
    fn from(value: &str) -> Self {
        match value {
            "currency" => Self::Currency,
            "currency_per_hour" => Self::CurrencyPerHour,
            "currency_per_month" => Self::CurrencyPerMonth,
            "currency_per_week" => Self::CurrencyPerWeek,
            "currency_per_year" => Self::CurrencyPerYear,
            "index" => Self::Index,
            "job" => Self::Job,
            "percent" => Self::Percent,
            "person" => Self::Person,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ResourceUnit {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ResourceUnit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ScalarEventAbove {
    Median,
    P10,
    P90,
    Unknown(String),
}
impl ScalarEventAbove {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Median => "median",
            Self::P10 => "p10",
            Self::P90 => "p90",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for ScalarEventAbove {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ScalarEventAbove {}
impl std::hash::Hash for ScalarEventAbove {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for ScalarEventAbove {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for ScalarEventAbove {
    fn from(value: &str) -> Self {
        match value {
            "median" => Self::Median,
            "p10" => Self::P10,
            "p90" => Self::P90,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for ScalarEventAbove {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ScalarEventAbove {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SecurityType {
    Adr,
    Common,
    Other,
    Preferred,
    Unknown(String),
}
impl SecurityType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Adr => "adr",
            Self::Common => "common",
            Self::Other => "other",
            Self::Preferred => "preferred",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for SecurityType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for SecurityType {}
impl std::hash::Hash for SecurityType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for SecurityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for SecurityType {
    fn from(value: &str) -> Self {
        match value {
            "adr" => Self::Adr,
            "common" => Self::Common,
            "other" => Self::Other,
            "preferred" => Self::Preferred,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for SecurityType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for SecurityType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SettlementStatus {
    Disputed,
    Pending,
    Settled,
    Void,
    Unknown(String),
}
impl SettlementStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disputed => "disputed",
            Self::Pending => "pending",
            Self::Settled => "settled",
            Self::Void => "void",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for SettlementStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for SettlementStatus {}
impl std::hash::Hash for SettlementStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for SettlementStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for SettlementStatus {
    fn from(value: &str) -> Self {
        match value {
            "disputed" => Self::Disputed,
            "pending" => Self::Pending,
            "settled" => Self::Settled,
            "void" => Self::Void,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for SettlementStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for SettlementStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TimingStepKind {
    Date,
    Meeting,
    Unknown(String),
}
impl TimingStepKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Date => "date",
            Self::Meeting => "meeting",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for TimingStepKind {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for TimingStepKind {}
impl std::hash::Hash for TimingStepKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for TimingStepKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for TimingStepKind {
    fn from(value: &str) -> Self {
        match value {
            "date" => Self::Date,
            "meeting" => Self::Meeting,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for TimingStepKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TimingStepKind {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TopicGroup {
    CentralBanks,
    Energy,
    GlobalActivity,
    GlobalPrices,
    Metals,
    UsActivity,
    UsJobs,
    UsPrices,
    Unknown(String),
}
impl TopicGroup {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CentralBanks => "central_banks",
            Self::Energy => "energy",
            Self::GlobalActivity => "global_activity",
            Self::GlobalPrices => "global_prices",
            Self::Metals => "metals",
            Self::UsActivity => "us_activity",
            Self::UsJobs => "us_jobs",
            Self::UsPrices => "us_prices",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for TopicGroup {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for TopicGroup {}
impl std::hash::Hash for TopicGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for TopicGroup {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for TopicGroup {
    fn from(value: &str) -> Self {
        match value {
            "central_banks" => Self::CentralBanks,
            "energy" => Self::Energy,
            "global_activity" => Self::GlobalActivity,
            "global_prices" => Self::GlobalPrices,
            "metals" => Self::Metals,
            "us_activity" => Self::UsActivity,
            "us_jobs" => Self::UsJobs,
            "us_prices" => Self::UsPrices,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for TopicGroup {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TopicGroup {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Venue {
    Kalshi,
    Polymarket,
    Unknown(String),
}
impl Venue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Kalshi => "kalshi",
            Self::Polymarket => "polymarket",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for Venue {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for Venue {}
impl std::hash::Hash for Venue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for Venue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for Venue {
    fn from(value: &str) -> Self {
        match value {
            "kalshi" => Self::Kalshi,
            "polymarket" => Self::Polymarket,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for Venue {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Venue {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum VenueDifference {
    OutcomeSet,
    Rules,
    Source,
    Timing,
    Unknown(String),
}
impl VenueDifference {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OutcomeSet => "outcome_set",
            Self::Rules => "rules",
            Self::Source => "source",
            Self::Timing => "timing",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for VenueDifference {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for VenueDifference {}
impl std::hash::Hash for VenueDifference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for VenueDifference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for VenueDifference {
    fn from(value: &str) -> Self {
        match value {
            "outcome_set" => Self::OutcomeSet,
            "rules" => Self::Rules,
            "source" => Self::Source,
            "timing" => Self::Timing,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for VenueDifference {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for VenueDifference {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum VenueExclusionReason {
    OutcomeSet,
    Rules,
    Source,
    Stale,
    Thin,
    Timing,
    Unknown(String),
}
impl VenueExclusionReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OutcomeSet => "outcome_set",
            Self::Rules => "rules",
            Self::Source => "source",
            Self::Stale => "stale",
            Self::Thin => "thin",
            Self::Timing => "timing",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for VenueExclusionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for VenueExclusionReason {}
impl std::hash::Hash for VenueExclusionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for VenueExclusionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for VenueExclusionReason {
    fn from(value: &str) -> Self {
        match value {
            "outcome_set" => Self::OutcomeSet,
            "rules" => Self::Rules,
            "source" => Self::Source,
            "stale" => Self::Stale,
            "thin" => Self::Thin,
            "timing" => Self::Timing,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for VenueExclusionReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for VenueExclusionReason {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum WebhookDeliveryState {
    Cancelled,
    Failed,
    Leased,
    Pending,
    Succeeded,
    Unknown(String),
}
impl WebhookDeliveryState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::Leased => "leased",
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for WebhookDeliveryState {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for WebhookDeliveryState {}
impl std::hash::Hash for WebhookDeliveryState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for WebhookDeliveryState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for WebhookDeliveryState {
    fn from(value: &str) -> Self {
        match value {
            "cancelled" => Self::Cancelled,
            "failed" => Self::Failed,
            "leased" => Self::Leased,
            "pending" => Self::Pending,
            "succeeded" => Self::Succeeded,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for WebhookDeliveryState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for WebhookDeliveryState {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum WebhookDisabledReason {
    Customer,
    PlanLimit,
    Unknown(String),
}
impl WebhookDisabledReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Customer => "customer",
            Self::PlanLimit => "plan_limit",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for WebhookDisabledReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for WebhookDisabledReason {}
impl std::hash::Hash for WebhookDisabledReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for WebhookDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for WebhookDisabledReason {
    fn from(value: &str) -> Self {
        match value {
            "customer" => Self::Customer,
            "plan_limit" => Self::PlanLimit,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for WebhookDisabledReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for WebhookDisabledReason {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum WebhookEventType {
    DatasetPublished,
    DatasetRevised,
    FilingPublished,
    HolidayCalendarRevised,
    PredictionEventClosed,
    PredictionEventSettled,
    ReleaseCancelled,
    ReleaseRescheduled,
    ReleaseScheduled,
    ReleaseUpdated,
    WebhookTest,
    YieldCurvePublished,
    YieldCurveRevised,
    Unknown(String),
}
impl WebhookEventType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DatasetPublished => "dataset.published",
            Self::DatasetRevised => "dataset.revised",
            Self::FilingPublished => "filing.published",
            Self::HolidayCalendarRevised => "holiday.calendar.revised",
            Self::PredictionEventClosed => "prediction.event.closed",
            Self::PredictionEventSettled => "prediction.event.settled",
            Self::ReleaseCancelled => "release.cancelled",
            Self::ReleaseRescheduled => "release.rescheduled",
            Self::ReleaseScheduled => "release.scheduled",
            Self::ReleaseUpdated => "release.updated",
            Self::WebhookTest => "webhook.test",
            Self::YieldCurvePublished => "yield_curve.published",
            Self::YieldCurveRevised => "yield_curve.revised",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for WebhookEventType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for WebhookEventType {}
impl std::hash::Hash for WebhookEventType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for WebhookEventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for WebhookEventType {
    fn from(value: &str) -> Self {
        match value {
            "dataset.published" => Self::DatasetPublished,
            "dataset.revised" => Self::DatasetRevised,
            "filing.published" => Self::FilingPublished,
            "holiday.calendar.revised" => Self::HolidayCalendarRevised,
            "prediction.event.closed" => Self::PredictionEventClosed,
            "prediction.event.settled" => Self::PredictionEventSettled,
            "release.cancelled" => Self::ReleaseCancelled,
            "release.rescheduled" => Self::ReleaseRescheduled,
            "release.scheduled" => Self::ReleaseScheduled,
            "release.updated" => Self::ReleaseUpdated,
            "webhook.test" => Self::WebhookTest,
            "yield_curve.published" => Self::YieldCurvePublished,
            "yield_curve.revised" => Self::YieldCurveRevised,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for WebhookEventType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for WebhookEventType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum WebhookFamily {
    Activity,
    Filings,
    Gdp,
    Holidays,
    Housing,
    Labour,
    Money,
    PolicyRate,
    Predictions,
    Prices,
    PublicFinance,
    Survey,
    Trade,
    YieldCurve,
    Unknown(String),
}
impl WebhookFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Activity => "activity",
            Self::Filings => "filings",
            Self::Gdp => "gdp",
            Self::Holidays => "holidays",
            Self::Housing => "housing",
            Self::Labour => "labour",
            Self::Money => "money",
            Self::PolicyRate => "policy_rate",
            Self::Predictions => "predictions",
            Self::Prices => "prices",
            Self::PublicFinance => "public_finance",
            Self::Survey => "survey",
            Self::Trade => "trade",
            Self::YieldCurve => "yield_curve",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for WebhookFamily {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for WebhookFamily {}
impl std::hash::Hash for WebhookFamily {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for WebhookFamily {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for WebhookFamily {
    fn from(value: &str) -> Self {
        match value {
            "activity" => Self::Activity,
            "filings" => Self::Filings,
            "gdp" => Self::Gdp,
            "holidays" => Self::Holidays,
            "housing" => Self::Housing,
            "labour" => Self::Labour,
            "money" => Self::Money,
            "policy_rate" => Self::PolicyRate,
            "predictions" => Self::Predictions,
            "prices" => Self::Prices,
            "public_finance" => Self::PublicFinance,
            "survey" => Self::Survey,
            "trade" => Self::Trade,
            "yield_curve" => Self::YieldCurve,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for WebhookFamily {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for WebhookFamily {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum WebhookTestStatus {
    Disabled,
    Queued,
    RateLimited,
    Unknown(String),
}
impl WebhookTestStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disabled => "disabled",
            Self::Queued => "queued",
            Self::RateLimited => "rate_limited",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for WebhookTestStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for WebhookTestStatus {}
impl std::hash::Hash for WebhookTestStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for WebhookTestStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for WebhookTestStatus {
    fn from(value: &str) -> Self {
        match value {
            "disabled" => Self::Disabled,
            "queued" => Self::Queued,
            "rate_limited" => Self::RateLimited,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for WebhookTestStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for WebhookTestStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum YieldCurveType {
    ConstantMaturityYtm,
    NominalPar,
    NominalZeroCoupon,
    SpotSvensson,
    ZeroCoupon,
    Unknown(String),
}
impl YieldCurveType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ConstantMaturityYtm => "constant_maturity_ytm",
            Self::NominalPar => "nominal_par",
            Self::NominalZeroCoupon => "nominal_zero_coupon",
            Self::SpotSvensson => "spot_svensson",
            Self::ZeroCoupon => "zero_coupon",
            Self::Unknown(v) => v,
        }
    }
}
impl PartialEq for YieldCurveType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for YieldCurveType {}
impl std::hash::Hash for YieldCurveType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl AsRef<str> for YieldCurveType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for YieldCurveType {
    fn from(value: &str) -> Self {
        match value {
            "constant_maturity_ytm" => Self::ConstantMaturityYtm,
            "nominal_par" => Self::NominalPar,
            "nominal_zero_coupon" => Self::NominalZeroCoupon,
            "spot_svensson" => Self::SpotSvensson,
            "zero_coupon" => Self::ZeroCoupon,
            v => Self::Unknown(v.to_owned()),
        }
    }
}
impl Serialize for YieldCurveType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for YieldCurveType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(d)?.as_str()))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AsReportedConcept {
    #[serde(rename = "concept")]
    pub concept: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "taxonomy")]
    pub taxonomy: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "values")]
    pub values: Vec<AsReportedValue>,
}
impl Model for AsReportedConcept {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.values.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AsReportedPage {
    #[serde(rename = "concepts")]
    pub concepts: Vec<AsReportedConcept>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for AsReportedPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.concepts.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AsReportedValue {
    #[serde(rename = "end")]
    pub end: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(rename = "value")]
    pub value: Decimal,
}
impl Model for AsReportedValue {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BinaryEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "labels")]
    pub labels: Option<BinaryLabels>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "probability")]
    pub probability: Option<String>,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "quotes")]
    pub quotes: Vec<Quote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_with")]
    pub resolves_with: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "result")]
    pub result: Option<PredictionResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(rename = "status")]
    pub status: PredictionEventStatus,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(rename = "venues")]
    pub venues: Vec<PredictionVenue>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for BinaryEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.labels {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.quotes.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.result {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.venues.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BinaryLabels {
    #[serde(rename = "no")]
    pub no: String,
    #[serde(rename = "yes")]
    pub yes: String,
}
impl Model for BinaryLabels {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarCoverage {
    #[serde(rename = "scope")]
    pub scope: HolidayScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subdivision_codes")]
    pub subdivision_codes: Option<Vec<String>>,
}
impl Model for CalendarCoverage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarHoliday {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "local_name")]
    pub local_name: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "observed")]
    pub observed: Option<bool>,
    #[serde(rename = "scope")]
    pub scope: HolidayScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subdivision_codes")]
    pub subdivision_codes: Option<Vec<String>>,
}
impl Model for CalendarHoliday {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarHolidays {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "coverage")]
    pub coverage: Vec<CalendarCoverage>,
    #[serde(rename = "holidays")]
    pub holidays: Vec<CalendarHoliday>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CalendarHolidays {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.coverage.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.holidays.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarHolidaysOnDate {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "coverage")]
    pub coverage: Vec<CalendarCoverage>,
    #[serde(rename = "holidays")]
    pub holidays: Vec<CalendarHoliday>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CalendarHolidaysOnDate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.coverage.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.holidays.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarNextHoliday {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "coverage")]
    pub coverage: Vec<CalendarCoverage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday")]
    pub holiday: Option<CalendarHoliday>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CalendarNextHoliday {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.coverage.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.holiday {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarSubscriptionCreated {
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "subscription_id")]
    pub subscription_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CalendarSubscriptionCreated {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarSubscriptionInput {
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "families")]
    pub families: Option<Vec<EconomicFamily>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include_holidays")]
    pub include_holidays: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include_releases")]
    pub include_releases: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    #[serde(rename = "start_date")]
    pub start_date: String,
}
impl Model for CalendarSubscriptionInput {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CatalogueBulkLatest {
    #[serde(rename = "missing")]
    pub missing: Vec<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<CatalogueLatest>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CatalogueBulkLatest {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CatalogueEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_period")]
    pub first_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frequency")]
    pub frequency: Option<CatalogueLatestFrequency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id_template")]
    pub id_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_period")]
    pub last_period: Option<String>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit_label")]
    pub unit_label: Option<String>,
}
impl Model for CatalogueEntry {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CatalogueHistory {
    #[serde(rename = "attribution")]
    pub attribution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "base_period")]
    pub base_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "fetched_at")]
    pub fetched_at: String,
    #[serde(rename = "frequency")]
    pub frequency: CatalogueLatestFrequency,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "licence_url")]
    pub licence_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<ResourceObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scale")]
    pub scale: Option<String>,
    #[serde(rename = "seasonal_adjustment")]
    pub seasonal_adjustment: CatalogueLatestSeasonalAdjustment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "unit_label")]
    pub unit_label: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CatalogueHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CatalogueLatest {
    #[serde(rename = "attribution")]
    pub attribution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "base_period")]
    pub base_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "fetched_at")]
    pub fetched_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(rename = "frequency")]
    pub frequency: CatalogueLatestFrequency,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "licence_url")]
    pub licence_url: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "released_on")]
    pub released_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scale")]
    pub scale: Option<String>,
    #[serde(rename = "seasonal_adjustment")]
    pub seasonal_adjustment: CatalogueLatestSeasonalAdjustment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<ResourceObservationStatus>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "unit_label")]
    pub unit_label: String,
    #[serde(rename = "value")]
    pub value: Decimal,
}
impl Model for CatalogueLatest {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CatalogueSearchPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<CatalogueEntry>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CatalogueSearchPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.results.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CatalogueSeries {
    #[serde(rename = "attribution")]
    pub attribution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "base_period")]
    pub base_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "fetched_at")]
    pub fetched_at: String,
    #[serde(rename = "first_period")]
    pub first_period: String,
    #[serde(rename = "frequency")]
    pub frequency: CatalogueLatestFrequency,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_period")]
    pub last_period: String,
    #[serde(rename = "licence_url")]
    pub licence_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scale")]
    pub scale: Option<String>,
    #[serde(rename = "seasonal_adjustment")]
    pub seasonal_adjustment: CatalogueLatestSeasonalAdjustment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "unit_label")]
    pub unit_label: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CatalogueSeries {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CentralBankBalanceSheet {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(rename = "scale")]
    pub scale: CentralBankBalanceSheetScale,
    #[serde(rename = "value")]
    pub value: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CentralBankBalanceSheet {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CentralBankReserves {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(rename = "scale")]
    pub scale: CentralBankBalanceSheetScale,
    #[serde(rename = "value")]
    pub value: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CentralBankReserves {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommodityBar {
    #[serde(rename = "close")]
    pub close: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "high")]
    pub high: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "low")]
    pub low: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "open")]
    pub open: Option<Decimal>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}
impl Model for CommodityBar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommodityBars {
    #[serde(rename = "bars")]
    pub bars: Vec<CommodityBar>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "unit")]
    pub unit: CommodityUnit,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CommodityBars {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.bars.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommodityOilPalmFfbAgePrice {
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "tree_age_max_years")]
    pub tree_age_max_years: i64,
    #[serde(rename = "tree_age_min_years")]
    pub tree_age_min_years: i64,
}
impl Model for CommodityOilPalmFfbAgePrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommoditySpot {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "unit")]
    pub unit: CommodityUnit,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CommoditySpot {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanyFilingsPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filed_from")]
    pub filed_from: Option<String>,
    #[serde(rename = "filings")]
    pub filings: Vec<ListedFiling>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CompanyFilingsPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.filings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanyIndustry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "industry")]
    pub industry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isic")]
    pub isic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sector")]
    pub sector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source_code")]
    pub source_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source_scheme")]
    pub source_scheme: Option<String>,
}
impl Model for CompanyIndustry {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanyNameSpan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "from")]
    pub from: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "to")]
    pub to: Option<String>,
}
impl Model for CompanyNameSpan {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanyRelation {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind")]
    pub kind: CompanyRelationKind,
}
impl Model for CompanyRelation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanySearchHit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delisted")]
    pub delisted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "former_code")]
    pub former_code: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "former_name")]
    pub former_name: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matched")]
    pub matched: Option<CompanySearchMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mic")]
    pub mic: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}
impl Model for CompanySearchHit {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanySearchPage {
    #[serde(rename = "companies")]
    pub companies: Vec<CompanySearchHit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coverage")]
    pub coverage: Option<Vec<RegistryCoverage>>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CompanySearchPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.companies.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.coverage {
            for v in v.iter_mut() {
                v.attach(ResponseMeta::default(), _raw);
            }
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanySecurity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "adr_ratio")]
    pub adr_ratio: Option<Decimal>,
    #[serde(rename = "class")]
    pub class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "figi")]
    pub figi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isin")]
    pub isin: Option<String>,
    #[serde(rename = "listings")]
    pub listings: Vec<SecurityListing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "primary")]
    pub primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: SecurityType,
}
impl Model for CompanySecurity {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.listings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompanyWithListings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delisted")]
    pub delisted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fiscal_year_end")]
    pub fiscal_year_end: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "industry")]
    pub industry: Option<CompanyIndustry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jp_corporate_number")]
    pub jp_corporate_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lei")]
    pub lei: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "listings")]
    pub listings: Option<Vec<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "names")]
    pub names: Vec<CompanyNameSpan>,
    #[serde(rename = "related")]
    pub related: Vec<CompanyRelation>,
    #[serde(rename = "securities")]
    pub securities: Vec<CompanySecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "standard")]
    pub standard: Option<CompanyStandard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "standard_variant")]
    pub standard_variant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status_date")]
    pub status_date: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CompanyWithListings {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.industry {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.names.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.related.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.securities.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostBenchmark {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "default")]
    pub default: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
}
impl Model for CostBenchmark {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostBeta {
    #[serde(rename = "adjusted")]
    pub adjusted: Decimal,
    #[serde(rename = "benchmark")]
    pub benchmark: CostBenchmark,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "frequency")]
    pub frequency: CostOfCapitalFrequency,
    #[serde(rename = "observations")]
    pub observations: i64,
    #[serde(rename = "r_squared")]
    pub r_squared: Decimal,
    #[serde(rename = "raw")]
    pub raw: Decimal,
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "std_error")]
    pub std_error: Decimal,
    #[serde(rename = "used")]
    pub used: CostOfCapitalBeta,
    #[serde(rename = "window")]
    pub window: CostOfCapitalWindow,
}
impl Model for CostBeta {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.benchmark.attach(ResponseMeta::default(), _raw);
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostEquityRiskPremium {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "basis")]
    pub basis: Option<CostEquityRiskPremiumBasis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "source")]
    pub source: CostEquityRiskPremiumSource,
}
impl Model for CostEquityRiskPremium {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostOfDebt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "after_tax")]
    pub after_tax: Option<Decimal>,
    #[serde(rename = "average_debt")]
    pub average_debt: Decimal,
    #[serde(rename = "basis")]
    pub basis: CostOfDebtBasis,
    #[serde(rename = "closing_debt")]
    pub closing_debt: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "default_spread")]
    pub default_spread: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "interest_coverage")]
    pub interest_coverage: Option<Decimal>,
    #[serde(rename = "interest_expense")]
    pub interest_expense: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "opening_debt")]
    pub opening_debt: Option<Decimal>,
    #[serde(rename = "period")]
    pub period: CostOfCapitalPeriod,
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "pre_tax")]
    pub pre_tax: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table_update")]
    pub table_update: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table_url")]
    pub table_url: Option<String>,
}
impl Model for CostOfDebt {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostRiskFree {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "basis")]
    pub basis: CostRiskFreeBasis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "curve_type")]
    pub curve_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maturity")]
    pub maturity: Option<String>,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "source")]
    pub source: CostRiskFreeSource,
}
impl Model for CostRiskFree {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostTaxRate {
    #[serde(rename = "basis")]
    pub basis: CostTaxRateBasis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "source")]
    pub source: CostTaxRateSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table_update")]
    pub table_update: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table_url")]
    pub table_url: Option<String>,
}
impl Model for CostTaxRate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostWarning {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
}
impl Model for CostWarning {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostWeights {
    #[serde(rename = "debt")]
    pub debt: Decimal,
    #[serde(rename = "equity")]
    pub equity: Decimal,
    #[serde(rename = "market_cap")]
    pub market_cap: Decimal,
    #[serde(rename = "market_cap_as_of")]
    pub market_cap_as_of: String,
    #[serde(rename = "total_debt")]
    pub total_debt: Decimal,
    #[serde(rename = "total_debt_as_of")]
    pub total_debt_as_of: String,
}
impl Model for CostWeights {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cpi {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "index_base")]
    pub index_base: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(rename = "value")]
    pub value: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for Cpi {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CpiHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "index_base")]
    pub index_base: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<CpiHistoryObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CpiHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CpiHistoryObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "value")]
    pub value: Decimal,
}
impl Model for CpiHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoBar {
    #[serde(rename = "close")]
    pub close: Decimal,
    #[serde(rename = "high")]
    pub high: Decimal,
    #[serde(rename = "low")]
    pub low: Decimal,
    #[serde(rename = "open")]
    pub open: Decimal,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}
impl Model for CryptoBar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoBars {
    #[serde(rename = "bars")]
    pub bars: Vec<CryptoBar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CryptoBars {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.bars.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoPair {
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "quote")]
    pub quote: String,
}
impl Model for CryptoPair {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoPairs {
    #[serde(rename = "pairs")]
    pub pairs: Vec<CryptoPair>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CryptoPairs {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.pairs.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoPrice {
    #[serde(rename = "base")]
    pub base: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change_24h")]
    pub change_24h: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change_24h_percent")]
    pub change_24h_percent: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delayed")]
    pub delayed: Option<bool>,
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "quote")]
    pub quote: String,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CryptoPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoPriceError {
    #[serde(rename = "code")]
    pub code: CryptoPriceErrorCode,
    #[serde(rename = "id")]
    pub id: String,
}
impl Model for CryptoPriceError {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CryptoPrices {
    #[serde(rename = "errors")]
    pub errors: Vec<CryptoPriceError>,
    #[serde(rename = "prices")]
    pub prices: Vec<CryptoPrice>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for CryptoPrices {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.errors.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.prices.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeliveryEvent {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "family")]
    pub family: DeliveryFamily,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday_generation")]
    pub holiday_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday_year")]
    pub holiday_year: Option<i64>,
    #[serde(rename = "recorded_at")]
    pub recorded_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resource_version")]
    pub resource_version: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl Model for DeliveryEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DividendsCalendar {
    #[serde(rename = "entries")]
    pub entries: Vec<DividendsCalendarEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for DividendsCalendar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.entries.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DividendsCalendarEntry {
    #[serde(rename = "amount")]
    pub amount: Decimal,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "ex_date")]
    pub ex_date: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
}
impl Model for DividendsCalendarEntry {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EconomicCalendar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "releases")]
    pub releases: Vec<EconomicReleaseSummary>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EconomicCalendar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.releases.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EconomicReleaseDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "at")]
    pub at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "between")]
    pub between: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "by")]
    pub by: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "date")]
    pub date: Option<String>,
    #[serde(rename = "family")]
    pub family: EconomicFamily,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notice")]
    pub notice: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "observed_at")]
    pub observed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "period")]
    pub period: Option<String>,
    #[serde(rename = "release_id")]
    pub release_id: String,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "series_id")]
    pub series_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stage")]
    pub stage: Option<ReleaseStage>,
    #[serde(rename = "status")]
    pub status: ReleaseStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topic")]
    pub topic: Option<ReleaseTopic>,
    #[serde(rename = "values")]
    pub values: Vec<EconomicReleaseValue>,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EconomicReleaseDetail {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.values.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EconomicReleaseSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "at")]
    pub at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "between")]
    pub between: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "by")]
    pub by: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "date")]
    pub date: Option<String>,
    #[serde(rename = "family")]
    pub family: EconomicFamily,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notice")]
    pub notice: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "period")]
    pub period: Option<String>,
    #[serde(rename = "release_id")]
    pub release_id: String,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "series_id")]
    pub series_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stage")]
    pub stage: Option<ReleaseStage>,
    #[serde(rename = "status")]
    pub status: ReleaseStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topic")]
    pub topic: Option<ReleaseTopic>,
    #[serde(rename = "values")]
    pub values: Vec<EconomicReleaseValue>,
}
impl Model for EconomicReleaseSummary {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.values.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EconomicReleaseValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actual")]
    pub actual: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualized")]
    pub annualized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "index_base")]
    pub index_base: Option<String>,
    #[serde(rename = "measure")]
    pub measure: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_seasonally_adjusted")]
    pub not_seasonally_adjusted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "period")]
    pub period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous")]
    pub previous: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous_revised_from")]
    pub previous_revised_from: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scale")]
    pub scale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transformation")]
    pub transformation: Option<String>,
    #[serde(rename = "unit")]
    pub unit: IndicatorUnit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value_status")]
    pub value_status: Option<EconomicReleaseValueValueStatus>,
}
impl Model for EconomicReleaseValue {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityBar {
    #[serde(rename = "close")]
    pub close: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "date")]
    pub date: Option<String>,
    #[serde(rename = "high")]
    pub high: Decimal,
    #[serde(rename = "low")]
    pub low: Decimal,
    #[serde(rename = "open")]
    pub open: Decimal,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "volume")]
    pub volume: Option<Decimal>,
}
impl Model for EquityBar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityBars {
    #[serde(rename = "adjusted")]
    pub adjusted: bool,
    #[serde(rename = "bars")]
    pub bars: Vec<EquityBar>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end")]
    pub end: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityBars {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.bars.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityCostOfCapital {
    #[serde(rename = "beta")]
    pub beta: CostBeta,
    #[serde(rename = "company_id")]
    pub company_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cost_of_debt")]
    pub cost_of_debt: Option<CostOfDebt>,
    #[serde(rename = "cost_of_equity")]
    pub cost_of_equity: Decimal,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "equity_risk_premium")]
    pub equity_risk_premium: CostEquityRiskPremium,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "methodology_url")]
    pub methodology_url: String,
    #[serde(rename = "risk_free")]
    pub risk_free: CostRiskFree,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tax_rate")]
    pub tax_rate: Option<CostTaxRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wacc")]
    pub wacc: Option<Decimal>,
    #[serde(rename = "warnings")]
    pub warnings: Vec<CostWarning>,
    #[serde(rename = "weights")]
    pub weights: CostWeights,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityCostOfCapital {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        self.beta.attach(ResponseMeta::default(), _raw);
        if let Some(v) = &mut self.cost_of_debt {
            v.attach(ResponseMeta::default(), _raw);
        }
        self.equity_risk_premium
            .attach(ResponseMeta::default(), _raw);
        self.risk_free.attach(ResponseMeta::default(), _raw);
        if let Some(v) = &mut self.tax_rate {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.warnings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        self.weights.attach(ResponseMeta::default(), _raw);
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityDividend {
    #[serde(rename = "amount")]
    pub amount: Decimal,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "ex_date")]
    pub ex_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pay_date")]
    pub pay_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "record_date")]
    pub record_date: Option<String>,
}
impl Model for EquityDividend {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityDividends {
    #[serde(rename = "dividends")]
    pub dividends: Vec<EquityDividend>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityDividends {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.dividends.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityEarning {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eps_actual")]
    pub eps_actual: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eps_estimate")]
    pub eps_estimate: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filing_id")]
    pub filing_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "surprise")]
    pub surprise: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "surprise_percent")]
    pub surprise_percent: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "time")]
    pub time: Option<EquityEarningTime>,
}
impl Model for EquityEarning {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityEarnings {
    #[serde(rename = "earnings")]
    pub earnings: Vec<EquityEarning>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityEarnings {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.earnings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityMover {
    #[serde(rename = "change_percent")]
    pub change_percent: Decimal,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "volume")]
    pub volume: Decimal,
}
impl Model for EquityMover {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityMovers {
    #[serde(rename = "direction")]
    pub direction: MoversDirection,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "movers")]
    pub movers: Vec<EquityMover>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityMovers {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.movers.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityPrice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change")]
    pub change: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change_percent")]
    pub change_percent: Option<Decimal>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delayed")]
    pub delayed: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_session")]
    pub market_session: EquityMarketSession,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous_close")]
    pub previous_close: Option<Decimal>,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "volume")]
    pub volume: Option<Decimal>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityPriceError {
    #[serde(rename = "code")]
    pub code: CryptoPriceErrorCode,
    #[serde(rename = "id")]
    pub id: String,
}
impl Model for EquityPriceError {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityPrices {
    #[serde(rename = "errors")]
    pub errors: Vec<EquityPriceError>,
    #[serde(rename = "prices")]
    pub prices: Vec<EquityPrice>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityPrices {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.errors.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.prices.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "company_id")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "industry")]
    pub industry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    #[serde(rename = "mic")]
    pub mic: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sector")]
    pub sector: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_: EquityType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "website")]
    pub website: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityProfile {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityRiskPremium {
    #[serde(rename = "as_of")]
    pub as_of: String,
    #[serde(rename = "basis")]
    pub basis: EquityRiskPremiumBasis,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "country_default_spread")]
    pub country_default_spread: Decimal,
    #[serde(rename = "country_risk_premium")]
    pub country_risk_premium: Decimal,
    #[serde(rename = "mature_erp")]
    pub mature_erp: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "methodology_url")]
    pub methodology_url: Option<String>,
    #[serde(rename = "total_erp")]
    pub total_erp: Decimal,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "volatility_ratio")]
    pub volatility_ratio: Option<Decimal>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityRiskPremium {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        self.basis.attach(ResponseMeta::default(), _raw);
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityRiskPremiumBasis {
    #[serde(rename = "country_default_spread")]
    pub country_default_spread: CostEquityRiskPremiumBasis,
    #[serde(rename = "country_risk_premium")]
    pub country_risk_premium: CostEquityRiskPremiumBasis,
    #[serde(rename = "mature_erp")]
    pub mature_erp: CostEquityRiskPremiumBasis,
    #[serde(rename = "total_erp")]
    pub total_erp: CostEquityRiskPremiumBasis,
    #[serde(rename = "volatility_ratio")]
    pub volatility_ratio: CostEquityRiskPremiumBasis,
}
impl Model for EquityRiskPremiumBasis {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityRiskPremiumHistoryResponse {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<EquityRiskPremium>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityRiskPremiumHistoryResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityRiskPremiumList {
    #[serde(rename = "countries")]
    pub countries: Vec<EquityRiskPremium>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityRiskPremiumList {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.countries.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquitySearchResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "company_id")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "mic")]
    pub mic: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_: EquityType,
}
impl Model for EquitySearchResult {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquitySearchResults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<EquitySearchResult>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquitySearchResults {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.results.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquitySplit {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "ratio")]
    pub ratio: String,
}
impl Model for EquitySplit {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquitySplits {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "splits")]
    pub splits: Vec<EquitySplit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquitySplits {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.splits.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityValuation {
    #[serde(rename = "company_id")]
    pub company_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dividend_per_share_ttm")]
    pub dividend_per_share_ttm: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dividend_yield")]
    pub dividend_yield: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dividends_period_end")]
    pub dividends_period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eps_period_end")]
    pub eps_period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eps_ttm")]
    pub eps_ttm: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "equity_as_of")]
    pub equity_as_of: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "market_cap")]
    pub market_cap: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pb_ratio")]
    pub pb_ratio: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pe_ratio")]
    pub pe_ratio: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ps_ratio")]
    pub ps_ratio: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "revenue_period_end")]
    pub revenue_period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "revenue_ttm")]
    pub revenue_ttm: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shares_as_of")]
    pub shares_as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shares_outstanding")]
    pub shares_outstanding: Option<Decimal>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "total_equity")]
    pub total_equity: Option<Decimal>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityValuation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityValuationHistory {
    #[serde(rename = "company_id")]
    pub company_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "points")]
    pub points: Vec<EquityValuationPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EquityValuationHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.points.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityValuationPoint {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dividend_yield")]
    pub dividend_yield: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "market_cap")]
    pub market_cap: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pb_ratio")]
    pub pb_ratio: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pe_ratio")]
    pub pe_ratio: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "price")]
    pub price: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "price_date")]
    pub price_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ps_ratio")]
    pub ps_ratio: Option<Decimal>,
}
impl Model for EquityValuationPoint {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventDetail {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "family")]
    pub family: DeliveryFamily,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday_generation")]
    pub holiday_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday_year")]
    pub holiday_year: Option<i64>,
    #[serde(rename = "recorded_at")]
    pub recorded_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resource_version")]
    pub resource_version: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EventDetail {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventsPage {
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(rename = "events")]
    pub events: Vec<DeliveryEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for EventsPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.events.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExtremeEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "down")]
    pub down: Vec<ExtremeLevel>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "observation")]
    pub observation: ExtremeObservation,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_with")]
    pub resolves_with: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "result")]
    pub result: Option<PredictionResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(rename = "status")]
    pub status: PredictionEventStatus,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(rename = "up")]
    pub up: Vec<ExtremeLevel>,
    #[serde(rename = "venues")]
    pub venues: Vec<PredictionVenue>,
    #[serde(rename = "window")]
    pub window: ExtremeWindow,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for ExtremeEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.down.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.result {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.up.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.venues.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        self.window.attach(ResponseMeta::default(), _raw);
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExtremeLevel {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "level")]
    pub level: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "probability")]
    pub probability: Option<String>,
    #[serde(rename = "quotes")]
    pub quotes: Vec<Quote>,
}
impl Model for ExtremeLevel {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.quotes.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExtremeWindow {
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "start")]
    pub start: String,
}
impl Model for ExtremeWindow {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Filing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amends")]
    pub amends: Option<String>,
    #[serde(rename = "company_id")]
    pub company_id: String,
    #[serde(rename = "filed_at")]
    pub filed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filer_id")]
    pub filer_id: Option<String>,
    #[serde(rename = "form")]
    pub form: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fp")]
    pub fp: Option<FiscalPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fy")]
    pub fy: Option<i64>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: FilingType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "withdrawn")]
    pub withdrawn: Option<bool>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for Filing {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilingWebhookEvent {
    #[serde(rename = "company_id")]
    pub company_id: String,
    #[serde(rename = "filed_at")]
    pub filed_at: String,
    #[serde(rename = "filing_id")]
    pub filing_id: String,
    #[serde(rename = "form")]
    pub form: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fp")]
    pub fp: Option<FiscalPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fy")]
    pub fy: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subject_id")]
    pub subject_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: FilingEventType,
}
impl Model for FilingWebhookEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FinancialRow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "derived")]
    pub derived: Option<Vec<String>>,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fh")]
    pub fh: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fq")]
    pub fq: Option<i64>,
    #[serde(rename = "fy")]
    pub fy: i64,
    #[serde(flatten, deserialize_with = "crate::meta::decimal_items")]
    pub items: std::collections::BTreeMap<String, Decimal>,
}
impl Model for FinancialRow {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Financials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annual")]
    pub annual: Option<Vec<FinancialRow>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "basis")]
    pub basis: Option<FinancialsBasis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "from_fy")]
    pub from_fy: Option<i64>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quarterly")]
    pub quarterly: Option<Vec<FinancialRow>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semiannual")]
    pub semiannual: Option<Vec<FinancialRow>>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for Financials {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.annual {
            for v in v.iter_mut() {
                v.attach(ResponseMeta::default(), _raw);
            }
        }
        if let Some(v) = &mut self.quarterly {
            for v in v.iter_mut() {
                v.attach(ResponseMeta::default(), _raw);
            }
        }
        if let Some(v) = &mut self.semiannual {
            for v in v.iter_mut() {
                v.attach(ResponseMeta::default(), _raw);
            }
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundDistribution {
    #[serde(rename = "amount")]
    pub amount: Decimal,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "ex_date")]
    pub ex_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "kind")]
    pub kind: Option<FundDistributionKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pay_date")]
    pub pay_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "record_date")]
    pub record_date: Option<String>,
}
impl Model for FundDistribution {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundDistributions {
    #[serde(rename = "distributions")]
    pub distributions: Vec<FundDistribution>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FundDistributions {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.distributions.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundEtfPrice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change")]
    pub change: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change_percent")]
    pub change_percent: Option<Decimal>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delayed")]
    pub delayed: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_session")]
    pub market_session: EquityMarketSession,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous_close")]
    pub previous_close: Option<Decimal>,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "volume")]
    pub volume: Option<Decimal>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FundEtfPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundNavPrice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change")]
    pub change: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "change_percent")]
    pub change_percent: Option<Decimal>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "nav")]
    pub nav: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous_nav")]
    pub previous_nav: Option<Decimal>,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FundNavPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundPriceError {
    #[serde(rename = "code")]
    pub code: CryptoPriceErrorCode,
    #[serde(rename = "id")]
    pub id: String,
}
impl Model for FundPriceError {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundPrices {
    #[serde(rename = "errors")]
    pub errors: Vec<FundPriceError>,
    #[serde(rename = "prices")]
    pub prices: Vec<FundPriceItem>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FundPrices {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.errors.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.prices.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mic")]
    pub mic: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_: FundType,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FundProfile {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundSearchResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mic")]
    pub mic: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_: FundType,
}
impl Model for FundSearchResult {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundSearchResults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<FundSearchResult>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FundSearchResults {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.results.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxBar {
    #[serde(rename = "close")]
    pub close: Decimal,
    #[serde(rename = "high")]
    pub high: Decimal,
    #[serde(rename = "low")]
    pub low: Decimal,
    #[serde(rename = "open")]
    pub open: Decimal,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}
impl Model for FxBar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxBars {
    #[serde(rename = "bars")]
    pub bars: Vec<FxBar>,
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FxBars {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.bars.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxConversion {
    #[serde(rename = "converted")]
    pub converted: Decimal,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "market_session")]
    pub market_session: MarketSession,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FxConversion {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxHistoricalObservation {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "rates")]
    pub rates: Vec<FxHistoricalRate>,
}
impl Model for FxHistoricalObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.rates.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxHistoricalRate {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "rate")]
    pub rate: Decimal,
}
impl Model for FxHistoricalRate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxHistory {
    #[serde(rename = "base")]
    pub base: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<FxHistoricalObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FxHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxRate {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "stale")]
    pub stale: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}
impl Model for FxRate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FxRates {
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "market_session")]
    pub market_session: MarketSession,
    #[serde(rename = "rates")]
    pub rates: Vec<FxRate>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for FxRates {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.rates.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Gdp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualized")]
    pub annualized: Option<bool>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_seasonally_adjusted")]
    pub not_seasonally_adjusted: Option<bool>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(rename = "value")]
    pub value: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for Gdp {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GdpGrowth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualized")]
    pub annualized: Option<bool>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_seasonally_adjusted")]
    pub not_seasonally_adjusted: Option<bool>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "quarter_over_quarter")]
    pub quarter_over_quarter: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quarter_over_quarter_not_seasonally_adjusted")]
    pub quarter_over_quarter_not_seasonally_adjusted: Option<bool>,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(rename = "year_over_year")]
    pub year_over_year: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_over_year_not_seasonally_adjusted")]
    pub year_over_year_not_seasonally_adjusted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_to_date")]
    pub year_to_date: Option<Decimal>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for GdpGrowth {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GdpGrowthHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualized")]
    pub annualized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_seasonally_adjusted")]
    pub not_seasonally_adjusted: Option<bool>,
    #[serde(rename = "observations")]
    pub observations: Vec<GdpGrowthHistoryObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quarter_over_quarter_not_seasonally_adjusted")]
    pub quarter_over_quarter_not_seasonally_adjusted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_over_year_not_seasonally_adjusted")]
    pub year_over_year_not_seasonally_adjusted: Option<bool>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for GdpGrowthHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GdpGrowthHistoryObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quarter_over_quarter")]
    pub quarter_over_quarter: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_over_year")]
    pub year_over_year: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_to_date")]
    pub year_to_date: Option<Decimal>,
}
impl Model for GdpGrowthHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GdpHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualized")]
    pub annualized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_seasonally_adjusted")]
    pub not_seasonally_adjusted: Option<bool>,
    #[serde(rename = "observations")]
    pub observations: Vec<GdpHistoryObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for GdpHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GdpHistoryObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "value")]
    pub value: Decimal,
}
impl Model for GdpHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Holder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_date")]
    pub event_date: Option<String>,
    #[serde(rename = "filed")]
    pub filed: String,
    #[serde(rename = "filing_id")]
    pub filing_id: String,
    #[serde(rename = "holder")]
    pub holder: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "percent")]
    pub percent: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "security")]
    pub security: Option<HolderSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shares")]
    pub shares: Option<Decimal>,
}
impl Model for Holder {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        if let Some(v) = &mut self.security {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HolderSecurity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "class")]
    pub class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isin")]
    pub isin: Option<String>,
}
impl Model for HolderSecurity {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HoldersPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filed_from")]
    pub filed_from: Option<String>,
    #[serde(rename = "holders")]
    pub holders: Vec<Holder>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for HoldersPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.holders.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Inflation {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "month_over_month")]
    pub month_over_month: Decimal,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(rename = "year_over_year")]
    pub year_over_year: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_to_date")]
    pub year_to_date: Option<Decimal>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for Inflation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InflationHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<InflationHistoryObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for InflationHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InflationHistoryObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "month_over_month")]
    pub month_over_month: Option<Decimal>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_over_year")]
    pub year_over_year: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "year_to_date")]
    pub year_to_date: Option<Decimal>,
}
impl Model for InflationHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LabourForceParticipation {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for LabourForceParticipation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LatestFiling {
    #[serde(rename = "company_id")]
    pub company_id: String,
    #[serde(rename = "filed")]
    pub filed: String,
    #[serde(rename = "form")]
    pub form: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: FilingType,
}
impl Model for LatestFiling {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LatestFilingsPage {
    #[serde(rename = "filings")]
    pub filings: Vec<LatestFiling>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for LatestFilingsPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.filings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListedFiling {
    #[serde(rename = "filed")]
    pub filed: String,
    #[serde(rename = "form")]
    pub form: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: FilingType,
}
impl Model for ListedFiling {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NoContent {
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for NoContent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilFfbMarketHistory {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<PalmoilFfbObservation>,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilFfbMarketHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilFfbMarketPrice {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilFfbMarketPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilFfbObservation {
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "revision")]
    pub revision: i64,
}
impl Model for PalmoilFfbObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilMarketHistory {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<PalmoilMarketObservation>,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilMarketHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilMarketObservation {
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "price_month")]
    pub price_month: String,
    #[serde(rename = "revision")]
    pub revision: i64,
}
impl Model for PalmoilMarketObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilMarketPrice {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "price_month")]
    pub price_month: String,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilMarketPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScalarHistory {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<PalmoilScalarObservation>,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilScalarHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScalarObservation {
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "revision")]
    pub revision: i64,
}
impl Model for PalmoilScalarObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScalarPrice {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilScalarPrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScalarSeries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "area")]
    pub area: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "coverage_end")]
    pub coverage_end: String,
    #[serde(rename = "coverage_start")]
    pub coverage_start: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "frequency")]
    pub frequency: PalmoilFrequency,
    #[serde(rename = "observation_type")]
    pub observation_type: String,
    #[serde(rename = "period_type")]
    pub period_type: PalmoilPeriodType,
    #[serde(rename = "price_type")]
    pub price_type: PalmoilScalarPriceType,
    #[serde(rename = "product")]
    pub product: PalmoilProduct,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: PalmoilScalarUnit,
}
impl Model for PalmoilScalarSeries {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScheduleHistory {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<PalmoilScheduleObservation>,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilScheduleHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScheduleObservation {
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "prices")]
    pub prices: Vec<CommodityOilPalmFfbAgePrice>,
    #[serde(rename = "revision")]
    pub revision: i64,
}
impl Model for PalmoilScheduleObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.prices.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilSchedulePrice {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "prices")]
    pub prices: Vec<CommodityOilPalmFfbAgePrice>,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilSchedulePrice {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.prices.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilScheduleSeries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "area")]
    pub area: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "coverage_end")]
    pub coverage_end: String,
    #[serde(rename = "coverage_start")]
    pub coverage_start: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "frequency")]
    pub frequency: PalmoilFrequency,
    #[serde(rename = "observation_type")]
    pub observation_type: String,
    #[serde(rename = "period_type")]
    pub period_type: String,
    #[serde(rename = "price_type")]
    pub price_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "producer_arrangement")]
    pub producer_arrangement: Option<ProducerArrangement>,
    #[serde(rename = "product")]
    pub product: String,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
}
impl Model for PalmoilScheduleSeries {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilSeriesDiscovery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "series")]
    pub series: Vec<PalmoilSeries>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PalmoilSeriesDiscovery {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.series.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhysicalLatest {
    #[serde(rename = "attribution")]
    pub attribution: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "published_on")]
    pub published_on: Option<String>,
    #[serde(rename = "revision")]
    pub revision: i64,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "unit_quantity")]
    pub unit_quantity: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PhysicalLatest {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhysicalObservation {
    #[serde(rename = "period_end")]
    pub period_end: String,
    #[serde(rename = "period_start")]
    pub period_start: String,
    #[serde(rename = "price")]
    pub price: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "published_on")]
    pub published_on: Option<String>,
    #[serde(rename = "revision")]
    pub revision: i64,
}
impl Model for PhysicalObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhysicalObservations {
    #[serde(rename = "attribution")]
    pub attribution: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<PhysicalObservation>,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "unit_quantity")]
    pub unit_quantity: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PhysicalObservations {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhysicalSeries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "area")]
    pub area: Option<String>,
    #[serde(rename = "attribution")]
    pub attribution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "basis")]
    pub basis: Option<String>,
    #[serde(rename = "category")]
    pub category: PhysicalCategory,
    #[serde(rename = "commodity")]
    pub commodity: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "coverage_end")]
    pub coverage_end: String,
    #[serde(rename = "coverage_start")]
    pub coverage_start: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "frequency")]
    pub frequency: PhysicalFrequency,
    #[serde(rename = "price_type")]
    pub price_type: PhysicalPriceType,
    #[serde(rename = "product")]
    pub product: String,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "unit_quantity")]
    pub unit_quantity: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PhysicalSeries {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhysicalSeriesPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "series")]
    pub series: Vec<PhysicalSeries>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PhysicalSeriesPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.series.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PointPolicyRate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "announced_on")]
    pub announced_on: Option<String>,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "effective_on")]
    pub effective_on: Option<String>,
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "observed_on")]
    pub observed_on: Option<String>,
    #[serde(rename = "qualifier")]
    pub qualifier: PointPolicyRateQualifier,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "rate_type")]
    pub rate_type: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PointPolicyRate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyPath {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "meetings")]
    pub meetings: Vec<PolicyPathMeeting>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PolicyPath {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.meetings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyPathDay {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "meetings")]
    pub meetings: Vec<PolicyPathOdds>,
}
impl Model for PolicyPathDay {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.meetings.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyPathHistory {
    #[serde(rename = "days")]
    pub days: Vec<PolicyPathDay>,
    #[serde(rename = "meeting_dates")]
    pub meeting_dates: Vec<PolicyPathMeetingDate>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PolicyPathHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.days.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.meeting_dates.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyPathMeeting {
    #[serde(rename = "cumulative_change_bps")]
    pub cumulative_change_bps: Decimal,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "expected_change_bps")]
    pub expected_change_bps: Decimal,
    #[serde(rename = "p_cut")]
    pub p_cut: String,
    #[serde(rename = "p_hike")]
    pub p_hike: String,
    #[serde(rename = "p_hold")]
    pub p_hold: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
}
impl Model for PolicyPathMeeting {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyPathMeetingDate {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
}
impl Model for PolicyPathMeetingDate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyPathOdds {
    #[serde(rename = "cumulative_change_bps")]
    pub cumulative_change_bps: Decimal,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "expected_change_bps")]
    pub expected_change_bps: Decimal,
    #[serde(rename = "p_cut")]
    pub p_cut: String,
    #[serde(rename = "p_hike")]
    pub p_hike: String,
    #[serde(rename = "p_hold")]
    pub p_hold: String,
}
impl Model for PolicyPathOdds {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyRateHistoryResponse {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<PolicyRateObservation>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PolicyRateHistoryResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionBar {
    #[serde(rename = "close")]
    pub close: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "high")]
    pub high: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "low")]
    pub low: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "open")]
    pub open: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}
impl Model for PredictionBar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionEventSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "result")]
    pub result: Option<PredictionResultSummary>,
    #[serde(rename = "shape")]
    pub shape: PredictionShape,
    #[serde(rename = "status")]
    pub status: PredictionEventStatus,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
}
impl Model for PredictionEventSummary {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        if let Some(v) = &mut self.result {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionEventsPage {
    #[serde(rename = "events")]
    pub events: Vec<PredictionEventSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PredictionEventsPage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.events.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backfill_venue")]
    pub backfill_venue: Option<Venue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "series")]
    pub series: Vec<PredictionSeries>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PredictionHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.series.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amended_at")]
    pub amended_at: Option<String>,
    #[serde(rename = "final")]
    pub final_: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "settled_at")]
    pub settled_at: Option<String>,
    #[serde(rename = "venues")]
    pub venues: Vec<PredictionResultVenue>,
}
impl Model for PredictionResult {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.venues.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionResultMarket {
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<Decimal>,
}
impl Model for PredictionResultMarket {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionResultSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amended_at")]
    pub amended_at: Option<String>,
    #[serde(rename = "final")]
    pub final_: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "settled_at")]
    pub settled_at: Option<String>,
}
impl Model for PredictionResultSummary {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionResultVenue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amended_at")]
    pub amended_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "final")]
    pub final_: Option<bool>,
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionResultMarket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "settled_at")]
    pub settled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unavailable")]
    pub unavailable: Option<bool>,
    #[serde(rename = "venue")]
    pub venue: Venue,
}
impl Model for PredictionResultVenue {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.markets.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionSeries {
    #[serde(rename = "active_from")]
    pub active_from: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "active_to")]
    pub active_to: Option<String>,
    #[serde(rename = "bars")]
    pub bars: Vec<PredictionBar>,
    #[serde(rename = "key")]
    pub key: String,
}
impl Model for PredictionSeries {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.bars.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionTopic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "calendar")]
    pub calendar: Option<TopicCalendar>,
    #[serde(rename = "group")]
    pub group: TopicGroup,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "next_event_id")]
    pub next_event_id: Option<String>,
    #[serde(rename = "resolution_kind")]
    pub resolution_kind: ResolutionKind,
    #[serde(rename = "shape")]
    pub shape: PredictionShape,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(rename = "venues")]
    pub venues: Vec<Venue>,
}
impl Model for PredictionTopic {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        if let Some(v) = &mut self.calendar {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionTopicDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "calendar")]
    pub calendar: Option<TopicCalendar>,
    #[serde(rename = "events")]
    pub events: Vec<PredictionTopicEvent>,
    #[serde(rename = "group")]
    pub group: TopicGroup,
    #[serde(rename = "resolution_kind")]
    pub resolution_kind: ResolutionKind,
    #[serde(rename = "shape")]
    pub shape: PredictionShape,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(rename = "venues")]
    pub venues: Vec<Venue>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PredictionTopicDetail {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.calendar {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.events.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionTopicEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "result")]
    pub result: Option<PredictionResultSummary>,
    #[serde(rename = "status")]
    pub status: PredictionEventStatus,
}
impl Model for PredictionTopicEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        if let Some(v) = &mut self.result {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionTopics {
    #[serde(rename = "topics")]
    pub topics: Vec<PredictionTopic>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for PredictionTopics {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.topics.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionVenue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "excluded_reason")]
    pub excluded_reason: Option<VenueExclusionReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "match")]
    pub match_: Option<VenueMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "open_interest_usd")]
    pub open_interest_usd: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rules_text")]
    pub rules_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rules_url")]
    pub rules_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "settlement")]
    pub settlement: Option<PredictionVenueSettlement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "settlement_source")]
    pub settlement_source: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "venue")]
    pub venue: Venue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "volume_24h_usd")]
    pub volume_24h_usd: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weight")]
    pub weight: Option<String>,
}
impl Model for PredictionVenue {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        if let Some(v) = &mut self.match_ {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionWebhookEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "prediction_event_id")]
    pub prediction_event_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(rename = "settlements")]
    pub settlements: Vec<PredictionWebhookSettlement>,
    #[serde(rename = "shape")]
    pub shape: PredictionShape,
    #[serde(rename = "status")]
    pub status: PredictionWebhookStatus,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
}
impl Model for PredictionWebhookEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.settlements.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PredictionWebhookSettlement {
    #[serde(rename = "status")]
    pub status: SettlementStatus,
    #[serde(rename = "venue")]
    pub venue: Venue,
}
impl Model for PredictionWebhookSettlement {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Problem {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "status")]
    pub status: i64,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl Model for Problem {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Quote {
    #[serde(rename = "market_ids")]
    pub market_ids: Vec<String>,
    #[serde(rename = "probability")]
    pub probability: String,
    #[serde(rename = "venue")]
    pub venue: Venue,
}
impl Model for Quote {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RangePolicyRate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "announced_on")]
    pub announced_on: Option<String>,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "effective_on")]
    pub effective_on: Option<String>,
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(rename = "lower_bound")]
    pub lower_bound: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "observed_on")]
    pub observed_on: Option<String>,
    #[serde(rename = "rate_type")]
    pub rate_type: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "upper_bound")]
    pub upper_bound: Decimal,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for RangePolicyRate {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RateHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<RateHistoryObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for RateHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RateHistoryObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "rate")]
    pub rate: Decimal,
}
impl Model for RateHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RegistryCoverage {
    #[serde(rename = "companies")]
    pub companies: i64,
    #[serde(rename = "registry")]
    pub registry: Registry,
}
impl Model for RegistryCoverage {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResourceHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<ResourceObservation>,
    #[serde(rename = "series_id")]
    pub series_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(rename = "unit")]
    pub unit: ResourceUnit,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for ResourceHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResourceObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "released_on")]
    pub released_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<ResourceObservationStatus>,
    #[serde(rename = "value")]
    pub value: Decimal,
}
impl Model for ResourceObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScalarEvent {
    #[serde(rename = "above")]
    pub above: Vec<ScalarEventAbove>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "median")]
    pub median: Option<Decimal>,
    #[serde(rename = "outcomes")]
    pub outcomes: Vec<ScalarOutcome>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "p10")]
    pub p10: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "p90")]
    pub p90: Option<Decimal>,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_with")]
    pub resolves_with: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "result")]
    pub result: Option<PredictionResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(rename = "status")]
    pub status: PredictionEventStatus,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(rename = "venues")]
    pub venues: Vec<PredictionVenue>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for ScalarEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.outcomes.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.result {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.venues.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScalarOutcome {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lower")]
    pub lower: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "probability")]
    pub probability: Option<String>,
    #[serde(rename = "quotes")]
    pub quotes: Vec<Quote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "upper")]
    pub upper: Option<Decimal>,
}
impl Model for ScalarOutcome {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.quotes.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityListing {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "from")]
    pub from: Option<String>,
    #[serde(rename = "mic")]
    pub mic: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "to")]
    pub to: Option<String>,
}
impl Model for SecurityListing {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SeriesSource {
    #[serde(rename = "distributor")]
    pub distributor: String,
    #[serde(rename = "publisher")]
    pub publisher: String,
}
impl Model for SeriesSource {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SplitsCalendar {
    #[serde(rename = "entries")]
    pub entries: Vec<SplitsCalendarEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for SplitsCalendar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.entries.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SplitsCalendarEntry {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ratio")]
    pub ratio: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
}
impl Model for SplitsCalendarEntry {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimingEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "closes_at")]
    pub closes_at: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "not_within_window")]
    pub not_within_window: Option<String>,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_at")]
    pub resolves_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolves_with")]
    pub resolves_with: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "result")]
    pub result: Option<PredictionResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stale")]
    pub stale: Option<bool>,
    #[serde(rename = "status")]
    pub status: PredictionEventStatus,
    #[serde(rename = "steps")]
    pub steps: Vec<TimingStep>,
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(rename = "venues")]
    pub venues: Vec<PredictionVenue>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for TimingEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.result {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.steps.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        for v in self.venues.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimingStep {
    #[serde(rename = "by")]
    pub by: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "kind")]
    pub kind: TimingStepKind,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "probability")]
    pub probability: Option<String>,
    #[serde(rename = "quotes")]
    pub quotes: Vec<Quote>,
}
impl Model for TimingStep {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.quotes.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopicCalendar {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "family")]
    pub family: String,
}
impl Model for TopicCalendar {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<TradeHistoryObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for TradeHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeHistoryObservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exports")]
    pub exports: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_reported_at")]
    pub first_reported_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imports")]
    pub imports: Option<Decimal>,
    #[serde(rename = "period")]
    pub period: String,
}
impl Model for TradeHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeSummary {
    #[serde(rename = "balance")]
    pub balance: Decimal,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "exports")]
    pub exports: Decimal,
    #[serde(rename = "imports")]
    pub imports: Decimal,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for TradeSummary {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Unemployment {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "rate")]
    pub rate: Decimal,
    #[serde(rename = "released_on")]
    pub released_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<SeriesSource>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for Unemployment {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        if let Some(v) = &mut self.source {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VenueMatch {
    #[serde(rename = "differences")]
    pub differences: Vec<VenueDifference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "note")]
    pub note: Option<String>,
}
impl Model for VenueMatch {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookConfigurationHistory {
    #[serde(rename = "config_version")]
    pub config_version: i64,
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "families")]
    pub families: Vec<WebhookFamily>,
    #[serde(rename = "recorded_at")]
    pub recorded_at: String,
    #[serde(rename = "url")]
    pub url: String,
}
impl Model for WebhookConfigurationHistory {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookDeliveriesResponse {
    #[serde(rename = "deliveries")]
    pub deliveries: Vec<WebhookDelivery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for WebhookDeliveriesResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.deliveries.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookDelivery {
    #[serde(rename = "attempts")]
    pub attempts: i64,
    #[serde(rename = "config_version")]
    pub config_version: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "delivery_id")]
    pub delivery_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_cursor")]
    pub event_cursor: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "is_test")]
    pub is_test: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_status")]
    pub last_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "next_attempt_at")]
    pub next_attempt_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "replays")]
    pub replays: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retry_until")]
    pub retry_until: Option<String>,
    #[serde(rename = "state")]
    pub state: WebhookDeliveryState,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}
impl Model for WebhookDelivery {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookDestinationCreated {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companies")]
    pub companies: Option<Vec<String>>,
    #[serde(rename = "config_version")]
    pub config_version: i64,
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "destination_id")]
    pub destination_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disabled_reason")]
    pub disabled_reason: Option<WebhookDisabledReason>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "families")]
    pub families: Vec<WebhookFamily>,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "secret_version")]
    pub secret_version: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for WebhookDestinationCreated {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookDestinationInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companies")]
    pub companies: Option<Vec<String>>,
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "families")]
    pub families: Vec<WebhookFamily>,
    #[serde(rename = "url")]
    pub url: String,
}
impl Model for WebhookDestinationInput {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookDestinationList {
    #[serde(rename = "destination_limit")]
    pub destination_limit: i64,
    #[serde(rename = "destinations")]
    pub destinations: Vec<WebhookDestinationResponse>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for WebhookDestinationList {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.destinations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookDestinationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companies")]
    pub companies: Option<Vec<String>>,
    #[serde(rename = "config_version")]
    pub config_version: i64,
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "destination_id")]
    pub destination_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disabled_reason")]
    pub disabled_reason: Option<WebhookDisabledReason>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "families")]
    pub families: Vec<WebhookFamily>,
    #[serde(rename = "secret_version")]
    pub secret_version: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for WebhookDestinationResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "family")]
    pub family: Option<DeliveryFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filing")]
    pub filing: Option<FilingWebhookEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday_generation")]
    pub holiday_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "holiday_year")]
    pub holiday_year: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prediction")]
    pub prediction: Option<PredictionWebhookEvent>,
    #[serde(rename = "recorded_at")]
    pub recorded_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "release_id")]
    pub release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resource_version")]
    pub resource_version: Option<String>,
    #[serde(rename = "schema_version")]
    pub schema_version: String,
    #[serde(rename = "type")]
    pub type_: WebhookEventType,
}
impl Model for WebhookEvent {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        if let Some(v) = &mut self.filing {
            v.attach(ResponseMeta::default(), _raw);
        }
        if let Some(v) = &mut self.prediction {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookHistoryResponse {
    #[serde(rename = "history")]
    pub history: Vec<WebhookConfigurationHistory>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for WebhookHistoryResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.history.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookTestResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delivery_id")]
    pub delivery_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retry_after_seconds")]
    pub retry_after_seconds: Option<i64>,
    #[serde(rename = "status")]
    pub status: WebhookTestStatus,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for WebhookTestResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct YieldCurve {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "curve_type")]
    pub curve_type: YieldCurveType,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "yields")]
    pub yields: Vec<YieldPoint>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for YieldCurve {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.yields.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct YieldCurveHistoryObservation {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "yields")]
    pub yields: Vec<YieldPoint>,
}
impl Model for YieldCurveHistoryObservation {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        for v in self.yields.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct YieldCurveHistoryResponse {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "curve_type")]
    pub curve_type: YieldCurveType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::meta::cursor")]
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "observations")]
    pub observations: Vec<YieldCurveHistoryObservation>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
impl Model for YieldCurveHistoryResponse {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {
        self.meta = _meta;
        for v in self.observations.iter_mut() {
            v.attach(ResponseMeta::default(), _raw);
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        Some(&self.meta)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct YieldPoint {
    #[serde(rename = "maturity")]
    pub maturity: String,
    #[serde(rename = "yield_percent")]
    pub yield_percent: Decimal,
}
impl Model for YieldPoint {
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundPriceUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FundPrice {
    FundEtfPrice(Box<FundEtfPrice>),
    FundNavPrice(Box<FundNavPrice>),
    Unknown(FundPriceUnknown),
}
impl FundPrice {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::FundEtfPrice(_) => "FundEtfPrice",
            Self::FundNavPrice(_) => "FundNavPrice",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for FundPrice {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if _value
            .get("type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("etf")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::FundEtfPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("mutual_fund")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::FundNavPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(FundPriceUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
            meta: ResponseMeta::default(),
        }))
    }
}
impl Model for FundPrice {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::FundEtfPrice(v) => v.attach(meta, raw),
            Self::FundNavPrice(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
                v.meta = meta;
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        match self {
            Self::FundEtfPrice(v) => Some(&v.meta),
            Self::FundNavPrice(v) => Some(&v.meta),
            Self::Unknown(v) => Some(&v.meta),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundPriceItemUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FundPriceItem {
    FundEtfPrice(Box<FundEtfPrice>),
    FundNavPrice(Box<FundNavPrice>),
    Unknown(FundPriceItemUnknown),
}
impl FundPriceItem {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::FundEtfPrice(_) => "FundEtfPrice",
            Self::FundNavPrice(_) => "FundNavPrice",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for FundPriceItem {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if _value
            .get("type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("etf")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::FundEtfPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("mutual_fund")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::FundNavPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(FundPriceItemUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
        }))
    }
}
impl Model for FundPriceItem {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::FundEtfPrice(v) => v.attach(meta, raw),
            Self::FundNavPrice(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilHistoryUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PalmoilHistory {
    PalmoilScheduleHistory(Box<PalmoilScheduleHistory>),
    PalmoilMarketHistory(Box<PalmoilMarketHistory>),
    PalmoilScalarHistory(Box<PalmoilScalarHistory>),
    PalmoilFfbMarketHistory(Box<PalmoilFfbMarketHistory>),
    Unknown(PalmoilHistoryUnknown),
}
impl PalmoilHistory {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::PalmoilScheduleHistory(_) => "PalmoilScheduleHistory",
            Self::PalmoilMarketHistory(_) => "PalmoilMarketHistory",
            Self::PalmoilScalarHistory(_) => "PalmoilScalarHistory",
            Self::PalmoilFfbMarketHistory(_) => "PalmoilFfbMarketHistory",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for PalmoilHistory {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        crate::meta::raw_cursor(_value.get("next_cursor").map(Box::as_ref))
            .map_err(serde::de::Error::custom)?;
        if _value
            .get("unit")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("kilogram")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilScheduleHistory(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("currency")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("MYR")
            && _value
                .get("unit")
                .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
                .as_deref()
                == Some("metric_ton")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilMarketHistory(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("currency")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("USD")
            && _value
                .get("unit")
                .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
                .as_deref()
                == Some("metric_ton")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilScalarHistory(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("unit")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("one_percent_oer")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilFfbMarketHistory(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(PalmoilHistoryUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
            meta: ResponseMeta::default(),
        }))
    }
}
impl Model for PalmoilHistory {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::PalmoilScheduleHistory(v) => v.attach(meta, raw),
            Self::PalmoilMarketHistory(v) => v.attach(meta, raw),
            Self::PalmoilScalarHistory(v) => v.attach(meta, raw),
            Self::PalmoilFfbMarketHistory(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
                v.meta = meta;
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        match self {
            Self::PalmoilScheduleHistory(v) => Some(&v.meta),
            Self::PalmoilMarketHistory(v) => Some(&v.meta),
            Self::PalmoilScalarHistory(v) => Some(&v.meta),
            Self::PalmoilFfbMarketHistory(v) => Some(&v.meta),
            Self::Unknown(v) => Some(&v.meta),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilObservationUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PalmoilObservation {
    PalmoilScheduleObservation(Box<PalmoilScheduleObservation>),
    PalmoilMarketObservation(Box<PalmoilMarketObservation>),
    PalmoilScalarObservation(Box<PalmoilScalarObservation>),
    PalmoilFfbObservation(Box<PalmoilFfbObservation>),
    Unknown(PalmoilObservationUnknown),
}
impl PalmoilObservation {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::PalmoilScheduleObservation(_) => "PalmoilScheduleObservation",
            Self::PalmoilMarketObservation(_) => "PalmoilMarketObservation",
            Self::PalmoilScalarObservation(_) => "PalmoilScalarObservation",
            Self::PalmoilFfbObservation(_) => "PalmoilFfbObservation",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for PalmoilObservation {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        Ok(Self::Unknown(PalmoilObservationUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
        }))
    }
}
impl Model for PalmoilObservation {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::PalmoilScheduleObservation(v) => v.attach(meta, raw),
            Self::PalmoilMarketObservation(v) => v.attach(meta, raw),
            Self::PalmoilScalarObservation(v) => v.attach(meta, raw),
            Self::PalmoilFfbObservation(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilPriceUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PalmoilPrice {
    PalmoilSchedulePrice(Box<PalmoilSchedulePrice>),
    PalmoilMarketPrice(Box<PalmoilMarketPrice>),
    PalmoilScalarPrice(Box<PalmoilScalarPrice>),
    PalmoilFfbMarketPrice(Box<PalmoilFfbMarketPrice>),
    Unknown(PalmoilPriceUnknown),
}
impl PalmoilPrice {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::PalmoilSchedulePrice(_) => "PalmoilSchedulePrice",
            Self::PalmoilMarketPrice(_) => "PalmoilMarketPrice",
            Self::PalmoilScalarPrice(_) => "PalmoilScalarPrice",
            Self::PalmoilFfbMarketPrice(_) => "PalmoilFfbMarketPrice",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for PalmoilPrice {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if _value
            .get("unit")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("kilogram")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilSchedulePrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("currency")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("MYR")
            && _value
                .get("unit")
                .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
                .as_deref()
                == Some("metric_ton")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilMarketPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("currency")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("USD")
            && _value
                .get("unit")
                .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
                .as_deref()
                == Some("metric_ton")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilScalarPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("unit")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("one_percent_oer")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilFfbMarketPrice(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(PalmoilPriceUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
            meta: ResponseMeta::default(),
        }))
    }
}
impl Model for PalmoilPrice {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::PalmoilSchedulePrice(v) => v.attach(meta, raw),
            Self::PalmoilMarketPrice(v) => v.attach(meta, raw),
            Self::PalmoilScalarPrice(v) => v.attach(meta, raw),
            Self::PalmoilFfbMarketPrice(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
                v.meta = meta;
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        match self {
            Self::PalmoilSchedulePrice(v) => Some(&v.meta),
            Self::PalmoilMarketPrice(v) => Some(&v.meta),
            Self::PalmoilScalarPrice(v) => Some(&v.meta),
            Self::PalmoilFfbMarketPrice(v) => Some(&v.meta),
            Self::Unknown(v) => Some(&v.meta),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PalmoilSeriesUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PalmoilSeries {
    PalmoilScheduleSeries(Box<PalmoilScheduleSeries>),
    PalmoilScalarSeries(Box<PalmoilScalarSeries>),
    Unknown(PalmoilSeriesUnknown),
}
impl PalmoilSeries {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::PalmoilScheduleSeries(_) => "PalmoilScheduleSeries",
            Self::PalmoilScalarSeries(_) => "PalmoilScalarSeries",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for PalmoilSeries {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if _value
            .get("observation_type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("age_schedule")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilScheduleSeries(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("observation_type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("scalar")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PalmoilScalarSeries(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(PalmoilSeriesUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
        }))
    }
}
impl Model for PalmoilSeries {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::PalmoilScheduleSeries(v) => v.attach(meta, raw),
            Self::PalmoilScalarSeries(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyRateUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
    #[serde(skip)]
    pub meta: ResponseMeta,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PolicyRate {
    PointPolicyRate(Box<PointPolicyRate>),
    RangePolicyRate(Box<RangePolicyRate>),
    Unknown(PolicyRateUnknown),
}
impl PolicyRate {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::PointPolicyRate(_) => "PointPolicyRate",
            Self::RangePolicyRate(_) => "RangePolicyRate",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for PolicyRate {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if _value
            .get("rate_type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("point")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PointPolicyRate(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("rate_type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("range")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::RangePolicyRate(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(PolicyRateUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
            meta: ResponseMeta::default(),
        }))
    }
}
impl Model for PolicyRate {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::PointPolicyRate(v) => v.attach(meta, raw),
            Self::RangePolicyRate(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
                v.meta = meta;
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
    fn meta(&self) -> Option<&ResponseMeta> {
        match self {
            Self::PointPolicyRate(v) => Some(&v.meta),
            Self::RangePolicyRate(v) => Some(&v.meta),
            Self::Unknown(v) => Some(&v.meta),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyRateObservationUnknown {
    pub raw: Box<serde_json::value::RawValue>,
    #[serde(skip)]
    pub raw_body: Vec<u8>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PolicyRateObservation {
    PointPolicyRate(Box<PointPolicyRate>),
    RangePolicyRate(Box<RangePolicyRate>),
    Unknown(PolicyRateObservationUnknown),
}
impl PolicyRateObservation {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::PointPolicyRate(_) => "PointPolicyRate",
            Self::RangePolicyRate(_) => "RangePolicyRate",
            Self::Unknown(_) => "unknown",
        }
    }
}
impl<'de> Deserialize<'de> for PolicyRateObservation {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let _value: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if _value
            .get("rate_type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("point")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::PointPolicyRate(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        if _value
            .get("rate_type")
            .and_then(|v| serde_json::from_str::<String>(v.get()).ok())
            .as_deref()
            == Some("range")
        {
            return serde_json::from_str(raw.get())
                .map(|v| Self::RangePolicyRate(Box::new(v)))
                .map_err(serde::de::Error::custom);
        }
        Ok(Self::Unknown(PolicyRateObservationUnknown {
            raw_body: raw.get().as_bytes().to_vec(),
            raw,
        }))
    }
}
impl Model for PolicyRateObservation {
    fn attach(&mut self, meta: ResponseMeta, raw: &[u8]) {
        match self {
            Self::PointPolicyRate(v) => v.attach(meta, raw),
            Self::RangePolicyRate(v) => v.attach(meta, raw),
            Self::Unknown(v) => {
                v.raw_body = raw.to_vec();
            }
        }
    }
    fn kind(&self) -> Option<&'static str> {
        Some(self.kind())
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        if let Self::Unknown(v) = self {
            Some(&v.raw)
        } else {
            None
        }
    }
}
impl crate::pagination::Page for CatalogueHistory {
    type Item = ResourceObservation;
    fn into_page(self) -> Result<(Vec<ResourceObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for CatalogueSearchPage {
    type Item = CatalogueEntry;
    fn into_page(self) -> Result<(Vec<CatalogueEntry>, Option<String>), crate::Error> {
        Ok((self.results, self.next_cursor))
    }
}
impl crate::pagination::Page for CommodityBars {
    type Item = CommodityBar;
    fn into_page(self) -> Result<(Vec<CommodityBar>, Option<String>), crate::Error> {
        Ok((self.bars, self.next_cursor))
    }
}
impl crate::pagination::Page for CompanyFilingsPage {
    type Item = ListedFiling;
    fn into_page(self) -> Result<(Vec<ListedFiling>, Option<String>), crate::Error> {
        Ok((self.filings, self.next_cursor))
    }
}
impl crate::pagination::Page for CpiHistory {
    type Item = CpiHistoryObservation;
    fn into_page(self) -> Result<(Vec<CpiHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for CryptoBars {
    type Item = CryptoBar;
    fn into_page(self) -> Result<(Vec<CryptoBar>, Option<String>), crate::Error> {
        Ok((self.bars, self.next_cursor))
    }
}
impl crate::pagination::Page for EconomicCalendar {
    type Item = EconomicReleaseSummary;
    fn into_page(self) -> Result<(Vec<EconomicReleaseSummary>, Option<String>), crate::Error> {
        Ok((self.releases, self.next_cursor))
    }
}
impl crate::pagination::Page for EquityBars {
    type Item = EquityBar;
    fn into_page(self) -> Result<(Vec<EquityBar>, Option<String>), crate::Error> {
        Ok((self.bars, self.next_cursor))
    }
}
impl crate::pagination::Page for EquityDividends {
    type Item = EquityDividend;
    fn into_page(self) -> Result<(Vec<EquityDividend>, Option<String>), crate::Error> {
        Ok((self.dividends, self.next_cursor))
    }
}
impl crate::pagination::Page for EquityEarnings {
    type Item = EquityEarning;
    fn into_page(self) -> Result<(Vec<EquityEarning>, Option<String>), crate::Error> {
        Ok((self.earnings, self.next_cursor))
    }
}
impl crate::pagination::Page for EquityRiskPremiumHistoryResponse {
    type Item = EquityRiskPremium;
    fn into_page(self) -> Result<(Vec<EquityRiskPremium>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for EquitySearchResults {
    type Item = EquitySearchResult;
    fn into_page(self) -> Result<(Vec<EquitySearchResult>, Option<String>), crate::Error> {
        Ok((self.results, self.next_cursor))
    }
}
impl crate::pagination::Page for EventsPage {
    type Item = DeliveryEvent;
    fn into_page(self) -> Result<(Vec<DeliveryEvent>, Option<String>), crate::Error> {
        Ok((self.events, self.next_cursor))
    }
}
impl crate::pagination::Page for FundDistributions {
    type Item = FundDistribution;
    fn into_page(self) -> Result<(Vec<FundDistribution>, Option<String>), crate::Error> {
        Ok((self.distributions, self.next_cursor))
    }
}
impl crate::pagination::Page for FundSearchResults {
    type Item = FundSearchResult;
    fn into_page(self) -> Result<(Vec<FundSearchResult>, Option<String>), crate::Error> {
        Ok((self.results, self.next_cursor))
    }
}
impl crate::pagination::Page for FxBars {
    type Item = FxBar;
    fn into_page(self) -> Result<(Vec<FxBar>, Option<String>), crate::Error> {
        Ok((self.bars, self.next_cursor))
    }
}
impl crate::pagination::Page for FxHistory {
    type Item = FxHistoricalObservation;
    fn into_page(self) -> Result<(Vec<FxHistoricalObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for GdpGrowthHistory {
    type Item = GdpGrowthHistoryObservation;
    fn into_page(self) -> Result<(Vec<GdpGrowthHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for GdpHistory {
    type Item = GdpHistoryObservation;
    fn into_page(self) -> Result<(Vec<GdpHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for InflationHistory {
    type Item = InflationHistoryObservation;
    fn into_page(self) -> Result<(Vec<InflationHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for PalmoilHistory {
    type Item = PalmoilObservation;
    fn into_page(self) -> Result<(Vec<PalmoilObservation>, Option<String>), crate::Error> {
        match self {
            Self::PalmoilScheduleHistory(p) => Ok((
                p.observations
                    .into_iter()
                    .map(|v| PalmoilObservation::PalmoilScheduleObservation(Box::new(v)))
                    .collect(),
                p.next_cursor,
            )),
            Self::PalmoilMarketHistory(p) => Ok((
                p.observations
                    .into_iter()
                    .map(|v| PalmoilObservation::PalmoilMarketObservation(Box::new(v)))
                    .collect(),
                p.next_cursor,
            )),
            Self::PalmoilScalarHistory(p) => Ok((
                p.observations
                    .into_iter()
                    .map(|v| PalmoilObservation::PalmoilScalarObservation(Box::new(v)))
                    .collect(),
                p.next_cursor,
            )),
            Self::PalmoilFfbMarketHistory(p) => Ok((
                p.observations
                    .into_iter()
                    .map(|v| PalmoilObservation::PalmoilFfbObservation(Box::new(v)))
                    .collect(),
                p.next_cursor,
            )),
            Self::Unknown(p) => {
                let fail = || {
                    crate::Error::Decode(crate::DecodeError {
                        raw_body: p.raw_body.clone(),
                    })
                };
                let envelope: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
                    serde_json::from_str(p.raw.get()).map_err(|_| fail())?;
                let values: Vec<Box<serde_json::value::RawValue>> =
                    serde_json::from_str(envelope.get("observations").ok_or_else(fail)?.get())
                        .map_err(|_| fail())?;
                let cursor = crate::meta::raw_cursor(envelope.get("next_cursor").map(Box::as_ref))
                    .map_err(|_| fail())?;
                let mut items = Vec::new();
                for value in values {
                    if !value.get().starts_with("{") {
                        return Err(fail());
                    }
                    items.push(PalmoilObservation::Unknown(PalmoilObservationUnknown {
                        raw: value,
                        raw_body: p.raw_body.clone(),
                    }));
                }
                Ok((items, cursor))
            }
        }
    }
}
impl crate::pagination::Page for PalmoilSeriesDiscovery {
    type Item = PalmoilSeries;
    fn into_page(self) -> Result<(Vec<PalmoilSeries>, Option<String>), crate::Error> {
        Ok((self.series, self.next_cursor))
    }
}
impl crate::pagination::Page for PhysicalObservations {
    type Item = PhysicalObservation;
    fn into_page(self) -> Result<(Vec<PhysicalObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for PhysicalSeriesPage {
    type Item = PhysicalSeries;
    fn into_page(self) -> Result<(Vec<PhysicalSeries>, Option<String>), crate::Error> {
        Ok((self.series, self.next_cursor))
    }
}
impl crate::pagination::Page for PolicyRateHistoryResponse {
    type Item = PolicyRateObservation;
    fn into_page(self) -> Result<(Vec<PolicyRateObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for PredictionEventsPage {
    type Item = PredictionEventSummary;
    fn into_page(self) -> Result<(Vec<PredictionEventSummary>, Option<String>), crate::Error> {
        Ok((self.events, self.next_cursor))
    }
}
impl crate::pagination::Page for PredictionHistory {
    type Item = PredictionSeries;
    fn into_page(self) -> Result<(Vec<PredictionSeries>, Option<String>), crate::Error> {
        Ok((self.series, self.next_cursor))
    }
}
impl crate::pagination::Page for RateHistory {
    type Item = RateHistoryObservation;
    fn into_page(self) -> Result<(Vec<RateHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for ResourceHistory {
    type Item = ResourceObservation;
    fn into_page(self) -> Result<(Vec<ResourceObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for TradeHistory {
    type Item = TradeHistoryObservation;
    fn into_page(self) -> Result<(Vec<TradeHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}
impl crate::pagination::Page for WebhookDeliveriesResponse {
    type Item = WebhookDelivery;
    fn into_page(self) -> Result<(Vec<WebhookDelivery>, Option<String>), crate::Error> {
        Ok((self.deliveries, self.next_cursor))
    }
}
impl crate::pagination::Page for YieldCurveHistoryResponse {
    type Item = YieldCurveHistoryObservation;
    fn into_page(
        self,
    ) -> Result<(Vec<YieldCurveHistoryObservation>, Option<String>), crate::Error> {
        Ok((self.observations, self.next_cursor))
    }
}

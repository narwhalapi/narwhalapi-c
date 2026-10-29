// Generated from spec/next/openapi.json sha256 5071bd97e9507eb56b344d00a24ba056ce1340fe1184faa4d755fe7e407e1cf6. Do not edit.
use crate::blocking::Client;
use crate::{
    models::*,
    request::{Request, RequestData},
    Error,
};
impl Client {
    pub fn calendar_subscriptions(&self) -> CalendarSubscriptions {
        CalendarSubscriptions {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CalendarSubscriptions {
    client: Client,
}
impl CalendarSubscriptions {
    /// Create Calendar Subscription
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `countries`: Required body builder method.
    /// - `end_date`: Required body builder method.
    /// - `families`: Optional body builder method.
    /// - `include_holidays`: Optional body builder method.
    /// - `include_releases`: Optional body builder method.
    /// - `scope`: Optional body builder method.
    /// - `start_date`: Required body builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CalendarSubscriptionCreated` with header metadata.
    pub fn create(&self) -> CalendarSubscriptionsCreateRequest {
        let data = RequestData::new(
            "POST",
            "/v1/calendars/subscriptions",
            &[],
            &[],
            &["countries", "end_date", "start_date"],
            true,
        );
        CalendarSubscriptionsCreateRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Revoke Calendar Subscription
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `subscription_id`: Required path argument. Calendar subscription ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `NoContent` with header metadata.
    pub fn revoke(&self, subscription_id: impl AsRef<str>) -> CalendarSubscriptionsRevokeRequest {
        let data = RequestData::new(
            "DELETE",
            "/v1/calendars/subscriptions/{subscription_id}",
            &[("subscription_id", subscription_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CalendarSubscriptionsRevokeRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Read Calendar Subscription
    /// Revocable calendar-only token URL returned once on subscription creation.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `token`: Required path argument.
    ///
    /// # Returns
    /// Calendar feed URL. No request is made.
    pub fn feed_url(&self, token: impl AsRef<str>) -> Result<String, Error> {
        self.client.feed_url(
            "/v1/calendars/subscriptions/{token}.ics",
            "token",
            token.as_ref(),
        )
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CalendarSubscriptionsCreateRequest {
    inner: Request<CalendarSubscriptionCreated, Client>,
}
impl CalendarSubscriptionsCreateRequest {
    /// Set countries.
    pub fn countries(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "countries",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Set end_date.
    pub fn end_date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.body(
            "end_date",
            serde_json::json!(value.as_ref().to_owned()),
            false,
        );
        self
    }
    /// Set families.
    pub fn families(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "families",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Set include_holidays.
    pub fn include_holidays(mut self, value: bool) -> Self {
        self.inner
            .data
            .body("include_holidays", serde_json::json!(value), false);
        self
    }
    /// Set include_releases.
    pub fn include_releases(mut self, value: bool) -> Self {
        self.inner
            .data
            .body("include_releases", serde_json::json!(value), false);
        self
    }
    /// Set scope.
    pub fn scope(mut self, value: impl AsRef<str>) -> Self {
        self.inner
            .data
            .body("scope", serde_json::json!(value.as_ref().to_owned()), false);
        self
    }
    /// Set start_date.
    pub fn start_date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.body(
            "start_date",
            serde_json::json!(value.as_ref().to_owned()),
            false,
        );
        self
    }
    /// Send the request and decode `CalendarSubscriptionCreated`.
    pub fn send(self) -> Result<CalendarSubscriptionCreated, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CalendarSubscriptionCreated, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CalendarSubscriptionsRevokeRequest {
    inner: Request<NoContent, Client>,
}
impl CalendarSubscriptionsRevokeRequest {
    /// Send the request and decode `NoContent`.
    pub fn send(self) -> Result<NoContent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(NoContent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn calendars(&self) -> Calendars {
        Calendars {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Calendars {
    client: Client,
}
impl Calendars {
    /// Get Calendar Holidays On Date
    /// Published public holidays on one date. Returns 404 when no calendar is published for the country, year or scope.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `date`: Required path argument. Date, YYYY-MM-DD. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `scope`: Optional query builder method. Only holidays and coverage of this scope, for example national_public.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CalendarHolidaysOnDate` with header metadata.
    pub fn holidays_on_date(
        &self,
        country: impl AsRef<str>,
        date: impl AsRef<str>,
    ) -> CalendarsHolidaysOnDateRequest {
        let data = RequestData::new(
            "GET",
            "/v1/calendars/holidays/{country}/on/{date}",
            &[
                ("country", country.as_ref().to_owned()),
                ("date", date.as_ref().to_owned()),
            ],
            &[],
            &[],
            false,
        );
        CalendarsHolidaysOnDateRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Calendar Next Holiday
    /// Next published public holiday on or after a date. Returns 404 when no calendar is published for the country, year or scope.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `from_date`: Required query argument. Date to search from, YYYY-MM-DD. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `scope`: Optional query builder method. Only holidays and coverage of this scope, for example national_public.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CalendarNextHoliday` with header metadata.
    pub fn next_holiday(
        &self,
        country: impl AsRef<str>,
        from_date: impl AsRef<str>,
    ) -> CalendarsNextHolidayRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/calendars/holidays/{country}/next",
            &[("country", country.as_ref().to_owned())],
            &["from_date"],
            &[],
            false,
        );
        data.query(
            "from_date",
            vec![from_date.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        CalendarsNextHolidayRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Calendar Holidays
    /// Published public holidays for a country and year, with the calendar coverage. An empty `subdivision_codes` list means the holiday applies nationwide. Returns 404 when no calendar is published for the country, year or scope.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `year`: Required path argument. Calendar year. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `scope`: Optional query builder method. Only holidays and coverage of this scope, for example national_public.
    /// - `version_id`: Optional query builder method. Immutable calendar version from a holiday event.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CalendarHolidays` with header metadata.
    pub fn list_holidays(
        &self,
        country: impl AsRef<str>,
        year: i64,
    ) -> CalendarsListHolidaysRequest {
        let data = RequestData::new(
            "GET",
            "/v1/calendars/holidays/{country}/{year}",
            &[
                ("country", country.as_ref().to_owned()),
                ("year", year.to_string()),
            ],
            &[],
            &[],
            false,
        );
        CalendarsListHolidaysRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CalendarsHolidaysOnDateRequest {
    inner: Request<CalendarHolidaysOnDate, Client>,
}
impl CalendarsHolidaysOnDateRequest {
    /// Only holidays and coverage of this scope, for example national_public.
    pub fn scope(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "scope",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CalendarHolidaysOnDate`.
    pub fn send(self) -> Result<CalendarHolidaysOnDate, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CalendarHolidaysOnDate, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CalendarsNextHolidayRequest {
    inner: Request<CalendarNextHoliday, Client>,
}
impl CalendarsNextHolidayRequest {
    /// Only holidays and coverage of this scope, for example national_public.
    pub fn scope(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "scope",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CalendarNextHoliday`.
    pub fn send(self) -> Result<CalendarNextHoliday, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CalendarNextHoliday, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CalendarsListHolidaysRequest {
    inner: Request<CalendarHolidays, Client>,
}
impl CalendarsListHolidaysRequest {
    /// Only holidays and coverage of this scope, for example national_public.
    pub fn scope(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "scope",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable calendar version from a holiday event.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CalendarHolidays`.
    pub fn send(self) -> Result<CalendarHolidays, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CalendarHolidays, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn commodities(&self) -> Commodities {
        Commodities {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Commodities {
    client: Client,
}
impl Commodities {
    /// Get Palmoil Observations
    /// Palm-oil prices of one series by native period. Supply start and end together.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `series_id`: Required path argument. Palm-oil series ID.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. First day to exclude from the range. Supply start and end together.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `price_month`: Optional query builder method. Month to read, YYYY-MM. MPOB CPO series only.
    /// - `start`: Optional query builder method. First date to include. Supply start and end together.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PalmoilHistory` with header metadata.
    /// Its `iter()` yields `PalmoilObservation` lazily across pages.
    pub fn palmoil_physical_observations(
        &self,
        series_id: impl AsRef<str>,
    ) -> CommoditiesPalmoilPhysicalObservationsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/palmoil/physical/{series_id}/observations",
            &[("series_id", series_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CommoditiesPalmoilPhysicalObservationsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Palmoil Physical
    /// Latest approved palm-oil price of one series: a single CPO price or a full FFB age schedule.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `series_id`: Required path argument. Palm-oil series ID.
    /// - `price_month`: Optional query builder method. Month to read, YYYY-MM. MPOB CPO series only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PalmoilPrice` with header metadata.
    pub fn palmoil_physical_price(
        &self,
        series_id: impl AsRef<str>,
    ) -> CommoditiesPalmoilPhysicalPriceRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/palmoil/physical/{series_id}",
            &[("series_id", series_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CommoditiesPalmoilPhysicalPriceRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Physical Latest
    /// Latest published price of one physical price series.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `series_id`: Required path argument. Series ID. `latest` is not a series ID.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PhysicalLatest` with header metadata.
    pub fn physical_latest(&self, series_id: impl AsRef<str>) -> CommoditiesPhysicalLatestRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/physical/series/{series_id}/latest",
            &[("series_id", series_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CommoditiesPhysicalLatestRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Physical Observations
    /// Published prices of one physical price series. `start` is inclusive and `end` is exclusive. Supply start and end together.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `series_id`: Required path argument. Series ID. `latest` is not a series ID.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. First day to exclude from the range. Supply start and end together.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `start`: Optional query builder method. First day to include. Supply start and end together.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PhysicalObservations` with header metadata.
    /// Its `iter()` yields `PhysicalObservation` lazily across pages.
    pub fn physical_observations(
        &self,
        series_id: impl AsRef<str>,
    ) -> CommoditiesPhysicalObservationsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/physical/series/{series_id}/observations",
            &[("series_id", series_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CommoditiesPhysicalObservationsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Physical Series
    /// Definition of one physical price series: commodity, category, product, country, area, price type, basis, currency, unit, unit_quantity, frequency, description, attribution, coverage_start and coverage_end.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `series_id`: Required path argument. Series ID. `latest` is not a series ID.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PhysicalSeries` with header metadata.
    pub fn get_physical_series(
        &self,
        series_id: impl AsRef<str>,
    ) -> CommoditiesGetPhysicalSeriesRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/physical/series/{series_id}",
            &[("series_id", series_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CommoditiesGetPhysicalSeriesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Commodity Price History
    /// Closed price bars for one commodity and quote currency, from one minute to one day. A request may span at most 3 days of 1m bars, 14 days of 5m bars, 60 days of 15m bars; a longer range returns 400. Wider intervals have no cap.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `commodity`: Required path argument.
    /// - `currency`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Required query argument.
    /// - `interval`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CommodityBars` with header metadata.
    /// Its `iter()` yields `CommodityBar` lazily across pages.
    pub fn price_history(
        &self,
        commodity: impl AsRef<str>,
        currency: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> CommoditiesPriceHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/commodities/spot/{commodity}/{currency}/bars",
            &[
                ("commodity", commodity.as_ref().to_owned()),
                ("currency", currency.as_ref().to_owned()),
            ],
            &["end", "start"],
            &[],
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        CommoditiesPriceHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Commodity Spot
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `commodity`: Required path argument.
    /// - `currency`: Required path argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CommoditySpot` with header metadata.
    pub fn spot_price(
        &self,
        commodity: impl AsRef<str>,
        currency: impl AsRef<str>,
    ) -> CommoditiesSpotPriceRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/spot/{commodity}/{currency}",
            &[
                ("commodity", commodity.as_ref().to_owned()),
                ("currency", currency.as_ref().to_owned()),
            ],
            &[],
            &[],
            false,
        );
        CommoditiesSpotPriceRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Palmoil Physical
    /// List approved palm-oil price series. Filters combine. A filter that matches nothing returns an empty list.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `area`: Optional query builder method. Area code inside the country.
    /// - `country`: Optional query builder method. ISO 3166-1 alpha-3 country code.
    /// - `currency`: Optional query builder method. Quote currency, ISO 4217 code.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `price_type`: Optional query builder method. Price definition.
    /// - `producer_arrangement`: Optional query builder method. Producer arrangement: purchase_agreement or supported_scheme.
    /// - `producer_type`: Optional query builder method. Producer type; only smallholder is supported.
    /// - `product`: Optional query builder method. ffb (fresh fruit bunches) or cpo (crude palm oil).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PalmoilSeriesDiscovery` with header metadata.
    /// Its `iter()` yields `PalmoilSeries` lazily across pages.
    pub fn list_palmoil_physical_series(&self) -> CommoditiesListPalmoilPhysicalSeriesRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/palmoil/physical",
            &[],
            &[],
            &[],
            false,
        );
        CommoditiesListPalmoilPhysicalSeriesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Physical Series
    /// List published physical and official reference price series. Filters combine. A filter that matches nothing returns an empty list.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `category`: Optional query builder method. Commodity category.
    /// - `commodity`: Optional query builder method. Commodity slug, for example wheat.
    /// - `country`: Optional query builder method. Market: ISO 3166-1 alpha-3 code, WLD (world) or EUU (European Union).
    /// - `currency`: Optional query builder method. Quote currency, ISO 4217 code.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `price_type`: Optional query builder method. Price type, for example retail_average.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PhysicalSeriesPage` with header metadata.
    /// Its `iter()` yields `PhysicalSeries` lazily across pages.
    pub fn list_physical_series(&self) -> CommoditiesListPhysicalSeriesRequest {
        let data = RequestData::new(
            "GET",
            "/v1/commodities/physical/series",
            &[],
            &[],
            &[],
            false,
        );
        CommoditiesListPhysicalSeriesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesPalmoilPhysicalObservationsRequest {
    inner: Request<PalmoilHistory, Client>,
}
impl CommoditiesPalmoilPhysicalObservationsRequest {
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First day to exclude from the range. Supply start and end together.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Month to read, YYYY-MM. MPOB CPO series only.
    pub fn price_month(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "price_month",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First date to include. Supply start and end together.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PalmoilHistory`.
    pub fn send(self) -> Result<PalmoilHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PalmoilHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PalmoilHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesPalmoilPhysicalPriceRequest {
    inner: Request<PalmoilPrice, Client>,
}
impl CommoditiesPalmoilPhysicalPriceRequest {
    /// Month to read, YYYY-MM. MPOB CPO series only.
    pub fn price_month(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "price_month",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PalmoilPrice`.
    pub fn send(self) -> Result<PalmoilPrice, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PalmoilPrice, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesPhysicalLatestRequest {
    inner: Request<PhysicalLatest, Client>,
}
impl CommoditiesPhysicalLatestRequest {
    /// Send the request and decode `PhysicalLatest`.
    pub fn send(self) -> Result<PhysicalLatest, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PhysicalLatest, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesPhysicalObservationsRequest {
    inner: Request<PhysicalObservations, Client>,
}
impl CommoditiesPhysicalObservationsRequest {
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First day to exclude from the range. Supply start and end together.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First day to include. Supply start and end together.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PhysicalObservations`.
    pub fn send(self) -> Result<PhysicalObservations, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PhysicalObservations, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PhysicalObservations> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesGetPhysicalSeriesRequest {
    inner: Request<PhysicalSeries, Client>,
}
impl CommoditiesGetPhysicalSeriesRequest {
    /// Send the request and decode `PhysicalSeries`.
    pub fn send(self) -> Result<PhysicalSeries, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PhysicalSeries, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesPriceHistoryRequest {
    inner: Request<CommodityBars, Client>,
}
impl CommoditiesPriceHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Send the request and decode `CommodityBars`.
    pub fn send(self) -> Result<CommodityBars, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CommodityBars, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<CommodityBars> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesSpotPriceRequest {
    inner: Request<CommoditySpot, Client>,
}
impl CommoditiesSpotPriceRequest {
    /// Send the request and decode `CommoditySpot`.
    pub fn send(self) -> Result<CommoditySpot, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CommoditySpot, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesListPalmoilPhysicalSeriesRequest {
    inner: Request<PalmoilSeriesDiscovery, Client>,
}
impl CommoditiesListPalmoilPhysicalSeriesRequest {
    /// Area code inside the country.
    pub fn area(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "area",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// ISO 3166-1 alpha-3 country code.
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Quote currency, ISO 4217 code.
    pub fn currency(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "currency",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Price definition.
    pub fn price_type(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "price_type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Producer arrangement: purchase_agreement or supported_scheme.
    pub fn producer_arrangement(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "producer_arrangement",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Producer type; only smallholder is supported.
    pub fn producer_type(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "producer_type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// ffb (fresh fruit bunches) or cpo (crude palm oil).
    pub fn product(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "product",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PalmoilSeriesDiscovery`.
    pub fn send(self) -> Result<PalmoilSeriesDiscovery, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PalmoilSeriesDiscovery, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PalmoilSeriesDiscovery> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommoditiesListPhysicalSeriesRequest {
    inner: Request<PhysicalSeriesPage, Client>,
}
impl CommoditiesListPhysicalSeriesRequest {
    /// Commodity category.
    pub fn category(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "category",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Commodity slug, for example wheat.
    pub fn commodity(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "commodity",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Market: ISO 3166-1 alpha-3 code, WLD (world) or EUU (European Union).
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Quote currency, ISO 4217 code.
    pub fn currency(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "currency",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Price type, for example retail_average.
    pub fn price_type(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "price_type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PhysicalSeriesPage`.
    pub fn send(self) -> Result<PhysicalSeriesPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PhysicalSeriesPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PhysicalSeriesPage> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
impl Client {
    pub fn company_calendars(&self) -> CompanyCalendars {
        CompanyCalendars {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CompanyCalendars {
    client: Client,
}
impl CompanyCalendars {
    /// Company dividends calendar
    /// Dividends going ex in the window: the ex-date, the amount per share and its currency. `start` and `end` are inclusive days (YYYY-MM-DD); left out, `start` is today (UTC) and `end` is `start`. Dates run from 30 days back to 90 days ahead (400 invalid_query outside), and a request spans at most 7 days (400 range_too_large). `country` is USA, the only country for now (404 not_covered). Only companies in our symbol list appear, with our id and name; units and share classes of one issuer are one row. Open to every plan and one call a request.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Optional query builder method. Three-letter country code of the listing. Left out means USA, the only country for now (another one is 404 not_covered).
    /// - `end`: Optional query builder method. Last day, inclusive. Left out means `start`. At most 7 days from `start` (400 range_too_large).
    /// - `start`: Optional query builder method. First day (YYYY-MM-DD), inclusive. Left out means today (UTC). Between 30 days back and 90 days ahead.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `DividendsCalendar` with header metadata.
    pub fn dividends(&self) -> CompanyCalendarsDividendsRequest {
        let data = RequestData::new("GET", "/v1/calendars/dividends", &[], &[], &[], false);
        CompanyCalendarsDividendsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Company splits calendar
    /// Stock splits taking effect in the window, with the ratio as new shares for old (4:1 gives four for one; 1:10 is a reverse split). `start` and `end` are inclusive days (YYYY-MM-DD); left out, `start` is today (UTC) and `end` is `start`. Dates run from 30 days back to 90 days ahead (400 invalid_query outside), and a request spans at most 7 days (400 range_too_large). `country` is USA, the only country for now (404 not_covered). Only companies in our symbol list appear, with our id and name; units and share classes of one issuer are one row. Open to every plan and one call a request.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Optional query builder method. Three-letter country code of the listing. Left out means USA, the only country for now (another one is 404 not_covered).
    /// - `end`: Optional query builder method. Last day, inclusive. Left out means `start`. At most 7 days from `start` (400 range_too_large).
    /// - `start`: Optional query builder method. First day (YYYY-MM-DD), inclusive. Left out means today (UTC). Between 30 days back and 90 days ahead.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `SplitsCalendar` with header metadata.
    pub fn splits(&self) -> CompanyCalendarsSplitsRequest {
        let data = RequestData::new("GET", "/v1/calendars/splits", &[], &[], &[], false);
        CompanyCalendarsSplitsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CompanyCalendarsDividendsRequest {
    inner: Request<DividendsCalendar, Client>,
}
impl CompanyCalendarsDividendsRequest {
    /// Three-letter country code of the listing. Left out means USA, the only country for now (another one is 404 not_covered).
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last day, inclusive. Left out means `start`. At most 7 days from `start` (400 range_too_large).
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First day (YYYY-MM-DD), inclusive. Left out means today (UTC). Between 30 days back and 90 days ahead.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `DividendsCalendar`.
    pub fn send(self) -> Result<DividendsCalendar, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(DividendsCalendar, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CompanyCalendarsSplitsRequest {
    inner: Request<SplitsCalendar, Client>,
}
impl CompanyCalendarsSplitsRequest {
    /// Three-letter country code of the listing. Left out means USA, the only country for now (another one is 404 not_covered).
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last day, inclusive. Left out means `start`. At most 7 days from `start` (400 range_too_large).
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First day (YYYY-MM-DD), inclusive. Left out means today (UTC). Between 30 days back and 90 days ahead.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `SplitsCalendar`.
    pub fn send(self) -> Result<SplitsCalendar, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(SplitsCalendar, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn crypto(&self) -> Crypto {
        Crypto {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Crypto {
    client: Client,
}
impl Crypto {
    /// Price bars of one crypto pair
    /// Daily (`1d`, one UTC day, closing at 00:00 UTC) or one-minute (`1m`) bars, oldest first. A day's bar is out at 00:00 UTC (Free: 15 minutes later, and `end` comes back when that cut the range); today's partial bar is never shown. One-minute bars are for Basic and Pro (Free gets 403 plan_upgrade_required). History: Free 12 months, Basic 5 years, Pro everything; `start` is sent back when the plan raised it. Up to 5,000 bars a page, with `next_cursor`; a cursor that no longer matches the prices is 409 cursor_expired. One call a page.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `pair`: Required path argument. Crypto pair: the coin, a dash and the quote currency, for example BTC-USD or ETH-EUR. Any case; a slash in place of the dash is accepted. The answer carries the canonical pair.
    /// - `cursor`: Optional query builder method. `next_cursor` of the previous page, with the same parameters.
    /// - `end`: Optional query builder method. Last bar. `1d`: a date, inclusive; left out means the latest finished day. `1m`: a UTC time, exclusive; left out means now, or five days after `start`. At most 5 days between `start` and `end` for `1m` (400 range_too_large).
    /// - `interval`: Optional query builder method. `1d` (one bar per UTC day, closing at 00:00 UTC) or `1m` (one-minute bars, Basic and Pro only). Anything else is 400 invalid_interval.
    /// - `start`: Optional query builder method. First bar. `1d`: a date (YYYY-MM-DD), inclusive; left out means the oldest bar your plan reaches. `1m`: a UTC time (YYYY-MM-DDTHH:MM:SSZ, or a date for 00:00 UTC), inclusive; left out means one day before `end`.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CryptoBars` with header metadata.
    /// Its `iter()` yields `CryptoBar` lazily across pages.
    pub fn bars(&self, pair: impl AsRef<str>) -> CryptoBarsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/crypto/{pair}/bars",
            &[("pair", pair.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CryptoBarsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Latest price of one crypto pair
    /// The latest composite price of a pair, with the 24-hour change (left out on Free and when it is not known). Crypto trades all day, so there is no market session; `stale` is true when the price is more than 5 minutes old. Free sees the price 15 minutes late and the answer carries `delayed: true`. `change_24h_percent` is in percent.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `pair`: Required path argument. Crypto pair: the coin, a dash and the quote currency, for example BTC-USD or ETH-EUR. Any case; a slash in place of the dash is accepted. The answer carries the canonical pair.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CryptoPrice` with header metadata.
    pub fn price(&self, pair: impl AsRef<str>) -> CryptoPriceRequest {
        let data = RequestData::new(
            "GET",
            "/v1/crypto/{pair}/price",
            &[("pair", pair.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        CryptoPriceRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Latest prices for up to 50 crypto pairs
    /// Latest price of each listed pair (Free sees them 15 minutes late). One call is counted for each pair that returns a price, taken in one step: if fewer calls are left than pairs that resolved, the whole request is refused with 429 quota_exceeded. Pairs that return no price are listed in `errors` (as `id`) and cost nothing. `change_24h_percent` is in percent.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `pairs`: Required query argument. Comma-separated pairs, at most 50 after duplicates are removed. Each pair follows the same rules as the single-price route. One call is counted for each pair that returns a price.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CryptoPrices` with header metadata.
    pub fn prices(&self, pairs: impl AsRef<str>) -> CryptoPricesRequest {
        let mut data = RequestData::new("GET", "/v1/crypto/prices", &[], &["pairs"], &[], false);
        data.query(
            "pairs",
            vec![pairs.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        CryptoPricesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Crypto pairs we price
    /// Every pair we price, by pair (`BTC-USD` style). Optional `quote` (USD, EUR or GBP). A pair is listed while it trades at least once every 30 seconds and dropped when it falls below once every 2 minutes. Read from our own pair list, so it is open to every plan and counts one call.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `quote`: Optional query builder method. Only pairs quoted in this currency (USD, EUR or GBP).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CryptoPairs` with header metadata.
    pub fn list_pairs(&self) -> CryptoListPairsRequest {
        let data = RequestData::new("GET", "/v1/crypto/pairs", &[], &[], &[], false);
        CryptoListPairsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CryptoBarsRequest {
    inner: Request<CryptoBars, Client>,
}
impl CryptoBarsRequest {
    /// `next_cursor` of the previous page, with the same parameters.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last bar. `1d`: a date, inclusive; left out means the latest finished day. `1m`: a UTC time, exclusive; left out means now, or five days after `start`. At most 5 days between `start` and `end` for `1m` (400 range_too_large).
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// `1d` (one bar per UTC day, closing at 00:00 UTC) or `1m` (one-minute bars, Basic and Pro only). Anything else is 400 invalid_interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First bar. `1d`: a date (YYYY-MM-DD), inclusive; left out means the oldest bar your plan reaches. `1m`: a UTC time (YYYY-MM-DDTHH:MM:SSZ, or a date for 00:00 UTC), inclusive; left out means one day before `end`.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CryptoBars`.
    pub fn send(self) -> Result<CryptoBars, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CryptoBars, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<CryptoBars> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CryptoPriceRequest {
    inner: Request<CryptoPrice, Client>,
}
impl CryptoPriceRequest {
    /// Send the request and decode `CryptoPrice`.
    pub fn send(self) -> Result<CryptoPrice, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CryptoPrice, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CryptoPricesRequest {
    inner: Request<CryptoPrices, Client>,
}
impl CryptoPricesRequest {
    /// Send the request and decode `CryptoPrices`.
    pub fn send(self) -> Result<CryptoPrices, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CryptoPrices, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CryptoListPairsRequest {
    inner: Request<CryptoPairs, Client>,
}
impl CryptoListPairsRequest {
    /// Only pairs quoted in this currency (USD, EUR or GBP).
    pub fn quote(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "quote",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CryptoPairs`.
    pub fn send(self) -> Result<CryptoPairs, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CryptoPairs, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn economics(&self) -> Economics {
        Economics {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Economics {
    client: Client,
}
impl Economics {
    /// Get Central Bank Balance Sheet
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument.
    /// - `period`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CentralBankBalanceSheet` with header metadata.
    pub fn central_bank_balance_sheet(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsCentralBankBalanceSheetRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/central-bank/balance-sheet",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsCentralBankBalanceSheetRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Central Bank Reserves
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument.
    /// - `period`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CentralBankReserves` with header metadata.
    pub fn central_bank_reserves(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsCentralBankReservesRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/central-bank/reserves",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsCentralBankReservesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Cpi
    /// Latest headline consumer price index for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY-MM. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `Cpi` with header metadata.
    pub fn cpi(&self, country: impl AsRef<str>) -> EconomicsCpiRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/cpi",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsCpiRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Cpi History
    /// The headline consumer price index by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CpiHistory` with header metadata.
    /// Its `iter()` yields `CpiHistoryObservation` lazily across pages.
    pub fn cpi_history(&self, country: impl AsRef<str>) -> EconomicsCpiHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/cpi/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsCpiHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Equity Risk Premium
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument.
    /// - `date`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityRiskPremium` with header metadata.
    pub fn get_equity_risk_premium(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsGetEquityRiskPremiumRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/equity-risk-premium",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsGetEquityRiskPremiumRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Equity Risk Premium History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `start`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityRiskPremiumHistoryResponse` with header metadata.
    /// Its `iter()` yields `EquityRiskPremium` lazily across pages.
    pub fn equity_risk_premium_history(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsEquityRiskPremiumHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/equity-risk-premium/history",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsEquityRiskPremiumHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Gdp
    /// Latest GDP at current prices for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `Gdp` with header metadata.
    pub fn gdp(&self, country: impl AsRef<str>) -> EconomicsGdpRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/gdp",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsGdpRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Gdp Growth
    /// Latest real GDP growth for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `GdpGrowth` with header metadata.
    pub fn gdp_growth(&self, country: impl AsRef<str>) -> EconomicsGdpGrowthRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/gdp-growth",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsGdpGrowthRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Gdp Growth History
    /// The real GDP growth by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `GdpGrowthHistory` with header metadata.
    /// Its `iter()` yields `GdpGrowthHistoryObservation` lazily across pages.
    pub fn gdp_growth_history(&self, country: impl AsRef<str>) -> EconomicsGdpGrowthHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/gdp-growth/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsGdpGrowthHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Gdp History
    /// The GDP at current prices by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `GdpHistory` with header metadata.
    /// Its `iter()` yields `GdpHistoryObservation` lazily across pages.
    pub fn gdp_history(&self, country: impl AsRef<str>) -> EconomicsGdpHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/gdp/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsGdpHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Inflation
    /// Latest headline inflation for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY-MM. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `Inflation` with header metadata.
    pub fn inflation(&self, country: impl AsRef<str>) -> EconomicsInflationRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/inflation",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsInflationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Inflation History
    /// The headline inflation by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `InflationHistory` with header metadata.
    /// Its `iter()` yields `InflationHistoryObservation` lazily across pages.
    pub fn inflation_history(&self, country: impl AsRef<str>) -> EconomicsInflationHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/inflation/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsInflationHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Labour Force Participation
    /// Latest labour-force participation rate for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY, YYYY-MM or YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `LabourForceParticipation` with header metadata.
    pub fn labour_force_participation(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsLabourForceParticipationRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/labour-force-participation",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsLabourForceParticipationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Labour Force Participation History
    /// The labour-force participation rate by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `RateHistory` with header metadata.
    /// Its `iter()` yields `RateHistoryObservation` lazily across pages.
    pub fn labour_force_participation_history(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsLabourForceParticipationHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/labour-force-participation/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsLabourForceParticipationHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Policy Rate
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument.
    /// - `date`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PolicyRate` with header metadata.
    pub fn policy_rate(&self, country: impl AsRef<str>) -> EconomicsPolicyRateRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/central-bank/policy-rate",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsPolicyRateRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Policy Rate History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `start`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PolicyRateHistoryResponse` with header metadata.
    /// Its `iter()` yields `PolicyRateObservation` lazily across pages.
    pub fn policy_rate_history(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsPolicyRateHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/central-bank/policy-rate/history",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsPolicyRateHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Economic Release
    /// One release calendar entry by release ID.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `release_id`: Required path argument. Release ID (UUID).
    /// - `version_id`: Optional query builder method. Immutable release version to read. Omit it for the current one.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EconomicReleaseDetail` with header metadata.
    pub fn get_release(&self, release_id: impl AsRef<str>) -> EconomicsGetReleaseRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/calendar/{release_id}",
            &[("release_id", release_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsGetReleaseRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Trade
    /// Latest merchandise trade for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY or YYYY-MM. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `TradeSummary` with header metadata.
    pub fn trade(&self, country: impl AsRef<str>) -> EconomicsTradeRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/trade",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsTradeRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Trade History
    /// The merchandise trade by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `TradeHistory` with header metadata.
    /// Its `iter()` yields `TradeHistoryObservation` lazily across pages.
    pub fn trade_history(&self, country: impl AsRef<str>) -> EconomicsTradeHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/trade/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsTradeHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Unemployment
    /// Latest unemployment rate for a country, or one period with `period`.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `period`: Optional query builder method. Period to read: YYYY, YYYY-MM or YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current version.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `Unemployment` with header metadata.
    pub fn unemployment(&self, country: impl AsRef<str>) -> EconomicsUnemploymentRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/unemployment",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsUnemploymentRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Unemployment History
    /// The unemployment rate by period, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `limit`: Optional query builder method. Maximum periods per page.
    /// - `start`: Optional query builder method. First period to include: YYYY, YYYY-MM or YYYY-QN.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `RateHistory` with header metadata.
    /// Its `iter()` yields `RateHistoryObservation` lazily across pages.
    pub fn unemployment_history(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsUnemploymentHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/unemployment/observations",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsUnemploymentHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Yield Curve
    /// Government bond yield curve for one source date, in percent. `curve_type` shows how the country's source defines the curve. Omit `date` for the latest curve.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `date`: Optional query builder method. Source date, YYYY-MM-DD. Omit it for the latest curve. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    /// - `maturity`: Optional query builder method. Comma-separated maturities to return, for example 2y,10y. Omit for all.
    /// - `version_id`: Optional query builder method. Immutable publication version to read. Omit it for the current one.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `YieldCurve` with header metadata.
    pub fn yield_curve(&self, country: impl AsRef<str>) -> EconomicsYieldCurveRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/yield-curve",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsYieldCurveRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Yield Curve History
    /// Government bond yield curves by source date, oldest first, in percent.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Required path argument. Uppercase three-letter country code, for example USA.
    /// - `as_of`: Optional query builder method. The curves as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last source date to include, YYYY-MM-DD.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `maturity`: Optional query builder method. Comma-separated maturities to return, for example 2y,10y. Omit for all.
    /// - `start`: Optional query builder method. First source date to include, YYYY-MM-DD.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `YieldCurveHistoryResponse` with header metadata.
    /// Its `iter()` yields `YieldCurveHistoryObservation` lazily across pages.
    pub fn yield_curve_history(
        &self,
        country: impl AsRef<str>,
    ) -> EconomicsYieldCurveHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/{country}/yield-curve/history",
            &[("country", country.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsYieldCurveHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Equity Risk Premium
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// No parameters.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityRiskPremiumList` with header metadata.
    pub fn list_equity_risk_premium(&self) -> EconomicsListEquityRiskPremiumRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/equity-risk-premium",
            &[],
            &[],
            &[],
            false,
        );
        EconomicsListEquityRiskPremiumRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Economic Releases
    /// Release calendar entries for the chosen countries within a date window.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `countries`: Optional query builder method. Uppercase three-letter country codes (EMU for the euro area). Omit for all.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end_date`: Optional query builder method. Last date of the window.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `start_date`: Optional query builder method. First date of the window.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EconomicCalendar` with header metadata.
    /// Its `iter()` yields `EconomicReleaseSummary` lazily across pages.
    pub fn list_releases(&self) -> EconomicsListReleasesRequest {
        let data = RequestData::new("GET", "/v1/economics/calendar", &[], &[], &[], false);
        EconomicsListReleasesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Read Catalogue Latest Batch
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `ids`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CatalogueBulkLatest` with header metadata.
    pub fn read_catalogue_latest_batch(
        &self,
        ids: impl AsRef<str>,
    ) -> EconomicsReadCatalogueLatestBatchRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/economics/catalogue/latest",
            &[],
            &["ids"],
            &[],
            false,
        );
        data.query(
            "ids",
            vec![ids.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        EconomicsReadCatalogueLatestBatchRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Read Catalogue History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `cursor`: Optional query builder method.
    /// - `end`: Optional query builder method.
    /// - `id`: Required query argument.
    /// - `limit`: Optional query builder method.
    /// - `start`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CatalogueHistory` with header metadata.
    /// Its `iter()` yields `ResourceObservation` lazily across pages.
    pub fn read_catalogue_observations(
        &self,
        id: impl AsRef<str>,
    ) -> EconomicsReadCatalogueObservationsRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/economics/catalogue/series/observations",
            &[],
            &["id"],
            &[],
            false,
        );
        data.query(
            "id",
            vec![id.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        EconomicsReadCatalogueObservationsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Read Catalogue Series
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CatalogueSeries` with header metadata.
    pub fn read_catalogue_series(
        &self,
        id: impl AsRef<str>,
    ) -> EconomicsReadCatalogueSeriesRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/economics/catalogue/series",
            &[],
            &["id"],
            &[],
            false,
        );
        data.query(
            "id",
            vec![id.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        EconomicsReadCatalogueSeriesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Read Resource History
    /// Observations of one economic series, oldest first.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `series_id`: Required path argument. Series ID (UUID).
    /// - `as_of`: Optional query builder method. The series as published up to the end of this UTC day. Pro plan only.
    /// - `cursor`: Optional query builder method. The next_cursor from the previous page.
    /// - `end`: Optional query builder method. Last observation period to include.
    /// - `limit`: Optional query builder method. Maximum rows per page.
    /// - `start`: Optional query builder method. First observation period to include.
    /// - `version`: Optional query builder method. first_reported: the first value stored per period. Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `ResourceHistory` with header metadata.
    /// Its `iter()` yields `ResourceObservation` lazily across pages.
    pub fn read_series_observations(
        &self,
        series_id: impl AsRef<str>,
    ) -> EconomicsReadSeriesObservationsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/economics/series/{series_id}/observations",
            &[("series_id", series_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EconomicsReadSeriesObservationsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Search Series Catalogue
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Optional query builder method.
    /// - `cursor`: Optional query builder method.
    /// - `frequency`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `q`: Optional query builder method.
    /// - `source`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CatalogueSearchPage` with header metadata.
    /// Its `iter()` yields `CatalogueEntry` lazily across pages.
    pub fn search_catalogue(&self) -> EconomicsSearchCatalogueRequest {
        let data = RequestData::new("GET", "/v1/economics/catalogue", &[], &[], &[], false);
        EconomicsSearchCatalogueRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsCentralBankBalanceSheetRequest {
    inner: Request<CentralBankBalanceSheet, Client>,
}
impl EconomicsCentralBankBalanceSheetRequest {
    /// Set period.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CentralBankBalanceSheet`.
    pub fn send(self) -> Result<CentralBankBalanceSheet, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CentralBankBalanceSheet, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsCentralBankReservesRequest {
    inner: Request<CentralBankReserves, Client>,
}
impl EconomicsCentralBankReservesRequest {
    /// Set period.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CentralBankReserves`.
    pub fn send(self) -> Result<CentralBankReserves, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CentralBankReserves, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsCpiRequest {
    inner: Request<Cpi, Client>,
}
impl EconomicsCpiRequest {
    /// Period to read: YYYY-MM. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `Cpi`.
    pub fn send(self) -> Result<Cpi, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(Cpi, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsCpiHistoryRequest {
    inner: Request<CpiHistory, Client>,
}
impl EconomicsCpiHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CpiHistory`.
    pub fn send(self) -> Result<CpiHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CpiHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<CpiHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsGetEquityRiskPremiumRequest {
    inner: Request<EquityRiskPremium, Client>,
}
impl EconomicsGetEquityRiskPremiumRequest {
    /// Set date.
    pub fn date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "date",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityRiskPremium`.
    pub fn send(self) -> Result<EquityRiskPremium, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityRiskPremium, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsEquityRiskPremiumHistoryRequest {
    inner: Request<EquityRiskPremiumHistoryResponse, Client>,
}
impl EconomicsEquityRiskPremiumHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set end.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set start.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityRiskPremiumHistoryResponse`.
    pub fn send(self) -> Result<EquityRiskPremiumHistoryResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityRiskPremiumHistoryResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EquityRiskPremiumHistoryResponse> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsGdpRequest {
    inner: Request<Gdp, Client>,
}
impl EconomicsGdpRequest {
    /// Period to read: YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `Gdp`.
    pub fn send(self) -> Result<Gdp, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(Gdp, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsGdpGrowthRequest {
    inner: Request<GdpGrowth, Client>,
}
impl EconomicsGdpGrowthRequest {
    /// Period to read: YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `GdpGrowth`.
    pub fn send(self) -> Result<GdpGrowth, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(GdpGrowth, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsGdpGrowthHistoryRequest {
    inner: Request<GdpGrowthHistory, Client>,
}
impl EconomicsGdpGrowthHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `GdpGrowthHistory`.
    pub fn send(self) -> Result<GdpGrowthHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(GdpGrowthHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<GdpGrowthHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsGdpHistoryRequest {
    inner: Request<GdpHistory, Client>,
}
impl EconomicsGdpHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `GdpHistory`.
    pub fn send(self) -> Result<GdpHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(GdpHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<GdpHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsInflationRequest {
    inner: Request<Inflation, Client>,
}
impl EconomicsInflationRequest {
    /// Period to read: YYYY-MM. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `Inflation`.
    pub fn send(self) -> Result<Inflation, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(Inflation, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsInflationHistoryRequest {
    inner: Request<InflationHistory, Client>,
}
impl EconomicsInflationHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `InflationHistory`.
    pub fn send(self) -> Result<InflationHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(InflationHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<InflationHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsLabourForceParticipationRequest {
    inner: Request<LabourForceParticipation, Client>,
}
impl EconomicsLabourForceParticipationRequest {
    /// Period to read: YYYY, YYYY-MM or YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `LabourForceParticipation`.
    pub fn send(self) -> Result<LabourForceParticipation, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(LabourForceParticipation, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsLabourForceParticipationHistoryRequest {
    inner: Request<RateHistory, Client>,
}
impl EconomicsLabourForceParticipationHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `RateHistory`.
    pub fn send(self) -> Result<RateHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(RateHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<RateHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsPolicyRateRequest {
    inner: Request<PolicyRate, Client>,
}
impl EconomicsPolicyRateRequest {
    /// Set date.
    pub fn date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "date",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PolicyRate`.
    pub fn send(self) -> Result<PolicyRate, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PolicyRate, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsPolicyRateHistoryRequest {
    inner: Request<PolicyRateHistoryResponse, Client>,
}
impl EconomicsPolicyRateHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set end.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set start.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PolicyRateHistoryResponse`.
    pub fn send(self) -> Result<PolicyRateHistoryResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PolicyRateHistoryResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PolicyRateHistoryResponse> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsGetReleaseRequest {
    inner: Request<EconomicReleaseDetail, Client>,
}
impl EconomicsGetReleaseRequest {
    /// Immutable release version to read. Omit it for the current one.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EconomicReleaseDetail`.
    pub fn send(self) -> Result<EconomicReleaseDetail, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EconomicReleaseDetail, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsTradeRequest {
    inner: Request<TradeSummary, Client>,
}
impl EconomicsTradeRequest {
    /// Period to read: YYYY or YYYY-MM. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `TradeSummary`.
    pub fn send(self) -> Result<TradeSummary, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(TradeSummary, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsTradeHistoryRequest {
    inner: Request<TradeHistory, Client>,
}
impl EconomicsTradeHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `TradeHistory`.
    pub fn send(self) -> Result<TradeHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(TradeHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<TradeHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsUnemploymentRequest {
    inner: Request<Unemployment, Client>,
}
impl EconomicsUnemploymentRequest {
    /// Period to read: YYYY, YYYY-MM or YYYY-QN. Omit it for the latest period. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current version.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `Unemployment`.
    pub fn send(self) -> Result<Unemployment, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(Unemployment, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsUnemploymentHistoryRequest {
    inner: Request<RateHistory, Client>,
}
impl EconomicsUnemploymentHistoryRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum periods per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First period to include: YYYY, YYYY-MM or YYYY-QN.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `RateHistory`.
    pub fn send(self) -> Result<RateHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(RateHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<RateHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsYieldCurveRequest {
    inner: Request<YieldCurve, Client>,
}
impl EconomicsYieldCurveRequest {
    /// Source date, YYYY-MM-DD. Omit it for the latest curve. Plan history window - Free: the last 12 months. Basic: the last 5 years. Pro: full history.
    pub fn date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "date",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Comma-separated maturities to return, for example 2y,10y. Omit for all.
    pub fn maturity(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.query(
            "maturity",
            value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            "comma_join",
            false,
        );
        self
    }
    /// Immutable publication version to read. Omit it for the current one.
    pub fn version_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `YieldCurve`.
    pub fn send(self) -> Result<YieldCurve, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(YieldCurve, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsYieldCurveHistoryRequest {
    inner: Request<YieldCurveHistoryResponse, Client>,
}
impl EconomicsYieldCurveHistoryRequest {
    /// The curves as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last source date to include, YYYY-MM-DD.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Comma-separated maturities to return, for example 2y,10y. Omit for all.
    pub fn maturity(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.query(
            "maturity",
            value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            "comma_join",
            false,
        );
        self
    }
    /// First source date to include, YYYY-MM-DD.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `YieldCurveHistoryResponse`.
    pub fn send(self) -> Result<YieldCurveHistoryResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(YieldCurveHistoryResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<YieldCurveHistoryResponse> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsListEquityRiskPremiumRequest {
    inner: Request<EquityRiskPremiumList, Client>,
}
impl EconomicsListEquityRiskPremiumRequest {
    /// Send the request and decode `EquityRiskPremiumList`.
    pub fn send(self) -> Result<EquityRiskPremiumList, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityRiskPremiumList, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsListReleasesRequest {
    inner: Request<EconomicCalendar, Client>,
}
impl EconomicsListReleasesRequest {
    /// Uppercase three-letter country codes (EMU for the euro area). Omit for all.
    pub fn countries(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.query(
            "countries",
            value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            "repeat",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last date of the window.
    pub fn end_date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end_date",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First date of the window.
    pub fn start_date(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start_date",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EconomicCalendar`.
    pub fn send(self) -> Result<EconomicCalendar, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EconomicCalendar, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EconomicCalendar> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsReadCatalogueLatestBatchRequest {
    inner: Request<CatalogueBulkLatest, Client>,
}
impl EconomicsReadCatalogueLatestBatchRequest {
    /// Send the request and decode `CatalogueBulkLatest`.
    pub fn send(self) -> Result<CatalogueBulkLatest, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CatalogueBulkLatest, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsReadCatalogueObservationsRequest {
    inner: Request<CatalogueHistory, Client>,
}
impl EconomicsReadCatalogueObservationsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set end.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set start.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CatalogueHistory`.
    pub fn send(self) -> Result<CatalogueHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CatalogueHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<CatalogueHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsReadCatalogueSeriesRequest {
    inner: Request<CatalogueSeries, Client>,
}
impl EconomicsReadCatalogueSeriesRequest {
    /// Send the request and decode `CatalogueSeries`.
    pub fn send(self) -> Result<CatalogueSeries, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CatalogueSeries, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsReadSeriesObservationsRequest {
    inner: Request<ResourceHistory, Client>,
}
impl EconomicsReadSeriesObservationsRequest {
    /// The series as published up to the end of this UTC day. Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// The next_cursor from the previous page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last observation period to include.
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum rows per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// First observation period to include.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported: the first value stored per period. Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `ResourceHistory`.
    pub fn send(self) -> Result<ResourceHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(ResourceHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<ResourceHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EconomicsSearchCatalogueRequest {
    inner: Request<CatalogueSearchPage, Client>,
}
impl EconomicsSearchCatalogueRequest {
    /// Set country.
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set frequency.
    pub fn frequency(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "frequency",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set q.
    pub fn q(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "q",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set source.
    pub fn source(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "source",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CatalogueSearchPage`.
    pub fn send(self) -> Result<CatalogueSearchPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CatalogueSearchPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<CatalogueSearchPage> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
impl Client {
    pub fn equities(&self) -> Equities {
        Equities {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Equities {
    client: Client,
}
impl Equities {
    /// Price bars of one stock
    /// Daily (`1d`) or one-minute (`1m`) bars, oldest first, adjusted for splits and dividends. History: Free 12 months, Basic 5 years, Pro everything; `start` is sent back when the plan raised it. One-minute bars are for Basic and Pro (Free gets 403 plan_upgrade_required), reach back to 2020 and cover at most 5 days a call (400 range_too_large). Today's daily bar appears after the exchange closes, and for Free 15 minutes after; `end` is sent back when that cut the range. Up to 5,000 bars a page, with `next_cursor`; a cursor that no longer matches the prices is 409 cursor_expired. One call a page.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    /// - `cursor`: Optional query builder method. `next_cursor` of the previous page, with the same other parameters. A cursor stops working when the prices behind it changed (a split or dividend): 409 cursor_expired.
    /// - `end`: Optional query builder method. Last bar. `1d`: a date, inclusive; left out means the latest session whose bar is out. `1m`: a UTC time, exclusive; left out means now, or five days after `start`. At most 5 days between `start` and `end` for `1m` (400 range_too_large).
    /// - `interval`: Optional query builder method. `1d` (one bar per session) or `1m` (one-minute bars, Basic and Pro only). Anything else is 400 invalid_interval.
    /// - `start`: Optional query builder method. First bar. `1d`: a date (YYYY-MM-DD), inclusive; left out means the oldest bar your plan reaches. `1m`: a UTC time (YYYY-MM-DDTHH:MM:SSZ, or a date for 00:00 UTC), inclusive; left out means one day before `end`.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityBars` with header metadata.
    /// Its `iter()` yields `EquityBar` lazily across pages.
    pub fn bars(&self, id: impl AsRef<str>) -> EquitiesBarsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/bars",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesBarsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Beta, cost of equity, cost of debt and WACC of one stock
    /// Beta of one US or Korean stock against a benchmark ETF (weekly returns over two years by default, from prices adjusted for splits and dividends), the risk-free rate (the 10-year point of the country's government yield curve, or yours), the cost of equity (risk-free plus beta times the equity risk premium: yours in `erp`, or by default our own published total premium of the stock's country), the cost of debt (interest expense over average total debt from company filings, or, where that cannot be used, the risk-free rate plus a public table's default spread for the company's interest coverage; before and after tax, with the country's corporate tax rate from a public table unless you send `tax_rate`) and WACC, weighted by market cap and total debt. Every input is in the answer with its value, its basis and its date. Banks and insurers are 422 not_applicable; an input that cannot be worked out correctly is named in 422 inputs_unavailable, never repaired; a number that is shown but needs care comes with a `warnings` entry. Basic and Pro only (Free gets 403 plan_upgrade_required). One call.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    /// - `benchmark`: Optional query builder method. Listing id of the market proxy beta is measured against: an ETF, fund or stock priced in the same currency as the stock. Left out, it is ARCX:SPY for a US stock, XKRX:069500 for a Korean one and XTKS:1306 for a Japanese one; any other market needs it.
    /// - `beta`: Optional query builder method. Which beta feeds the cost of equity: `raw` (default) or `adjusted` (0.67 raw plus 0.33). Both are always in the answer.
    /// - `erp`: Optional query builder method. Your equity risk premium, in percent (5 means 5%). Left out, it is our own published total equity risk premium of the stock's country (the answer names its date and basis). Where we publish none, without it the answer is 400 erp_required. Above 0 and at most 25.
    /// - `frequency`: Optional query builder method. Return interval: `weekly` (default; the last common trading day of each ISO week) or `monthly`.
    /// - `period`: Optional query builder method. Period of interest expense: the last 12 months (`ttm`, default) or the latest full fiscal year (`annual`). Korea and Japan have annual figures only, so `annual` is what they get.
    /// - `risk_free`: Optional query builder method. Your own risk-free rate, in percent. Left out, it is the 10-year point of the country's government yield curve. Needed where we have no curve (Korea): without it the answer is 422 risk_free_unavailable.
    /// - `tax_rate`: Optional query builder method. Your own tax rate for the debt shield, in percent. Left out, it is the corporate tax rate of the stock's country from Damodaran's public table (PwC rates). Between 0 and 60.
    /// - `window`: Optional query builder method. How far back the returns reach: `2y` (default) or `5y`.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityCostOfCapital` with header metadata.
    pub fn cost_of_capital(&self, id: impl AsRef<str>) -> EquitiesCostOfCapitalRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/cost-of-capital",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesCostOfCapitalRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Dividends of one stock
    /// Cash dividends of one listing, newest ex-date first, paged. The list may include announced future rows. The plan sets how far back the past goes: Free 12 months, Basic 5 years, Pro full. `start` is sent only when older rows were cut off; upcoming rows are never cut. Each request counts one call. An unknown id is 404 not_found; a market without data yet is 404 not_covered.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    /// - `cursor`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityDividends` with header metadata.
    /// Its `iter()` yields `EquityDividend` lazily across pages.
    pub fn dividends(&self, id: impl AsRef<str>) -> EquitiesDividendsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/dividends",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesDividendsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Earnings of one stock
    /// Past earnings reports of one listing (dated today or earlier at the exchange), with estimate, actual and surprise, newest first, paged. Upcoming report dates are not included yet. `surprise_percent` is in percent. `eps_actual` is on the market data basis and can differ from the filings EPS in the valuation. `filing_id` names the annual or quarterly report that covers a past report when we hold it. The plan sets how far back the past goes: Free 12 months, Basic 5 years, Pro full. `start` is sent only when older rows were cut off. Each request counts one call. An unknown id is 404 not_found; a market without data yet is 404 not_covered.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    /// - `cursor`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityEarnings` with header metadata.
    /// Its `iter()` yields `EquityEarning` lazily across pages.
    pub fn earnings(&self, id: impl AsRef<str>) -> EquitiesEarningsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/earnings",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesEarningsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Top US stock movers
    /// The top 25 US stocks and ETFs by `direction`: `gainers`, `losers` or `active` (most traded by value among the day's biggest gainers and losers, not the whole market). Only ordinary shares and ETFs priced at $1 or more, with at least 100,000 shares and $1 million traded today; ranked by us and cached for 10 minutes. `change_percent` is in percent. Basic and Pro only (Free gets 403 plan_upgrade_required). `market` is `us` for now; another market is 404 not_covered, an unknown `direction` is 400 invalid_query. One call.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `direction`: Optional query builder method. `gainers` (biggest rise), `losers` (biggest fall) or `active` (most traded by value among the day's biggest gainers and losers, not the whole market). Anything else is 400 invalid_query.
    /// - `market`: Optional query builder method. Market to rank. Only `us` for now; another market is 404 not_covered.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityMovers` with header metadata.
    pub fn movers(&self) -> EquitiesMoversRequest {
        let data = RequestData::new("GET", "/v1/equities/movers", &[], &[], &[], false);
        EquitiesMoversRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Latest price of one stock
    /// Latest price of one listing. Free plans see the price 15 minutes late and the answer carries `delayed: true`. `change_percent` is in percent. An unknown id is 404 not_found; a market without prices yet is 404 not_covered.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityPrice` with header metadata.
    pub fn price(&self, id: impl AsRef<str>) -> EquitiesPriceRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/price",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesPriceRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Latest prices for up to 50 stocks
    /// Latest price of each listed id. One call is counted for each id that returns a price, taken in one step: if fewer calls are left than ids that resolved, the whole request is refused with 429 quota_exceeded. Ids that return no price are listed in `errors` and cost nothing. Free plans see prices 15 minutes late. `change_percent` is in percent.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `ids`: Required query argument. Comma-separated listing ids, at most 50 after duplicates are removed. Each id follows the same rules as the single-price route. One call is counted for each id that returns a price.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityPrices` with header metadata.
    pub fn prices(&self, ids: impl AsRef<str>) -> EquitiesPricesRequest {
        let mut data = RequestData::new("GET", "/v1/equities/prices", &[], &["ids"], &[], false);
        data.query(
            "ids",
            vec![ids.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        EquitiesPricesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Profile of one stock
    /// The listing (symbol, name, type, exchange, country, currency), the company's sector, industry and website, and `company_id` when the listing matches a company in our filings. Open to every plan. Japan listings carry only name, exchange, country, currency and `company_id`. Fields that are not known are left out.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityProfile` with header metadata.
    pub fn profile(&self, id: impl AsRef<str>) -> EquitiesProfileRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesProfileRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Splits of one stock
    /// Share splits of one listing, newest first. The plan sets how far back the past goes: Free 12 months, Basic 5 years, Pro full. `start` is sent only when older rows were cut off; upcoming rows are never cut. Each request counts one call. An unknown id is 404 not_found; a market without data yet is 404 not_covered.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquitySplits` with header metadata.
    pub fn splits(&self, id: impl AsRef<str>) -> EquitiesSplitsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/splits",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesSplitsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Market cap, P/E, P/S, P/B and dividend yield of one stock
    /// Market cap (latest price times shares outstanding), P/E (latest price over trailing 12-month diluted EPS from company filings, GAAP or IFRS as reported), P/S (market cap over trailing 12-month revenue), P/B (market cap over total equity of the latest balance sheet) and dividend yield (trailing 12-month dividends per share over the price, in percent) of one US or Korean listing, with the inputs and the dates they are as of. Korea has P/E only for now; market cap, P/S, P/B and yield come with share counts and dividends. A figure that cannot be worked out correctly is left out, never sent as 0: market cap after a split since the share count, for several share classes without a count each, and for a depositary receipt or foreign filer; P/E when EPS is negative or under 0.01, reported in another currency, or for a depositary receipt. Another class of a company, such as a preferred line, has no figures of its own and shows none. Basic and Pro only (Free gets 403 plan_upgrade_required). A listing outside the US and Korea, or without a clean company match, is 404 not_covered. One call; paid plans see live prices.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityValuation` with header metadata.
    pub fn valuation(&self, id: impl AsRef<str>) -> EquitiesValuationRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/valuation",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesValuationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Valuation history of one stock
    /// Market cap, P/E, P/S, P/B and dividend yield at every fiscal period end (quarter or year end) of one US or Korean listing, oldest first. Each point is the closing price on or just before that day, adjusted for splits only, and the figures filed for that period (trailing 12-month EPS, revenue and dividends, total equity, the nearest share count), so a later restatement is in it. A figure that cannot be worked out is left out, never sent as 0, by the same rules as the valuation. History: Basic 5 years, Pro everything; `start` is sent back when the plan cut older periods. Basic and Pro only (Free gets 403 plan_upgrade_required). A listing outside the US and Korea, or without a clean company match, is 404 not_covered. One call.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Listing id: the exchange's four-letter code, a colon and the ticker, for example XNAS:MSFT or XNYS:BRK.B. A bare US ticker such as MSFT also works when it names one US listing; the answer carries the canonical id. Share classes use a dot (BRK.B); BRK-B is accepted.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityValuationHistory` with header metadata.
    pub fn valuation_history(&self, id: impl AsRef<str>) -> EquitiesValuationHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/equities/{id}/valuation/history",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EquitiesValuationHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Search stocks by ticker or name
    /// Listings whose ticker or name starts with `q`, any case: an exact ticker first, then other tickers, then names. Optional `country` (three letters) and `type` filters. Up to 50 a page, with `next_cursor`. Read from our own symbol list, so it is open to every plan and counts one call.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Optional query builder method. Only listings in this country, three letters (USA, KOR).
    /// - `cursor`: Optional query builder method. `next_cursor` of the previous page, with the same other parameters.
    /// - `q`: Required query argument. Start of a ticker or of a company name, any case, at least 2 characters (for example `ms` or `micro`). Order: the exact ticker, other tickers, the start of a name, then the start of a later word of a name; US primary listings first within each.
    /// - `type_`: Optional query builder method. Only this kind of listing.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquitySearchResults` with header metadata.
    /// Its `iter()` yields `EquitySearchResult` lazily across pages.
    pub fn search(&self, q: impl AsRef<str>) -> EquitiesSearchRequest {
        let mut data = RequestData::new("GET", "/v1/equities/search", &[], &["q"], &[], false);
        data.query(
            "q",
            vec![q.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        EquitiesSearchRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesBarsRequest {
    inner: Request<EquityBars, Client>,
}
impl EquitiesBarsRequest {
    /// `next_cursor` of the previous page, with the same other parameters. A cursor stops working when the prices behind it changed (a split or dividend): 409 cursor_expired.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last bar. `1d`: a date, inclusive; left out means the latest session whose bar is out. `1m`: a UTC time, exclusive; left out means now, or five days after `start`. At most 5 days between `start` and `end` for `1m` (400 range_too_large).
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// `1d` (one bar per session) or `1m` (one-minute bars, Basic and Pro only). Anything else is 400 invalid_interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First bar. `1d`: a date (YYYY-MM-DD), inclusive; left out means the oldest bar your plan reaches. `1m`: a UTC time (YYYY-MM-DDTHH:MM:SSZ, or a date for 00:00 UTC), inclusive; left out means one day before `end`.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityBars`.
    pub fn send(self) -> Result<EquityBars, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityBars, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EquityBars> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesCostOfCapitalRequest {
    inner: Request<EquityCostOfCapital, Client>,
}
impl EquitiesCostOfCapitalRequest {
    /// Listing id of the market proxy beta is measured against: an ETF, fund or stock priced in the same currency as the stock. Left out, it is ARCX:SPY for a US stock, XKRX:069500 for a Korean one and XTKS:1306 for a Japanese one; any other market needs it.
    pub fn benchmark(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "benchmark",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Which beta feeds the cost of equity: `raw` (default) or `adjusted` (0.67 raw plus 0.33). Both are always in the answer.
    pub fn beta(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "beta",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Your equity risk premium, in percent (5 means 5%). Left out, it is our own published total equity risk premium of the stock's country (the answer names its date and basis). Where we publish none, without it the answer is 400 erp_required. Above 0 and at most 25.
    pub fn erp(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "erp",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Return interval: `weekly` (default; the last common trading day of each ISO week) or `monthly`.
    pub fn frequency(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "frequency",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Period of interest expense: the last 12 months (`ttm`, default) or the latest full fiscal year (`annual`). Korea and Japan have annual figures only, so `annual` is what they get.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Your own risk-free rate, in percent. Left out, it is the 10-year point of the country's government yield curve. Needed where we have no curve (Korea): without it the answer is 422 risk_free_unavailable.
    pub fn risk_free(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "risk_free",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Your own tax rate for the debt shield, in percent. Left out, it is the corporate tax rate of the stock's country from Damodaran's public table (PwC rates). Between 0 and 60.
    pub fn tax_rate(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "tax_rate",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// How far back the returns reach: `2y` (default) or `5y`.
    pub fn window(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "window",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityCostOfCapital`.
    pub fn send(self) -> Result<EquityCostOfCapital, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityCostOfCapital, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesDividendsRequest {
    inner: Request<EquityDividends, Client>,
}
impl EquitiesDividendsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityDividends`.
    pub fn send(self) -> Result<EquityDividends, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityDividends, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EquityDividends> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesEarningsRequest {
    inner: Request<EquityEarnings, Client>,
}
impl EquitiesEarningsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityEarnings`.
    pub fn send(self) -> Result<EquityEarnings, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityEarnings, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EquityEarnings> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesMoversRequest {
    inner: Request<EquityMovers, Client>,
}
impl EquitiesMoversRequest {
    /// `gainers` (biggest rise), `losers` (biggest fall) or `active` (most traded by value among the day's biggest gainers and losers, not the whole market). Anything else is 400 invalid_query.
    pub fn direction(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "direction",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Market to rank. Only `us` for now; another market is 404 not_covered.
    pub fn market(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "market",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityMovers`.
    pub fn send(self) -> Result<EquityMovers, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityMovers, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesPriceRequest {
    inner: Request<EquityPrice, Client>,
}
impl EquitiesPriceRequest {
    /// Send the request and decode `EquityPrice`.
    pub fn send(self) -> Result<EquityPrice, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityPrice, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesPricesRequest {
    inner: Request<EquityPrices, Client>,
}
impl EquitiesPricesRequest {
    /// Send the request and decode `EquityPrices`.
    pub fn send(self) -> Result<EquityPrices, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityPrices, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesProfileRequest {
    inner: Request<EquityProfile, Client>,
}
impl EquitiesProfileRequest {
    /// Send the request and decode `EquityProfile`.
    pub fn send(self) -> Result<EquityProfile, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityProfile, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesSplitsRequest {
    inner: Request<EquitySplits, Client>,
}
impl EquitiesSplitsRequest {
    /// Send the request and decode `EquitySplits`.
    pub fn send(self) -> Result<EquitySplits, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquitySplits, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesValuationRequest {
    inner: Request<EquityValuation, Client>,
}
impl EquitiesValuationRequest {
    /// Send the request and decode `EquityValuation`.
    pub fn send(self) -> Result<EquityValuation, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityValuation, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesValuationHistoryRequest {
    inner: Request<EquityValuationHistory, Client>,
}
impl EquitiesValuationHistoryRequest {
    /// Send the request and decode `EquityValuationHistory`.
    pub fn send(self) -> Result<EquityValuationHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityValuationHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EquitiesSearchRequest {
    inner: Request<EquitySearchResults, Client>,
}
impl EquitiesSearchRequest {
    /// Only listings in this country, three letters (USA, KOR).
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// `next_cursor` of the previous page, with the same other parameters.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Only this kind of listing.
    pub fn type_(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquitySearchResults`.
    pub fn send(self) -> Result<EquitySearchResults, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquitySearchResults, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EquitySearchResults> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
impl Client {
    pub fn events(&self) -> Events {
        Events {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Events {
    client: Client,
}
impl Events {
    /// Get Delivery Event
    /// One delivery event by ID.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument. Event ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EventDetail` with header metadata.
    pub fn get(&self, event_id: impl AsRef<str>) -> EventsGetRequest {
        let data = RequestData::new(
            "GET",
            "/v1/events/{event_id}",
            &[("event_id", event_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        EventsGetRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Delivery Events
    /// A page of delivery events, oldest first. `next_cursor` is null at the head.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `countries`: Optional query builder method. Uppercase three-letter country codes (EMU for the euro area). Omit for all.
    /// - `cursor`: Optional query builder method. Event position to read from: the cursor or next_cursor of an earlier page.
    /// - `limit`: Optional query builder method. Maximum events per page.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EventsPage` with header metadata.
    /// Its `iter()` yields `DeliveryEvent` lazily across pages.
    pub fn list(&self) -> EventsListRequest {
        let data = RequestData::new("GET", "/v1/events", &[], &[], &[], false);
        EventsListRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EventsGetRequest {
    inner: Request<EventDetail, Client>,
}
impl EventsGetRequest {
    /// Send the request and decode `EventDetail`.
    pub fn send(self) -> Result<EventDetail, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EventDetail, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EventsListRequest {
    inner: Request<EventsPage, Client>,
}
impl EventsListRequest {
    /// Uppercase three-letter country codes (EMU for the euro area). Omit for all.
    pub fn countries(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.query(
            "countries",
            value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            "repeat",
            false,
        );
        self
    }
    /// Event position to read from: the cursor or next_cursor of an earlier page.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Maximum events per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Send the request and decode `EventsPage`.
    pub fn send(self) -> Result<EventsPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EventsPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EventsPage> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
impl Client {
    pub fn filings(&self) -> Filings {
        Filings {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Filings {
    client: Client,
}
impl Filings {
    /// Get As Reported
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `filing_id`: Required path argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `AsReportedPage` with header metadata.
    pub fn as_reported(&self, filing_id: impl AsRef<str>) -> FilingsAsReportedRequest {
        let data = RequestData::new(
            "GET",
            "/v1/filings/{filing_id}/financials/as-reported",
            &[("filing_id", filing_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FilingsAsReportedRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Company
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `company_id`: Required path argument.
    /// - `as_of`: Optional query builder method.
    /// - `lang`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CompanyWithListings` with header metadata.
    pub fn company(&self, company_id: impl AsRef<str>) -> FilingsCompanyRequest {
        let data = RequestData::new(
            "GET",
            "/v1/companies/{company_id}",
            &[("company_id", company_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FilingsCompanyRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Filing
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `filing_id`: Required path argument.
    /// - `lang`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `Filing` with header metadata.
    pub fn filing(&self, filing_id: impl AsRef<str>) -> FilingsFilingRequest {
        let data = RequestData::new(
            "GET",
            "/v1/filings/{filing_id}",
            &[("filing_id", filing_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FilingsFilingRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Financials
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `company_id`: Required path argument.
    /// - `as_of`: Optional query builder method. Pro plan only.
    /// - `basis`: Optional query builder method.
    /// - `fields`: Optional query builder method.
    /// - `fiscal_year`: Optional query builder method.
    /// - `period`: Optional query builder method.
    /// - `statement`: Optional query builder method.
    /// - `version`: Optional query builder method. first_reported is Pro plan only.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `Financials` with header metadata.
    pub fn financials(&self, company_id: impl AsRef<str>) -> FilingsFinancialsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/companies/{company_id}/financials",
            &[("company_id", company_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FilingsFinancialsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Company Filings
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `company_id`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `filed_from`: Optional query builder method.
    /// - `filed_to`: Optional query builder method.
    /// - `form`: Optional query builder method.
    /// - `include_amendments`: Optional query builder method.
    /// - `lang`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `type_`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CompanyFilingsPage` with header metadata.
    /// Its `iter()` yields `ListedFiling` lazily across pages.
    pub fn list_company_filings(
        &self,
        company_id: impl AsRef<str>,
    ) -> FilingsListCompanyFilingsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/companies/{company_id}/filings",
            &[("company_id", company_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FilingsListCompanyFilingsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Holders
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `company_id`: Required path argument.
    /// - `lang`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `security`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `HoldersPage` with header metadata.
    pub fn list_holders(&self, company_id: impl AsRef<str>) -> FilingsListHoldersRequest {
        let data = RequestData::new(
            "GET",
            "/v1/companies/{company_id}/holders",
            &[("company_id", company_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FilingsListHoldersRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Latest Filings
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Optional query builder method.
    /// - `lang`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `type_`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `LatestFilingsPage` with header metadata.
    pub fn list_latest_filings(&self) -> FilingsListLatestFilingsRequest {
        let data = RequestData::new("GET", "/v1/filings/latest", &[], &[], &[], false);
        FilingsListLatestFilingsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Search Companies
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `as_of`: Optional query builder method.
    /// - `country`: Optional query builder method.
    /// - `lang`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `q`: Optional query builder method.
    /// - `registry`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `CompanySearchPage` with header metadata.
    pub fn search_companies(&self) -> FilingsSearchCompaniesRequest {
        let data = RequestData::new("GET", "/v1/companies", &[], &[], &[], false);
        FilingsSearchCompaniesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsAsReportedRequest {
    inner: Request<AsReportedPage, Client>,
}
impl FilingsAsReportedRequest {
    /// Send the request and decode `AsReportedPage`.
    pub fn send(self) -> Result<AsReportedPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(AsReportedPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsCompanyRequest {
    inner: Request<CompanyWithListings, Client>,
}
impl FilingsCompanyRequest {
    /// Set as_of.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set lang.
    pub fn lang(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "lang",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CompanyWithListings`.
    pub fn send(self) -> Result<CompanyWithListings, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CompanyWithListings, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsFilingRequest {
    inner: Request<Filing, Client>,
}
impl FilingsFilingRequest {
    /// Set lang.
    pub fn lang(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "lang",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `Filing`.
    pub fn send(self) -> Result<Filing, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(Filing, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsFinancialsRequest {
    inner: Request<Financials, Client>,
}
impl FilingsFinancialsRequest {
    /// Pro plan only.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set basis.
    pub fn basis(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "basis",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set fields.
    pub fn fields(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "fields",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set fiscal_year.
    pub fn fiscal_year(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("fiscal_year", vec![value.to_string()], "single", false);
        self
    }
    /// Set period.
    pub fn period(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "period",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set statement.
    pub fn statement(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "statement",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// first_reported is Pro plan only.
    pub fn version(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "version",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `Financials`.
    pub fn send(self) -> Result<Financials, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(Financials, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsListCompanyFilingsRequest {
    inner: Request<CompanyFilingsPage, Client>,
}
impl FilingsListCompanyFilingsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set filed_from.
    pub fn filed_from(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "filed_from",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set filed_to.
    pub fn filed_to(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "filed_to",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set form.
    pub fn form(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "form",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set include_amendments.
    pub fn include_amendments(mut self, value: bool) -> Self {
        self.inner.data.query(
            "include_amendments",
            vec![value.to_string()],
            "single",
            false,
        );
        self
    }
    /// Set lang.
    pub fn lang(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "lang",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set type.
    pub fn type_(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CompanyFilingsPage`.
    pub fn send(self) -> Result<CompanyFilingsPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CompanyFilingsPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<CompanyFilingsPage> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsListHoldersRequest {
    inner: Request<HoldersPage, Client>,
}
impl FilingsListHoldersRequest {
    /// Set lang.
    pub fn lang(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "lang",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set security.
    pub fn security(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "security",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `HoldersPage`.
    pub fn send(self) -> Result<HoldersPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(HoldersPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsListLatestFilingsRequest {
    inner: Request<LatestFilingsPage, Client>,
}
impl FilingsListLatestFilingsRequest {
    /// Set country.
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set lang.
    pub fn lang(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "lang",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set type.
    pub fn type_(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `LatestFilingsPage`.
    pub fn send(self) -> Result<LatestFilingsPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(LatestFilingsPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FilingsSearchCompaniesRequest {
    inner: Request<CompanySearchPage, Client>,
}
impl FilingsSearchCompaniesRequest {
    /// Set as_of.
    pub fn as_of(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "as_of",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set country.
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set lang.
    pub fn lang(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "lang",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set q.
    pub fn q(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "q",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set registry.
    pub fn registry(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "registry",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `CompanySearchPage`.
    pub fn send(self) -> Result<CompanySearchPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(CompanySearchPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn funds(&self) -> Funds {
        Funds {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Funds {
    client: Client,
}
impl Funds {
    /// Price bars of one fund
    /// Daily (`1d`) or one-minute (`1m`) bars, oldest first, in the stock bar shape. A mutual fund's daily bar is its daily NAV (open, high, low and close are the NAV, no volume); it has no one-minute bars (400 invalid_interval). One-minute bars are for ETFs on Basic and Pro (Free gets 403 plan_upgrade_required). History: Free 12 months, Basic 5 years, Pro everything; `start` is sent back when the plan raised it. Up to 5,000 bars a page, with `next_cursor`; a cursor that no longer matches the prices is 409 cursor_expired. One call a page.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Fund id. An ETF is its listing id: the exchange's four-letter code, a colon and the ticker (ARCX:SPY). A mutual fund is FUND, a colon and its ticker (FUND:VFIAX). A bare US ticker such as SPY also works when it names one US listing; the answer carries the canonical id.
    /// - `cursor`: Optional query builder method. `next_cursor` of the previous page, with the same other parameters. A cursor stops working when the prices behind it changed (a split or dividend): 409 cursor_expired.
    /// - `end`: Optional query builder method. Last bar. `1d`: a date, inclusive; left out means the latest session whose bar is out. `1m`: a UTC time, exclusive; left out means now, or five days after `start`. At most 5 days between `start` and `end` for `1m` (400 range_too_large).
    /// - `interval`: Optional query builder method. `1d` (one bar per session; a mutual fund's bar is its daily NAV) or `1m` (one-minute bars: ETFs only, Basic and Pro only). Anything else is 400 invalid_interval.
    /// - `start`: Optional query builder method. First bar. `1d`: a date (YYYY-MM-DD), inclusive; left out means the oldest bar your plan reaches. `1m`: a UTC time (YYYY-MM-DDTHH:MM:SSZ, or a date for 00:00 UTC), inclusive; left out means one day before `end`.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `EquityBars` with header metadata.
    /// Its `iter()` yields `EquityBar` lazily across pages.
    pub fn bars(&self, id: impl AsRef<str>) -> FundsBarsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/funds/{id}/bars",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FundsBarsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Distributions of one fund
    /// Distributions of one fund, newest ex-date first, paged. The plan sets how far back the past goes: Free 12 months, Basic 5 years, Pro full; `start` is sent only when older rows were cut off. Each request counts one call. An unknown id is 404 not_found.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Fund id. An ETF is its listing id: the exchange's four-letter code, a colon and the ticker (ARCX:SPY). A mutual fund is FUND, a colon and its ticker (FUND:VFIAX). A bare US ticker such as SPY also works when it names one US listing; the answer carries the canonical id.
    /// - `cursor`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FundDistributions` with header metadata.
    /// Its `iter()` yields `FundDistribution` lazily across pages.
    pub fn distributions(&self, id: impl AsRef<str>) -> FundsDistributionsRequest {
        let data = RequestData::new(
            "GET",
            "/v1/funds/{id}/distributions",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FundsDistributionsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Latest price of one fund
    /// An ETF's last price, in the stock price shape plus `type: etf` (Free sees it 15 minutes late and the answer carries `delayed: true`), or a mutual fund's latest NAV (`type: mutual_fund`: `nav`, `previous_nav`, `change`, `change_percent` and the NAV's `date`; an end-of-day value, never delayed, `stale` once it is older than the next business day). `change_percent` is in percent. A stock id is 404 not_found.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Fund id. An ETF is its listing id: the exchange's four-letter code, a colon and the ticker (ARCX:SPY). A mutual fund is FUND, a colon and its ticker (FUND:VFIAX). A bare US ticker such as SPY also works when it names one US listing; the answer carries the canonical id.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FundPrice` with header metadata.
    pub fn price(&self, id: impl AsRef<str>) -> FundsPriceRequest {
        let data = RequestData::new(
            "GET",
            "/v1/funds/{id}/price",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FundsPriceRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Latest prices for up to 50 funds
    /// Latest price of each listed fund: an ETF's last price (Free sees it 15 minutes late) or a mutual fund's latest NAV. One call is counted for each id that returns a price, taken in one step: if fewer calls are left than ids that resolved, the whole request is refused with 429 quota_exceeded. Ids that return no price are listed in `errors` and cost nothing. `change_percent` is in percent.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `ids`: Required query argument. Comma-separated fund ids, at most 50 after duplicates are removed. Each id follows the same rules as the single-price route. One call is counted for each id that returns a price.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FundPrices` with header metadata.
    pub fn prices(&self, ids: impl AsRef<str>) -> FundsPricesRequest {
        let mut data = RequestData::new("GET", "/v1/funds/prices", &[], &["ids"], &[], false);
        data.query(
            "ids",
            vec![ids.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        FundsPricesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Profile of one fund
    /// The fund's symbol, name, type (`etf` or `mutual_fund`), exchange (ETFs only), country and currency, as our symbol list holds them. Open to every plan. Fields that are not known are left out.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `id`: Required path argument. Fund id. An ETF is its listing id: the exchange's four-letter code, a colon and the ticker (ARCX:SPY). A mutual fund is FUND, a colon and its ticker (FUND:VFIAX). A bare US ticker such as SPY also works when it names one US listing; the answer carries the canonical id.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FundProfile` with header metadata.
    pub fn profile(&self, id: impl AsRef<str>) -> FundsProfileRequest {
        let data = RequestData::new(
            "GET",
            "/v1/funds/{id}",
            &[("id", id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FundsProfileRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Search ETFs and mutual funds by ticker or name
    /// Funds whose ticker or name starts with `q`, any case: an exact ticker first, then other tickers, then names. Optional `type` (`etf` or `mutual_fund`) and `country` (three letters) filters. Up to 50 a page, with `next_cursor`. Read from our own symbol list, so it is open to every plan and counts one call.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `country`: Optional query builder method. Only funds in this country, three letters (USA).
    /// - `cursor`: Optional query builder method. `next_cursor` of the previous page, with the same other parameters.
    /// - `q`: Required query argument. Start of a ticker or of a fund name, any case, at least 2 characters (for example `vf` or `vanguard 500`). Order: the exact ticker, other tickers, the start of a name, then the start of a later word of a name.
    /// - `type_`: Optional query builder method. Only ETFs or only mutual funds.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FundSearchResults` with header metadata.
    /// Its `iter()` yields `FundSearchResult` lazily across pages.
    pub fn search(&self, q: impl AsRef<str>) -> FundsSearchRequest {
        let mut data = RequestData::new("GET", "/v1/funds/search", &[], &["q"], &[], false);
        data.query(
            "q",
            vec![q.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        FundsSearchRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FundsBarsRequest {
    inner: Request<EquityBars, Client>,
}
impl FundsBarsRequest {
    /// `next_cursor` of the previous page, with the same other parameters. A cursor stops working when the prices behind it changed (a split or dividend): 409 cursor_expired.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Last bar. `1d`: a date, inclusive; left out means the latest session whose bar is out. `1m`: a UTC time, exclusive; left out means now, or five days after `start`. At most 5 days between `start` and `end` for `1m` (400 range_too_large).
    pub fn end(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "end",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// `1d` (one bar per session; a mutual fund's bar is its daily NAV) or `1m` (one-minute bars: ETFs only, Basic and Pro only). Anything else is 400 invalid_interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// First bar. `1d`: a date (YYYY-MM-DD), inclusive; left out means the oldest bar your plan reaches. `1m`: a UTC time (YYYY-MM-DDTHH:MM:SSZ, or a date for 00:00 UTC), inclusive; left out means one day before `end`.
    pub fn start(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "start",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `EquityBars`.
    pub fn send(self) -> Result<EquityBars, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(EquityBars, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<EquityBars> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FundsDistributionsRequest {
    inner: Request<FundDistributions, Client>,
}
impl FundsDistributionsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `FundDistributions`.
    pub fn send(self) -> Result<FundDistributions, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FundDistributions, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<FundDistributions> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FundsPriceRequest {
    inner: Request<FundPrice, Client>,
}
impl FundsPriceRequest {
    /// Send the request and decode `FundPrice`.
    pub fn send(self) -> Result<FundPrice, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FundPrice, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FundsPricesRequest {
    inner: Request<FundPrices, Client>,
}
impl FundsPricesRequest {
    /// Send the request and decode `FundPrices`.
    pub fn send(self) -> Result<FundPrices, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FundPrices, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FundsProfileRequest {
    inner: Request<FundProfile, Client>,
}
impl FundsProfileRequest {
    /// Send the request and decode `FundProfile`.
    pub fn send(self) -> Result<FundProfile, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FundProfile, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FundsSearchRequest {
    inner: Request<FundSearchResults, Client>,
}
impl FundsSearchRequest {
    /// Only funds in this country, three letters (USA).
    pub fn country(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "country",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// `next_cursor` of the previous page, with the same other parameters.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Only ETFs or only mutual funds.
    pub fn type_(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "type",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `FundSearchResults`.
    pub fn send(self) -> Result<FundSearchResults, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FundSearchResults, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<FundSearchResults> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
impl Client {
    pub fn fx(&self) -> Fx {
        Fx {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Fx {
    client: Client,
}
impl Fx {
    /// Convert Fx
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `amount`: Required query argument.
    /// - `from`: Required query argument.
    /// - `to`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FxConversion` with header metadata.
    pub fn convert_currency(
        &self,
        amount: impl AsRef<str>,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> FxConvertCurrencyRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/fx/convert",
            &[],
            &["amount", "from", "to"],
            &[],
            false,
        );
        data.query(
            "amount",
            vec![amount.as_ref().to_owned().to_string()],
            "single",
            true,
        );
        data.query(
            "from",
            vec![from.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "to",
            vec![to.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        FxConvertCurrencyRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Fx Rate Bars
    /// Closed open-high-low-close bars for one currency against USD, from one minute to one day. A request may span at most 3 days of 1m bars, 14 days of 5m bars, 60 days of 15m bars; a longer range returns 400. Wider intervals have no cap.
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `base`: Required path argument.
    /// - `currency`: Required query argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Required query argument.
    /// - `interval`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FxBars` with header metadata.
    /// Its `iter()` yields `FxBar` lazily across pages.
    pub fn rate_bars(
        &self,
        base: impl AsRef<str>,
        currency: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> FxRateBarsRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/fx/rates/{base}/bars",
            &[("base", base.as_ref().to_owned())],
            &["currency", "end", "start"],
            &[],
            false,
        );
        data.query(
            "currency",
            vec![currency.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        FxRateBarsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Fx Rate History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `base`: Required path argument.
    /// - `currencies`: Required query argument.
    /// - `cursor`: Optional query builder method.
    /// - `end_date`: Required query argument.
    /// - `limit`: Optional query builder method.
    /// - `start_date`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FxHistory` with header metadata.
    /// Its `iter()` yields `FxHistoricalObservation` lazily across pages.
    pub fn rate_history(
        &self,
        base: impl AsRef<str>,
        currencies: impl IntoIterator<Item = impl AsRef<str>>,
        end_date: impl AsRef<str>,
        start_date: impl AsRef<str>,
    ) -> FxRateHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/fx/rates/{base}/observations",
            &[("base", base.as_ref().to_owned())],
            &["currencies", "end_date", "start_date"],
            &[],
            false,
        );
        data.query(
            "currencies",
            currencies
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            "comma_join",
            false,
        );
        data.query(
            "end_date",
            vec![end_date.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start_date",
            vec![start_date.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        FxRateHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Fx Rates
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `base`: Required path argument.
    /// - `currencies`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `FxRates` with header metadata.
    pub fn rates(&self, base: impl AsRef<str>) -> FxRatesRequest {
        let data = RequestData::new(
            "GET",
            "/v1/fx/rates/{base}",
            &[("base", base.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        FxRatesRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FxConvertCurrencyRequest {
    inner: Request<FxConversion, Client>,
}
impl FxConvertCurrencyRequest {
    /// Send the request and decode `FxConversion`.
    pub fn send(self) -> Result<FxConversion, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FxConversion, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FxRateBarsRequest {
    inner: Request<FxBars, Client>,
}
impl FxRateBarsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Send the request and decode `FxBars`.
    pub fn send(self) -> Result<FxBars, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FxBars, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<FxBars> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FxRateHistoryRequest {
    inner: Request<FxHistory, Client>,
}
impl FxRateHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Send the request and decode `FxHistory`.
    pub fn send(self) -> Result<FxHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FxHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<FxHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FxRatesRequest {
    inner: Request<FxRates, Client>,
}
impl FxRatesRequest {
    /// Set currencies.
    pub fn currencies(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.query(
            "currencies",
            value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            "comma_join",
            false,
        );
        self
    }
    /// Send the request and decode `FxRates`.
    pub fn send(self) -> Result<FxRates, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(FxRates, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn predictions(&self) -> Predictions {
        Predictions {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Predictions {
    client: Client,
}
impl Predictions {
    /// Get Binary Prediction
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `at`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `BinaryEvent` with header metadata.
    pub fn binary(&self, event_id: impl AsRef<str>) -> PredictionsBinaryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/predictions/binary/{event_id}",
            &[("event_id", event_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        PredictionsBinaryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Binary Prediction History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Required query argument.
    /// - `interval`: Optional query builder method.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionHistory` with header metadata.
    /// Its `iter()` yields `PredictionSeries` lazily across pages.
    pub fn binary_history(
        &self,
        event_id: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> PredictionsBinaryHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/predictions/binary/{event_id}/history",
            &[("event_id", event_id.as_ref().to_owned())],
            &["end", "start"],
            &[],
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        PredictionsBinaryHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Extreme Prediction
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `at`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `ExtremeEvent` with header metadata.
    pub fn extreme(&self, event_id: impl AsRef<str>) -> PredictionsExtremeRequest {
        let data = RequestData::new(
            "GET",
            "/v1/predictions/extreme/{event_id}",
            &[("event_id", event_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        PredictionsExtremeRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Extreme Prediction History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Required query argument.
    /// - `interval`: Optional query builder method.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionHistory` with header metadata.
    /// Its `iter()` yields `PredictionSeries` lazily across pages.
    pub fn extreme_history(
        &self,
        event_id: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> PredictionsExtremeHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/predictions/extreme/{event_id}/history",
            &[("event_id", event_id.as_ref().to_owned())],
            &["end", "start"],
            &[],
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        PredictionsExtremeHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Policy Path
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `authority`: Required path argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PolicyPath` with header metadata.
    pub fn policy_path(&self, authority: impl AsRef<str>) -> PredictionsPolicyPathRequest {
        let data = RequestData::new(
            "GET",
            "/v1/predictions/policy-paths/{authority}",
            &[("authority", authority.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        PredictionsPolicyPathRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Policy Path History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `authority`: Required path argument.
    /// - `end`: Required query argument.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PolicyPathHistory` with header metadata.
    pub fn policy_path_history(
        &self,
        authority: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> PredictionsPolicyPathHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/predictions/policy-paths/{authority}/history",
            &[("authority", authority.as_ref().to_owned())],
            &["end", "start"],
            &[],
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        PredictionsPolicyPathHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Scalar Prediction
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `at`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `ScalarEvent` with header metadata.
    pub fn scalar(&self, event_id: impl AsRef<str>) -> PredictionsScalarRequest {
        let data = RequestData::new(
            "GET",
            "/v1/predictions/scalar/{event_id}",
            &[("event_id", event_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        PredictionsScalarRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Scalar Prediction History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Required query argument.
    /// - `interval`: Optional query builder method.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionHistory` with header metadata.
    /// Its `iter()` yields `PredictionSeries` lazily across pages.
    pub fn scalar_history(
        &self,
        event_id: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> PredictionsScalarHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/predictions/scalar/{event_id}/history",
            &[("event_id", event_id.as_ref().to_owned())],
            &["end", "start"],
            &[],
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        PredictionsScalarHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Timing Prediction
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `at`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `TimingEvent` with header metadata.
    pub fn timing(&self, event_id: impl AsRef<str>) -> PredictionsTimingRequest {
        let data = RequestData::new(
            "GET",
            "/v1/predictions/timing/{event_id}",
            &[("event_id", event_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        PredictionsTimingRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Timing Prediction History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `event_id`: Required path argument.
    /// - `cursor`: Optional query builder method.
    /// - `end`: Required query argument.
    /// - `interval`: Optional query builder method.
    /// - `start`: Required query argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionHistory` with header metadata.
    /// Its `iter()` yields `PredictionSeries` lazily across pages.
    pub fn timing_history(
        &self,
        event_id: impl AsRef<str>,
        end: impl AsRef<str>,
        start: impl AsRef<str>,
    ) -> PredictionsTimingHistoryRequest {
        let mut data = RequestData::new(
            "GET",
            "/v1/predictions/timing/{event_id}/history",
            &[("event_id", event_id.as_ref().to_owned())],
            &["end", "start"],
            &[],
            false,
        );
        data.query(
            "end",
            vec![end.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        data.query(
            "start",
            vec![start.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        PredictionsTimingHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Prediction Topic
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `topic_id`: Required path argument.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionTopicDetail` with header metadata.
    pub fn get_topic(&self, topic_id: impl AsRef<str>) -> PredictionsGetTopicRequest {
        let data = RequestData::new(
            "GET",
            "/v1/predictions/topics/{topic_id}",
            &[("topic_id", topic_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        PredictionsGetTopicRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Prediction Events
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `cursor`: Optional query builder method.
    /// - `from`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    /// - `release_id`: Optional query builder method.
    /// - `shape`: Optional query builder method.
    /// - `status`: Optional query builder method.
    /// - `to`: Optional query builder method.
    /// - `topic`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionEventsPage` with header metadata.
    /// Its `iter()` yields `PredictionEventSummary` lazily across pages.
    pub fn list_events(&self) -> PredictionsListEventsRequest {
        let data = RequestData::new("GET", "/v1/predictions/events", &[], &[], &[], false);
        PredictionsListEventsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Prediction Topics
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `group`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `PredictionTopics` with header metadata.
    pub fn list_topics(&self) -> PredictionsListTopicsRequest {
        let data = RequestData::new("GET", "/v1/predictions/topics", &[], &[], &[], false);
        PredictionsListTopicsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsBinaryRequest {
    inner: Request<BinaryEvent, Client>,
}
impl PredictionsBinaryRequest {
    /// Set at.
    pub fn at(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "at",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `BinaryEvent`.
    pub fn send(self) -> Result<BinaryEvent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(BinaryEvent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsBinaryHistoryRequest {
    inner: Request<PredictionHistory, Client>,
}
impl PredictionsBinaryHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PredictionHistory`.
    pub fn send(self) -> Result<PredictionHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PredictionHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsExtremeRequest {
    inner: Request<ExtremeEvent, Client>,
}
impl PredictionsExtremeRequest {
    /// Set at.
    pub fn at(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "at",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `ExtremeEvent`.
    pub fn send(self) -> Result<ExtremeEvent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(ExtremeEvent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsExtremeHistoryRequest {
    inner: Request<PredictionHistory, Client>,
}
impl PredictionsExtremeHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PredictionHistory`.
    pub fn send(self) -> Result<PredictionHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PredictionHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsPolicyPathRequest {
    inner: Request<PolicyPath, Client>,
}
impl PredictionsPolicyPathRequest {
    /// Send the request and decode `PolicyPath`.
    pub fn send(self) -> Result<PolicyPath, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PolicyPath, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsPolicyPathHistoryRequest {
    inner: Request<PolicyPathHistory, Client>,
}
impl PredictionsPolicyPathHistoryRequest {
    /// Send the request and decode `PolicyPathHistory`.
    pub fn send(self) -> Result<PolicyPathHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PolicyPathHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsScalarRequest {
    inner: Request<ScalarEvent, Client>,
}
impl PredictionsScalarRequest {
    /// Set at.
    pub fn at(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "at",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `ScalarEvent`.
    pub fn send(self) -> Result<ScalarEvent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(ScalarEvent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsScalarHistoryRequest {
    inner: Request<PredictionHistory, Client>,
}
impl PredictionsScalarHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PredictionHistory`.
    pub fn send(self) -> Result<PredictionHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PredictionHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsTimingRequest {
    inner: Request<TimingEvent, Client>,
}
impl PredictionsTimingRequest {
    /// Set at.
    pub fn at(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "at",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `TimingEvent`.
    pub fn send(self) -> Result<TimingEvent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(TimingEvent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsTimingHistoryRequest {
    inner: Request<PredictionHistory, Client>,
}
impl PredictionsTimingHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set interval.
    pub fn interval(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "interval",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PredictionHistory`.
    pub fn send(self) -> Result<PredictionHistory, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionHistory, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PredictionHistory> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsGetTopicRequest {
    inner: Request<PredictionTopicDetail, Client>,
}
impl PredictionsGetTopicRequest {
    /// Send the request and decode `PredictionTopicDetail`.
    pub fn send(self) -> Result<PredictionTopicDetail, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionTopicDetail, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsListEventsRequest {
    inner: Request<PredictionEventsPage, Client>,
}
impl PredictionsListEventsRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set from.
    pub fn from(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "from",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Set release_id.
    pub fn release_id(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "release_id",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set shape.
    pub fn shape(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "shape",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set status.
    pub fn status(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "status",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set to.
    pub fn to(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "to",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set topic.
    pub fn topic(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "topic",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PredictionEventsPage`.
    pub fn send(self) -> Result<PredictionEventsPage, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionEventsPage, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<PredictionEventsPage> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PredictionsListTopicsRequest {
    inner: Request<PredictionTopics, Client>,
}
impl PredictionsListTopicsRequest {
    /// Set group.
    pub fn group(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "group",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Send the request and decode `PredictionTopics`.
    pub fn send(self) -> Result<PredictionTopics, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(PredictionTopics, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
impl Client {
    pub fn webhooks(&self) -> Webhooks {
        Webhooks {
            client: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Webhooks {
    client: Client,
}
impl Webhooks {
    /// Configuration History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookHistoryResponse` with header metadata.
    pub fn configuration_history(
        &self,
        destination_id: impl AsRef<str>,
    ) -> WebhooksConfigurationHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/webhooks/destinations/{destination_id}/history",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        WebhooksConfigurationHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Create Destination
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `companies`: Optional body builder method. Companies to watch, as `usa:0000320193`. Required with the `filings` family and refused without it: only filings of a listed company (or, for 13D/G, its subject) are sent. Basic 50, Pro 1,000.
    /// - `countries`: Required body builder method. Countries from the active profile's catalogue.
    /// - `enabled`: Optional body builder method.
    /// - `families`: Required body builder method.
    /// - `url`: Required body builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookDestinationCreated` with header metadata.
    pub fn create_destination(&self) -> WebhooksCreateDestinationRequest {
        let data = RequestData::new(
            "POST",
            "/v1/webhooks/destinations",
            &[],
            &[],
            &["countries", "families", "url"],
            true,
        );
        WebhooksCreateDestinationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Delete Destination
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `NoContent` with header metadata.
    pub fn delete_destination(
        &self,
        destination_id: impl AsRef<str>,
    ) -> WebhooksDeleteDestinationRequest {
        let data = RequestData::new(
            "DELETE",
            "/v1/webhooks/destinations/{destination_id}",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        WebhooksDeleteDestinationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Delivery History
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    /// - `cursor`: Optional query builder method.
    /// - `limit`: Optional query builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookDeliveriesResponse` with header metadata.
    /// Its `iter()` yields `WebhookDelivery` lazily across pages.
    pub fn delivery_history(
        &self,
        destination_id: impl AsRef<str>,
    ) -> WebhooksDeliveryHistoryRequest {
        let data = RequestData::new(
            "GET",
            "/v1/webhooks/destinations/{destination_id}/deliveries",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        WebhooksDeliveryHistoryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Get Destination
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookDestinationResponse` with header metadata.
    pub fn get_destination(
        &self,
        destination_id: impl AsRef<str>,
    ) -> WebhooksGetDestinationRequest {
        let data = RequestData::new(
            "GET",
            "/v1/webhooks/destinations/{destination_id}",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        WebhooksGetDestinationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// List Destinations
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// No parameters.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookDestinationList` with header metadata.
    pub fn list_destinations(&self) -> WebhooksListDestinationsRequest {
        let data = RequestData::new("GET", "/v1/webhooks/destinations", &[], &[], &[], false);
        WebhooksListDestinationsRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Replay Delivery
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `delivery_id`: Required path argument. Delivery ID (UUID).
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `NoContent` with header metadata.
    pub fn replay_delivery(
        &self,
        delivery_id: impl AsRef<str>,
        destination_id: impl AsRef<str>,
    ) -> WebhooksReplayDeliveryRequest {
        let data = RequestData::new(
            "POST",
            "/v1/webhooks/destinations/{destination_id}/deliveries/{delivery_id}/replay",
            &[
                ("delivery_id", delivery_id.as_ref().to_owned()),
                ("destination_id", destination_id.as_ref().to_owned()),
            ],
            &[],
            &[],
            false,
        );
        WebhooksReplayDeliveryRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Rotate Secret
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookDestinationCreated` with header metadata.
    pub fn rotate_secret(&self, destination_id: impl AsRef<str>) -> WebhooksRotateSecretRequest {
        let data = RequestData::new(
            "POST",
            "/v1/webhooks/destinations/{destination_id}/rotate-secret",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        WebhooksRotateSecretRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Test Destination
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookTestResponse` with header metadata.
    pub fn test_destination(
        &self,
        destination_id: impl AsRef<str>,
    ) -> WebhooksTestDestinationRequest {
        let data = RequestData::new(
            "POST",
            "/v1/webhooks/destinations/{destination_id}/test",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &[],
            false,
        );
        WebhooksTestDestinationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
    /// Update Destination
    ///
    /// Stability: stable.
    ///
    /// # Parameters
    /// - `destination_id`: Required path argument. Webhook destination ID (UUID).
    /// - `companies`: Optional body builder method. Companies to watch, as `usa:0000320193`. Required with the `filings` family and refused without it: only filings of a listed company (or, for 13D/G, its subject) are sent. Basic 50, Pro 1,000.
    /// - `countries`: Required body builder method. Countries from the active profile's catalogue.
    /// - `enabled`: Required body builder method.
    /// - `families`: Required body builder method.
    /// - `url`: Required body builder method.
    ///
    /// # Returns
    /// A request builder whose `send()` returns `WebhookDestinationResponse` with header metadata.
    pub fn update_destination(
        &self,
        destination_id: impl AsRef<str>,
    ) -> WebhooksUpdateDestinationRequest {
        let data = RequestData::new(
            "PATCH",
            "/v1/webhooks/destinations/{destination_id}",
            &[("destination_id", destination_id.as_ref().to_owned())],
            &[],
            &["countries", "enabled", "families", "url"],
            true,
        );
        WebhooksUpdateDestinationRequest {
            inner: Request::new(self.client.clone(), data),
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksConfigurationHistoryRequest {
    inner: Request<WebhookHistoryResponse, Client>,
}
impl WebhooksConfigurationHistoryRequest {
    /// Send the request and decode `WebhookHistoryResponse`.
    pub fn send(self) -> Result<WebhookHistoryResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookHistoryResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksCreateDestinationRequest {
    inner: Request<WebhookDestinationCreated, Client>,
}
impl WebhooksCreateDestinationRequest {
    /// Companies to watch, as `usa:0000320193`. Required with the `filings` family and refused without it: only filings of a listed company (or, for 13D/G, its subject) are sent. Basic 50, Pro 1,000.
    pub fn companies(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "companies",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Countries from the active profile's catalogue.
    pub fn countries(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "countries",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Set enabled.
    pub fn enabled(mut self, value: bool) -> Self {
        self.inner
            .data
            .body("enabled", serde_json::json!(value), false);
        self
    }
    /// Set families.
    pub fn families(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "families",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Set url.
    pub fn url(mut self, value: impl AsRef<str>) -> Self {
        self.inner
            .data
            .body("url", serde_json::json!(value.as_ref().to_owned()), false);
        self
    }
    /// Send the request and decode `WebhookDestinationCreated`.
    pub fn send(self) -> Result<WebhookDestinationCreated, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookDestinationCreated, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksDeleteDestinationRequest {
    inner: Request<NoContent, Client>,
}
impl WebhooksDeleteDestinationRequest {
    /// Send the request and decode `NoContent`.
    pub fn send(self) -> Result<NoContent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(NoContent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksDeliveryHistoryRequest {
    inner: Request<WebhookDeliveriesResponse, Client>,
}
impl WebhooksDeliveryHistoryRequest {
    /// Set cursor.
    pub fn cursor(mut self, value: impl AsRef<str>) -> Self {
        self.inner.data.query(
            "cursor",
            vec![value.as_ref().to_owned().to_string()],
            "single",
            false,
        );
        self
    }
    /// Set limit.
    pub fn limit(mut self, value: i64) -> Self {
        self.inner
            .data
            .query("limit", vec![value.to_string()], "single", false);
        self
    }
    /// Send the request and decode `WebhookDeliveriesResponse`.
    pub fn send(self) -> Result<WebhookDeliveriesResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookDeliveriesResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
    /// Lazily fetch pages; cursor expiry is returned without restarting.
    pub fn iter(self) -> crate::pagination::PageIterator<WebhookDeliveriesResponse> {
        crate::pagination::PageIterator::new(self.inner)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksGetDestinationRequest {
    inner: Request<WebhookDestinationResponse, Client>,
}
impl WebhooksGetDestinationRequest {
    /// Send the request and decode `WebhookDestinationResponse`.
    pub fn send(self) -> Result<WebhookDestinationResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookDestinationResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksListDestinationsRequest {
    inner: Request<WebhookDestinationList, Client>,
}
impl WebhooksListDestinationsRequest {
    /// Send the request and decode `WebhookDestinationList`.
    pub fn send(self) -> Result<WebhookDestinationList, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookDestinationList, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksReplayDeliveryRequest {
    inner: Request<NoContent, Client>,
}
impl WebhooksReplayDeliveryRequest {
    /// Send the request and decode `NoContent`.
    pub fn send(self) -> Result<NoContent, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(NoContent, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksRotateSecretRequest {
    inner: Request<WebhookDestinationCreated, Client>,
}
impl WebhooksRotateSecretRequest {
    /// Send the request and decode `WebhookDestinationCreated`.
    pub fn send(self) -> Result<WebhookDestinationCreated, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookDestinationCreated, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksTestDestinationRequest {
    inner: Request<WebhookTestResponse, Client>,
}
impl WebhooksTestDestinationRequest {
    /// Send the request and decode `WebhookTestResponse`.
    pub fn send(self) -> Result<WebhookTestResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookTestResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdateDestinationRequest {
    inner: Request<WebhookDestinationResponse, Client>,
}
impl WebhooksUpdateDestinationRequest {
    /// Companies to watch, as `usa:0000320193`. Required with the `filings` family and refused without it: only filings of a listed company (or, for 13D/G, its subject) are sent. Basic 50, Pro 1,000.
    pub fn companies(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "companies",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Countries from the active profile's catalogue.
    pub fn countries(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "countries",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Set enabled.
    pub fn enabled(mut self, value: bool) -> Self {
        self.inner
            .data
            .body("enabled", serde_json::json!(value), false);
        self
    }
    /// Set families.
    pub fn families(mut self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.inner.data.body(
            "families",
            serde_json::json!(value
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()),
            false,
        );
        self
    }
    /// Set url.
    pub fn url(mut self, value: impl AsRef<str>) -> Self {
        self.inner
            .data
            .body("url", serde_json::json!(value.as_ref().to_owned()), false);
        self
    }
    /// Send the request and decode `WebhookDestinationResponse`.
    pub fn send(self) -> Result<WebhookDestinationResponse, Error> {
        self.inner.send()
    }
    /// The decoded value and the exact response bytes (used by the C library).
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(WebhookDestinationResponse, Vec<u8>), Error> {
        self.inner.send_keeping_body()
    }
}

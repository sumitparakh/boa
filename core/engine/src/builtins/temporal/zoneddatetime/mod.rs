use std::str::FromStr;

use crate::{
    builtins::{
        options::{get_option, get_options_object},
        BuiltInBuilder, BuiltInConstructor, BuiltInObject, IntrinsicObject,
    },
    context::intrinsics::{Intrinsics, StandardConstructor, StandardConstructors},
    js_string,
    object::internal_methods::get_prototype_from_constructor,
    property::Attribute,
    realm::Realm,
    string::StaticJsStrings,
    value::{IntoOrUndefined, PreferredType},
    Context, JsArgs, JsBigInt, JsData, JsError, JsNativeError, JsObject, JsResult, JsString,
    JsSymbol, JsValue,
};
use boa_gc::{Finalize, Trace};
use boa_profiler::Profiler;
use num_traits::ToPrimitive;
use temporal_rs::{
    options::{ArithmeticOverflow, Disambiguation, OffsetDisambiguation},
    partial::PartialZonedDateTime,
    Calendar, TimeZone, ZonedDateTime as ZonedDateTimeInner,
};

use super::{
    calendar::get_temporal_calendar_slot_value_with_default, to_partial_date_record,
    to_partial_time_record, to_temporal_duration,
};

/// The `Temporal.ZonedDateTime` object.
#[derive(Debug, Clone, Trace, Finalize, JsData)]
#[boa_gc(unsafe_empty_trace)]
pub struct ZonedDateTime {
    pub(crate) inner: ZonedDateTimeInner,
}

impl ZonedDateTime {
    pub(crate) fn new(inner: ZonedDateTimeInner) -> Self {
        Self { inner }
    }
}

impl BuiltInObject for ZonedDateTime {
    const NAME: JsString = StaticJsStrings::ZONED_DT_NAME;
}

impl IntrinsicObject for ZonedDateTime {
    fn init(realm: &Realm) {
        let _timer = Profiler::global().start_event(std::any::type_name::<Self>(), "init");

        let get_calendar_id = BuiltInBuilder::callable(realm, Self::get_calendar_id)
            .name(js_string!("get calendarId"))
            .build();

        let get_timezone_id = BuiltInBuilder::callable(realm, Self::get_timezone_id)
            .name(js_string!("get timeZoneId"))
            .build();

        let get_era = BuiltInBuilder::callable(realm, Self::get_era)
            .name(js_string!("get era"))
            .build();

        let get_era_year = BuiltInBuilder::callable(realm, Self::get_era_year)
            .name(js_string!("get eraYear"))
            .build();

        let get_year = BuiltInBuilder::callable(realm, Self::get_year)
            .name(js_string!("get year"))
            .build();

        let get_month = BuiltInBuilder::callable(realm, Self::get_month)
            .name(js_string!("get month"))
            .build();

        let get_month_code = BuiltInBuilder::callable(realm, Self::get_month_code)
            .name(js_string!("get monthCode"))
            .build();

        let get_day = BuiltInBuilder::callable(realm, Self::get_day)
            .name(js_string!("get day"))
            .build();

        let get_hour = BuiltInBuilder::callable(realm, Self::get_hour)
            .name(js_string!("get hour"))
            .build();

        let get_minute = BuiltInBuilder::callable(realm, Self::get_minute)
            .name(js_string!("get minute"))
            .build();

        let get_second = BuiltInBuilder::callable(realm, Self::get_second)
            .name(js_string!("get second"))
            .build();

        let get_millisecond = BuiltInBuilder::callable(realm, Self::get_millisecond)
            .name(js_string!("get millisecond"))
            .build();

        let get_microsecond = BuiltInBuilder::callable(realm, Self::get_microsecond)
            .name(js_string!("get microsecond"))
            .build();

        let get_nanosecond = BuiltInBuilder::callable(realm, Self::get_nanosecond)
            .name(js_string!("get nanosecond"))
            .build();

        let get_epoch_millisecond = BuiltInBuilder::callable(realm, Self::get_epoch_millisecond)
            .name(js_string!("get epochMillisecond"))
            .build();

        let get_epoch_nanosecond = BuiltInBuilder::callable(realm, Self::get_epoch_nanosecond)
            .name(js_string!("get epochNanosecond"))
            .build();

        let get_day_of_week = BuiltInBuilder::callable(realm, Self::get_day_of_week)
            .name(js_string!("get dayOfWeek"))
            .build();

        let get_day_of_year = BuiltInBuilder::callable(realm, Self::get_day_of_year)
            .name(js_string!("get dayOfYear"))
            .build();

        let get_week_of_year = BuiltInBuilder::callable(realm, Self::get_week_of_year)
            .name(js_string!("get weekOfYear"))
            .build();

        let get_hours_in_day = BuiltInBuilder::callable(realm, Self::get_hours_in_day)
            .name(js_string!("get daysInWeek"))
            .build();

        let get_year_of_week = BuiltInBuilder::callable(realm, Self::get_year_of_week)
            .name(js_string!("get yearOfWeek"))
            .build();

        let get_days_in_week = BuiltInBuilder::callable(realm, Self::get_days_in_week)
            .name(js_string!("get daysInWeek"))
            .build();

        let get_days_in_month = BuiltInBuilder::callable(realm, Self::get_days_in_month)
            .name(js_string!("get daysInMonth"))
            .build();

        let get_days_in_year = BuiltInBuilder::callable(realm, Self::get_days_in_year)
            .name(js_string!("get daysInYear"))
            .build();

        let get_months_in_year = BuiltInBuilder::callable(realm, Self::get_months_in_year)
            .name(js_string!("get monthsInYear"))
            .build();

        let get_in_leap_year = BuiltInBuilder::callable(realm, Self::get_in_leap_year)
            .name(js_string!("get inLeapYear"))
            .build();

        BuiltInBuilder::from_standard_constructor::<Self>(realm)
            .property(
                JsSymbol::to_string_tag(),
                StaticJsStrings::ZONED_DT_TAG,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("calendarId"),
                Some(get_calendar_id),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("timeZoneId"),
                Some(get_timezone_id),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("era"),
                Some(get_era),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("eraYear"),
                Some(get_era_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("year"),
                Some(get_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("month"),
                Some(get_month),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("monthCode"),
                Some(get_month_code),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("day"),
                Some(get_day),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("hour"),
                Some(get_hour),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("minute"),
                Some(get_minute),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("second"),
                Some(get_second),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("millisecond"),
                Some(get_millisecond),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("microsecond"),
                Some(get_microsecond),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("nanosecond"),
                Some(get_nanosecond),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("epochMillisecond"),
                Some(get_epoch_millisecond),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("epochNanosecond"),
                Some(get_epoch_nanosecond),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("dayOfWeek"),
                Some(get_day_of_week),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("dayOfYear"),
                Some(get_day_of_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("weekOfYear"),
                Some(get_week_of_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("yearOfWeek"),
                Some(get_year_of_week),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("hoursInDay"),
                Some(get_hours_in_day),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("daysInWeek"),
                Some(get_days_in_week),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("daysInMonth"),
                Some(get_days_in_month),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("daysInYear"),
                Some(get_days_in_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("monthsInYear"),
                Some(get_months_in_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .accessor(
                js_string!("inLeapYear"),
                Some(get_in_leap_year),
                None,
                Attribute::CONFIGURABLE,
            )
            .static_method(Self::from, js_string!("from"), 1)
            .method(Self::add, js_string!("add"), 1)
            .method(Self::subtract, js_string!("subtract"), 1)
            .method(Self::value_of, js_string!("valueOf"), 0)
            .build();
    }

    fn get(intrinsics: &Intrinsics) -> JsObject {
        Self::STANDARD_CONSTRUCTOR(intrinsics.constructors()).constructor()
    }
}

impl BuiltInConstructor for ZonedDateTime {
    const LENGTH: usize = 2;
    const P: usize = 1;
    const SP: usize = 0;

    const STANDARD_CONSTRUCTOR: fn(&StandardConstructors) -> &StandardConstructor =
        StandardConstructors::zoned_date_time;

    fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        // 1. If NewTarget is undefined, then
        if new_target.is_undefined() {
            // a. Throw a TypeError exception.
            return Err(JsNativeError::typ()
                .with_message("NewTarget cannot be undefined.")
                .into());
        }
        //  2. Set epochNanoseconds to ? ToBigInt(epochNanoseconds).
        let epoch_nanos = args.get_or_undefined(0).to_bigint(context)?;
        //  3. If IsValidEpochNanoseconds(epochNanoseconds) is false, throw a RangeError exception.
        // TODO: Better primitive for handling epochNanoseconds is needed in temporal_rs
        let Some(nanos) = epoch_nanos.to_f64().to_i128() else {
            return Err(JsNativeError::range()
                .with_message("epochNanoseconds exceeded valid range.")
                .into());
        };

        //  4. If timeZone is not a String, throw a TypeError exception.
        let JsValue::String(timezone_str) = args.get_or_undefined(1) else {
            return Err(JsNativeError::typ()
                .with_message("timeZone must be a string.")
                .into());
        };

        //  5. Let timeZoneParse be ? ParseTimeZoneIdentifier(timeZone).
        //  6. If timeZoneParse.[[OffsetMinutes]] is empty, then
        // a. Let identifierRecord be GetAvailableNamezdtimeZoneIdentifier(timeZoneParse.[[Name]]).
        // b. If identifierRecord is empty, throw a RangeError exception.
        // c. Set timeZone to identifierRecord.[[Identifier]].
        //  7. Else,
        // a. Set timeZone to FormatOffsetTimeZoneIdentifier(timeZoneParse.[[OffsetMinutes]]).
        let timezone = TimeZone::try_from_str_with_provider(
            &timezone_str.to_std_string_escaped(),
            context.tz_provider(),
        )?;

        //  8. If calendar is undefined, set calendar to "iso8601".
        //  9. If calendar is not a String, throw a TypeError exception.
        //  10. Set calendar to ? CanonicalizeCalendar(calendar).
        let calendar = args
            .get(2)
            .map(|v| {
                if let JsValue::String(calendar_str) = v {
                    Calendar::from_str(&calendar_str.to_std_string_escaped())
                        .map_err(Into::<JsError>::into)
                } else {
                    Err(JsNativeError::typ()
                        .with_message("calendar must be a string.")
                        .into())
                }
            })
            .transpose()?
            .unwrap_or_default();

        let inner = ZonedDateTimeInner::try_new(nanos, calendar, timezone)?;

        //  11. Return ? CreateTemporalZonedDateTime(epochNanoseconds, timeZone, calendar, NewTarget).
        create_temporal_zoneddatetime(inner, Some(new_target), context).map(Into::into)
    }
}

// ==== `ZonedDateTime` accessor property methods ====

impl ZonedDateTime {
    /// 6.3.3 get `Temporal.ZonedDateTime.prototype.calendarId`
    fn get_calendar_id(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a PlainDate object.")
            })?;

        Ok(JsString::from(zdt.inner.calendar().identifier()).into())
    }

    /// 6.3.4 get `Temporal.ZonedDateTime.prototype.timeZoneId`
    fn get_timezone_id(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        let _zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a PlainDate object.")
            })?;

        Err(JsNativeError::error()
            .with_message("Not yet implemented.")
            .into())
    }

    /// 6.3.5 get `Temporal.ZonedDateTime.prototype.era`
    fn get_era(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a PlainDate object.")
            })?;

        let era = zdt.inner.era_with_provider(context.tz_provider())?;
        Ok(era
            .map(|tinystr| JsString::from(tinystr.to_lowercase()))
            .into_or_undefined())
    }

    /// 6.3.6 get `Temporal.ZonedDateTime.prototype.eraYear`
    fn get_era_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a PlainDate object.")
            })?;

        Ok(zdt
            .inner
            .era_year_with_provider(context.tz_provider())?
            .into_or_undefined())
    }

    /// 6.3.7 get `Temporal.ZonedDateTime.prototype.year`
    fn get_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt.inner.year_with_provider(context.tz_provider())?.into())
    }

    /// 6.3.8 get `Temporal.ZonedDateTime.prototype.month`
    fn get_month(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt.inner.month_with_provider(context.tz_provider())?.into())
    }

    /// 6.3.9 get Temporal.ZonedDateTime.prototype.monthCode
    fn get_month_code(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(JsString::from(
            zdt.inner
                .month_code_with_provider(context.tz_provider())?
                .as_str(),
        )
        .into())
    }

    /// 6.3.10 get `Temporal.ZonedDateTime.prototype.day`
    fn get_day(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt.inner.day_with_provider(context.tz_provider())?.into())
    }

    /// 6.3.11 get `Temporal.ZonedDateTime.prototype.hour`
    fn get_hour(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt.inner.hour_with_provider(context.tz_provider())?.into())
    }

    /// 6.3.12 get `Temporal.ZonedDateTime.prototype.minute`
    fn get_minute(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .minute_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.13 get `Temporal.ZonedDateTime.prototype.second`
    fn get_second(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .second_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.14 get `Temporal.ZonedDateTime.prototype.millisecond`
    fn get_millisecond(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .millisecond_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.15 get `Temporal.ZonedDateTime.prototype.microsecond`
    fn get_microsecond(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .microsecond_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.16 get `Temporal.ZonedDateTime.prototype.nanosecond`
    fn get_nanosecond(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .nanosecond_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.17 get `Temporal.ZonedDateTime.prototype.epochMilliseconds`
    fn get_epoch_millisecond(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok((zdt.inner.epoch_milliseconds()).into())
    }

    /// 6.3.18 get `Temporal.ZonedDateTime.prototype.epochNanosecond`
    fn get_epoch_nanosecond(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(JsBigInt::from(zdt.inner.epoch_nanoseconds()).into())
    }

    /// 6.3.19 get `Temporal.ZonedDateTime.prototype.dayOfWeek`
    fn get_day_of_week(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .day_of_week_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.20 get `Temporal.ZonedDateTime.prototype.dayOfYear`
    fn get_day_of_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .day_of_year_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.21 get `Temporal.ZonedDateTime.prototype.weekOfYear`
    fn get_week_of_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .week_of_year_with_provider(context.tz_provider())?
            .into_or_undefined())
    }

    /// 6.3.22 get `Temporal.ZonedDateTime.prototype.yearOfWeek`
    fn get_year_of_week(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .year_of_week_with_provider(context.tz_provider())?
            .into_or_undefined())
    }

    /// 6.3.23 get `Temporal.ZonedDateTime.prototype.hoursInDay`
    fn get_hours_in_day(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .hours_in_day_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.24 get `Temporal.ZonedDateTime.prototype.daysInWeek`
    fn get_days_in_week(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .days_in_week_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.25 get `Temporal.ZonedDateTime.prototype.daysInMonth`
    fn get_days_in_month(
        this: &JsValue,
        _: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .days_in_month_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.26 get `Temporal.ZonedDateTime.prototype.daysInYear`
    fn get_days_in_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .days_in_year_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.27 get `Temporal.ZonedDateTime.prototype.monthsInYear`
    fn get_months_in_year(
        this: &JsValue,
        _: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .months_in_year_with_provider(context.tz_provider())?
            .into())
    }

    /// 6.3.28 get `Temporal.ZonedDateTime.prototype.inLeapYear`
    fn get_in_leap_year(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        Ok(zdt
            .inner
            .in_leap_year_with_provider(context.tz_provider())?
            .into())
    }
}

// ==== `ZonedDateTime` method implementations ====

impl ZonedDateTime {
    /// 6.2.2 Temporal.ZonedDateTime.from ( item [ , options ] )
    fn from(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        // 1. Return ? ToTemporalZonedDateTime(item, options).
        let item = args.get_or_undefined(0);
        let options = args.get(1);
        let inner = to_temporal_zoneddatetime(item, options.cloned(), context)?;
        create_temporal_zoneddatetime(inner, None, context).map(Into::into)
    }

    fn add(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        let duration = to_temporal_duration(args.get_or_undefined(0), context)?;

        let options = get_options_object(args.get_or_undefined(1))?;
        let overflow = get_option::<ArithmeticOverflow>(&options, js_string!("overflow"), context)?;

        create_temporal_zoneddatetime(
            zdt.inner
                .add_with_provider(&duration, overflow, context.tz_provider())?,
            None,
            context,
        )
        .map(Into::into)
    }

    fn subtract(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let zdt = this
            .as_object()
            .and_then(JsObject::downcast_ref::<Self>)
            .ok_or_else(|| {
                JsNativeError::typ().with_message("the this object must be a ZonedDateTime object.")
            })?;

        let duration = to_temporal_duration(args.get_or_undefined(0), context)?;

        let options = get_options_object(args.get_or_undefined(1))?;
        let overflow = get_option::<ArithmeticOverflow>(&options, js_string!("overflow"), context)?;

        create_temporal_zoneddatetime(
            zdt.inner
                .subtract_with_provider(&duration, overflow, context.tz_provider())?,
            None,
            context,
        )
        .map(Into::into)
    }

    pub(crate) fn value_of(_this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Err(JsNativeError::typ()
            .with_message("`valueOf` not supported by Temporal built-ins. See 'compare', 'equals', or `toString`")
            .into())
    }
}

// -- ZonedDateTime Abstract Operations --

pub(crate) fn create_temporal_zoneddatetime(
    inner: ZonedDateTimeInner,
    new_target: Option<&JsValue>,
    context: &mut Context,
) -> JsResult<JsObject> {
    // 1. Assert: IsValidEpochNanoseconds(epochNanoseconds) is true.
    // 2. If newTarget is not present, set newTarget to %Temporal.ZonedDateTime%.
    let new_target = new_target.cloned().unwrap_or(
        context
            .realm()
            .intrinsics()
            .constructors()
            .zoned_date_time()
            .constructor()
            .into(),
    );
    // 3. Let object be ? OrdinaryCreateFromConstructor(newTarget, "%Temporal.ZonedDateTime.prototype%", « [[InitializezdtemporalZonedDateTime]], [[EpochNanoseconds]], [[TimeZone]], [[Calendar]] »).
    let prototype = get_prototype_from_constructor(
        &new_target,
        StandardConstructors::zoned_date_time,
        context,
    )?;
    // 4. Set object.[[EpochNanoseconds]] to epochNanoseconds.
    // 5. Set object.[[TimeZone]] to timeZone.
    // 6. Set object.[[Calendar]] to calendar.
    let obj = JsObject::from_proto_and_data(prototype, ZonedDateTime::new(inner));

    // 7. Return object.
    Ok(obj)
}

pub(crate) fn to_temporal_zoneddatetime(
    value: &JsValue,
    options: Option<JsValue>,
    context: &mut Context,
) -> JsResult<ZonedDateTimeInner> {
    // 1. If options is not present, set options to undefined.
    // 2. Let offsetBehaviour be option.
    // 3. Let matchBehaviour be match-exactly.
    // 4. If item is an Object, then
    match value {
        JsValue::Object(object) => {
            // a. If item has an [[InitializedTemporalZonedDateTime]] internal slot, then
            if let Some(zdt) = object.downcast_ref::<ZonedDateTime>() {
                // i. NOTE: The following steps, and similar ones below, read options
                // and perform independent validation in alphabetical order
                // (GetTemporalDisambiguationOption reads "disambiguation", GetTemporalOffsetOption
                // reads "offset", and GetTemporalOverflowOption reads "overflow").
                // ii. Let resolvedOptions be ? GetOptionsObject(options).
                let options = get_options_object(&options.unwrap_or_default())?;
                // iii. Perform ? GetTemporalDisambiguationOption(resolvedOptions).
                let _disambiguation =
                    get_option::<Disambiguation>(&options, js_string!("disambiguation"), context)?
                        .unwrap_or(Disambiguation::Compatible);
                // iv. Perform ? GetTemporalOffsetOption(resolvedOptions, reject).
                let _offset_option =
                    get_option::<OffsetDisambiguation>(&options, js_string!("offset"), context)?
                        .unwrap_or(OffsetDisambiguation::Reject);
                // v. Perform ? GetTemporalOverflowOption(resolvedOptions).
                let _overflow =
                    get_option::<ArithmeticOverflow>(&options, js_string!("overflow"), context)?
                        .unwrap_or_default();
                // vi. Return ! CreateTemporalZonedDateTime(item.[[EpochNanoseconds]], item.[[TimeZone]], item.[[Calendar]]).
                return Ok(zdt.inner.clone());
            }
            // b. Let calendar be ? GetTemporalCalendarIdentifierWithISODefault(item).
            let calendar = get_temporal_calendar_slot_value_with_default(object, context)?;
            // c. Let fields be ? PrepareCalendarFields(calendar, item, « year, month, month-code, day », « hour, minute, second, millisecond, microsecond, nanosecond, offset, time-zone », « time-zone »).
            let date = to_partial_date_record(object, context)?;
            let time = to_partial_time_record(object, context)?;
            // d. Let timeZone be fields.[[TimeZone]].
            let timezone = object
                .get(js_string!("timeZone"), context)?
                .map(|v| {
                    // TODO: to_temporal_timezone_identifier
                    to_temporal_timezone_identifier(v, context)
                })
                .transpose()?
                .unwrap_or_default();
            // e. Let offsetString be fields.[[OffsetString]].
            let offset = object
                .get(js_string!("offset"), context)?
                .map(|v| to_offset_string(v, context))
                .transpose()?;
            let partial = PartialZonedDateTime {
                date,
                time,
                offset,
                timezone,
            };
            // f. If offsetString is unset, then
            // i. Set offsetBehaviour to wall.
            // g. Let resolvedOptions be ? GetOptionsObject(options).
            let options = get_options_object(&options.unwrap_or_default())?;
            // h. Let disambiguation be ? GetTemporalDisambiguationOption(resolvedOptions).
            let disambiguation =
                get_option::<Disambiguation>(&options, js_string!("disambiguation"), context)?;
            // i. Let offsetOption be ? GetTemporalOffsetOption(resolvedOptions, reject).
            let offset_option =
                get_option::<OffsetDisambiguation>(&options, js_string!("offset"), context)?;
            // j. Let overflow be ? GetTemporalOverflowOption(resolvedOptions).
            let overflow =
                get_option::<ArithmeticOverflow>(&options, js_string!("overflow"), context)?;
            // k. Let result be ? InterpretTemporalDateTimeFields(calendar, fields, overflow).
            // l. Let isoDate be result.[[ISODate]].
            // m. Let time be result.[[Time]].
            Ok(ZonedDateTimeInner::from_partial_with_provider(
                partial,
                Some(calendar),
                overflow,
                disambiguation,
                offset_option,
                context.tz_provider(),
            )?)
        }
        JsValue::String(zdt_source) => {
            // b. Let result be ? ParseISODateTime(item, « TemporalDateTimeString[+Zoned] »).
            // c. Let annotation be result.[[TimeZone]].[[TimeZoneAnnotation]].
            // d. Assert: annotation is not empty.
            // e. Let timeZone be ? ToTemporalTimeZoneIdentifier(annotation).
            // f. Let offsetString be result.[[TimeZone]].[[OffsetString]].
            // g. If result.[[TimeZone]].[[Z]] is true, then
            // i. Set offsetBehaviour to exact.
            // h. Else if offsetString is empty, then
            // i. Set offsetBehaviour to wall.
            // i. Let calendar be result.[[Calendar]].
            // j. If calendar is empty, set calendar to "iso8601".
            // k. Set calendar to ? CanonicalizeCalendar(calendar).
            // l. Set matchBehaviour to match-minutes.
            // m. Let resolvedOptions be ? GetOptionsObject(options).
            let options = get_options_object(&options.unwrap_or_default())?;
            // n. Let disambiguation be ? GetTemporalDisambiguationOption(resolvedOptions).
            let disambiguation =
                get_option::<Disambiguation>(&options, js_string!("disambiguation"), context)?
                    .unwrap_or(Disambiguation::Compatible);
            // o. Let offsetOption be ? GetTemporalOffsetOption(resolvedOptions, reject).
            let offset_option =
                get_option::<OffsetDisambiguation>(&options, js_string!("offset"), context)?
                    .unwrap_or(OffsetDisambiguation::Reject);
            // p. Perform ? GetTemporalOverflowOption(resolvedOptions).
            // q. Let isoDate be CreateISODateRecord(result.[[Year]], result.[[Month]], result.[[Day]]).
            // r. Let time be result.[[Time]].
            // 6. Let offsetNanoseconds be 0.
            // 7. If offsetBehaviour is option, then
            //        a. Set offsetNanoseconds to ! ParseDateTimeUTCOffset(offsetString).
            // 8. Let epochNanoseconds be ? InterpretISODateTimeOffset(isoDate, time, offsetBehaviour, offsetNanoseconds, timeZone, disambiguation, offsetOption, matchBehaviour).
            Ok(ZonedDateTimeInner::from_str_with_provider(
                &zdt_source.to_std_string_escaped(),
                disambiguation,
                offset_option,
                context.tz_provider(),
            )?)
        }
        // 5. Else,
        // a. If item is not a String, throw a TypeError exception.
        _ => Err(JsNativeError::typ()
            .with_message("Temporal.ZonedDateTime.from only accepts an object or string.")
            .into()),
    }
    // 9. Return ! CreateTemporalZonedDateTime(epochNanoseconds, timeZone, calendar).
}

pub(crate) fn to_temporal_timezone_identifier(
    value: &JsValue,
    context: &mut Context,
) -> JsResult<TimeZone> {
    // 1. If temporalTimeZoneLike is an Object, then
    if let Some(obj) = value.as_object() {
        // a. If temporalTimeZoneLike has an [[InitializedTemporalZonedDateTime]] internal slot, then
        if let Some(zdt) = obj.downcast_ref::<ZonedDateTime>() {
            // i. Return temporalTimeZoneLike.[[TimeZone]].
            return Ok(zdt.inner.timezone().clone());
        }
    }

    // 2. If temporalTimeZoneLike is not a String, throw a TypeError exception.
    let JsValue::String(tz_string) = value else {
        return Err(JsNativeError::typ()
            .with_message("timeZone must be a string or Temporal.ZonedDateTime")
            .into());
    };

    // 3. Let parseResult be ? ParseTemporalTimeZoneString(temporalTimeZoneLike).
    // 4. Let offsetMinutes be parseResult.[[OffsetMinutes]].
    // 5. If offsetMinutes is not empty, return FormatOffsetTimeZoneIdentifier(offsetMinutes).
    // 6. Let name be parseResult.[[Name]].
    // 7. Let timeZoneIdentifierRecord be GetAvailableNamedTimeZoneIdentifier(name).
    // 8. If timeZoneIdentifierRecord is empty, throw a RangeError exception.
    // 9. Return timeZoneIdentifierRecord.[[Identifier]].
    Ok(TimeZone::try_from_str_with_provider(
        &tz_string.to_std_string_escaped(),
        context.tz_provider(),
    )?)
}

fn to_offset_string(value: &JsValue, context: &mut Context) -> JsResult<String> {
    // 1. Let offset be ? ToPrimitive(argument, string).
    let offset = value.to_primitive(context, PreferredType::String)?;
    // 2. If offset is not a String, throw a TypeError exception.
    let JsValue::String(offset_string) = offset else {
        return Err(JsNativeError::typ()
            .with_message("offset must be a String.")
            .into());
    };
    // 3. Perform ? ParseDateTimeUTCOffset(offset).
    let result = offset_string.to_std_string_escaped();
    let _u = TimeZone::try_from_str_with_provider(&result, context.tz_provider())?;
    // 4. Return offset.
    Ok(result)
}
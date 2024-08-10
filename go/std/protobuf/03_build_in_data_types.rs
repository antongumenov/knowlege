//_ BUILD-IN СТРУКТУРЫ PROTOBUF
//~ Duration
message Duration {
  // Signed seconds of the span of time. Must be from -315,576,000,000
  // to +315,576,000,000 inclusive. Note: these bounds are computed from:
  // 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
  int64 seconds = 1;

  // Signed fractions of a second at nanosecond resolution of the span
  // of time. Durations less than one second are represented with a 0
  // `seconds` field and a positive or negative `nanos` field. For durations
  // of one second or more, a non-zero value for the `nanos` field must be
  // of the same sign as the `seconds` field. Must be from -999,999,999
  // to +999,999,999 inclusive.
  int32 nanos = 2;
}

//~ Timestamp
message Timestamp {
  // Represents seconds of UTC time since Unix epoch
  // 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
  // 9999-12-31T23:59:59Z inclusive.
  int64 seconds = 1;

  // Non-negative fractions of a second at nanosecond resolution. Negative
  // second values with fractions must still have non-negative nanos values
  // that count forward in time. Must be from 0 to 999,999,999
  // inclusive.
  int32 nanos = 2;
}

//~ Interval
message Interval {
  // Optional. Inclusive start of the interval.
  //
  // If specified, a Timestamp matching this interval will have to be the same
  // or after the start.
  google.protobuf.Timestamp start_time = 1;

  // Optional. Exclusive end of the interval.
  //
  // If specified, a Timestamp matching this interval will have to be before the
  // end.
  google.protobuf.Timestamp end_time = 2;
}

//~ Date
message Date {
  // Year of the date. Must be from 1 to 9999, or 0 to specify a date without
  // a year.
  int32 year = 1;

  // Month of a year. Must be from 1 to 12, or 0 to specify a year without a
  // month and day.
  int32 month = 2;

  // Day of a month. Must be from 1 to 31 and valid for the year and month, or 0
  // to specify a year by itself or a year and month where the day isn't
  // significant.
  int32 day = 3;
}

//~ Month
enum Month {
  MONTH_UNSPECIFIED = 0;
  JANUARY = 1;
  FEBRUARY = 2;
  MARCH = 3;
  APRIL = 4;
  MAY = 5;
  JUNE = 6;
  JULY = 7;
  AUGUST = 8;
  SEPTEMBER = 9;
  OCTOBER = 10;
  NOVEMBER = 11;
  DECEMBER = 12;
}

//~ DayOfWeek
enum DayOfWeek {
  DAY_OF_WEEK_UNSPECIFIED = 0;
  MONDAY = 1;
  TUESDAY = 2;
  WEDNESDAY = 3;
  THURSDAY = 4;
  FRIDAY = 5;
  SATURDAY = 6;
  SUNDAY = 7;
}

//~ TimeOfDay
message TimeOfDay {
  // Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
  // to allow the value "24:00:00" for scenarios like business closing time.
  int32 hours = 1;

  // Minutes of hour of day. Must be from 0 to 59.
  int32 minutes = 2;

  // Seconds of minutes of the time. Must normally be from 0 to 59. An API may
  // allow the value 60 if it allows leap-seconds.
  int32 seconds = 3;

  // Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
  int32 nanos = 4;
}

//~ FieldMask
message FieldMask {
  // The set of field mask paths.
  repeated string paths = 1;
}

//~ PostalAddress
message PostalAddress {
  // The schema revision of the `PostalAddress`. This must be set to 0, which is
  // the latest revision.
  //
  // All new revisions **must** be backward compatible with old revisions.
  int32 revision = 1;

  // Required. CLDR region code of the country/region of the address. This
  // is never inferred and it is up to the user to ensure the value is
  // correct. See http://cldr.unicode.org/ and
  // http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
  // for details. Example: "CH" for Switzerland.
  string region_code = 2;

  // Optional. BCP-47 language code of the contents of this address (if
  // known). This is often the UI language of the input form or is expected
  // to match one of the languages used in the address' country/region, or their
  // transliterated equivalents.
  // This can affect formatting in certain countries, but is not critical
  // to the correctness of the data and will never affect any validation or
  // other non-formatting related operations.
  //
  // If this value is not known, it should be omitted (rather than specifying a
  // possibly incorrect default).
  //
  // Examples: "zh-Hant", "ja", "ja-Latn", "en".
  string language_code = 3;

  // Optional. Postal code of the address. Not all countries use or require
  // postal codes to be present, but where they are used, they may trigger
  // additional validation with other parts of the address (e.g. state/zip
  // validation in the U.S.A.).
  string postal_code = 4;

  // Optional. Additional, country-specific, sorting code. This is not used
  // in most regions. Where it is used, the value is either a string like
  // "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number
  // alone, representing the "sector code" (Jamaica), "delivery area indicator"
  // (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
  string sorting_code = 5;

  // Optional. Highest administrative subdivision which is used for postal
  // addresses of a country or region.
  // For example, this can be a state, a province, an oblast, or a prefecture.
  // Specifically, for Spain this is the province and not the autonomous
  // community (e.g. "Barcelona" and not "Catalonia").
  // Many countries don't use an administrative area in postal addresses. E.g.
  // in Switzerland this should be left unpopulated.
  string administrative_area = 6;

  // Optional. Generally refers to the city/town portion of the address.
  // Examples: US city, IT comune, UK post town.
  // In regions of the world where localities are not well defined or do not fit
  // into this structure well, leave locality empty and use address_lines.
  string locality = 7;

  // Optional. Sublocality of the address.
  // For example, this can be neighborhoods, boroughs, districts.
  string sublocality = 8;

  // Unstructured address lines describing the lower levels of an address.
  //
  // Because values in address_lines do not have type information and may
  // sometimes contain multiple values in a single field (e.g.
  // "Austin, TX"), it is important that the line order is clear. The order of
  // address lines should be "envelope order" for the country/region of the
  // address. In places where this can vary (e.g. Japan), address_language is
  // used to make it explicit (e.g. "ja" for large-to-small ordering and
  // "ja-Latn" or "en" for small-to-large). This way, the most specific line of
  // an address can be selected based on the language.
  //
  // The minimum permitted structural representation of an address consists
  // of a region_code with all remaining information placed in the
  // address_lines. It would be possible to format such an address very
  // approximately without geocoding, but no semantic reasoning could be
  // made about any of the address components until it was at least
  // partially resolved.
  //
  // Creating an address only containing a region_code and address_lines, and
  // then geocoding is the recommended way to handle completely unstructured
  // addresses (as opposed to guessing which parts of the address should be
  // localities or administrative areas).
  repeated string address_lines = 9;

  // Optional. The recipient at the address.
  // This field may, under certain circumstances, contain multiline information.
  // For example, it might contain "care of" information.
  repeated string recipients = 10;

  // Optional. The name of the organization at the address.
  string organization = 11;
}

//~ Money
message Money {
  // The three-letter currency code defined in ISO 4217.
  string currency_code = 1;

  // The whole units of the amount.
  // For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
  int64 units = 2;

  // Number of nano (10^-9) units of the amount.
  // The value must be between -999,999,999 and +999,999,999 inclusive.
  // If `units` is positive, `nanos` must be positive or zero.
  // If `units` is zero, `nanos` can be positive, zero, or negative.
  // If `units` is negative, `nanos` must be negative or zero.
  // For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
  int32 nanos = 3;
}

//~ LatLng
message LatLng {
  // The latitude in degrees. It must be in the range [-90.0, +90.0].
  double latitude = 1;

  // The longitude in degrees. It must be in the range [-180.0, +180.0].
  double longitude = 2;
}

//~ Color
message Color {
  // The amount of red in the color as a value in the interval [0, 1].
  float red = 1;

  // The amount of green in the color as a value in the interval [0, 1].
  float green = 2;

  // The amount of blue in the color as a value in the interval [0, 1].
  float blue = 3;

  // The fraction of this color that should be applied to the pixel. That is,
  // the final pixel color is defined by the equation:
  //
  //   `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)`
  //
  // This means that a value of 1.0 corresponds to a solid color, whereas
  // a value of 0.0 corresponds to a completely transparent color. This
  // uses a wrapper message rather than a simple float scalar so that it is
  // possible to distinguish between a default value and the value being unset.
  // If omitted, this color object is rendered as a solid color
  // (as if the alpha value had been explicitly given a value of 1.0).
  google.protobuf.FloatValue alpha = 4;
}

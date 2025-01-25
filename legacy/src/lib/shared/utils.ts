export function notNull<T>(value: T | null): value is T {
  return value !== null;
}

export function notNullOrUndefined<T>(value: T | null | undefined): value is T {
  return value !== null && value !== undefined;
}

// This script is released to the public domain and may be used, modified and
// distributed without restrictions. Attribution not necessary but appreciated.
// Source: https://weeknumber.com/how-to/javascript

// Returns the ISO week of the date.
Date.prototype.getWeek = function () {
  const date = new Date(this.getTime());
  date.setHours(0, 0, 0, 0);
  // Thursday in current week decides the year.
  date.setDate(date.getDate() + 3 - ((date.getDay() + 6) % 7));
  // January 4 is always in week 1.
  const week1 = new Date(date.getFullYear(), 0, 4);
  // Adjust to Thursday in week 1 and count number of weeks from date to week1.
  return (
    1 +
    Math.round(
      ((date.getTime() - week1.getTime()) / 86400000 -
        3 +
        ((week1.getDay() + 6) % 7)) /
        7,
    )
  );
};

/**
 * Get the date from an ISO 8601 week and year
 *
 * https://en.wikipedia.org/wiki/ISO_week_date
 *
 * @param {number} week ISO 8601 week number
 * @param {number} year ISO year
 *
 * Examples:
 *  getDateOfIsoWeek(53, 1976) -> Mon Dec 27 1976
 *  getDateOfIsoWeek( 1, 1978) -> Mon Jan 02 1978
 *  getDateOfIsoWeek( 1, 1980) -> Mon Dec 31 1979
 *  getDateOfIsoWeek(53, 2020) -> Mon Dec 28 2020
 *  getDateOfIsoWeek( 1, 2021) -> Mon Jan 04 2021
 *  getDateOfIsoWeek( 0, 2023) -> Invalid (no week 0)
 *  getDateOfIsoWeek(53, 2023) -> Invalid (no week 53 in 2023)
 */
export function getDateOfIsoWeek(
  week: number,
  year: number,
  day?: number,
): Date {
  // week = parseFloat(week);
  // year = parseFloat(year);

  if (week < 1 || week > 53) {
    throw new RangeError("ISO 8601 weeks are numbered from 1 to 53");
  } else if (!Number.isInteger(week)) {
    throw new TypeError("Week must be an integer");
  } else if (!Number.isInteger(year)) {
    throw new TypeError("Year must be an integer");
  }

  const simple = new Date(year, 0, 1 + (week - 1) * 7);
  const dayOfWeek = simple.getDay();
  const isoWeekStart = simple;

  // Get the Monday past, and add a week if the day was
  // Friday, Saturday or Sunday.

  isoWeekStart.setDate(simple.getDate() - dayOfWeek + 1);
  if (dayOfWeek > 4) {
    isoWeekStart.setDate(isoWeekStart.getDate() + 7);
  }

  // The latest possible ISO week starts on December 28 of the current year.
  if (
    isoWeekStart.getFullYear() > year ||
    (isoWeekStart.getFullYear() == year &&
      isoWeekStart.getMonth() == 11 &&
      isoWeekStart.getDate() > 28)
  ) {
    throw new RangeError(`${year} has no ISO week ${week}`);
  }

  if (day !== undefined) {
    isoWeekStart.setDate(isoWeekStart.getDate() + day);
    // isoWeekStart.set
    return isoWeekStart;
  }

  return isoWeekStart;
}

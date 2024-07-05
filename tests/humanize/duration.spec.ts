import test from 'ava';
import { humanizeDuration } from '../../packages/humanize/lib';

const
  // best way to test. is used discord timestamp from snowflake
  DiscordEpoch = 1420070400000n,
  // Discord ID (snowflake)
  DiscordID = 604227193651986443n,
  // Static date for test
  Utc = new Date('2024-06-09T03:04:41.165Z'),
  // Timestamp from snowflake (discord ID)
  Timestamp = (Utc.getTime() - Number((DiscordID >> 22n) + DiscordEpoch)) / 1000,
  // Normal result without args and shorts
  SinceAbbrev = '4y 10m 2w 15d 18h 41min 32sec',
  // Normal result without args
  Since = '4 years, 10 months, 2 weeks, 15 days, 18 hours, 41 minutes and 32 seconds';

test('duration', (t) => {
  let duration = humanizeDuration(Timestamp);

  t.true(!!duration);
  t.not(duration, '0');
  t.is(duration, Since, 'duration is not equal');
  t.not(duration, SinceAbbrev);

  duration = humanizeDuration(Timestamp, 7, true);
  t.true(!!duration);
  t.not(duration, '0');
  t.not(duration, Since);
  t.is(duration, SinceAbbrev, 'duration is not equal in abbrev');
});

test('maxUnits', (t) => {
  let duration = humanizeDuration(Timestamp, 1);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4 years', 'duration is not equal with max 1');

  duration = humanizeDuration(Timestamp, 2);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4 years and 10 months', 'duration is not equal with max 2');

  duration = humanizeDuration(Timestamp, 3);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4 years, 10 months and 2 weeks', 'duration is not equal with max 3');

  duration = humanizeDuration(Timestamp, 4);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4 years, 10 months, 2 weeks and 15 days', 'duration is not equal with max 4');

  duration = humanizeDuration(Timestamp, 5);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4 years, 10 months, 2 weeks, 15 days and 18 hours', 'duration is not equal with max 5');

  duration = humanizeDuration(Timestamp, 6);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4 years, 10 months, 2 weeks, 15 days, 18 hours and 41 minutes', 'duration is not equal with max 6');
});

test('short', (t) => {
  let duration = humanizeDuration(Timestamp, 1, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4y', 'duration is not equal with max 1 and short');

  duration = humanizeDuration(Timestamp, 2, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4y 10m', 'duration is not equal with max 2 and short');

  duration = humanizeDuration(Timestamp, 3, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4y 10m 2w', 'duration is not equal with max 3 and short');

  duration = humanizeDuration(Timestamp, 4, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4y 10m 2w 15d', 'duration is not equal with max 4 and short');

  duration = humanizeDuration(Timestamp, 5, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4y 10m 2w 15d 18h', 'duration is not equal with max 5 and short');

  duration = humanizeDuration(Timestamp, 6, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, '4y 10m 2w 15d 18h 41min', 'duration is not equal with max 6 and short');

  duration = humanizeDuration(Timestamp, 7, true);
  t.true(duration != '0' && !!duration);
  t.is(duration, SinceAbbrev, 'duration is not equal with max 7 and short');
});

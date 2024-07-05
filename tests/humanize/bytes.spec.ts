import test from 'ava';

import humanize from '../../packages/humanize/lib';

test('bytes', (t) => {
  t.is(humanize.bytes(0), '0 B');
  t.is(humanize.bytes(512), '512 B');
  t.is(humanize.bytes(1024), '1 KB');
  t.is(humanize.bytes(1024 * 1024), '1 MB');
  t.is(humanize.bytes(1024 * 1024 * 1024), '1 GB');
  t.is(humanize.bytes(1024 * 1024 * 1024 * 7.48212), '7.48 GB');
});
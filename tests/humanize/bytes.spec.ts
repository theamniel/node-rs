import test from 'ava';

import { humanizeBytes } from '../../packages/humanize/lib';

test('bytes', (t) => {
  t.is(humanizeBytes(0), '0 B');
  t.is(humanizeBytes(512), '512 B');
  t.is(humanizeBytes(1024), '1 KB');
  t.is(humanizeBytes(1024 * 1024), '1 MB');
  t.is(humanizeBytes(1024 * 1024 * 1024), '1 GB');
  t.is(humanizeBytes(1024 * 1024 * 1024 * 7.48212), '7.48 GB');
});
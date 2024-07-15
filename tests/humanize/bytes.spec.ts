import test from 'ava';

import humanize from '../../packages/humanize/lib';

const
  KB = 1024,
  MB = KB * 1024,
  GB = MB * 1024;

test('bytes', (t) => {
  t.is(humanize.bytes(0), '0 B');
  t.is(humanize.bytes(512), '512 B');

  t.is(humanize.bytes(KB), '1 KB');
  t.is(humanize.bytes(KB * 134), '134 KB');

  t.is(humanize.bytes(MB), '1 MB');
  t.is(humanize.bytes(MB * 98), '98 MB');

  t.is(humanize.bytes(GB), '1 GB');
  t.is(humanize.bytes(GB * 7.482), '7.48 GB');
});
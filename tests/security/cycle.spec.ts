import test from 'ava';
import crypto from 'node:crypto';
import { cycle } from '../../packages/security/lib';

const id = '604227193651986443';

test('negative:false', (t) => {
  let n = cycle(9, 198);
  t.not(n, 9);
  t.is(n, 7, 'Invalid cycle');
});

test('negative:true', (t) => {
  let decycle = cycle(7, 198, true);
  t.not(decycle, 7);
  t.is(decycle, 9, 'Invalid cycle');
});

test('cycle', (t) => {
  let gen = () => crypto.randomBytes(Math.floor(Math.random() * 20) + 1).reduce((a, b) => a + b);
  let result, retry = false, jitter = gen();

  do {
    if (retry) {
      t.log('Jitter', jitter, 'caused same number retry...');
      jitter = gen();
    }
    result = id.split('').map(n => cycle(Number(n), jitter, false)).join('');
    if (!retry) retry = true;
  } while (result == id); // WTF, first time I need this loop

  t.not(result, id, 'Cycle function is not working');

  result = result.split('').map(n => cycle(Number(n), jitter, true)).join('');
  t.is(result, id, 'Decycle is not working');
});

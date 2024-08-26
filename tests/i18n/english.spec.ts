import ava, { TestFn } from 'ava';
import path from 'node:path';

import { I18n } from '../../packages/i18n/lib';


const test = ava as TestFn<{ lang: I18n; }>;

test.before(t => {
  // All cache are shared for all instances (global cache by process)
  t.context.lang = new I18n({
    directory: path.join(__dirname, 'locales'),
    locales: ['es-ES', 'en-US'],
    default: 'en-US',
    preload: true,
  });
});

test('english', ({ is, context: { lang } }) => {
  // Basic usage
  is(lang.t('common:hello'), 'Hello, world!');

  // With argument (replace #{name} to -> name: <val>)
  is(lang.t('common:user.name'), 'Hello, {{name}}!');
  is(lang.t('common:user.name', { name: 'Amniel' }), 'Hello, Amniel!');

  // With arguments
  is(lang.t('common:user.greeting'), 'Hello, {{name}}! You are {{age}} years old and your birthday is on {{birthday}}');
  is(lang.t('common:user.greeting', { name: 'Amniel', age: 21, birthday: '05/21' }), 'Hello, Amniel! You are 21 years old and your birthday is on 05/21');

  // fallback (see locales/en-US/common and locales/es-ES/common)
  is(lang.t('common:fallback.to.es'), 'Esto lo traduce al español');
});

test.after((t) => {
  t.context.lang.reload();
});

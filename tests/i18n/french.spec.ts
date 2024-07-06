import test from 'ava';
import path from 'node:path';

import { I18n } from '../../packages/i18n/lib';

const lang = new I18n({
  directory: path.join(__dirname, 'locales'),
  locales: ['en-US', 'fr-FR'],
  default: 'fr-FR',
  preload: true,
});

test('french', (t) => {
  // Basic usage
  t.is(lang.t('common:hello'), 'Salut monde!');

  // With argument (replace #{name} to -> name: <val>)
  t.is(lang.t('common:user.name'), 'Bonjour, #{name}!');
  t.is(lang.t('common:user.name', { name: 'Amniel' }), 'Bonjour, Amniel!');

  // With arguments
  t.is(lang.t('common:user.greeting'), 'Bonjour, #{name}! Vous avez #{age} ans et votre anniversaire est le #{birthday}');
  t.is(lang.t('common:user.greeting', { name: 'Amniel', age: 21, birthday: '05/21' }), 'Bonjour, Amniel! Vous avez 21 ans et votre anniversaire est le 05/21');

  // fallback (see locales/en-US/common and locales/es-ES/common)
  t.is(lang.t('common:fallback.to.en'), 'This translate to english');
});

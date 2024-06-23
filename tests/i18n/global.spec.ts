import test from 'ava';
import path from 'node:path';

import { I18n } from '../../packages/i18n/lib';

const lang = new I18n({
  directory: path.join(__dirname, 'locales'),
  locales: ['en-US', 'es-ES', 'fr-FR'],
  fallback: 'en-US', // this is default lang if defaultLang is undefined
  default: 'es-ES',
  preload: true
});

test('global:spanish', (t) => {
  // SPANISH
  // Basic usage
  t.is(lang.t('common:hello'), '¡Hola, mundo!');

  // With argument (replace #{name} to -> name: <val>)
  t.is(lang.t('common:user.name'), '¡Hola, #{name}!');
  t.is(lang.t('common:user.name', { name: 'Amniel' }), '¡Hola, Amniel!');

  // With arguments
  t.is(lang.t('common:user.greeting'), '¡Hola, #{name}! Tienes #{age} años y tu cumpleaños es el #{birthday}.');
  t.is(lang.t('common:user.greeting', { name: 'Amniel', age: 21, birthday: '21/05' }), '¡Hola, Amniel! Tienes 21 años y tu cumpleaños es el 21/05.');

  // fallback (see locales/en-US/common and locales/es-ES/common)
  t.is(lang.t('common:fallback.to.en'), 'This translate to english');
});

test('global:english', (t) => {
  // ENGLISH
  lang.setLocale('en-US');
  lang.setFallback('es-ES');

  // Basic usage
  t.is(lang.t('common:hello'), 'Hello, world!');

  // With argument (replace #{name} to -> name: <val>)
  t.is(lang.t('common:user.name'), 'Hello, #{name}!');
  t.is(lang.t('common:user.name', { name: 'Amniel' }), 'Hello, Amniel!');

  // With arguments
  t.is(lang.t('common:user.greeting'), 'Hello, #{name}! You are #{age} years old and your birthday is on #{birthday}');
  t.is(lang.t('common:user.greeting', { name: 'Amniel', age: 21, birthday: '05/21' }), 'Hello, Amniel! You are 21 years old and your birthday is on 05/21');

  // fallback (see locales/en-US/common and locales/es-ES/common)
  t.is(lang.t('common:fallback.to.es'), 'Esto lo traduce al español');
});

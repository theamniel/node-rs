import test from 'ava';
import path from 'node:path';

import { init, t, setLocale, setFallback } from '../../packages/i18n/lib';

init({
  directory: path.join(__dirname, 'locales'),
  locales: ['en-US', 'es-ES', 'fr-FR'],
  fallback: 'en-US',
  default: 'es-ES',
  preload: true
});

test('global:spanish', (tt) => {
  // SPANISH
  // Basic usage
  tt.is(t('common:hello'), '¡Hola, mundo!');

  // With argument (replace #{name} to -> name: <val>)
  tt.is(t('common:user.name'), '¡Hola, {{name}}!');
  tt.is(t('common:user.name', { name: 'Amniel' }), '¡Hola, Amniel!');

  // With arguments
  tt.is(t('common:user.greeting'), '¡Hola, {{name}}! Tienes {{age}} años y tu cumpleaños es el {{birthday}}.');
  tt.is(t('common:user.greeting', { name: 'Amniel', age: 21, birthday: '21/05' }), '¡Hola, Amniel! Tienes 21 años y tu cumpleaños es el 21/05.');

  // fallback (see locales/en-US/common and locales/es-ES/common)
  tt.is(t('common:fallback.to.en'), 'This translate to english');
});

test('global:english', (tt) => {
  // ENGLISH
  setLocale('en-US');
  setFallback('es-ES');

  // Basic usage
  tt.is(t('common:hello'), 'Hello, world!');

  // With argument (replace #{name} to -> name: <val>)
  tt.is(t('common:user.name'), 'Hello, {{name}}!');
  tt.is(t('common:user.name', { name: 'Amniel' }), 'Hello, Amniel!');

  // With arguments
  tt.is(t('common:user.greeting'), 'Hello, {{name}}! You are {{age}} years old and your birthday is on {{birthday}}');
  tt.is(t('common:user.greeting', { name: 'Amniel', age: 21, birthday: '05/21' }), 'Hello, Amniel! You are 21 years old and your birthday is on 05/21');

  // fallback (see locales/en-US/common and locales/es-ES/common)
  tt.is(t('common:fallback.to.es'), 'Esto lo traduce al español');
});

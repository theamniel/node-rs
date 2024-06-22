import test from 'ava';
import path from 'node:path';

import { I18n } from '../../packages/i18n/lib';

test('i18n', (tt) => {
  new I18n({
    directory: path.join(__dirname, 'locales'),
    locales: ['en-US', 'es-ES', 'fr-FR'],
    preload: true
  });

  tt.pass();
});


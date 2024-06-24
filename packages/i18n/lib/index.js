/**
 * @typedef {import('./index').I18n} I18n
 * @typedef {import('./index').I18nConfig} I18nConfig
 */

/**
 * @type {?I18n}
 */
let i18n = null;


module.exports = {
  ...require('./bindings'),
  /**
   * 
   * @param {I18nConfig} options
   * @param {(err: Error) => void} [cb]
   * @returns {void}
   */
  init(options, cb) {
    if (!i18n) {
      try {
        i18n = new (require('./bindings')).I18n(options);
      } catch (err) {
        if (!cb) throw err;
        cb(err);
      }
    }
  },
  /**
   * @param {string} key
   * @param {Record<string, string | number | boolean>} [args]
   * @returns {string}
   */
  t(key, args) {
    if (!i18n) throw new Error('I18n not initialized');
    return i18n.t(key, args);
  },
  /**
   * 
   * @param {string} locale 
   * @param {string} key 
   * @param {Record<string, string | number | boolean>} [args] 
   * @returns {string}
   */
  translate(locale, key, args) {
    if (!i18n) throw new Error('I18n not initialized');
    return i18n.translate(locale, key, args);
  },
  /**
   * 
   * @param {string} locale 
   * @returns {void}
   */
  setLocale(locale) {
    if (!i18n) throw new Error('I18n not initialized');
    i18n.setLocale(locale);
  },
  /**
   * 
   * @param {string} locale 
   * @returns {void}
   */
  setFallback(locale) {
    if (!i18n) throw new Error('I18n not initialized');
    i18n.setFallback(locale);
  },
  /**
   * 
   * @param {string} [locale] 
   * @param {string} [key]
   * @returns {void}
   */
  reload(locale, key) {
    if (!i18n) throw new Error('I18n not initialized');
    i18n.reload(locale, key);
  }
};
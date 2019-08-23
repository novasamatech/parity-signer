

/**
 * @typedef {Object} SURIObject
 * @property {string} phrase - The valid bip39 seed phrase
 * @property {string} derivePath - The derivation path consisting in `/soft` and or `//hard`, can be repeated and interchanges
 * @property {string} password - The optionnal password password without the `///`
 */

/**
 * @description ExtraaccountIdcts the phrase, path and password from a SURI format for specifying secret keys `<secret>/<soft-key>//<hard-key>///<password>` (the `///password` may be omitted, and `/<soft-key>` and `//<hard-key>` maybe repeated and mixed).
 * @param {string} suri The SURI to be parsed
 * @returns {SURIObject}
 */

export function parseSURI (suri) {
  const RE_CAPTURE = /^(\w+(?: \w+)*)?((?:\/\/?[^/]+)*)(?:\/\/\/(.*))?$/;
  const matches = suri.match(RE_CAPTURE);
  let phrase, derivePath, password = '';

  if (matches) {
    [, phrase = '', derivePath = '', password = ''] = matches;
  }

  return {
    phrase,
    derivePath,
    password
  };
}

/**
 * @description Return a SURI format from a bip39 phrase, a derivePath, e.g `//hard/soft` and a password.
 * @param {SURIObject} SURIObject
 * @returns {string}
 */

export function constructSURI ({ derivePath = '', password = '', phrase = '' }) {
  
  return `${phrase}${derivePath}///${password}`;
}
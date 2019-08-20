

/**
 * @description Extracts the phrase, path and password from a SURI format for specifying secret keys `<secret>/<soft-key>//<hard-key>///<password>` (the `///password` may be omitted, and `/<soft-key>` and `//<hard-key>` maybe repeated and mixed).
 */

export default function keyExtract (suri) {
  const RE_CAPTURE = /^(\w+( \w+)*)?((\/\/?[^/]+)*)(\/\/\/(.*))?$/;
  const matches = suri.match(RE_CAPTURE);

  let phrase, derivePath, password = '';

  if (matches) {
    [, phrase = '', , derivePath = '', , , password = ''] = matches;
  }


  return {
    phrase,
    derivePath,
    password
  };
}
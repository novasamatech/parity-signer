/**
 * @description Extracts the path and password from a SURI format for specifying secret keys `<secret>/<soft-key>//<hard-key>///<password>` (the `///password` may be omitted, and `/<soft-key>` and `//<hard-key>` maybe repeated and mixed).
 */

export default function keyExtract (suri) {
  const RE_CAPTURE = /^((\/\/?[^/]+)*)(\/\/\/(.*))?$/;
  const matches = suri.match(RE_CAPTURE);

  if(!matches) return false;

  const [, derivePath, , , password] = matches;

  return {
    derivePath,
    password
  };
}
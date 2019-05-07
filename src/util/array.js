/**
 * Default comparator, should work for strings and numbers
 */
function defaultCompare(a, b) {
  if (a > b) {
      return 1;
  }

  if (a < b) {
      return -1;
  }

  return 0;
}

/**
 * Find an index of an element within a sorted array. This should be substantially
 * faster than `indexOf` for large arrays.
 *
 * @type  {T}                item    type
 * @param {T}                item    to find
 * @param {Array<T>}         array   to look through
 * @param {(a, b) => number} [compare = defaultCompare] comparator function
 *
 * @return {{ hit: bool, index: number }} if `hit` is `true` -> index at which the item was found
 *                                        if `hit` is `false` -> index at which the item can be inserted
 */
export function binarySearch(array, item, compare = defaultCompare) {
  if (array.length === 0) {
    return -1;
  }

  let min = 0;
  let max = array.length - 1;

  while (min !== max) {
    let guess = (min + max) / 2 | 0;
    const other = array[guess];

    if (item === other) {
      return { hit: true, index: guess };
    }

    const result = compare(item, other);

    if (result < 0) {
      max = Math.max(min, guess - 1);
    } else if (result > 0) {
      min = Math.min(max, guess + 1);
    } else {
      // Equal sort value, but different reference, do value search from min
      return { hit: true, index: array.indexOf(item, min) };
    }
  }

  const result = compare(item, array[min]);

  if (result < 0) {
    return { hit: false, index: min };
  } else if (result > 0) {
    return { hit: false, index: min + 1 };
  }

  return { hit: true, index: min };
}

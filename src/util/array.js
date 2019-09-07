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
    return { hit: false, index: 0 };
  }

  let min = 0;
  let max = array.length - 1;

  while (min <= max) {
    let guess = (min + max) >> 1; // fast integer division by 2

    const result = compare(item, array[guess]);

    if (result < 0) {
      max = guess - 1;
    } else if (result > 0) {
      min = guess + 1;
    } else {
      return { hit: true, index: guess };
    }
  }

  return { hit: false, index: min };
}

export function zip(left, right) {
  let lindex = 0;
  let rindex = 0;
  let oindex = 0;

  // allocate enough memory to merge two arrays
  const out = new Array(left.length + right.length);

  while (lindex < left.length && rindex < right.length) {
    let lword = left[lindex];
    let rword = right[rindex];

    if (lword < rword) {
      out[oindex] = lword;
      lindex += 1;
    } else if (lword > rword) {
      out[oindex] = rword;
      rindex += 1;
    } else {
      out[oindex] = lword;
      lindex += 1;
      rindex += 1;
    }

    oindex += 1;
  }

  return out.slice(0, oindex);
}

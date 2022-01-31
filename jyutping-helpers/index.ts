type ConversionFn = (inputs: Array<string>) => Array<string>;

const jyutpingToYaleDict = {
  oe: "eu",
  j: "y",
  aa: "a",
  z: "j",
  c: "ch",
  // REF https://en.wikipedia.org/wiki/Jyutping#:~:text=sik6%2F9-,Comparison%20with%20Yale%20romanisation,nasal%20stop%3A%20m%2C%20ng.
  //  In Jyutping, if no consonant precedes the vowel yu, then the initial j is appended before the vowel. In Yale, the corresponding initial y is never appended before yu under any circumstances.
  jy: "y",
};

export const jyutpingToYale: ConversionFn = (inputs) => {
  const result = [...inputs];
  let index = 0;

  for (const word of result) {
    let i = 0;
    let j = 1;
    const mutable = [...word];

    while (i < mutable.length) {
      const cur = mutable[i];

      // NOTE Check for double letters replacement first, as jyu > yu conversion will be eaten up by j -> y conversion
      const doubleSub = jyutpingToYaleDict[`${mutable[i]}${mutable[j]}`];
      if (doubleSub) {
        mutable[i] = doubleSub[0];
        mutable[j] = doubleSub[1] ?? "";
        i++;
        j++;
      } else {
        // NOTE Check for single letter replacement
        const sub = jyutpingToYaleDict[cur];
        if (sub) {
          mutable[i] = sub;
        }
      }

      i++;
      j++;
    }
    result[index] = mutable.join("");
    index++;
  }

  return result;
};

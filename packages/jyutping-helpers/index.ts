type ConversionDict = {
  [key: string]: string;
};
// NOTE Assuming the input might have multiple characters per word in the array
// Example inputs: ['ngo5', 'hai6', 'hoeng1gong2' 'jan4']
type ConversionFn = (inputs: string) => string;

const jyutpingToYaleDict: ConversionDict = {
  oe: "eu",
  eo: "eu",
  j: "y",
  //  Only convert when not ending consonant follow aa
  aa: "a",
  z: "j",
  c: "ch",
  // REF https://en.wikipedia.org/wiki/Jyutping#:~:text=sik6%2F9-,Comparison%20with%20Yale%20romanisation,nasal%20stop%3A%20m%2C%20ng.
  //  In Jyutping, if no consonant precedes the vowel yu, then the initial j is appended before the vowel. In Yale, the corresponding initial y is never appended before yu under any circumstances.
  jy: "y",
};

export const jyutpingToYale: ConversionFn = (word) => {
  const dict = jyutpingToYaleDict;
  const numberRegex = /\d/;
  let i = 0;
  let j = 1;
  const mutable = [...word];

  while (i < mutable.length) {
    const cur = mutable[i];
    const doubleCur = mutable[i] + mutable[j];

    // NOTE Check for double letters replacement first, as jyu > yu conversion will be eaten up by j -> y conversion
    const doubleSub = dict[doubleCur];
    if (doubleSub) {
      //Check if ending consonant exists. Only do the conversion when
      if (doubleCur !== "aa" || numberRegex.test(mutable[j + 1])) {
        mutable[j] = "";
        mutable[i] = doubleSub;
      }
      i++;
      j++;
    } else {
      // NOTE Check for single letter replacement
      const sub = dict[cur];
      if (sub) {
        mutable[i] = sub;
      }
    }

    i++;
    j++;
  }
  return mutable.join("");
};

const toneMarkDict: ConversionDict = {
  "1": "\u0304",
  "2": "\u0301",
  "3": "",
  "4": "\u0300h",
  "5": "\u0301h",
  "6": "h",
};

//  Dict that list all vowels in Yale, as it will be transfered from Jyutping to Yale first
const vowelDict = {
  a: true,
  e: true,
  i: true,
  o: true,
  u: true,
  m: true,
  n: true,
};

export const jyutpingToTraditionalYale: ConversionFn = (word) => {
  const dict = jyutpingToYaleDict;
  const numberRegex = /\d/;
  let i = 0;
  let j = 1;
  let k = 1;
  let toneNumber = "";
  const mutable = [...word];

  while (i < mutable.length) {
    if (!toneNumber && k < mutable.length) {
      if (numberRegex.test(mutable[k])) {
        toneNumber = mutable[k];
        mutable[k] = "";
      }
      k++;
    } else {
      const cur = mutable[i];
      const doubleCur = mutable[i] + mutable[j];

      // NOTE Check for double letters replacement first, as jyu > yu conversion will be eaten up by j -> y conversion
      const doubleSub = dict[doubleCur];
      if (doubleSub) {
        //  Check if ending consonant exists. Only do the conversion when
        if (doubleCur !== "aa" || numberRegex.test(mutable[j + 1])) {
          mutable[j] = doubleSub[1];
          mutable[i] = doubleSub[0];
        }
      } else {
        // NOTE Check for single letter replacement
        const sub = dict[cur];
        if (sub) {
          mutable[i] = sub;
        }
      }

      //  Handle tone replacement
      if (vowelDict[cur] && toneNumber) {
        switch (cur) {
          case "m":
            // NOTE Only add tone to m if the next characters is tone
            if (!mutable[i + 1]) {
              mutable[i] += toneMarkDict[toneNumber];
              //  Move i and j forward to the last found tone position, to avoid match again after match
              i = k;
              j = i + 1;
              toneNumber = "";
            }
            break;
          case "n":
            // NOTE check if being used as a vowel
            //  The number will be stripped off previously
            if (!mutable[i + 2]) {
              mutable[i] += toneMarkDict[toneNumber][0];
              mutable[i + 1] += toneMarkDict[toneNumber][1];
              //  Move i and j forward to the last found tone position, to avoid match again after match
              i = k;
              j = i + 1;
              toneNumber = "";
            }
            break;

          default:
            if (
              toneNumber === "4" ||
              toneNumber === "5" ||
              toneNumber === "6"
            ) {
              if (
                vowelDict[mutable[j]] &&
                mutable[j] !== "m" &&
                mutable[j] !== "n"
              ) {
                mutable[i] +=
                  toneMarkDict[toneNumber].length === 1
                    ? ""
                    : toneMarkDict[toneNumber][0];
                mutable[j] += "h";
              } else {
                mutable[i] += toneMarkDict[toneNumber];
              }
            } else {
              mutable[i] += toneMarkDict[toneNumber];
            }
            //  Move i and j forward to the last found tone position, to avoid match again after match
            i = k;
            j = i + 1;
            toneNumber = "";
            break;
        }
      }

      i++;
      j++;
    }
  }

  return mutable.join("");
};

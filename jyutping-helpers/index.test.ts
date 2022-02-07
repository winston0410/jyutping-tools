import { jyutpingToYale, jyutpingToTraditionalYale } from "./index";

describe("oe in Jyutping", () => {
  it("should be converted to eu in Yale", () => {
    //  香港
    expect(jyutpingToYale(["hoeng1gong2"])).toEqual(["heung1gong2"]);
  });
});

describe("eo in Jyutping", () => {
  it("should be converted to eu in Yale", () => {
    //  鋸
    expect(jyutpingToYale(["geoi3"])).toEqual(["geui3"]);
  });
});

describe("j in Jyutping", () => {
  it("should be converted to y in Yale", () => {
    //  人
    expect(jyutpingToYale(["jan4"])).toEqual(["yan4"]);
  });
});

describe("aa in Jyutping", () => {
  it("should be converted to a in Yale", () => {
    //  沙
    expect(jyutpingToYale(["saa1"])).toEqual(["sa1"]);
  });

  describe("when aa is followed by ending consonant", () => {
    it("should be converted to aa in Yale", () => {
      //  生
      expect(jyutpingToYale(["saang1"])).toEqual(["saang1"]);
    });
  });
});

describe("z in Jyutping", () => {
  it("should be converted to j in Yale", () => {
    //  正
    expect(jyutpingToYale(["zeng3"])).toEqual(["jeng3"]);
  });
});

describe("c in Jyutping", () => {
  it("should be converted to ch in Yale", () => {
    //  車
    expect(jyutpingToYale(["ce1"])).toEqual(["che1"]);
  });
});

describe("jyu in Jyutping", () => {
  it("should be converted to ch in Yale", () => {
    //  魚
    expect(jyutpingToYale(["jyu4"])).toEqual(["yu4"]);
  });
});

describe("tone 1 in Jyutping", () => {
  //  因
  describe("when the final is a single character", () => {
    it("should be converted correctly in traditional Yale", () => {
      expect(jyutpingToTraditionalYale(["jan1"])).toEqual(["yān"]);
    });
  });

  //  生
  describe("when the final is two characters", () => {
    it("should be converted correctly in traditional Yale", () => {
      expect(jyutpingToTraditionalYale(["saang1"])).toEqual(["sāang"]);
    });
  });
});

describe("tone 2 in Jyutping", () => {
  //  隱
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan2"])).toEqual(["yán"]);
  });
});

describe("tone 3 in Jyutping", () => {
  //  印
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan3"])).toEqual(["yan"]);
  });
});

describe("tone 4 in Jyutping", () => {
  //  人
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan4"])).toEqual(["yàhn"]);
  });

  describe("when the final is two characters", () => {
    it("should be converted correctly in traditional Yale", () => {
      //  牆
      expect(jyutpingToTraditionalYale(["coeng4"])).toEqual(["chèuhng"]);
    });
  });
});

describe("tone 5 in Jyutping", () => {
  //  癮
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan5"])).toEqual(["yáhn"]);
  });

  describe("when the final is two characters", () => {
    it("should be converted correctly in traditional Yale", () => {
      expect(jyutpingToTraditionalYale(["laang5"])).toEqual(["láahng"]);
    });
  });
});

describe("tone 6 in Jyutping", () => {
  //  韌
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan6"])).toEqual(["yahn"]);
  });

  describe("when the final is two characters", () => {
    it("should be converted correctly in traditional Yale", () => {
      //  上
      expect(jyutpingToTraditionalYale(["soeng6"])).toEqual(["seuhng"]);
      //  弱
      expect(jyutpingToTraditionalYale(["joek6"])).toEqual(["yeuhk"]);
    });
  });
});

describe("vowel i in Jyutping", () => {
  //  英
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jing1"])).toEqual(["yīng"]);
  });
});

describe("vowel e in Jyutping", () => {
  //  咩
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["me1"])).toEqual(["mē"]);
  });
});

describe("vowel o in Jyutping", () => {
  //  好
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["hou2"])).toEqual(["hóu"]);
    //  浪
    expect(jyutpingToTraditionalYale(["long6"])).toEqual(["lohng"]);
  });
});

describe("vowel u in Jyutping", () => {
  //  碗
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["wun2"])).toEqual(["wún"]);
  });
});

describe("vowel m in Jyutping", () => {
  //  唔
  it("should be mark with tone mark correctly in traditional Yale", () => {
    // NOTE Cannot display m with tone mark in unicode correctly in terminal
    expect(jyutpingToTraditionalYale(["m4"])).toEqual(["m̀h"]);
  });
});

describe("vowel ng in Jyutping", () => {
  //  五
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["ng5"])).toEqual(["ńgh"]);
  });
});

describe("vowel oe in Jyutping", () => {
  //  香
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["hoeng1"])).toEqual(["hēung"]);
  });
});

describe("vowel eo in Jyutping", () => {
  //  律
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["leot6"])).toEqual(["leuht"]);
  });
});

describe("vowel eoi in Jyutping", () => {
  //  水
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["seoi2"])).toEqual(["séui"]);
  });
});

describe("when given two characters", () => {
  //  香港
  it("should be mark with tone mark correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["hoeng1gong2"])).toEqual(["hēunggóng"]);
  });
});

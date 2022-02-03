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
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan1"])).toEqual(["yān"]);
  });
});

describe("tone 2 in Jyutping", () => {
  //  隱
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan2"])).toEqual(["yán"]);
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
    expect(jyutpingToTraditionalYale(["jan4"])).toEqual(["yàhn"]);
  });
});

describe("tone 5 in Jyutping", () => {
  //  癮
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan5"])).toEqual(["yáhn"]);
  });
});

describe("tone 6 in Jyutping", () => {
  //  韌
  it("should be converted correctly in traditional Yale", () => {
    expect(jyutpingToTraditionalYale(["jan6"])).toEqual(["yahn"]);
  });
});

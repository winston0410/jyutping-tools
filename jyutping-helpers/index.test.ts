import { jyutpingToYale } from "./index";

describe("oe in Jyutping", () => {
  it("should be converted to eu in Yale", () => {
    //  香港
    expect(jyutpingToYale(["hoeng1gong2"])).toEqual(["heung1gong2"]);
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

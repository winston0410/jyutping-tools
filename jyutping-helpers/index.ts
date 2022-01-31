type ConversionFn = (inputs: Array<string>) => string;

export const jyutpingToYale: ConversionFn = (inputs) => {
  return inputs[0];
};

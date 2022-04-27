from transformers import XLNetTokenizerFast, XLNetTokenizer
import os

tmp_dir = os.getenv('TMPDIR')
tokenizer = XLNetTokenizer.from_pretrained(tmp_dir + "tokenizer")

# Getting the tokenized token
result = tokenizer.tokenize("你係香港人？")
print(result)

# Getting the output for tokenizer/input for the model
print(tokenizer("你係香港人？"))
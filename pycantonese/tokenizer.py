from transformers import XLNetTokenizerFast, XLNetTokenizer
import os

tokenizer = XLNetTokenizer()

print(tokenizer)

# Save dataset to $TMPDIR
# tmp_dir = os.getenv('TMPDIR')
# tokenizer.save_to_disk(tmp_dir + "tokenizer")
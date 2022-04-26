from transformers import XLNetTokenizerFast, XLNetTokenizer
import os

# vocab_file should be given *.model instead of *.vocab
tokenizer = XLNetTokenizer(vocab_file="./dummy.model")

# Save dataset to $TMPDIR
tmp_dir = os.getenv('TMPDIR')
tokenizer.save_pretrained(tmp_dir + "tokenizer")
import torch
from transformers import XLNetTokenizerFast, XLNetTokenizer
import os

tmp_dir = os.getenv('TMPDIR')
tokenizer = XLNetTokenizer.from_pretrained(tmp_dir + "tokenizer")
# Example script for using the dataset
# Assuming the dataset is saved into disk
from datasets import load_from_disk
import os

# Loading dataset from $TMPDIR. Need to save datasets over there first
tmp_dir = os.getenv('TMPDIR')
dataset = load_from_disk(tmp_dir + "datasets")

print(dataset)

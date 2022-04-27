from datasets import load_dataset
from operator import itemgetter
import os

dataset_raw = load_dataset("text", data_files="dummy_dataset.txt")

dataset = dataset_raw["train"].train_test_split(test_size=0.4)

# Split full dataset into train, validation and test
validation_set, test_set = itemgetter("train", "test")(dataset["test"].train_test_split(test_size=0.5))

dataset["test"] = test_set
dataset["validation"] = validation_set

# Save dataset to $TMPDIR
tmp_dir = os.getenv('TMPDIR')
dataset.save_to_disk(tmp_dir + "datasets")
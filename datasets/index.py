from datasets import load_dataset
from operator import itemgetter

mock_dataset_raw = load_dataset("text", data_files="mock_data.txt")

mock_dataset = mock_dataset_raw["train"].train_test_split(test_size=0.4)

# Split full dataset into train, validation and test
validation_set, test_set = itemgetter("train", "test")(mock_dataset["test"].train_test_split(test_size=0.5))

mock_dataset["test"] = test_set
mock_dataset["validation"] = validation_set

# mock_dataset is the complete dataset
print(mock_dataset)
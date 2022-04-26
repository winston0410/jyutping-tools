from transformers import XLNetConfig, XLNetModel
from datasets import load_dataset

# Load a dataset, just something random now
dataset = load_dataset("conll2003")

# Create a new model without pretrained weight
configuration = XLNetConfig()
model = XLNetModel(configuration)

print(dataset)

print("Done!")
from transformers import XLNetConfig, XLNetModel
from datasets import load_dataset

# Create a new model without pretrained weight
configuration = XLNetConfig()
model = XLNetModel(configuration)

print("Done!")
from transformers import AutoTokenizer

# A tokenizer for English
tokenizer = AutoTokenizer.from_pretrained("xlnet-base-cased")

example = "我係香港人"

# The result would show that the incoming string is unknown to the tokenizer
# with 0 as the input_ids
print("Tokenized token: ", tokenizer.tokenize(example))
print("Converted ID token: ", tokenizer.convert_tokens_to_ids(tokenizer.tokenize(example)))
print("Overall output: ", tokenizer(example))
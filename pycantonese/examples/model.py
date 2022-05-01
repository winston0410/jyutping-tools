# REF https://colab.research.google.com/drive/1yXQBpQLmt05W2RLLa0ceg6SGxCKk3YcY?usp=sharing#scrollTo=8XrlYxEhNDv-
import torch
import os
from transformers import XLNetTokenizerFast, XLNetTokenizer, DataCollatorWithPadding, TrainingArguments, XLNetConfig, XLNetModel, Trainer
from datasets import load_dataset, load_from_disk
# Use the pipeline for POS tagging
# from transformers import pipeline

# For the metric
from datasets import load_metric
import numpy as np

# tmp_dir = os.getenv('TMPDIR')
# tokenizer = XLNetTokenizer.from_pretrained(tmp_dir + "tokenizer")

# Custom dataset
# raw_datasets = load_from_disk(tmp_dir + "datasets")

# # Tokenize the raw dataset, preparing it for the model
# def tokenize_fn(dataset):
#     return tokenizer(dataset["text"], truncation=True)

# tokenized_datasets = raw_datasets.map(tokenize_fn, batched=True)

# Datset used from colab to train XLNET
from transformers import AutoTokenizer
tokenizer = AutoTokenizer.from_pretrained("xlnet-base-cased")
dataset = load_dataset("conll2003")

print('check dataset', dataset["train"].features)

def align_labels_with_tokens(labels, word_ids):
    new_labels = []
    current_word = None
    for word_id in word_ids:
        if word_id != current_word:
            # Start of a new word!
            current_word = word_id
            label = -100 if word_id is None else labels[word_id]
            new_labels.append(label)
        elif word_id is None:
            # Special token
            new_labels.append(-100)
        else:
            # Same word as previous token
            label = labels[word_id]
            # If the label is B-XXX we change it to I-XXX
            if label % 2 == 1:
                label += 1
            new_labels.append(label)

    return new_labels

def tokenize_and_align_labels(examples):
    tokenized_inputs = tokenizer(
        examples["tokens"], truncation=True, is_split_into_words=True
    )
    all_labels = examples["ner_tags"]
    new_labels = []
    for i, labels in enumerate(all_labels):
        word_ids = tokenized_inputs.word_ids(i)
        new_labels.append(align_labels_with_tokens(labels, word_ids))

    tokenized_inputs["labels"] = new_labels
    return tokenized_inputs

tokenized_datasets = dataset.map(
    tokenize_and_align_labels,
    batched=True,
    remove_columns=dataset["train"].column_names,
)

print('check tokenized', tokenized_datasets)

# data_collator = DataCollatorWithPadding(tokenizer=tokenizer)

# training_args = TrainingArguments(tmp_dir + "trainer")

# configuration = XLNetConfig()
# model = XLNetModel(configuration)

# metric = load_metric("seqeval")

# def compute_metrics(eval_preds):
#     logits, labels = eval_preds
#     predictions = np.argmax(logits, axis=-1)

#     # Remove ignored index (special tokens) and convert to labels
#     true_labels = [[label_names[l] for l in label if l != -100] for label in labels]
#     true_predictions = [
#         [label_names[p] for (p, l) in zip(prediction, label) if l != -100]
#         for prediction, label in zip(predictions, labels)
#     ]
#     all_metrics = metric.compute(predictions=true_predictions, references=true_labels)
#     return {
#         "precision": all_metrics["overall_precision"],
#         "recall": all_metrics["overall_recall"],
#         "f1": all_metrics["overall_f1"],
#         "accuracy": all_metrics["overall_accuracy"],
#     }

# trainer = Trainer(
#     model,
#     training_args,
#     train_dataset=tokenized_datasets["train"],
#     eval_dataset=tokenized_datasets["validation"],
#     compute_metrics=compute_metrics,
#     tokenizer=tokenizer,
# )

# trainer.train()
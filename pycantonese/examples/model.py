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

tmp_dir = os.getenv('TMPDIR')

tokenizer = XLNetTokenizer.from_pretrained(tmp_dir + "tokenizer")

raw_datasets = load_from_disk(tmp_dir + "datasets")

# Tokenize the raw dataset, preparing it for the model
def tokenize_fn(dataset):
    return tokenizer(dataset["text"], truncation=True)

tokenized_datasets = raw_datasets.map(tokenize_fn, batched=True)

data_collator = DataCollatorWithPadding(tokenizer=tokenizer)

training_args = TrainingArguments(tmp_dir + "trainer")

configuration = XLNetConfig()
model = XLNetModel(configuration)

metric = load_metric("seqeval")

def compute_metrics(eval_preds):
    logits, labels = eval_preds
    predictions = np.argmax(logits, axis=-1)

    # Remove ignored index (special tokens) and convert to labels
    true_labels = [[label_names[l] for l in label if l != -100] for label in labels]
    true_predictions = [
        [label_names[p] for (p, l) in zip(prediction, label) if l != -100]
        for prediction, label in zip(predictions, labels)
    ]
    all_metrics = metric.compute(predictions=true_predictions, references=true_labels)
    return {
        "precision": all_metrics["overall_precision"],
        "recall": all_metrics["overall_recall"],
        "f1": all_metrics["overall_f1"],
        "accuracy": all_metrics["overall_accuracy"],
    }

# trainer = Trainer(
#     model,
#     training_args,
#     train_dataset=tokenized_datasets["train"],
#     eval_dataset=tokenized_datasets["validation"],
#     compute_metrics=compute_metrics,
#     tokenizer=tokenizer,
# )

# trainer.train()
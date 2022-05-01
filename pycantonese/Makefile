# export SHELL = /bin/zsh
run@model:
	python3.10 model.py
run@model@example:
	python3.10 ./examples/model.py
run@tokenizer:
	python3.10 tokenizer.py
run@tokenizer@example:
	python3.10 ./examples/tokenizer.py
run@tokenizer@example@incompatible_tokenizer:
	python3.10 ./examples/incompatible_tokenizer.py
run@dataset:
	python3.10 dataset.py
run@dataset@example:
	python3.10 ./examples/dataset.py
run@sentencepiece:
	spm_train --input=./dummy.txt --model_prefix=dummy --vocab_size=18 --character_coverage=0.9995
run@sentencepiece@unwanted_token:
	spm_train --input=./dummy_unwanted_token.txt --model_prefix=dummy --vocab_size=87 --character_coverage=0.9995
update:
	conda env update --file environment.yml
# Cannot run conda activate pycantonese in make, not even if you have set the shell to zsh
# activate:
# 	conda activate pycantonese
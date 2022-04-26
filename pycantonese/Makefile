# export SHELL = /bin/zsh

run:
	python3.10 index.py
run@tokenizer:
	python3.10 tokenizer.py
run@tokenizer@example:
	python3.10 ./examples/tokenizer.py
run@sentencepiece:
	spm_train --input=./dummy.txt --model_prefix=dummy --vocab_size=18 --character_coverage=0.9995
update:
	conda env update --file environment.yml
# Cannot run conda activate pycantonese in make, not even if you have set the shell to zsh
# activate:
# 	conda activate pycantonese
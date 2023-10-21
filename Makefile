install:
	pip install --upgrade pip &&\
		pip install --prefer-binary -r requirements.txt

test:
	python -m pytest -vv  python.py

format:
	black *.py

lint:
	pylint --disable=R,C --ignore-patterns=pytho_.*?py *.py

all: install lint format test

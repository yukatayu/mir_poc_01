.PHONY: check docs new-report cargo-check

check:
	python scripts/validate_docs.py
	cargo check

docs:
	python scripts/validate_docs.py

new-report:
	python scripts/new_report.py --slug $(slug)

cargo-check:
	cargo check

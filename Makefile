.PHONY: check docs new-report cargo-check

check:
	python3 scripts/check_source_hierarchy.py
	python3 scripts/validate_docs.py
	cargo check

docs:
	python3 scripts/check_source_hierarchy.py
	python3 scripts/validate_docs.py

new-report:
	python3 scripts/new_report.py --slug $(slug)

cargo-check:
	cargo check

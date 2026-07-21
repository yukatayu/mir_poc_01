.PHONY: check docs new-report cargo-check

check:
	cd mirrorea_canon && python3 meta/build-index.py --check
	python3 scripts/check_source_hierarchy.py
	python3 scripts/validate_docs.py
	cargo check

docs:
	cd mirrorea_canon && python3 meta/build-index.py --check
	python3 scripts/check_source_hierarchy.py
	python3 scripts/validate_docs.py

new-report:
	python3 scripts/new_report.py --slug $(slug)

cargo-check:
	cargo check

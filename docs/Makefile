# Makefile to compile the documentation for miniclj
# Mario Jiménez <mario.emilio.j@gmail.com>

IND1.pdf: commits.txt logs.md ../examples/*
	tectonic -Zshell-escape IND1.tex

commits.txt:
	git log --oneline --pretty='format:%C(auto)%h - %s (%ad)' --date=short --reverse > commits.txt

logs.md:
	awk '/# Avance 1/,0' ../README.md > logs.md

../examples/%.mclj: ../examples/%.clj ../target/debug/miniclj
	cd .. && cargo run build examples/$*.clj

.PHONY: clean
clean:
	rm commits.txt logs.md

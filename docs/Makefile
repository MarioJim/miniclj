# Makefile to compile the documentation for miniclj
# Mario Jiménez <mario.emilio.j@gmail.com>

all: DesignDoc.pdf UserManual.pdf

DesignDoc.pdf: DesignDoc.tex commits.txt logs.md ../examples/*
	tectonic -Zshell-escape DesignDoc.tex

UserManual.pdf: UserManual.tex
	tectonic -Zshell-escape UserManual.tex

commits.txt: ../.git/refs/heads/main
	git log --oneline --pretty='format:%C(auto)%h - %s (%ad)' --date=short --reverse > commits.txt

logs.md: ../README.md
	awk '/# Avance 1/,0' ../README.md > logs.md

../examples/%.mclj: ../examples/%.clj ../target/debug/miniclj
	cd .. && cargo run build examples/$*.clj

.PHONY: clean
clean:
	rm commits.txt logs.md

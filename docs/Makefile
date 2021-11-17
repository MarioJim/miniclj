# Makefile to compile the documentation for miniclj
# Mario Jiménez <mario.emilio.j@gmail.com>

IND1.pdf: commits.txt logs.md
	tectonic -Zshell-escape IND1.tex

commits.txt:
	git log --oneline --pretty='format:%C(auto)%h - %s (%ad)' --date=short --reverse > commits.txt

logs.md:
	awk '/# Avance 1/,0' ../README.md > logs.md

.PHONY: clean
clean:
	rm commits.txt logs.md

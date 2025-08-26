.PHONY: clean zip

all: cover_letter.pdf termal.pdf termal-preprint.pdf cover_letter_revision.pdf

cover_letter_revision.pdf: cover_letter_revision.tex
	xelatex $<

cover_letter.pdf: cover_letter.tex
	xelatex $<

termal.pdf: termal.tex termal.bib 
	xelatex $<
	bibtex termal
	xelatex $<
	xelatex $<

termal-preprint.pdf: termal-preprint.tex termal.bib 
	xelatex $<
	bibtex termal-preprint
	xelatex $<
	xelatex $<

SUBMISSION_DIR = latex-src

latex-src.zip: termal.tex termal.bib mynat.bst README.md figure-1.pdf
	# Clean and recreate submission dir
	rm -rf $(SUBMISSION_DIR)
	mkdir $(SUBMISSION_DIR)
	# Symlink required files (adjust as needed)
	ln -s ../termal.tex $(SUBMISSION_DIR)/termal.tex
	ln -s ../termal.bib $(SUBMISSION_DIR)/termal.bib
	ln -s ../mynat.bst $(SUBMISSION_DIR)/mynat.bst
	ln -s ../figure-1.pdf $(SUBMISSION_DIR)/figure-1.pdf
	ln -s ../README.md $(SUBMISSION_DIR)/README.md
	# Create zip archive, following symlinks
	zip -r $@ $(SUBMISSION_DIR)

clean:
	$(RM) *.aux *.log *.bbl *.blg *.out *.toc *.lof *.lot latex-src.zip

.PHONY: clean zip

all: cover_letter.pdf termal.pdf

cover_letter.pdf: cover_letter.tex
	xelatex $<

termal.pdf: termal.tex termal.bib 
	xelatex $<
	bibtex termal
	xelatex $<
	xelatex $<

ZIP_TARGET = latex-src.zip
SUBMISSION_DIR = latex-src

zip: $(ZIP_TARGET)

$(ZIP_TARGET):
	# Clean and recreate submission dir
	rm -rf $(SUBMISSION_DIR)
	mkdir $(SUBMISSION_DIR)
	# Symlink required files (adjust as needed)
	ln -s ../termal.tex $(SUBMISSION_DIR)/termal.tex
	ln -s ../termal.bib $(SUBMISSION_DIR)/termal.bib
	ln -s ../figure-1.pdf $(SUBMISSION_DIR)/figure-1.pdf
	ln -s ../oup-authoring-template.cls $(SUBMISSION_DIR)/
	# Create zip archive, following symlinks
	zip -r $(ZIP_TARGET) $(SUBMISSION_DIR)

clean:
	$(RM) *.aux *.log *.bbl *.blg *.out *.toc *.lof *.lot latex-src.zip

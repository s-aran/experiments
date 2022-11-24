# python_docx

This directory is an experiment for python-docx.

## install

1. use Python 3.10

1. ```sh
   pip install -r requirements.txt
   ```

### notes

python-docs installation failed on Windows with Python 3.11.
It compiles libxml2 during the installation, but is occurring include errors.

This problem can be worked around by using Python 3.10.
Probably because the wheel file is provided.

## run

1. ```sh
   python docx_exp.py
   ```

1. Terminate the Python script and output `test_doc_edit.docx`.

## memo

- The dashed line of the table border style follow the spacing of the dashed lines in terms of tickness.
- The default tickness in Word is `sz: 4`

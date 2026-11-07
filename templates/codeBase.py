# Code Base
# Author: Mauricio Morales

from datetime import date
from pathlib import Path

from IPython.display import HTML


def tiff_pdf_plot(plot, height, width, directory, file_name):
    """Generate TIFF and PDF plots.

    Args:
        plot: matplotlib Figure to save.
        height: in inches.
        width: in inches.
        directory: path to the output directory.
        file_name: without extension.

    Returns:
        None. TIFF and PDF files are written to disk.
    """
    plot.set_size_inches(width, height)

    plot.savefig(Path(directory) / f"{file_name}.tiff", dpi=300)
    plot.savefig(Path(directory) / f"{file_name}.pdf")


def print_table(table, height="500px", width="100%", digits=5):
    """Print a pandas DataFrame as a scrollable HTML table.

    Args:
        table: pandas to print.
        height: maximum height of the scroll box.
        width: of the scroll box.
        digits: number of decimals for numeric columns.

    Returns:
        IPython HTML object.
    """
    html_table = table.round(digits).to_html(
        classes="table table-striped table-hover table-sm",
        border=0,
    )

    return HTML(
        f'<div style="overflow: auto; max-height: {height}; width: {width};">'
        f"{html_table}</div>"
    )


def date_file_name(file_name, date_value=None):
    """Add the date as a prefix to a file name.

    Args:
        file_name: with extension if needed.
        date_value: defaults to today.

    Returns:
        String with the format `%Y_%b_%d_file_name`.
    """
    if date_value is None:
        date_value = date.today()

    return f"{date_value.strftime('%Y_%b_%d')}_{file_name}"
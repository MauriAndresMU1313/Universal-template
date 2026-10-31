# Code Base

.tiff_pdf_plot <- function(
    PLOT,
    HEIGHT,
    WIDTH,
    DIRECTORY,
    FILENAME) {
  #' Generate TIFF and PDF plots
  #'
  #' @param PLOT ggplot2 object to save
  #' @param HEIGHT Plot height in inches
  #' @param WIDTH Plot width in inches
  #' @param DIRECTORY Path to the output directory
  #' @return TIFF and PDF files written to disk
  
  ggplot2::ggsave(
    filename = file.path(DIRECTORY, paste0(FILENAME, ".tiff")),
    plot = PLOT,
    height = HEIGHT,
    width = WIDTH,
    dpi = 300)
  
  ggplot2::ggsave(
    filename = file.path(DIRECTORY, paste0(FILENAME, ".pdf")),
    plot = PLOT,
    height = HEIGHT,
    width = WIDTH)
}

.print_table <- function(
    TABLE,
    KABLE_HEIGHT = "500px",
    KABLE_WIDTH = "100%",
    BOOTSTRAP_OPTION = "responsive") {
  #' Print `Kable` table with predefined structure
  #'
  #' @param TABLE representing the data to print
  #' @param KABLE_HEIGHT predefined
  #' @param KABLE_WIDTH predefined
  #' @param BOOTSTRAP_OPTION styling
  #' @return table
  
  knitr::kable(
    TABLE,
    digits = 5) %>%
    kableExtra::kable_styling(
      bootstrap_options = BOOTSTRAP_OPTION) %>%
    kableExtra::scroll_box(
      height = KABLE_HEIGHT,
      width = KABLE_WIDTH)
}

.date_file_name <- function(FILE_NAME, DATE = Sys.Date()) {
  #' Add the date as a prefix to a file name
  #'
  #' @param FILE_NAME Name of the file (with extension if needed)
  #' @param DATE Date to use, defaults to today
  #' @return String with the format `%Y_%b_%d_file_name`
  
  paste0(format(DATE, "%Y_%b_%d"), "_", FILE_NAME)
}
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
  src.lines()
     .map(|line| {
      // For each line in the source
      line.map(|line| {
        // If the line was read successfully, process it, if not, return the error
        line.split(',') // Split the line separated by commas
            .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
            .collect() // Collect all strings in a row into a Vec<String>
      })
     })
     .collect() // Collect all lines into a Vec<Vec<String>>
}

fn parse_csv_document1(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
  src.lines()
     .map(|line| {
      // For each line in the source
      line.map(|line| {
        // If the line was read successfully, process it, if not, return the error
        line.split(',') // Split the line separated by commas
            .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
            .collect() // Collect all strings in a row into a Vec<String>
      })
     })
     .collect() // Collect all lines into a Vec<Vec<String>>
}
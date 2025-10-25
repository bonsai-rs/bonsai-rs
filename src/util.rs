pub fn get_longest_line_len(lines: &Vec<&str>) -> usize {
    let mut longest_line = 0;
    for line in lines {
        if line.len() > longest_line {
            longest_line = line.len()
        }
    }
    longest_line
}

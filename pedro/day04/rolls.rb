# frozen_string_literal: true

# I'm sure this can be smarter, but it worked on the first try and I didn't have time to optimise it <3

all_lines = File.readlines("input.txt", chomp: true)
count = 0
while true
  deleted = false
  all_lines.map.each_with_index do |line, row|
    row_before = row.zero? ? nil : row - 1
    row_after = all_lines.size-1==row ? nil : row + 1
    line.chars.each_with_index do |c, col|
      next unless c == "@"

      neighbours = [
        col.zero? ? nil : line[col.pred],
        line.size.pred==col ? nil : line[col.next],
        row_before.nil? ? nil : all_lines[row_before][col],
        row_before.nil? || col.zero? ? nil : all_lines[row_before][col.pred],
        row_before.nil? || line.size.pred==col ? nil : all_lines[row_before][col.next],
        row_after.nil? ? nil : all_lines[row_after][col],
        row_after.nil? || col.zero? ? nil : all_lines[row_after][col.pred],
        row_after.nil? || line.size.pred==col ? nil : all_lines[row_after][col.next]
      ].compact
      next unless neighbours.tally(Hash.new(0))["@"] < 4

      count += 1
      all_lines[row][col] = "x"
      deleted = true
    end
  end
  break unless deleted
end

puts count

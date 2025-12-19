#frozen_string_literal: true

all_chars = File.readlines("input.txt", chomp: true).map do |line|
  line.chars
end

partials = []
total = 0
all_chars.transpose.reverse.map do |column|
  col_chars = column.reject { |char| char == " " }
  if col_chars.last == "*" || col_chars.last == "+"
    op = col_chars.pop.to_sym
    partials << col_chars.join.to_i
    total += partials.reduce(&op)
    next
  end
  partials << col_chars.join.to_i
  partials = [] if col_chars.size.zero?
end

p total

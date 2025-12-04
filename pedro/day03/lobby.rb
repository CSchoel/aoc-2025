# frozen_string_literal: true

num_batteries = 12

joltages = File.readlines("input.txt", chomp: true).map do |line|
  digits = line.chars.map(&:to_i)
  num_batteries.downto(1).map do |i|
    max_num, max_index = digits[0..-i].each_with_index.max_by { |num, _index| num }
    digits = digits[(max_index + 1)..-1]
    max_num
  end.join.to_i
end

p "Total: #{joltages.sum}"

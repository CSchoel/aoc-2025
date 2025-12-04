# frozen_string_literal: true

TOTAL_BATTERIES = 12

total_joltage = File.readlines("input.txt", chomp: true).map do |line|
  joltages = line.chars.map(&:to_i)
  TOTAL_BATTERIES.downto(1).map do |remaining_batteries|
    max_joltage, max_index = joltages[0..-remaining_batteries].each_with_index.max_by { |num, _index| num }
    joltages.slice!(0, max_index.next)
    max_joltage
  end.join.to_i
end.sum

p "Total: #{total_joltage}"

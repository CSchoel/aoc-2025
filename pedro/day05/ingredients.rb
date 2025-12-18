# frozen_string_literal: true

fresh = []
available_and_fresh = []

blank_appeared = false
File.readlines("input.txt", chomp: true).each do |line|
  if line == ""
    blank_appeared = true
    next
  end
  if blank_appeared == false
    fresh << Range.new(*line.split("-").map(&:to_i))
    next
  end
  available_and_fresh << line.to_i if fresh.any? { |range| range.include?(line.to_i) }
end

master_ranges = []
fresh.sort_by!(&:begin)
fresh.each do |range|
  if master_ranges.empty?
    master_ranges << range
    next
  end

  last_range = master_ranges.last
  if range.begin <= last_range.end.next
    last_range_end = [last_range.end, range.end].max
    master_ranges[-1] = Range.new(last_range.begin, last_range_end)
  else
    master_ranges << range
  end
end

total_fresh = master_ranges.map(&:size).sum

puts "Part 1: #{available_and_fresh.size}"
puts "Part 2: #{total_fresh}"

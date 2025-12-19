# frozen_string_literal: true

num_splits = 0
beam_locations = Set[]
File.readlines("input.txt", chomp: true).each_with_index do |line, i|
  fields = line.chars
  if i.zero?
    beam_locations << fields.index("S")
    next
  end
  new_locations = Set[]
  beam_locations.each do |loc|
    next unless fields[loc] == "^"

    beam_locations.delete(loc)
    new_locations << loc.pred unless loc.zero?
    new_locations << loc.next unless loc.pred == fields.size
    num_splits += 1 unless new_locations.empty?
  end
  beam_locations.merge(new_locations)
end

p num_splits

# frozen_string_literal: true

beams = Hash.new(0)
File.readlines("input.txt", chomp: true).each_with_index do |line, i|
  if i.zero?
    beams[line.index("S")] += 1
    next
  end
  new_beams = Hash.new(0)
  beams.each do |beam, count|
    if line[beam] == "."
      new_beams[beam] += count
      next
    end
    new_beams[beam.pred] += count if beam.positive? && line[beam.pred] == "."
    new_beams[beam.next] += count if beam < line.size.pred && line[beam.next] == "."
  end
  beams = new_beams
end

p beams.values.sum


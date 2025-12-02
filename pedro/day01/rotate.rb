# frozen_string_literal: true

current_pos = 50
touched_zero = 0

File.readlines("input.txt", chomp: true).each do |line|
  rotate, distance = (line[0] == "R" ? :+ : :-), line[1..].to_i
  initial_pos = current_pos
  overflow_pos = current_pos.send(rotate, distance)
  current_pos = overflow_pos % 100
  if overflow_pos >= 0 && overflow_pos <= 100
    touched_zero += 1 if current_pos.zero?
  else
    touched_zero += (overflow_pos.abs / 100)
    if overflow_pos.negative?
      touched_zero += 1 unless initial_pos.zero?
    end
  end
end

puts "Total: #{touched_zero}"

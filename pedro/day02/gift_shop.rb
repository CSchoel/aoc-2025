# frozen_string_literal: true

require 'prime'

def simple_invalid?(id)
  return id.digits.length.even? && id.to_s[0...id.digits.length / 2] * 2 == id.to_s
end

def complex_invalid?(id)
  return false if id.digits.length < 2

  return id.digits[0] == id.digits[1] if id.digits.length == 2

  return true if id.digits.tally.length == 1  # all digits the same

  id.digits.length.prime_division.each do |fac, exp|
    (1..exp).each do |i|
      seg_len = fac ** i
      next if seg_len == id.digits.length

      repeated_seg = id.to_s[0...seg_len]
      return true if repeated_seg * (id.digits.length / seg_len) == id.to_s
    end
  end
  false
end

invalid_ids = File.read("input.txt", chomp: true).split(",").map do |id|
  pair = id.split("-").map(&:strip)
  next if pair.any? { |i| i.start_with?("0") }  # not an ID at all

  (pair[0].to_i..pair[1].to_i).to_a.select { |i| complex_invalid?(i) }
end.compact.flatten

puts "Sum of invalid IDs: #{invalid_ids.sum(&:to_i)}"

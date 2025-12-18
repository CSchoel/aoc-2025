#frozen_string_literal: true

columns = []
ops = []
File.readlines("test.txt", chomp: true).each do |line|
  nums = line.split(" ")
  if columns.empty?
    nums.each_with_index { |_, i| columns[i] = [] }
  end
  if nums.any? {|n| n == "*" || n == "+"}
    ops = nums.map(&:to_sym)
    break
  end
  nums.each_with_index { |num, i| columns[i] << num.to_i }
end

def reorg_numbers(numbers)
  nums = numbers.map { |n| n.to_s }
  max_length = nums.map(&:length).max
  padded_nums = nums.map { |n| n.rjust(max_length, " ") }
  # lmao wait, how the fuck are these aligned?
  # do i need to reach each character, find the fully empty column, *and then* pad?
  # and then die i suppose??
  # i dont have time for this
  # padded_nums.map(&:chars).transpose.map { |char_array| char_array.reject { |c| c == " " }.join.to_i }
end

# p columns.map { |nums| reorg_numbers(nums) }

p columns.zip(ops).map { |nums, op| nums.reduce(op) }.sum

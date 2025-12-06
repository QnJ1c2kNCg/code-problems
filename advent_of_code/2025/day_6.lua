local utils = require("utils")

local function reduce_numbers(array, operator)
	local result = 0
	if operator == "*" then
		result = 1
	end
	for i = 1, #array do
		if operator == "*" then
			result = result * array[i]
		else
			result = result + array[i]
		end
	end
	return result
end

local function part_1(content)
	local numbers_table = {}
	local operators = {}
	for line in content:gmatch("[^\n]+") do
		local index = 1
		for number in line:gmatch("(%d+)") do
			local numbers = {}
			if numbers_table[index] then
				numbers = numbers_table[index]
			else
				numbers_table[index] = numbers
			end
			table.insert(numbers, tonumber(number))
			index = index + 1
		end

		for operator in line:gmatch("[%+%*]") do
			table.insert(operators, operator)
		end
	end

	assert(#numbers_table == #operators)

	local grand_total = 0
	for i = 1, #numbers_table do
		grand_total = grand_total + reduce_numbers(numbers_table[i], operators[i])
	end
	return grand_total
end

local content = utils.read_file("day_6.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)

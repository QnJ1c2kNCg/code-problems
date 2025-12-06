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
			if not numbers_table[index] then
				numbers_table[index] = {}
			end
			table.insert(numbers_table[index], tonumber(number))
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

local function part_2(content)
	local n_cols = content:find("\n") - 1
	local _, n_rows = content:gsub("\n", "\n")

	local grouped_cols = {}
	local current_grouped_cols = {}
	for col = 1, n_cols do
		local row_value = {}
		for row = 1, n_rows - 1 do
			local coords = col + (row - 1) * (n_cols + 1)
			table.insert(row_value, content:sub(coords, coords))
		end

		local number = tonumber(table.concat(row_value))
		if number then
			table.insert(current_grouped_cols, number)
		else
			-- This is the case where it's all whitespace
			table.insert(grouped_cols, current_grouped_cols)
			current_grouped_cols = {}
		end
	end
	table.insert(grouped_cols, current_grouped_cols)

	local operators = {}
	for operator in content:gmatch("[%+%*]") do
		table.insert(operators, operator)
	end

	assert(#grouped_cols == #operators)

	local grand_total = 0
	for i = 1, #grouped_cols do
		grand_total = grand_total + reduce_numbers(grouped_cols[i], operators[i])
	end
	return grand_total
end

local content = utils.read_file("day_6.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)
utils.time(function()
	print("Part 2: ", part_2(content))
end)

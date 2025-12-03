local function find_max_char(str)
	local max = "0"
	local max_index = 1
	for i = 1, #str do
		local char = str:sub(i, i)
		if char > max then
			max = char
			max_index = i
		end
	end
	return max, max_index
end

local function part_1(content)
	local joltage = 0
	for line in content:gmatch("[^%s]+") do
		-- ignore the last character
		local first_digit = find_max_char(line:sub(1, -2))
		local second_digit = find_max_char(line:sub(line:find(first_digit, 1, true) + 1))
		joltage = joltage + first_digit * 10
		joltage = joltage + second_digit
	end
	return joltage
end

local function part_2(content)
	local joltage = 0
	for line in content:gmatch("[^%s]+") do
		local digits = {}
		local start_index = 1

		for i = -12, -1, 1 do
			local digit, digit_index = find_max_char(line:sub(start_index, i))
			table.insert(digits, digit)
			start_index = start_index + digit_index
		end
		joltage = joltage + table.concat(digits)
	end
	return joltage
end

local input_file = io.open("day_3.input", "r")
local content = input_file:read("*all")
input_file:close()

print("Part 1: ", part_1(content))
print("Part 2: ", part_2(content))

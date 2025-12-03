local function find_max_char(str)
	local max = "0"
	for char in str:gmatch(".") do
		if char > max then
			max = char
		end
	end
	return max
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

local input_file = io.open("day_3.input", "r")
local content = input_file:read("*all")
input_file:close()

print("Part 1: ", part_1(content))

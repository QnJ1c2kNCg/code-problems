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

local function part_1_2(content, length)
	local joltage = 0
	for line in content:gmatch("[^%s]+") do
		local digits = {}
		local start_index = 1

		for i = -length, -1, 1 do
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

print("Part 1: ", part_1_2(content, 2))
print("Part 2: ", part_1_2(content, 12))

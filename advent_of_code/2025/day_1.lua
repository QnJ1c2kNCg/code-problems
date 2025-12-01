local function part_1(content)
	local current_dial = 50
	local number_of_zeros = 0

	for line in content:gmatch("[^\n]+") do
		local direction, amount = line:match("(%a)(%d+)")
		if direction == "R" then
			current_dial = current_dial + amount
		else
			current_dial = current_dial - amount
		end

		if current_dial % 100 == 0 then
			number_of_zeros = number_of_zeros + 1
		end
	end
	return number_of_zeros
end

local function part_2(content)
	local current_dial = 50
	local passing_zero = 0

	for line in content:gmatch("[^\n]+") do
		local direction, amount = line:match("(%a)(%d+)")

		if direction == "R" then
			current_dial = current_dial + amount
			if current_dial > 99 then
				passing_zero = passing_zero + (current_dial // 100)
			end
		else
			current_dial = current_dial - amount
			if current_dial == 0 then
				passing_zero = passing_zero + 1
			end
			if current_dial < 0 and current_dial + amount ~= 0 then
				passing_zero = passing_zero + (math.abs(current_dial) // 100) + 1
			end
			if current_dial < 0 and current_dial + amount == 0 then
				passing_zero = passing_zero + (math.abs(current_dial) // 100)
			end
		end
		current_dial = current_dial % 100
	end
	return passing_zero
end

local input_file = io.open("day_1.input", "r")
local content = input_file:read("*all")
input_file:close()

print("Part 1: ", part_1(content))
print("Part 2: ", part_2(content))

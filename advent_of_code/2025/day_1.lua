local input_file = io.open("day_1.input", "r")

local current_dial = 50
local number_of_zeros = 0

for line in input_file:lines() do
	local direction = string.sub(line, 1, 1)
	local amount = tonumber(string.sub(line, 2))
	if direction == "R" then
		current_dial = current_dial + amount
	else
		current_dial = current_dial - amount
	end

	if current_dial % 100 == 0 then
		number_of_zeros = number_of_zeros + 1
	end
end

print("Part 1: ", number_of_zeros)

input_file:close()

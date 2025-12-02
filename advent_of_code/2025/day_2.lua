local function part_1(content)
	local sum = 0
	for start_index, end_index in content:gmatch("(%d+)-(%d+)") do
		for i = tonumber(start_index), end_index do
			local i_str = tostring(i)
			if i_str:len() % 2 == 0 then
				local middle = i_str:len() / 2
				if i_str:sub(1, middle) == i_str:sub(middle + 1) then
					sum = sum + tonumber(i_str)
				end
			end
		end
	end
	return sum
end

local input_file = io.open("day_2.input", "r")
local content = input_file:read("*all")
input_file:close()

print("Part 1: ", part_1(content))

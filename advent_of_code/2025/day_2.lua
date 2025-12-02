local function part_1_2(content)
	local part_1, part_2 = 0, 0
	for start_index, end_index in content:gmatch("(%d+)-(%d+)") do
		for i = tonumber(start_index), end_index do
			local i_str = tostring(i)
			local middle = i_str:len() // 2
			if i_str:sub(1, middle) == i_str:sub(middle + 1) then
				part_1 = part_1 + i
			end
			for c = 1, middle do
				local sub = i_str:sub(1, c)
				if i_str:gsub(sub, ""):len() == 0 then
					part_2 = part_2 + i
					break
				end
			end
		end
	end
	return part_1, part_2
end

local input_file = io.open("day_2.input", "r")
local content = input_file:read("*all")
input_file:close()

local part_1, part_2 = part_1_2(content)
print("Part 1: ", part_1)
print("Part 2: ", part_2)

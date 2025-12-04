local function neighoring_rolls(table, row, col)
	local rolls_of_paper = 0
	for row_offset = -1, 1 do
		for col_offset = -1, 1 do
			if
				not (row_offset == 0 and col_offset == 0)
				and table[row + row_offset]
				and table[row + row_offset][col + col_offset]
				and table[row + row_offset][col + col_offset] == "@"
			then
				rolls_of_paper = rolls_of_paper + 1
			end
		end
	end
	return rolls_of_paper
end

local function part_1(content, length)
	local accessible_rolls = 0
	local map = {}
	-- TODO: Two passes is not ideal :)
	for line in content:gmatch("[^%s]+") do
		local row = {}
		for c in line:gmatch(".") do
			table.insert(row, c)
		end
		table.insert(map, row)
	end
	for row = 1, #map do
		for col = 1, #map[row] do
			if neighoring_rolls(map, row, col) < 4 and map[row][col] == "@" then
				accessible_rolls = accessible_rolls + 1
			end
		end
	end
	return accessible_rolls
end

local input_file = io.open("day_4.input", "r")
local content = input_file:read("*all")
input_file:close()

print("Part 1: ", part_1(content))

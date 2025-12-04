local function neighboring_rolls(grid, row, col)
	local rolls_of_paper = 0
	for row_offset = -1, 1 do
		for col_offset = -1, 1 do
			if
				not (row_offset == 0 and col_offset == 0)
				and grid[row + row_offset]
				and grid[row + row_offset][col + col_offset]
				and grid[row + row_offset][col + col_offset] == "@"
			then
				rolls_of_paper = rolls_of_paper + 1
			end
		end
	end
	return rolls_of_paper
end

local function parse_grid(content)
	local grid = {}
	for line in content:gmatch("[^%s]+") do
		local row = {}
		for c in line:gmatch(".") do
			table.insert(row, c)
		end
		table.insert(grid, row)
	end
	return grid
end

local function part_1_2(grid, remove_accessible)
	local accessible_rolls = 0
	local rolls_have_been_removed = true
	while rolls_have_been_removed do
		rolls_have_been_removed = false
		for row = 1, #grid do
			for col = 1, #grid[row] do
				if neighboring_rolls(grid, row, col) < 4 and grid[row][col] == "@" then
					accessible_rolls = accessible_rolls + 1
					if remove_accessible then
						grid[row][col] = "."
						rolls_have_been_removed = true
					end
				end
			end
		end
	end
	return accessible_rolls
end

local input_file = io.open("day_4.input", "r")
local content = input_file:read("*all")
input_file:close()

local grid = parse_grid(content)

print("Part 1: ", part_1_2(grid, false))
print("Part 2: ", part_1_2(grid, true))

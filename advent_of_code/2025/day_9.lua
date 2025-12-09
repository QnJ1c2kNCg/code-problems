local utils = require("utils")

local function part_1(content)
	local all_points = {}

	for line in content:gmatch("[^%s]+") do
		local x, y = line:match("(%d+)%,(%d+)")
		table.insert(all_points, { x = tonumber(x), y = tonumber(y) })
	end

	local max_area = 0
	for i = 1, #all_points - 1 do
		for j = i + 1, #all_points do
			local area = (math.abs(all_points[i].x - all_points[j].x) + 1)
				* (math.abs(all_points[i].y - all_points[j].y) + 1)
			max_area = math.max(max_area, area)
		end
	end
	return max_area
end

local content = utils.read_file("day_9.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)

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

local function check_in_ranges(ranges, y1, y2)
	local min = math.huge
	local max = -math.huge
	for _, range in ipairs(ranges) do
		min = math.min(min, range[1], range[2])
		max = math.max(max, range[1], range[2])
	end
	return min <= y1 and y1 <= max and min <= y2 and y2 <= max
end

local function part_2(content)
	local all_points = {}
	local map = {}

	local last_x, last_y = content:match("(%d+)%,(%d+)\n$")
	local previous_point = { x = tonumber(last_x), y = tonumber(last_y) }
	for line in content:gmatch("[^%s]+") do
		local x, y = line:match("(%d+)%,(%d+)")
		x, y = tonumber(x), tonumber(y)
		table.insert(all_points, { x = x, y = y })

		local min_x = math.min(previous_point.x, x)
		local max_x = math.max(previous_point.x, x)
		for i = min_x, max_x do
			map[i] = map[i] or {}
			table.insert(map[i], {
				math.min(previous_point.y, y),
				math.max(previous_point.y, y),
			})
		end
		previous_point = { x = x, y = y }
	end

	local max_area = 0
	for i = 1, #all_points - 1 do
		for j = i + 1, #all_points do
			local rect_starting_x = math.min(all_points[i].x, all_points[j].x)
			local rect_ending_x = math.max(all_points[i].x, all_points[j].x)
			local rect_starting_y = math.min(all_points[i].y, all_points[j].y)
			local rect_ending_y = math.max(all_points[i].y, all_points[j].y)

			-- Verify that there are no points within the rectangle
			for x = rect_starting_x + 1, rect_ending_x - 1 do
				for _, range in ipairs(map[x]) do
					if rect_starting_y < range[1] and range[1] < rect_ending_y then
						goto continue
					end
					if rect_starting_y < range[2] and range[2] < rect_ending_y then
						goto continue
					end
				end
			end
			-- Verify that all four corners are in
			if
				not (
					check_in_ranges(map[rect_starting_x], rect_starting_y, rect_ending_y)
					and check_in_ranges(map[rect_ending_x], rect_starting_y, rect_ending_y)
				)
			then
				goto continue
			end

			-- Compute the area
			local area = (math.abs(all_points[i].x - all_points[j].x) + 1)
				* (math.abs(all_points[i].y - all_points[j].y) + 1)
			max_area = math.max(max_area, area)

			::continue::
		end
	end
	return max_area
end

local content = utils.read_file("day_9.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)
utils.time(function()
	print("Part 2: ", part_2(content))
end)

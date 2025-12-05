local utils = require("utils")

local function parse_input(content)
	local ranges = {}
	local ids = {}
	for line in content:gmatch("[^%s]+") do
		local range_left, range_right = line:match("(%d+)-(%d+)")
		if range_left then
			-- we are a range
			table.insert(ranges, { tonumber(range_left), tonumber(range_right) })
		else
			-- we are an id
			table.insert(ids, tonumber(line))
		end
	end

	return ranges, ids
end

local function merge_ranges(ranges)
	local merged_ranges = {}
	table.sort(ranges, function(lhs, rhs)
		return lhs[1] < rhs[1]
	end)

	local current_range = ranges[1]
	for i = 2, #ranges do
		if ranges[i][1] <= current_range[2] then
			current_range[2] = math.max(current_range[2], ranges[i][2])
		else
			table.insert(merged_ranges, current_range)
			current_range = ranges[i]
		end
	end
	table.insert(merged_ranges, current_range)

	return merged_ranges
end

local function part_1(ranges, ids)
	local fresh_ingredients = 0
	for _, id in ipairs(ids) do
		for _, range in ipairs(ranges) do
			if range[1] <= id and id <= range[2] then
				fresh_ingredients = fresh_ingredients + 1
				break
			end
		end
	end
	return fresh_ingredients
end

local function part_2(ranges)
	local fresh_ids = 0
	for _, range in ipairs(ranges) do
		fresh_ids = fresh_ids + (range[2] + 1 - range[1])
	end
	return fresh_ids
end

local content = utils.read_file("day_5.input")
local ranges, ids = parse_input(content)
local merged_ranges = merge_ranges(ranges)

utils.time(function()
	print("Part 1: ", part_1(merged_ranges, ids))
end)
utils.time(function()
	print("Part 2: ", part_2(merged_ranges))
end)

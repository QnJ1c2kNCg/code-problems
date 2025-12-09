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

local function part_2(content)
	local all_points = {}
	local map = {}

	-- local previous_point = { x =97914, y= 50388}
	local previous_point = {}
	for line in content:gmatch("[^%s]+") do
		local x, y = line:match("(%d+)%,(%d+)")
		x = tonumber(x)
		y = tonumber(y)
		table.insert(all_points, { x = x, y = y })

		if previous_point.x then
			if previous_point.x > x then
				for i = x, previous_point.x do
					local min_val = map[i] and map[i][1] or previous_point.y
					local max_val = map[i] and map[i][2] or previous_point.y
					map[i] = {
						math.min(min_val, previous_point.y, y),
						math.max(max_val, previous_point.y, y),
					}
				end
			else
				for i = previous_point.x, x do
					local min_val = map[i] and map[i][1] or previous_point.y
					local max_val = map[i] and map[i][2] or previous_point.y
					map[i] = {
						math.min(min_val, previous_point.y, y),
						math.max(max_val, previous_point.y, y),
					}
				end
			end
		end
		previous_point = { x = x, y = y }
	end
	-- TODO: the wrap around
	--
	for k, v in pairs(map) do
		print(k, "[" .. v[1] .. ", " .. v[2] .. "]")
	end

	local max_area = 0
	for i = 1, #all_points - 1 do
		for j = i + 1, #all_points do
			-- Check if the entire rectange is in the map
			if map[all_points[i].x][1] > all_points[j].y or map[all_points[i].x][2] < all_points[j].y then
				goto continue
			end
			if map[all_points[j].x][1] > all_points[i].y or map[all_points[j].x][2] < all_points[i].y then
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

	-- Thinking
	-- we could hashmap, key by x, and then value is a range of y?
	-- Then we just loop over all the rectange, pick the reverse corners anc heck if they belong?
end

local content = utils.read_file("day_9.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)
utils.time(function()
	print("Part 2: ", part_2(content))
end)

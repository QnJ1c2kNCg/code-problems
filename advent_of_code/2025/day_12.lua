local utils = require("utils")

local function parse_input(content)
	local shapes = {}
	for _, line1, line2, line3 in content:gmatch("(%d+):\n(.-)\n(.-)\n(.-)%s*\n") do
		table.insert(shapes, line1 .. line2 .. line3)
	end
	local requirements = {}
	for width, length, rest in content:gmatch("(%d+)x(%d+): ([^\n]+)") do
		local quantities = {}
		for number in rest:gmatch("%d+") do
			table.insert(quantities, tonumber(number))
		end
		table.insert(requirements, { tonumber(width), tonumber(length), quantities })
	end

	return shapes, requirements
end

local function solve_requirement(shapes, requirement)
	-- Sadly, this problem can be solved by a very basic heuristic.
	-- We simply have to count the number of tiles the present occupy
	-- and the number total number of available tiles. There is no need
	-- to compute how the gifts would be stacked.

	-- 1. Convert shapes to tiles
	local shapes_tiles = {}
	for _, shape in ipairs(shapes) do
		local _, tile_count = shape:gsub("#", "#")
		table.insert(shapes_tiles, tile_count)
	end

	-- 2. Multiply by their quantities
	local total_tiles = 0
	for i, quantity in ipairs(requirement[3]) do
		total_tiles = total_tiles + (shapes_tiles[i] * quantity)
	end

	local area_tiles = requirement[1] * requirement[2]
	if total_tiles <= area_tiles then
		return true
	else
		return false
	end
end

local function part_1(content)
	local shapes, requirements = parse_input(content)
	local regions_fit = 0
	for _, requirement in ipairs(requirements) do
		if solve_requirement(shapes, requirement) then
			regions_fit = regions_fit + 1
		end
	end
	return regions_fit
end

local content = utils.read_file("day_12.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)

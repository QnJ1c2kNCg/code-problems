local utils = require("utils")

local function part_1(content)
	local n_splits = 0
	local beam_locations = {}
	beam_locations[content:find("S")] = content:find("S")

	for line in content:gmatch("[^%s]+") do
		local new_beam_locations = {}
		for _, beam_location in pairs(beam_locations) do
			if line:sub(beam_location, beam_location) == "^" then
				-- It's impossible to out of bound with the input
				new_beam_locations[beam_location - 1] = beam_location - 1
				new_beam_locations[beam_location + 1] = beam_location + 1
				beam_locations[beam_location] = nil
				n_splits = n_splits + 1
			end
		end
		for k, v in pairs(new_beam_locations) do
			beam_locations[k] = v
		end
	end
	return n_splits
end

local function part_2(content)
	local function recurse_inner(beam, lines, line_index, dp)
		local key = tostring(beam) .. "|" .. tostring(line_index)
		if dp[key] then
			return dp[key]
		end
		-- Reached the end
		if not lines[line_index] then
			return 1
		end

		local out = nil
		if lines[line_index]:sub(beam, beam) == "^" then
			out = recurse_inner(beam - 1, lines, line_index + 1, dp)
				+ recurse_inner(beam + 1, lines, line_index + 1, dp)
		else
			out = recurse_inner(beam, lines, line_index + 1, dp)
		end

		dp[key] = out
		return out
	end

	local dp = {}
	local lines = {}
	for line in content:gmatch("[^%s]+") do
		table.insert(lines, line)
	end

	return recurse_inner(content:find("S"), lines, 2, dp)
end

local content = utils.read_file("day_7.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)
utils.time(function()
	print("Part 2: ", part_2(content))
end)

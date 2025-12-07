local utils = require("utils")

local function part_1(content)
	local n_splits = 0
	local beam_locations = {}
	beam_locations[content:find("S")] = content:find("S")

	for line in content:gmatch("[^%s]+") do
		local new_beam_locations = {}
		for _, beam_location in pairs(beam_locations) do
			if line:sub(beam_location, beam_location) == "^" then
				if beam_location > 1 then
					new_beam_locations[beam_location - 1] = beam_location - 1
				end
				-- TODO: maybe there should be a -1 here
				if beam_location < #line then
					new_beam_locations[beam_location + 1] = beam_location + 1
				end
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

local content = utils.read_file("day_7.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)

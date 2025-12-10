local utils = require("utils")

local function parse_line(line)
	local target_lights = 0
	local lights = line:match("%[(.*)%]")
	for i = 0, #lights - 1 do
		if lights:sub(i + 1, i + 1) == "#" then
			target_lights = bit.bor(target_lights, bit.lshift(1, i))
		end
	end

	local buttons = {}
	for button_str in line:gmatch("%(([^)]+)%)") do
		local button = {}
		for light_index in button_str:gmatch("%d+") do
			table.insert(button, tonumber(light_index))
		end
		table.insert(buttons, button)
	end
	return target_lights, buttons
end

local function press_button(current_lights, button)
	for _, light_index in ipairs(button) do
		current_lights = bit.bxor(current_lights, bit.lshift(1, light_index))
	end
	return current_lights
end

local function format_lights(lights)
	local out = ""
	for i = 0, 3 do
		if bit.band(bit.rshift(lights, i), 1) > 0 then
			out = "#" .. out
		else
			out = "." .. out
		end
	end
	return out
end

-- TODO: Add mem
local function inner(target_lights, current_lights, buttons, n_presses, mem)
	if mem[current_lights] and n_presses >= mem[current_lights] then
		return math.huge
	end
	mem[current_lights] = n_presses
	--print(
	--"Processing press #" .. n_presses,
	--"target: " .. format_lights(target_lights),
	--"current: " .. format_lights(current_lights)
	--)
	local out = math.huge
	for _, button in ipairs(buttons) do
		local new_lights = press_button(current_lights, button)
		if new_lights == target_lights then
			return n_presses + 1
		else
			out = math.min(out, inner(target_lights, new_lights, buttons, n_presses + 1, mem))
		end
	end
	return out
end

local function min_presses_to_configure(target_lights, buttons)
	return inner(target_lights, 0, buttons, 0, {})
end

local function part_1(content)
	local sum_min_presses = 0
	for line in content:gmatch("[^\n]+") do
		local target_lights, buttons = parse_line(line)
		sum_min_presses = sum_min_presses + min_presses_to_configure(target_lights, buttons)
	end
	return sum_min_presses
end

local content = utils.read_file("day_10.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)

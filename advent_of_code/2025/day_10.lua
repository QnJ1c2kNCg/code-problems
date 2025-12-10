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
	table.sort(buttons, function(a, b)
		return #a > #b
	end)

	-- TODO: Flip the joltage? MSB right?
	local joltages = {}
	local joltage_str = line:match("%{(.*)%}")
	for joltage in joltage_str:gmatch("%d+") do
		table.insert(joltages, tonumber(joltage))
	end

	return target_lights, buttons, joltages
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

local function min_presses_to_configure_part_1(target_lights, current_lights, buttons, n_presses, mem)
	if mem[current_lights] and n_presses >= mem[current_lights] then
		return math.huge
	end
	mem[current_lights] = n_presses
	local out = math.huge
	for _, button in ipairs(buttons) do
		local new_lights = press_button(current_lights, button)
		if new_lights == target_lights then
			return n_presses + 1
		else
			out = math.min(out, min_presses_to_configure_part_1(target_lights, new_lights, buttons, n_presses + 1, mem))
		end
	end
	return out
end

local function part_1(content)
	local sum_min_presses = 0
	for line in content:gmatch("[^\n]+") do
		local target_lights, buttons = parse_line(line)
		sum_min_presses = sum_min_presses + min_presses_to_configure_part_1(target_lights, 0, buttons, 0, {})
	end
	return sum_min_presses
end

local function joltages_is_good(joltages)
	for _, v in ipairs(joltages) do
		if v ~= 0 then
			return false
		end
	end
	return true
end

local function get_available_buttons(all_buttons, buttons_mask)
	local buttons = {}
	for i, button in ipairs(all_buttons) do
		if buttons_mask[i] then
			table.insert(buttons, button)
		end
	end
	return buttons
end

-- Find the joltage that is affected by the fewest buttons
-- 	This returns a 0 based index
local function find_joltage_affected_by_fewest_buttons(current_joltages, buttons)
	local min_n_buttons_affecting_joltage = math.huge

	local min_buttons_affecting_map = {}
	for i = 0, #current_joltages - 1 do
		if current_joltages[i + 1] ~= 0 then
			local n_buttons_affecting_joltage = 0
			for _, button in ipairs(buttons) do
				for _, joltage_index in ipairs(button) do
					if joltage_index == i then
						n_buttons_affecting_joltage = n_buttons_affecting_joltage + 1
						break
					end
				end
			end
			min_buttons_affecting_map[n_buttons_affecting_joltage] = min_buttons_affecting_map[n_buttons_affecting_joltage]
				or {}
			table.insert(min_buttons_affecting_map[n_buttons_affecting_joltage], i)

			if n_buttons_affecting_joltage < min_n_buttons_affecting_joltage then
				min_n_buttons_affecting_joltage = n_buttons_affecting_joltage
			end
		end
	end
	local min_key, min_value = math.huge, {}
	for k, v in pairs(min_buttons_affecting_map) do
		if k < min_key then
			min_key = k
			min_value = v
		end
	end
	table.sort(min_value)
	return min_value[#min_value]
end

local function compute_buttons_subset(buttons, target_joltage_index)
	local buttons_subset = {}
	for _, button in ipairs(buttons) do
		for _, joltage_index in ipairs(button) do
			if joltage_index == target_joltage_index then
				table.insert(buttons_subset, button)
				break
			end
		end
	end
	return buttons_subset
end

local function create_buttons_mask(all_buttons, buttons_subset, buttons_mask)
	local new_mask = {}
	for k, v in pairs(buttons_mask) do
		new_mask[k] = v
	end

	for i, button in ipairs(all_buttons) do
		for _, button_subset in ipairs(buttons_subset) do
			if button == button_subset then
				new_mask[i] = false
			end
		end
	end
	return new_mask
end

local function permutation_iterator(m, n)
	local current = {}
	for i = 1, m do
		current[i] = 0
	end
	current[m] = n

	return function()
		if not current then
			return nil
		end

		-- Return current permutation
		local result = { unpack(current) }

		-- Generate next permutation
		local pos = m
		while pos > 1 do
			if current[pos] > 0 then
				current[pos] = current[pos] - 1
				current[pos - 1] = current[pos - 1] + 1

				-- Move all remaining to the last position
				local sum = 0
				for i = pos, m do
					sum = sum + current[i]
				end
				for i = pos, m do
					current[i] = 0
				end
				current[m] = sum

				return result
			end
			pos = pos - 1
		end

		-- No more permutations
		current = nil
		return result
	end
end

-- Idea from Michel Krämer: Focus on one joltage at a time, in order of fewest button that affects it, to most
local function min_presses_to_configure_part_2(current_joltages, all_buttons, buttons_mask, n_presses, mem)
	if joltages_is_good(current_joltages) then
		return n_presses
	end

	local key = table.concat(current_joltages, ",")
	if mem[key] and n_presses >= mem[key] then
		return math.huge
	end

	-- 0. Compute our available buttons from the mask
	local buttons = get_available_buttons(all_buttons, buttons_mask)

	-- 1. Find the joltage that is affected by the fewest buttons
	local target_joltage_index = find_joltage_affected_by_fewest_buttons(current_joltages, buttons)

	-- 2. Compute button subset for this particular joltage index
	local buttons_subset = compute_buttons_subset(buttons, target_joltage_index)

	-- 3. Iteratively, go through all possibilities, if it works, then recurse
	local min_presses = math.huge
	local amount_of_jolt_to_decrease = current_joltages[target_joltage_index + 1]
	for perm in permutation_iterator(#buttons_subset, amount_of_jolt_to_decrease) do
		-- Press all the button from this permutation
		for i = 1, #perm do
			for _, joltage_index in ipairs(buttons_subset[i]) do
				current_joltages[joltage_index + 1] = current_joltages[joltage_index + 1] - perm[i]
			end
		end
		-- A joltage is negative, this means we can stop the current search
		for _, joltage in ipairs(current_joltages) do
			if joltage < 0 then
				goto continue
			end
		end

		-- We are done, we were able to bring the targeted joltage to 0
		if current_joltages[target_joltage_index + 1] == 0 then
			-- Continue with DFS, with updated jolt and button mask
			-- 4. Update the buttons mask for the next iterations
			local new_buttons_mask = create_buttons_mask(all_buttons, buttons_subset, buttons_mask)
			local new_min = min_presses_to_configure_part_2(
				current_joltages,
				all_buttons,
				new_buttons_mask,
				n_presses + amount_of_jolt_to_decrease,
				mem
			)
			min_presses = math.min(min_presses, new_min)
			goto continue
		end

		-- Un-Press all the button from this permutation
		::continue::
		for i = 1, #perm do
			for _, joltage_index in ipairs(buttons_subset[i]) do
				current_joltages[joltage_index + 1] = current_joltages[joltage_index + 1] + perm[i]
			end
		end
	end

	mem[key] = min_presses
	return min_presses
end

local function part_2(content)
	local sum_min_presses = 0
	local file = io.open(arg[1] .. "output", "w")
	for line in content:gmatch("[^\n]+") do
		local _, buttons, joltages = parse_line(line)
		local initial_buttons_mask = {}
		for i, _ in ipairs(buttons) do
			initial_buttons_mask[i] = true
		end

		local min = min_presses_to_configure_part_2(joltages, buttons, initial_buttons_mask, 0, {})
		sum_min_presses = sum_min_presses + min
		print("Current sum is " .. sum_min_presses .. " with a min of " .. min .. " for line: " .. line)
		file:write("Current sum is " .. sum_min_presses .. " with a min of " .. min .. " for line: " .. line .. "\n")
		file:flush()
	end
	file:close()
	return sum_min_presses
end

local content = utils.read_file(arg[1])
io.output(arg[1] .. "output")
print("reading file: " .. arg[1])

utils.time(function()
	print("Part 1: ", part_1(content))
end)
utils.time(function()
	print("Part 2: ", part_2(content))
end)

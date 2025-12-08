local utils = require("utils")

local Point = {}
Point.__index = Point

function Point:__tostring()
	return string.format("Point(%d, %d, %d)", self.x, self.y, self.z)
end

function Point:__eq(other)
	return self.x == other.x and self.y == other.y and self.z == other.z
end

function Point:new(x, y, z)
	return setmetatable({ x = x, y = y, z = z }, Point)
end

local function distance(p1, p2)
	return math.sqrt(math.pow(p2.x - p1.x, 2) + math.pow(p2.y - p1.y, 2) + math.pow(p2.z - p1.z, 2))
end

local function closest_pair(points, greater_than)
	local min_distance = 10000000000000
	local out = {}
	for i = 1, #points - 1 do
		for j = i + 1, #points do
			local current_distance = distance(points[i], points[j])
			if current_distance < min_distance and current_distance > greater_than then
				min_distance = current_distance
				out = { points[i], points[j] }
			end
		end
	end
	return out, min_distance
end

local function table_contains(t, point)
	for _, v in ipairs(t) do
		if v == point then
			return true
		end
	end
	return false
end

local function connect_circuits(circuits, p1, p2)
	local p1_circuit, p2_circuit, p2_index
	for i, circuit in ipairs(circuits) do
		if table_contains(circuit, p1) then
			p1_circuit = circuit
		end
		if table_contains(circuit, p2) then
			p2_circuit = circuit
			p2_index = i
		end
	end

	if p1_circuit == p2_circuit then
		-- nothing to do
	else
		-- we need to connect the circuit
		for _, v in ipairs(p2_circuit) do
			table.insert(p1_circuit, v)
		end
		table.remove(circuits, p2_index)
	end
end

local function part_1(content)
	local all_points = {}
	local circuits = {}

	for line in content:gmatch("[^%s]+") do
		local x, y, z = line:match("(%d+)%,(%d+)%,(%d+)")
		local point = Point:new(x, y, z)
		table.insert(all_points, point)
		table.insert(circuits, { point })
	end

	local last_min_distance = 0
	for _ = 1, 1000 do
		pair, last_min_distance = closest_pair(all_points, last_min_distance)
		connect_circuits(circuits, pair[1], pair[2])
	end

	table.sort(circuits, function(a, b)
		return #a > #b
	end)
	return #circuits[1] * #circuits[2] * #circuits[3]
end

local function part_2(content)
	local all_points = {}
	local circuits = {}

	for line in content:gmatch("[^%s]+") do
		local x, y, z = line:match("(%d+)%,(%d+)%,(%d+)")
		local point = Point:new(x, y, z)
		table.insert(all_points, point)
		table.insert(circuits, { point })
	end

	local pair
	local last_min_distance = 0
	while #circuits > 1 do
		pair, last_min_distance = closest_pair(all_points, last_min_distance)
		connect_circuits(circuits, pair[1], pair[2])
	end
	print("last pair: ", pair[1], pair[2])

	return pair[1].x * pair[2].x
end

local content = utils.read_file("day_8.input")

--utils.time(function()
--print("Part 1: ", part_1(content))
--end)
utils.time(function()
	print("Part 2: ", part_2(content))
end)

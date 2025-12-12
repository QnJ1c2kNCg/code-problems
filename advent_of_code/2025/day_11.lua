local utils = require("utils")

local function create_graph(content)
	local graph = {}
	for line in content:gmatch("[^\n]+") do
		local source_node = line:match("(.*):")
		graph[source_node] = graph[source_node] or {}
		for target_node in line:match(":%s*(.*)"):gmatch("(%S+)") do
			table.insert(graph[source_node], target_node)
		end
	end
	return graph
end

local function visit1(graph, node, visited_nodes)
	if node == "out" then
		return 1
	end
	-- We have already visited this node
	if visited_nodes[node] then
		return 0
	end

	-- Visit all the neighbords
	local paths = 0
	for _, neigh in ipairs(graph[node]) do
		paths = paths + visit1(graph, neigh, visited_nodes)
	end
	visited_nodes[node] = false
	return paths
end

local function part_1(graph)
	return visit1(graph, "you", {})
end

local function visit2(graph, node, cache, pois)
	-- The cache is just memoization
	-- We can just have the visited nodes cause we might have cycles
	local key = node .. table.concat(pois)
	if cache[key] then
		return cache[key]
	end

	if node == "out" then
		if #pois == 0 then
			return 1
		else
			return 0
		end
	end

	local removed = 0
	for i, v in ipairs(pois) do
		if node == v then
			removed = i
			table.remove(pois, i)
			break
		end
	end

	-- Visit all the neighbords
	local paths = 0
	for _, neigh in ipairs(graph[node]) do
		paths = paths + visit2(graph, neigh, cache, pois)
	end
	cache[key] = paths
	if removed > 0 then
		table.insert(pois, removed, node)
	end
	return paths
end

local function part_2(graph)
	return visit2(graph, "svr", {}, { "dac", "fft" })
end

local content = utils.read_file("day_11.input")

local graph = create_graph(content)
utils.time(function()
	print("Part 1: ", part_1(graph))
end)
utils.time(function()
	print("Part 2: ", part_2(graph))
end)

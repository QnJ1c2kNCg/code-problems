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

local function visit(graph, node, visited_nodes)
	if node == "out" then
		return 1
	end
	-- We have already visited this node
	if visited_nodes[node] then
		return 0
	end

	-- Visit all the neighbords
	local paths = 0
	visited_nodes[node] = true
	for _, neigh in ipairs(graph[node]) do
		paths = paths + visit(graph, neigh, visited_nodes)
	end
	visited_nodes[node] = false
	return paths
end

local function part_1(content)
	local graph = create_graph(content)
	return visit(graph, "you", {})
end

local content = utils.read_file("day_11.input")

utils.time(function()
	print("Part 1: ", part_1(content))
end)

local M = {}

function M.time(fn)
	local start_time = os.clock()
	fn()
	local end_time = os.clock()
	local elapsed_time = end_time - start_time
	print("In: " .. elapsed_time .. " seconds")
end

function M.read_file(path)
	local input_file, err = io.open(path, "r")
	if not input_file then
		return nil, "Error opening file " .. path .. ": " .. err
	end
	local content, err = input_file:read("*all")
	if not content then
		return nil, "Error reading file " .. path .. ": " .. err
	end
	input_file:close()

	return content
end

return M

--task.delay(60, function() print("HI") end)
local a = 0
function incra()
	a = a + 1
end

function geta()
	return a
end

return {
	incra = incra,
	geta = geta,
}
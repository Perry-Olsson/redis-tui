local num_users = ARGV[1]
local num_posts = ARGV[2]

math.randomseed(redis.call("TIME")[1])
for i = 1, tonumber(num_users) do
    local randomNumber = math.random(15, 40)
    redis.call("HSET", "user:"..i, "name", "John Doe | "..i, "email", "john_"..i.."@gmail.com", "age", randomNumber);
end

for i = 1, tonumber(num_posts) do
    local randomNumber = math.random(1, 6)
    redis.call("HSET", "post:"..i, "title", "Post Title | "..i, "content", "content | "..i, "user", "user:"..randomNumber)
end

return "OK"

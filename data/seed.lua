local num_users = ARGV[1]
local num_posts = ARGV[2]

math.randomseed(redis.call("TIME")[1])
redis.call("SET", "user_id", 0)
for i = 1, tonumber(num_users) do
    local randomNumber = math.random(15, 40)
    local id = redis.call("INCR", "user_id")
    redis.call("HSET", "user:"..id, "name", "John Doe | "..id, "email", "john_"..id.."@gmail.com", "age", randomNumber);
end

redis.call("SET", "post_id", 0)
for i = 1, tonumber(num_posts) do
    local randomNumber = math.random(1, 6)
    local id = redis.call("INCR", "post_id")
    redis.call("HSET", "post:"..id, "title", "Post Title | "..id, "content", "content | "..id, "likes", math.random(0, 5000), "user", ""..randomNumber)
end

redis.call("FT.CREATE", "post", "ON", "HASH", "PREFIX", "1", "post:", "SCHEMA", "likes", "NUMERIC", "SORTABLE", "user", "NUMERIC")

return "OK"

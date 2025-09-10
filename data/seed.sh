#!/bin/bash
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
num_users=5
num_posts=20

redis-cli flushall

redis-cli --eval "$script_dir/seed.lua" , $num_users $num_posts

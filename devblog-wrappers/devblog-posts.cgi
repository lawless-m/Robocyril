#!/bin/bash
# Wrapper for /devblog/api/posts - routes to correct binary based on method

export BLOG_DB_PATH="/var/lib/devblog/blog.db"
export BLOG_API_KEY_PATH="/etc/devblog-api-key"

case "$REQUEST_METHOD" in
    POST)
        exec /usr/lib/cgi-bin/blog-post
        ;;
    GET)
        exec /usr/lib/cgi-bin/blog-list
        ;;
    *)
        echo "Status: 405 Method Not Allowed"
        echo "Content-Type: application/json"
        echo ""
        echo '{"error":"Method not allowed"}'
        ;;
esac

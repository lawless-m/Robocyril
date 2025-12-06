#!/bin/bash
# Wrapper for /devblog/api/post - routes to correct binary based on method

export BLOG_DB_PATH="/var/lib/devblog/blog.db"
export BLOG_API_KEY_PATH="/etc/devblog-api-key"

case "$REQUEST_METHOD" in
    GET)
        exec /usr/lib/cgi-bin/blog-get
        ;;
    PATCH)
        exec /usr/lib/cgi-bin/blog-update
        ;;
    DELETE)
        exec /usr/lib/cgi-bin/blog-delete
        ;;
    *)
        echo "Status: 405 Method Not Allowed"
        echo "Content-Type: application/json"
        echo ""
        echo '{"error":"Method not allowed"}'
        ;;
esac

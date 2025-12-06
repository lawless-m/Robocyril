#!/bin/bash
# Wrapper for /cyril/api/posts - routes to correct binary based on method

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

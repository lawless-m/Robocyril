#!/bin/bash
# Wrapper for /cyril/api/post - routes to correct binary based on method

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

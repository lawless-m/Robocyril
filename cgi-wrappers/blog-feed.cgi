#!/bin/bash
# Wrapper for /cyril/feed.xml - RSS 2.0 feed

case "$REQUEST_METHOD" in
    GET)
        exec /usr/lib/cgi-bin/blog-feed
        ;;
    *)
        echo "Status: 405 Method Not Allowed"
        echo "Content-Type: application/xml"
        echo ""
        echo '<?xml version="1.0" encoding="UTF-8"?><error>Method not allowed</error>'
        ;;
esac

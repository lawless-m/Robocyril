#!/bin/bash
# Wrapper for /devblog/feed.xml - RSS 2.0 feed

export BLOG_DB_PATH="/var/lib/devblog/blog.db"
export BLOG_API_KEY_PATH="/etc/devblog-api-key"
export BLOG_BASE_URL="https://steponnopets.net/devblog"
export BLOG_TITLE="Devblog"
export BLOG_DESCRIPTION="Internal development notes and technical logs"

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

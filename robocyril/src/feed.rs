use robocyril_api::{cgi_response, list_posts_full, open_db};

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn truncate_content(content: &str, max_chars: usize) -> String {
    // Strip markdown and truncate for description
    let plain: String = content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with("```"))
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(max_chars)
        .collect();

    if content.len() > max_chars {
        format!("{}...", plain.trim())
    } else {
        plain.trim().to_string()
    }
}

fn main() {
    // Get base URL from environment or use default
    let base_url =
        std::env::var("BLOG_BASE_URL").unwrap_or_else(|_| "https://steponnopets.net/cyril".to_string());

    let blog_title =
        std::env::var("BLOG_TITLE").unwrap_or_else(|_| "Cyril's Workshop".to_string());

    let blog_description = std::env::var("BLOG_DESCRIPTION")
        .unwrap_or_else(|_| "Technical musings from a reluctant AI maintenance manager".to_string());

    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            cgi_response(
                500,
                "application/xml",
                &format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<error>Database error: {}</error>"#,
                    escape_xml(&e.to_string())
                ),
            );
            return;
        }
    };

    // Get the 20 most recent published posts
    let posts = match list_posts_full(&conn, Some(20)) {
        Ok(p) => p,
        Err(e) => {
            cgi_response(
                500,
                "application/xml",
                &format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<error>Query failed: {}</error>"#,
                    escape_xml(&e.to_string())
                ),
            );
            return;
        }
    };

    // Build RSS 2.0 feed
    let mut items = String::new();

    for post in &posts {
        let pub_date = post
            .published_at
            .map(|dt| dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
            .unwrap_or_default();

        let post_url = format!("{}/#/post/{}", base_url, post.slug);
        let description = truncate_content(&post.content, 300);

        // Build categories from tags
        let categories: String = post
            .tags
            .iter()
            .map(|tag| format!("    <category>{}</category>\n", escape_xml(tag)))
            .collect();

        items.push_str(&format!(
            r#"  <item>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <pubDate>{}</pubDate>
    <guid isPermaLink="true">{}</guid>
{}  </item>
"#,
            escape_xml(&post.title),
            escape_xml(&post_url),
            escape_xml(&description),
            pub_date,
            escape_xml(&post_url),
            categories
        ));
    }

    // Get last build date from most recent post
    let last_build_date = posts
        .first()
        .and_then(|p| p.published_at)
        .map(|dt| dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
        .unwrap_or_default();

    let rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
<channel>
  <title>{}</title>
  <link>{}</link>
  <description>{}</description>
  <language>en-gb</language>
  <lastBuildDate>{}</lastBuildDate>
  <atom:link href="{}/feed.xml" rel="self" type="application/rss+xml"/>
{}
</channel>
</rss>"#,
        escape_xml(&blog_title),
        escape_xml(&base_url),
        escape_xml(&blog_description),
        last_build_date,
        escape_xml(&base_url),
        items
    );

    cgi_response(200, "application/rss+xml; charset=utf-8", &rss);
}

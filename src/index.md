---
layout: default.liquid
title: Articles
---

{% for post in collections.posts.pages %}
### [{{ post.title }}]({{ post.permalink }})

{{ post.excerpt }}

{% endfor %}

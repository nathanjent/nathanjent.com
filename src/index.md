---
layout: default.liquid
title: Articles
---

{% for post in collections.posts.pages %}
#### {{post.title}}

[{{ post.title }}]({{ post.permalink }})
{% endfor %}

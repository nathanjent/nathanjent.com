---
layout: default.liquid
---
## News

{% for page in collections.posts.pages %}
#### {{ post.title }}

{{ post.excerpt }}

[{{ post.title }}]({{ post.permalink }})
{% endfor %}

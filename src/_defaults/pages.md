---
layout: default.liquid
---
## Blog!

{% for page in collections.posts.pages %}
#### {{ post.title }}

{{ post.excerpt }}

[{{ post.title }}]({{ post.permalink }})
{% endfor %}

select st.id as id
from semantic_tag as st
where name in (<SEMANTIC_TAGS_PLACEHOLDERS>)

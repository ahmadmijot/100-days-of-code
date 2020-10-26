# Notes on Python

## Notes on String and Dictionary
> A researcher has gathered thousands of news articles. But she wants to focus her attention on articles including a specific word.
>Complete the function below to help her filter her list of articles.
>Your function should meet the following criteria:
> 1. Do not include documents where the keyword string shows up only as a part of a larger word. 
>For example, if she were looking for the keyword “closed”, you would not include the string “enclosed.”
> 2. She does not want you to distinguish upper case from lower case letters. 
>So the phrase “Closed the case.” would be included when the keyword is “closed”
> 3. Do not let periods or commas affect what is matched. “It is closed.” would be included when the keyword is “closed”.
> But you can assume there are no other types of punctuation.

```python
#This only search for char not str!
def word_search(documents, keyword):
    # list to hold the indices of matching documents
    indices = [] 
    # Iterate through the indices (i) and elements (doc) of documents
    for i, doc in enumerate(documents):
        # Split the string doc into a list of words (according to whitespace)
        tokens = doc.split()
        # Make a transformed list where we 'normalize' each word to facilitate matching.
        # Periods and commas are removed from the end of each word, and it's set to all lowercase.
        normalized = [token.rstrip('.,').lower() for token in tokens]
        # Is there a match? If so, update the list of matching indices.
        if keyword.lower() in normalized:
            indices.append(i)
    return indices
```

> Multiple keyword

```python
#use above function, so still search for char not str
def multi_word_search(documents, keywords):
    keyword_to_indices = {}
    for keyword in keywords:
        keyword_to_indices[keyword] = word_search(documents, keyword)
    return keyword_to_indices
```

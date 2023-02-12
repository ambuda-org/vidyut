Integration tests
=================

The integration tests in this directory are a complement to our full end-to-end
test suite.

Our full test suite examines millions of words, which is too much for us to
feasibly inspect manually. These tests help verify that the system's outputs
have not changed, but they are also slower and might contain bugs of their own.

In comparison, these integration tests examine hundreds of words and are
created manually. Although these tests are much less thorough, we can run them
quickly and have confidence that the underlying tests have been examined
carefuly.

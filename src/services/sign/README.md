# Signing logic

1. new_sign_request -> Request a new signature for the message for a specific key
2. list_sign_requests -> Returns a list of signing requests
3. round1 -> output of round 1 of signing from individual key shares
4. round2 -> output of round2 of signing from individual key shares
At this point, a joint signature can be produced.
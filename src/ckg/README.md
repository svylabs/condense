# Coordinated Key Generation logic

It uses the 3 round distributed key generation logic using a centralized coordinator.

# Key Generation flow:

new_ckg_request -> Someone issues a new key generation request
list_ckg_requests -> List all key generation requests (or use push notifications)
submit_session_key -> Each client submits a session key that will be used by others to securely transfer data through coordinator for the ckg request.
clients run round1 and shares the results with coordinator
followed by round2 and shares the results with coordinator
followed by round3 and shares the results with coordinator
At this point, the key is available to be used.


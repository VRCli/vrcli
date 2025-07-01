// Mock response helpers for testing VRChat API interactions
use serde_json::json;

/// Create a mock user response for testing
pub fn mock_user_response() -> serde_json::Value {
    json!({
        "id": "usr_12345678-1234-1234-1234-123456789012",
        "username": "testuser",
        "displayName": "Test User",
        "userIcon": "",
        "bio": "This is a test user",
        "bioLinks": [],
        "profilePicOverride": "",
        "statusDescription": "",
        "currentAvatarImageUrl": "",
        "currentAvatarThumbnailImageUrl": "",
        "state": "online",
        "status": "active",
        "statusDescription": "Testing",
        "tags": ["system_trust_basic"],
        "developerType": "none",
        "last_login": "2024-01-01T00:00:00Z",
        "last_platform": "standalonewindows",
        "allowAvatarCopying": true,
        "friendKey": "friend_key_123",
        "friendRequestStatus": "none",
        "isFriend": false,
        "location": "private",
        "worldId": "",
        "instanceId": ""
    })
}

/// Create a mock friend response for testing
pub fn mock_friend_response() -> serde_json::Value {
    json!({
        "id": "usr_12345678-1234-1234-1234-123456789012",
        "username": "frienduser",
        "displayName": "Friend User",
        "userIcon": "",
        "bio": "This is a friend user",
        "profilePicOverride": "",
        "statusDescription": "Playing VRChat",
        "currentAvatarImageUrl": "",
        "currentAvatarThumbnailImageUrl": "",
        "state": "online",
        "status": "join me",
        "tags": ["system_trust_known"],
        "developerType": "none",
        "last_login": "2024-01-01T12:00:00Z",
        "last_platform": "android",
        "allowAvatarCopying": false,
        "friendKey": "friend_key_456",
        "isFriend": true,
        "location": "wrld_12345678-1234-1234-1234-123456789012:12345~hidden(usr_aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa)~region(us)~nonce(1234567890abcdef1234567890abcdef)",
        "worldId": "wrld_12345678-1234-1234-1234-123456789012",
        "instanceId": "12345~hidden(usr_aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa)~region(us)~nonce(1234567890abcdef1234567890abcdef)"
    })
}

/// Create a mock world response for testing
pub fn mock_world_response() -> serde_json::Value {
    json!({
        "id": "wrld_12345678-1234-1234-1234-123456789012",
        "name": "Test World",
        "description": "A test world for unit testing",
        "featured": false,
        "authorId": "usr_12345678-1234-1234-1234-123456789012",
        "authorName": "Test Author",
        "capacity": 16,
        "recommendedCapacity": 8,
        "tags": ["test", "world"],
        "releaseStatus": "public",
        "imageUrl": "",
        "thumbnailImageUrl": "",
        "assetUrl": "",
        "assetUrlObject": {},
        "pluginUrlObject": {},
        "unityPackageUrlObject": {},
        "namespace": "",
        "version": 1,
        "organization": "vrchat",
        "previewYoutubeId": "",
        "favorites": 42,
        "visits": 1337,
        "popularity": 100,
        "heat": 50,
        "publicationDate": "2024-01-01T00:00:00Z",
        "labsPublicationDate": "2024-01-01T00:00:00Z",
        "instances": [],
        "publicOccupants": 0,
        "privateOccupants": 0,
        "occupants": 0,
        "udonProducts": []
    })
}

/// Create a mock authentication error response
pub fn mock_auth_error_response() -> serde_json::Value {
    json!({
        "error": {
            "message": "\"Invalid Username/Email or Password\"",
            "status_code": 401
        }
    })
}

/// Create a mock rate limit error response
pub fn mock_rate_limit_error() -> serde_json::Value {
    json!({
        "error": {
            "message": "\"Too Many Requests\"",
            "status_code": 429
        }
    })
}

# Auth

A implementation of OAuth 2.0 (2.1) to get a better understanding of how it works.

This includes a idenity-server for user management.

Even though [OAuth 2.1](https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1) is currently work in progress, I will try to follow the current draft and make changes if necessary.

Since OAuth 2.1 is compatible with [OAuth 2.0](https://www.rfc-editor.org/info/rfc6749/) and includes the [Best current security practice](https://datatracker.ietf.org/doc/html/rfc9700) and more updates I'll be looking into them as well.

## Links

- [Video](https://www.youtube.com/watch?v=996OiexHze0)

## TODO

- Scopes
- Token introspection - [RFC7662](https://www.rfc-editor.org/info/rfc7662)
- Auth server metadata - [RFC8414](https://www.rfc-editor.org/info/rfc8414)
- Dynamic Client Registration / Management - [RFC7591](https://www.rfc-editor.org/info/rfc7591) and [RFC7592](https://www.rfc-editor.org/info/rfc7592)
- TLS
- proof-of-possession - DPoP [RFC9449](https://www.rfc-editor.org/info/rfc9449) or mTLS [RFC8705](https://www.rfc-editor.org/info/rfc8705)

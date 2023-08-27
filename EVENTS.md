# Event Info

### Info verified

```json
{
  "crate": "$CRATE",
  "version": "$VERSION",
  "event_version": "1",
  "event": "info_verified",
  "data": "true"
}
```

### Hashes verified

```json
{
  "crate": "$CRATE",
  "version": "$VERSION",
  "event_version": "1",
  "event": "hashes_verified",
  "data": "true"
}
```

### Target

```json
{
  "crate": "$CRATE",
  "version": "$VERSION",
  "event_version": "1",
  "event": "target",
  "data": "$TARGET"
}
```

### Binary Installed

```json
{
  "crate": "$CRATE",
  "version": "$VERSION",
  "event_version": "1",
  "event": "bin_installed",
  "data": "$PATH"
}
```

### Installed

```json
{
  "crate": "$CRATE",
  "version": "$VERSION",
  "event_version": "1",
  "event": "installed",
  "data": "$CRATE@$VERSION"
}
```

### Latest Version (--get-latest)

```json
{
  "crate": "$CRATE",
  "version": "$LATEST_VERSION",
  "event_version": "1",
  "event": "latest_version",
  "data": "$LATEST_VERSION"
}
```
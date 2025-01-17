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

### Wrote Report

```json
{
  "crate": "$CRATE",
  "version": "$LATEST_VERSION",
  "event_version": "1",
  "event": "wrote_report",
  "data": "$REPORT_TYPE"
}
```

### Print License (license_event) {#print-license}

```json
{
  "crate": "$CRATE",
  "version": "$LATEST_VERSION",
  "event_version": "1",
  "event": "print_license",
  "data": "$TEXT"
}
```

### Print Deps (deps_event) {#print-deps}

```json
{
  "crate": "$CRATE",
  "version": "$LATEST_VERSION",
  "event_version": "1",
  "event": "print_deps",
  "data": "$TEXT"
}
```

### Print Audit (audit_event) {#print-audit}

```json
{
  "crate": "$CRATE",
  "version": "$LATEST_VERSION",
  "event_version": "1",
  "event": "print_audit",
  "data": "$TEXT"
}
```

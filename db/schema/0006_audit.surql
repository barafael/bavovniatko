-- 0006_audit: keep the previous state of audited records on every UPDATE/DELETE in `revision`.
-- (Point-in-time reads are also available via SurrealKV `VERSION`, but that is storage-bound;
--  `revision` survives dumps and engine changes.)

DEFINE EVENT IF NOT EXISTS audit ON source      WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON claim       WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON observation WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON passage     WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON supports    WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON weakens     WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON contradicts WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON refines     WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON supersedes  WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());
DEFINE EVENT IF NOT EXISTS audit ON counters    WHEN $event IN ["UPDATE", "DELETE"] THEN (CREATE revision SET target = $before.id, event = $event, before = $before, at = time::now());

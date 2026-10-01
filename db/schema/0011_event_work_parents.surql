-- 0011_event_work_parents: hierarchies for events (phases of an operation) and works (series, editions).
DEFINE FIELD IF NOT EXISTS parent ON event TYPE option<record<event>>;
DEFINE FIELD IF NOT EXISTS parent ON work  TYPE option<record<work>>;

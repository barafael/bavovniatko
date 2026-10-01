-- severity: warn
-- A place whose centroid lies outside its parent's polygon is probably mis-geocoded (e.g. a same-named hamlet elsewhere).
SELECT VALUE id FROM place WHERE centroid != NONE AND parent != NONE AND parent.geometry != NONE
  AND geo::area(parent.geometry) > 0 AND !(centroid INSIDE parent.geometry);

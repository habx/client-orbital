# Viewer

## Orbital manifest updates needed (from version `2.0.0`)

- Added field `meta.path` that defines the location of the folder containing all images assets
- Added fields `lots.typology`
- Reorder the fields given the jq transformation `'{ meta: .meta, lots: .lots, views: .views }'`

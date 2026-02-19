# APS Reagent Tables Report

This report lists the database tables that need to be created in APS to support a true reagent model (beyond the current fallback to `physical_asset_models`).

## Required Table

### `reagent_models`

This is the minimum required table for reagent-specific templates.

```sql
-- Catalog of reagent models with chemistry metadata.
CREATE TABLE reagent_models (
    -- Stable model identifier inherited from physical_asset_models.
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    -- Reagent purity percentage.
    purity REAL NOT NULL CHECK (purity > 0.0 AND purity <= 100.0),
    -- Canonical CAS registry code.
    cas_code CAS NOT NULL UNIQUE,
    -- Molecular formula.
    molecular_formula MolecularFormula NOT NULL
);

-- Register table for APS table-name lookup.
INSERT INTO table_names (id) VALUES ('reagent_models') ON CONFLICT DO NOTHING;
```

## APS Schema Prerequisites (Non-table)

- `CAS` SQL type available in APS migrations/extensions.
- `MolecularFormula` SQL type available in APS migrations/extensions.

## Optional Next Tables (Only if Needed)

If APS needs full reagent lifecycle tracking (commercial catalogs, lots, or instantiated assets), additional tables would be needed, similar to PPE/commercial product patterns:

- `commercial_reagent_models`
- `commercial_reagent_lots`
- `reagents` (physical assets)

These are not required for the current templates, which only create reagent models.

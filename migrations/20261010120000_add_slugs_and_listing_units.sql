ALTER TABLE properties
    ADD CONSTRAINT fk_properties_landlord
    FOREIGN KEY (landlord_id) REFERENCES users (id) NOT VALID;

ALTER TABLE units
    ADD COLUMN slug VARCHAR(200);

WITH slugified AS (
    SELECT
        id,
        property_id,
        COALESCE(
            NULLIF(trim(BOTH '-' FROM lower(regexp_replace(unit_number, '[^a-zA-Z0-9]+', '-', 'g'))), ''),
            'unit-' || left(id::text, 8)
        ) AS base_slug
    FROM units
),
duplicate_slugs AS (
    SELECT property_id, base_slug
    FROM slugified
    GROUP BY property_id, base_slug
    HAVING count(*) > 1
)
UPDATE units AS unit
SET slug = CASE
    WHEN duplicate.property_id IS NULL THEN slugified.base_slug
    ELSE slugified.base_slug || '-' || left(unit.id::text, 8)
END
FROM slugified
LEFT JOIN duplicate_slugs AS duplicate
    ON duplicate.property_id = slugified.property_id
    AND duplicate.base_slug = slugified.base_slug
WHERE unit.id = slugified.id;

ALTER TABLE units
    ALTER COLUMN slug SET NOT NULL;

CREATE UNIQUE INDEX idx_units_property_slug ON units (property_id, slug);

ALTER TABLE listings
    ADD COLUMN unit_id UUID REFERENCES units (id) ON DELETE SET NULL;

CREATE INDEX idx_listings_status ON listings (status);
CREATE INDEX idx_listings_property_id ON listings (property_id);
CREATE UNIQUE INDEX idx_users_email_lower ON users (lower(email));
